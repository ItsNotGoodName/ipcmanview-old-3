use chrono::{DateTime, Duration, Local, TimeZone, Utc};
use sqlx::SqliteConnection;

use anyhow::Result;

use crate::core::Camera;
use crate::db::{self, CameraScanResult};

pub struct ScanRange {
    start: DateTime<Utc>,
    end: DateTime<Utc>,
}

struct ScanRangeIterator {
    start: DateTime<Utc>,
    cursor: DateTime<Utc>,
}

impl ScanRange {
    pub fn new(start: DateTime<Utc>, end: DateTime<Utc>) -> ScanRange {
        ScanRange { start, end }
    }

    fn into_iter(self) -> ScanRangeIterator {
        ScanRangeIterator {
            start: self.start,
            cursor: self.end,
        }
    }
}

impl Iterator for ScanRangeIterator {
    type Item = ScanRange;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor == self.start {
            return None;
        }

        let end = self.cursor;
        let start = {
            let maybe_start = self.cursor - Duration::days(30);

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

async fn run(
    pool: &mut SqliteConnection,
    cam: &Camera,
    range: ScanRange,
) -> Result<CameraScanResult> {
    let task = db::camera_tasks_start(pool, &cam).await?;

    let mut res = CameraScanResult::default();
    for range in range.into_iter() {
        dbg!(range.start);
        dbg!(range.end);
        res += db::camera_scan(pool, cam, range.start, range.end).await?;
    }

    if let Err(err) = db::camera_tasks_end(pool, &cam, task).await {
        eprint!("scan::run: {}", err);
    };

    Ok(res)
}

pub async fn full(pool: &mut SqliteConnection, cam: &Camera) -> Result<CameraScanResult> {
    let start_range = Local
        .with_ymd_and_hms(2010, 1, 1, 0, 0, 0)
        .unwrap()
        .with_timezone(&Utc);
    let end_range = Utc::now();
    let range = ScanRange::new(start_range, end_range);

    run(pool, cam, range).await
}

pub async fn range(
    pool: &mut SqliteConnection,
    cam: &Camera,
    range: ScanRange,
) -> Result<CameraScanResult> {
    run(pool, cam, range).await
}
