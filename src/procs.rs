use sqlx::SqliteConnection;

use anyhow::Result;

use crate::core::CameraManager;
use crate::db::CameraScanResult;
use crate::models::{ScanRange, ScanTask};

async fn scan_range_run(
    pool: &mut SqliteConnection,
    man: &CameraManager,
    range: &ScanRange,
) -> Result<CameraScanResult> {
    let mut res = CameraScanResult::default();
    for range in range.iter() {
        dbg!(range.start);
        dbg!(range.end);
        res += man.scan_files(pool, range.start, range.end).await?;
        dbg!(&res);
    }

    Ok(res)
}

pub async fn scan_task_run(
    pool: &mut SqliteConnection,
    man: &CameraManager,
    task: ScanTask,
) -> Result<CameraScanResult> {
    let task = task.start(pool).await?;
    let res = scan_range_run(pool, man, &task.range).await;
    if let Err(err) = task.end(pool).await {
        dbg!(err);
    };

    res
}
