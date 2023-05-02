use serde::Deserialize;
use serde_json::{json, Value};

use super::super::{utils::de_int_float_to_i64, Error, RequestBuilder};

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

#[derive(Deserialize, Default, Debug)]
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

pub async fn reboot(rpc: RequestBuilder) -> Result<bool, Error> {
    Ok(rpc
        .method("magicBox.reboot")
        .send::<Value>()
        .await?
        .result())
}

pub async fn need_reboot(rpc: RequestBuilder) -> Result<bool, Error> {
    rpc.method("magicBox.needReboot")
        .send::<NeedReboot>()
        .await?
        .params_map(|p, _| p.need_reboot != 0)
}

pub async fn get_serial_no(rpc: RequestBuilder) -> Result<String, Error> {
    rpc.method("magicBox.getSerialNo")
        .send::<GetSerialNo>()
        .await?
        .params_map(|p, _| p.sn)
}

pub async fn get_device_type(rpc: RequestBuilder) -> Result<String, Error> {
    rpc.method("magicBox.getDeviceType")
        .send::<GetType>()
        .await?
        .params_map(|p, _| p.r#type)
}

pub async fn get_memory_info(rpc: RequestBuilder) -> Result<GetMemoryInfo, Error> {
    rpc.method("magicBox.getMemoryInfo")
        .send::<GetMemoryInfo>()
        .await?
        .params_map(|p, _| p)
}

pub async fn get_cpu_usage(rpc: RequestBuilder) -> Result<i32, Error> {
    rpc.method("magicBox.getCPUUsage")
        .params(json!({
            "index": 0,
        }))
        .send::<GetCPUUsage>()
        .await?
        .params_map(|p, _| p.usage)
}

pub async fn get_device_class(rpc: RequestBuilder) -> Result<String, Error> {
    rpc.method("magicBox.getDeviceClass")
        .send::<GetType>()
        .await?
        .params_map(|p, _| p.r#type)
}

pub async fn get_process_info(rpc: RequestBuilder) -> Result<String, Error> {
    rpc.method("magicBox.getProcessInfo")
        .send::<GetProcessInfo>()
        .await?
        .params_map(|p, _| p.info)
}

pub async fn get_hardware_version(rpc: RequestBuilder) -> Result<String, Error> {
    rpc.method("magicBox.getHardwareVersion")
        .send::<GetHardwareVersion>()
        .await?
        .params_map(|p, _| p.version)
}

pub async fn get_vendor(rpc: RequestBuilder) -> Result<String, Error> {
    rpc.method("magicBox.getVendor")
        .send::<GetVendor>()
        .await?
        .params_map(|p, _| p.vendor)
}

pub async fn get_software_version(rpc: RequestBuilder) -> Result<GetSoftwareVersion, Error> {
    rpc.method("magicBox.getSoftwareVersion")
        .send::<GetSoftwareVersionInternal>()
        .await?
        .params_map(|p, _| p.version)
}

pub async fn get_market_area(rpc: RequestBuilder) -> Result<String, Error> {
    rpc.method("magicBox.getMarketArea")
        .send::<GetMarketArea>()
        .await?
        .params_map(|p, _| p.abroad_info)
}
