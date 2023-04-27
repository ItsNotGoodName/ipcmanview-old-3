use std::time::Instant;

use anyhow::{bail, Result};
use chrono::{DateTime, Duration, Local, TimeZone, Utc};

pub struct Scan {}

impl Scan {
    pub fn current_cursor() -> DateTime<Utc> {
        Utc::now() - Duration::hours(8)
    }

    pub fn epoch() -> DateTime<Utc> {
        // TODO: make this at compile time
        Local
            .with_ymd_and_hms(2010, 1, 1, 0, 0, 0)
            .unwrap()
            .with_timezone(&Utc)
    }

    pub fn period() -> Duration {
        // TODO: make this at compile time
        Duration::days(30)
    }
}

pub struct ScanRange {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

impl ScanRange {
    pub fn new(start: DateTime<Utc>, end: DateTime<Utc>) -> Result<ScanRange> {
        if start < end {
            bail!("start date less than end date: {start} < {end}")
        }
        if end > Utc::now() {
            bail!("end date is in future: {end}")
        }
        if start < Scan::epoch() {
            bail!("start date is before epoch: {start}")
        }
        Ok(ScanRange { start, end })
    }

    pub fn iter(&self) -> ScanRangeIterator {
        ScanRangeIterator {
            start: self.start,
            cursor: self.end,
            end: self.end,
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
    end: DateTime<Utc>,
}

impl Iterator for ScanRangeIterator {
    type Item = (ScanRange, f64);

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

        let percent = (((self.end - self.cursor).num_days()) as f64
            / ((self.end - self.start).num_days() as f64))
            * 100.0;

        Some((ScanRange { start, end }, percent))
    }
}

#[derive(sqlx::Type, serde::Serialize, Debug)]
#[sqlx(rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum ScanKind {
    Full,
    Cursor,
    Manual,
}

#[derive(sqlx::Type, Debug)]
#[sqlx(rename_all = "snake_case")]
pub enum ScanKindPending {
    Full,
    Cursor,
}

pub struct ScanHandle {
    pub camera_id: i64,
    pub range: ScanRange,
    pub kind: ScanKind,
    pub started_at: DateTime<Utc>,
    pub instant: Instant,
    pub error: Option<String>,
}

struct ScanTask {
    camera_id: i64,
    range: ScanRange,
    kind: ScanKind,
}

pub struct ScanCamera {
    pub id: i64,
    pub scan_cursor: DateTime<Utc>,
}

impl ScanHandle {
    fn new(builder: ScanTask) -> ScanHandle {
        ScanHandle {
            camera_id: builder.camera_id,
            range: builder.range,
            kind: builder.kind,
            started_at: Utc::now(),
            instant: Instant::now(),
            error: None,
        }
    }

    pub fn manual(camera_id: i64, range: ScanRange) -> ScanHandle {
        Self::new(ScanTask {
            camera_id,
            range,
            kind: ScanKind::Manual,
        })
    }

    pub fn full(camera_id: i64) -> ScanHandle {
        Self::new(ScanTask {
            camera_id,
            range: ScanRange {
                start: Scan::epoch(),
                end: Utc::now(),
            },
            kind: ScanKind::Full,
        })
    }

    pub fn cursor(scan_camera: ScanCamera) -> ScanHandle {
        Self::new(ScanTask {
            camera_id: scan_camera.id,
            range: ScanRange {
                start: scan_camera.scan_cursor,
                end: Utc::now(),
            },
            kind: ScanKind::Cursor,
        })
    }

    pub fn with_error(mut self, error: String) -> Self {
        self.error = Some(error);
        self
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
