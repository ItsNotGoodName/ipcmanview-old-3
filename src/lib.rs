pub mod rpc;

use rpc::global;
use rpc::license;
use rpc::magicbox;
use rpc::rpclogin;

pub fn man_print(man: &mut rpclogin::Manager) {
    println!("Cookie: {}", man.client.cookie());
    println!(
        "global.getCurrentTime: {:?}",
        man.rpc(|rpc| global::get_current_time(rpc)),
    );
    println!(
        "magicBox.needReboot: {:?}",
        man.rpc(|rpc| magicbox::need_reboot(rpc)),
    );
    println!(
        "magicBox.getSerialNo: {:?}",
        man.rpc(|rpc| magicbox::get_serial_no(rpc)),
    );
    println!(
        "magicBox.getDeviceType: {:?}",
        man.rpc(|rpc| magicbox::get_device_type(rpc)),
    );
    println!(
        "magicBox.getMemoryInfo: {:?}",
        man.rpc(|rpc| magicbox::get_memory_info(rpc)),
    );
    println!(
        "magicBox.getCPUUsage: {:?}",
        man.rpc(|rpc| magicbox::get_cpu_usage(rpc)),
    );
    println!(
        "magicBox.getDeviceClass: {:?}",
        man.rpc(|rpc| magicbox::get_device_class(rpc)),
    );
    println!(
        "magicBox.getProcessInfo: {:?}",
        man.rpc(|rpc| magicbox::get_process_info(rpc)),
    );
    println!(
        "magicBox.getHardwareVersion: {:?}",
        man.rpc(|rpc| magicbox::get_hardware_version(rpc)),
    );
    println!(
        "magicBox.getVendor: {:?}",
        man.rpc(|rpc| magicbox::get_vendor(rpc)),
    );
    println!(
        "magicBox.getSoftwareVersion: {:?}",
        man.rpc(|rpc| magicbox::get_software_version(rpc)),
    );
    println!(
        "magicBox.getMarketArea: {:?}",
        man.rpc(|rpc| magicbox::get_market_area(rpc)),
    );
    println!(
        "License.getLicenseInfo: {:?}",
        man.rpc(|rpc| license::get_license_info(rpc)),
    );
}
