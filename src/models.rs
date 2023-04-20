use std::time::Instant;

use chrono::{DateTime, Duration, Local, TimeZone, Utc};

pub struct CameraCreate {
    pub ip: String,
    pub username: String,
    pub password: String,
}

pub struct CameraUpdate {
    pub id: i64,
    pub ip: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
}

pub struct Camera {
    pub id: i64,
    pub ip: String,
    pub username: String,
    pub password: String,
}

pub struct CameraScanCursor {
    pub id: i64,
    pub scan_cursor: DateTime<Utc>,
}

pub struct Scan {}

impl Scan {
    pub fn current_cursor() -> DateTime<Utc> {
        Utc::now() - Duration::hours(8)
    }

    pub fn epoch() -> DateTime<Utc> {
        Local
            .with_ymd_and_hms(2010, 1, 1, 0, 0, 0)
            .unwrap()
            .with_timezone(&Utc)
    }

    pub fn period() -> Duration {
        Duration::days(30)
    }
}

pub struct ScanRange {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

impl ScanRange {
    pub fn new(start: DateTime<Utc>, end: DateTime<Utc>) -> ScanRange {
        ScanRange { start, end }
    }

    pub fn iter(&self) -> ScanRangeIterator {
        ScanRangeIterator {
            start: self.start,
            cursor: self.end,
        }
    }

    pub fn scan_cursor(&self) -> DateTime<Utc> {
        let current = Scan::current_cursor();

        if self.end < current {
            self.end
        } else {
            current
        }
    }
}

pub struct ScanRangeIterator {
    start: DateTime<Utc>,
    cursor: DateTime<Utc>,
}

impl Iterator for ScanRangeIterator {
    type Item = ScanRange;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor == self.start {
            return None;
        }

        let end = self.cursor;
        let start = {
            let maybe_start = self.cursor - Scan::period();

            if maybe_start < self.start {
                self.cursor = self.start;
                self.cursor
            } else {
                self.cursor = maybe_start;
                maybe_start
            }
        };

        Some(ScanRange { start, end })
    }
}

#[derive(sqlx::Type)]
pub enum ScanKind {
    Full,
    Cursor,
    Manual,
}

pub struct ScanHandle {
    pub camera_id: i64,
    pub range: ScanRange,
    pub kind: ScanKind,
    pub started_at: DateTime<Utc>,
    pub instant: Instant,
}

impl ScanHandle {
    pub fn new(task: ScanTask) -> ScanHandle {
        ScanHandle {
            camera_id: task.camera_id,
            range: task.range,
            kind: task.kind,
            started_at: Utc::now(),
            instant: Instant::now(),
        }
    }

    pub fn should_update_scan_cursor(&self) -> bool {
        match self.kind {
            ScanKind::Full | ScanKind::Cursor => true,
            ScanKind::Manual => false,
        }
    }

    pub fn should_save(&self) -> bool {
        match self.kind {
            ScanKind::Full | ScanKind::Manual => true,
            ScanKind::Cursor => false,
        }
    }
}

pub struct ScanTask {
    pub camera_id: i64,
    pub range: ScanRange,
    pub kind: ScanKind,
}

pub struct ScanTaskBuilder {
    camera_id: i64,
}

impl ScanTaskBuilder {
    pub fn new(camera_id: i64) -> ScanTaskBuilder {
        ScanTaskBuilder { camera_id }
    }

    pub fn full(self) -> ScanTask {
        ScanTask {
            camera_id: self.camera_id,
            range: ScanRange {
                start: Scan::epoch(),
                end: Utc::now(),
            },
            kind: ScanKind::Full,
        }
    }

    pub fn manual(self, range: ScanRange) -> ScanTask {
        ScanTask {
            camera_id: self.camera_id,
            range,
            kind: ScanKind::Manual,
        }
    }
}

impl CameraScanCursor {
    pub fn to_scan_task(self) -> ScanTask {
        ScanTask {
            camera_id: self.id,
            range: ScanRange {
                start: self.scan_cursor,
                end: Utc::now(),
            },
            kind: ScanKind::Cursor,
        }
    }
}
