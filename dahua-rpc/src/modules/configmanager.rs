use std::collections::HashMap;

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{to_value, Value};

use crate::{Error, RequestBuilder};

#[derive(Deserialize, Debug)]
pub struct ConfigResponse<T> {
    pub table: T,
}

#[derive(Serialize, Debug)]
pub struct ConfigRequest {
    pub name: &'static str,
    #[serde(skip_serializing_if = "Self::is_zero")]
    pub channel: i32,
}

impl ConfigRequest {
    pub fn new(name: &'static str) -> Self {
        Self { name, channel: 0 }
    }

    fn is_zero(num: &i32) -> bool {
        *num == 0
    }
}

pub async fn get<T>(rpc: RequestBuilder, params: ConfigRequest) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    rpc.method("configManager.getConfig")
        .params(to_value(params).expect("could not serialize ConfigRequest"))
        .send::<ConfigResponse<T>>()
        .await?
        .params_map(|p, _| p.table)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct General {
    #[serde(rename = "LocalNo")]
    pub local_no: i32,
    #[serde(rename = "LockLoginEnable")]
    pub lock_login_enable: bool,
    #[serde(rename = "LockLoginTimes")]
    pub lock_login_times: i32,
    #[serde(rename = "LoginFailLockTime")]
    pub login_fail_lock_time: i32,
    #[serde(rename = "MachineName")]
    pub machine_name: String,
    #[serde(rename = "MaxOnlineTime")]
    pub max_online_time: i32,

    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

impl General {
    pub async fn get(rpc: RequestBuilder) -> Result<Self, Error> {
        get::<Self>(rpc, ConfigRequest::new("General")).await
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NTP {
    #[serde(rename = "Address")]
    pub address: String,
    #[serde(rename = "Enable")]
    pub enable: bool,
    #[serde(rename = "Port")]
    pub port: i32,
    #[serde(rename = "TimeZone")]
    pub time_zone: i32,
    #[serde(rename = "TimeZoneDesc")]
    pub time_zone_desc: String,
    #[serde(rename = "UpdatePeriod")]
    pub update_period: i32,

    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

impl NTP {
    pub async fn get(rpc: RequestBuilder) -> Result<Self, Error> {
        get::<Self>(rpc, ConfigRequest::new("NTP")).await
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VideoInMode(Vec<VideoInModeConfig>);

#[derive(Serialize, Deserialize, Debug)]
pub struct VideoInModeConfig {
    #[serde(rename = "Config")]
    pub config: Vec<i32>,
    #[serde(rename = "Mode")]
    pub mode: i32,
    #[serde(rename = "TimeSection")]
    pub time_section: Vec<Vec<String>>,

    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

impl VideoInMode {
    pub async fn get(rpc: RequestBuilder) -> Result<Self, Error> {
        get::<Self>(rpc, ConfigRequest::new("VideoInMode")).await
    }
}
