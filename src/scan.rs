use std::time::Instant;

use anyhow::{bail, Result};
use chrono::{DateTime, Duration, Local, TimeZone, Utc};

use crate::models::ScanCompleted;

pub struct Scan {}

impl Scan {
    pub fn cursor() -> DateTime<Utc> {
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
    pub fn new(start: DateTime<Utc>, end: DateTime<Utc>) -> Result<Self> {
        if start > end {
            bail!("start date greater than end date: {start} > {end}")
        }
        if end > Utc::now() {
            bail!("end date is in the future: {end}")
        }
        if start < Scan::epoch() {
            bail!(
                "start date is before epoch: {start}<{epoch}",
                epoch = Scan::epoch()
            )
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

        let percent = ((((self.end - self.cursor).num_days()) as f64
            / ((self.end - self.start).num_days() as f64))
            * 10000.0)
            .round()
            / 100.0;
        let percent = if percent.is_nan() { 0.0 } else { percent };

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

impl From<ScanKindPending> for ScanKind {
    fn from(value: ScanKindPending) -> Self {
        match value {
            ScanKindPending::Full => ScanKind::Full,
            ScanKindPending::Cursor => ScanKind::Cursor,
            ScanKindPending::Manual(_) => ScanKind::Manual,
        }
    }
}

pub enum ScanKindPending {
    Full,
    Cursor,
    Manual(ScanRange),
}

impl ScanKindPending {
    pub fn range(&self) -> (DateTime<Utc>, DateTime<Utc>) {
        if let ScanKindPending::Manual(scan_range) = self {
            (scan_range.start, scan_range.end)
        } else {
            (Scan::epoch(), Scan::epoch())
        }
    }
}

pub struct ScanActor {
    pub camera_id: i64,
    pub range: ScanRange,
    pub kind: ScanKind,
    pub started_at: DateTime<Utc>,
    pub instant: Instant,
    pub error: String,
}

struct ScanActorBuilder {
    camera_id: i64,
    range: ScanRange,
    kind: ScanKind,
}

pub struct ScanCamera {
    pub id: i64,
    pub scan_cursor: DateTime<Utc>,
}

impl ScanActor {
    fn new(builder: ScanActorBuilder) -> Self {
        ScanActor {
            camera_id: builder.camera_id,
            range: builder.range,
            kind: builder.kind,
            started_at: Utc::now(),
            instant: Instant::now(),
            error: "".to_string(),
        }
    }

    pub fn manual(camera_id: i64, range: ScanRange) -> Self {
        Self::new(ScanActorBuilder {
            camera_id,
            range,
            kind: ScanKind::Manual,
        })
    }

    pub fn full(camera_id: i64) -> Self {
        Self::new(ScanActorBuilder {
            camera_id,
            range: ScanRange {
                start: Scan::epoch(),
                end: Utc::now(),
            },
            kind: ScanKind::Full,
        })
    }

    pub fn cursor(scan_camera: ScanCamera) -> Self {
        Self::new(ScanActorBuilder {
            camera_id: scan_camera.id,
            range: ScanRange {
                start: scan_camera.scan_cursor,
                end: Utc::now(),
            },
            kind: ScanKind::Cursor,
        })
    }

    pub fn should_update_scan_cursor(&self) -> Option<DateTime<Utc>> {
        match self.kind {
            ScanKind::Full | ScanKind::Cursor => {
                let current = Scan::cursor();

                if self.range.end < current {
                    Some(self.range.end)
                } else {
                    Some(current)
                }
            }
            ScanKind::Manual => None,
        }
    }

    pub fn should_save(&self) -> bool {
        match self.kind {
            ScanKind::Full | ScanKind::Manual => true,
            ScanKind::Cursor => false,
        }
    }

    pub fn duration(&self) -> i64 {
        self.instant.elapsed().as_millis() as i64
    }

    pub fn success(&self) -> bool {
        self.error.is_empty()
    }

    pub fn can_retry(&self) -> bool {
        !self.error.is_empty()
    }
}

impl From<ScanCompleted> for ScanActor {
    fn from(value: ScanCompleted) -> Self {
        Self::new(ScanActorBuilder {
            camera_id: value.camera_id,
            range: ScanRange {
                start: value.range_start,
                end: value.range_cursor,
            },
            kind: value.kind,
        })
    }
}
