pub mod rpc;

use rpc::global;
use rpc::license;
use rpc::magicbox;
use rpc::rpclogin;

pub fn require_env(name: &str) -> Result<String, String> {
    std::env::var(name).map_err(|_| format!("{} not set", name))
}

pub async fn man_print(man: &mut rpclogin::Manager) {
    println!("Keep Alive: {:?}", man.keep_alive_or_login().await);
    println!("Cookie: {}", man.client.cookie());
    println!(
        "global.getCurrentTime: {:?}",
        global::get_current_time(man.client.rpc()).await,
    );
    println!(
        "magicBox.needReboot: {:?}",
        magicbox::need_reboot(man.client.rpc()).await,
    );
    println!(
        "magicBox.getSerialNo: {:?}",
        magicbox::get_serial_no(man.client.rpc()).await,
    );
    println!(
        "magicBox.getDeviceType: {:?}",
        magicbox::get_device_type(man.client.rpc()).await,
    );
    println!(
        "magicBox.getMemoryInfo: {:?}",
        magicbox::get_memory_info(man.client.rpc()).await,
    );
    println!(
        "magicBox.getCPUUsage: {:?}",
        magicbox::get_cpu_usage(man.client.rpc()).await,
    );
    println!(
        "magicBox.getDeviceClass: {:?}",
        magicbox::get_device_class(man.client.rpc()).await,
    );
    println!(
        "magicBox.getProcessInfo: {:?}",
        magicbox::get_process_info(man.client.rpc()).await,
    );
    println!(
        "magicBox.getHardwareVersion: {:?}",
        magicbox::get_hardware_version(man.client.rpc()).await,
    );
    println!(
        "magicBox.getVendor: {:?}",
        magicbox::get_vendor(man.client.rpc()).await,
    );
    println!(
        "magicBox.getSoftwareVersion: {:?}",
        magicbox::get_software_version(man.client.rpc()).await,
    );
    println!(
        "magicBox.getMarketArea: {:?}",
        magicbox::get_market_area(man.client.rpc()).await,
    );
    println!(
        "License.getLicenseInfo: {:?}",
        license::get_license_info(man.client.rpc()).await,
    );
}

pub mod core;
pub mod db;
pub mod scan;
