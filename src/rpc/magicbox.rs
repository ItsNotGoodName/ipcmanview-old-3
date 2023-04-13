use serde::Deserialize;
use serde_json::{json, Value};

use super::utils::de_int_float_to_i64;
use super::{Error, RequestBuilder};

#[derive(Deserialize, Debug)]
struct NeedReboot {
    #[serde(rename = "needReboot")]
    need_reboot: i32,
}

#[derive(Deserialize, Debug)]
struct GetSerialNo {
    sn: String,
}

#[derive(Deserialize, Debug)]
struct GetType {
    #[serde(rename = "type")]
    r#type: String,
}

#[derive(Deserialize, Debug)]
pub struct GetMemoryInfo {
    #[serde(deserialize_with = "de_int_float_to_i64")]
    pub free: i64,
    #[serde(deserialize_with = "de_int_float_to_i64")]
    pub total: i64,
}

#[derive(Deserialize, Debug)]
struct GetCPUUsage {
    usage: i32,
}

#[derive(Deserialize, Debug)]
struct GetProcessInfo {
    info: String,
}

#[derive(Deserialize, Debug)]
struct GetHardwareVersion {
    version: String,
}

#[derive(Deserialize, Debug)]
struct GetVendor {
    #[serde(rename = "Vendor")]
    vendor: String,
}

#[derive(Deserialize, Debug)]
pub struct GetSoftwareVersion {
    #[serde(default, rename = "Build")]
    pub build: String,
    #[serde(rename = "BuildDate")]
    pub build_date: String,
    #[serde(rename = "SecurityBaseLineVersion")]
    pub security_base_line_version: String,
    #[serde(rename = "Version")]
    pub version: String,
    #[serde(rename = "WebVersion")]
    pub web_version: String,
}

#[derive(Deserialize, Debug)]
struct GetSoftwareVersionInternal {
    version: GetSoftwareVersion,
}

#[derive(Deserialize, Debug)]
struct GetMarketArea {
    #[serde(rename = "AbroadInfo")]
    abroad_info: String,
}

pub fn reboot(rpc: RequestBuilder) -> Result<bool, Error> {
    rpc.method("magicBox.reboot").send::<Value>()?.result()
}

pub fn need_reboot(rpc: RequestBuilder) -> Result<bool, Error> {
    rpc.method("magicBox.needReboot")
        .send::<NeedReboot>()?
        .params_as(|p, _| p.need_reboot != 0)
}

pub fn get_serial_no(rpc: RequestBuilder) -> Result<String, Error> {
    rpc.method("magicBox.getSerialNo")
        .send::<GetSerialNo>()?
        .params_as(|p, _| p.sn)
}

pub fn get_device_type(rpc: RequestBuilder) -> Result<String, Error> {
    rpc.method("magicBox.getDeviceType")
        .send::<GetType>()?
        .params_as(|p, _| p.r#type)
}

pub fn get_memory_info(rpc: RequestBuilder) -> Result<GetMemoryInfo, Error> {
    rpc.method("magicBox.getMemoryInfo")
        .send::<GetMemoryInfo>()?
        .params_as(|p, _| p)
}

pub fn get_cpu_usage(rpc: RequestBuilder) -> Result<i32, Error> {
    rpc.method("magicBox.getCPUUsage")
        .params(json!({
            "index": 0,
        }))
        .send::<GetCPUUsage>()?
        .params_as(|p, _| p.usage)
}

pub fn get_device_class(rpc: RequestBuilder) -> Result<String, Error> {
    rpc.method("magicBox.getDeviceClass")
        .send::<GetType>()?
        .params_as(|p, _| p.r#type)
}

pub fn get_process_info(rpc: RequestBuilder) -> Result<String, Error> {
    rpc.method("magicBox.getProcessInfo")
        .send::<GetProcessInfo>()?
        .params_as(|p, _| p.info)
}

pub fn get_hardware_version(rpc: RequestBuilder) -> Result<String, Error> {
    rpc.method("magicBox.getHardwareVersion")
        .send::<GetHardwareVersion>()?
        .params_as(|p, _| p.version)
}

pub fn get_vendor(rpc: RequestBuilder) -> Result<String, Error> {
    rpc.method("magicBox.getVendor")
        .send::<GetVendor>()?
        .params_as(|p, _| p.vendor)
}

pub fn get_software_version(rpc: RequestBuilder) -> Result<GetSoftwareVersion, Error> {
    rpc.method("magicBox.getSoftwareVersion")
        .send::<GetSoftwareVersionInternal>()?
        .params_as(|p, _| p.version)
}

pub fn get_market_area(rpc: RequestBuilder) -> Result<String, Error> {
    rpc.method("magicBox.getMarketArea")
        .send::<GetMarketArea>()?
        .params_as(|p, _| p.abroad_info)
}
