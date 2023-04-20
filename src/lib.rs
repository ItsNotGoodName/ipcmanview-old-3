pub mod rpc;

use anyhow::{Context, Result};

use rpc::global;
use rpc::license;
use rpc::magicbox;

pub fn require_env(name: &str) -> Result<String> {
    std::env::var(name).with_context(|| format!("{} not set", name))
}

pub async fn client_print(client: ipc::IpcManager) -> Result<(), rpc::Error> {
    println!(
        "global.getCurrentTime: {:?}",
        global::get_current_time(client.rpc().await?).await,
    );
    println!(
        "magicBox.needReboot: {:?}",
        magicbox::need_reboot(client.rpc().await?).await,
    );
    println!(
        "magicBox.getSerialNo: {:?}",
        magicbox::get_serial_no(client.rpc().await?).await,
    );
    println!(
        "magicBox.getDeviceType: {:?}",
        magicbox::get_device_type(client.rpc().await?).await,
    );
    println!(
        "magicBox.getMemoryInfo: {:?}",
        magicbox::get_memory_info(client.rpc().await?).await,
    );
    println!(
        "magicBox.getCPUUsage: {:?}",
        magicbox::get_cpu_usage(client.rpc().await?).await,
    );
    println!(
        "magicBox.getDeviceClass: {:?}",
        magicbox::get_device_class(client.rpc().await?).await,
    );
    println!(
        "magicBox.getProcessInfo: {:?}",
        magicbox::get_process_info(client.rpc().await?).await,
    );
    println!(
        "magicBox.getHardwareVersion: {:?}",
        magicbox::get_hardware_version(client.rpc().await?).await,
    );
    println!(
        "magicBox.getVendor: {:?}",
        magicbox::get_vendor(client.rpc().await?).await,
    );
    println!(
        "magicBox.getSoftwareVersion: {:?}",
        magicbox::get_software_version(client.rpc().await?).await,
    );
    println!(
        "magicBox.getMarketArea: {:?}",
        magicbox::get_market_area(client.rpc().await?).await,
    );
    println!(
        "License.getLicenseInfo: {:?}",
        license::get_license_info(client.rpc().await?).await,
    );

    Ok(())
}

pub mod db;
pub mod ipc;
pub mod models;
pub mod procs;
pub mod scan;

pub async fn camera_update(pool: &sqlx::SqlitePool, man: &ipc::IpcManager) -> Result<()> {
    ipc::IpcDetail::get(&man).await?.save(pool, man.id).await?;
    ipc::IpcSoftwareVersion::get(man)
        .await?
        .save(pool, man.id)
        .await?;
    Ok(())
}
