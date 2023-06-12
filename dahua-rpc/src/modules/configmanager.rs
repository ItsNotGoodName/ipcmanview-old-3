use std::collections::HashMap;

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{to_value, Value};

use crate::{Error, RequestBuilder};

fn is_zero(num: &i32) -> bool {
    *num == 0
}

#[derive(Deserialize, Debug)]
pub struct GetConfigResponse<T> {
    pub table: T,
}

#[derive(Serialize, Debug)]
pub struct GetConfigRequest {
    pub name: &'static str,
    #[serde(skip_serializing_if = "is_zero")]
    pub channel: i32,
}

impl GetConfigRequest {
    pub fn new(name: &'static str) -> Self {
        Self { name, channel: 0 }
    }

    pub async fn send<T>(self, rpc: RequestBuilder) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        rpc.method("configManager.getConfig")
            .params(to_value(self).expect("could not serialize GetConfigRequest"))
            .send::<GetConfigResponse<T>>()
            .await?
            .params_map(|p, _| p.table)
    }
}

#[derive(Serialize, Debug)]
pub struct SetConfigRequest<T: Serialize> {
    pub name: &'static str,
    #[serde(skip_serializing_if = "is_zero")]
    pub channel: i32,
    pub table: T,
}

impl<T> SetConfigRequest<T>
where
    T: Serialize,
{
    pub fn new(name: &'static str, table: T) -> Self {
        Self {
            name,
            table,
            channel: 0,
        }
    }

    pub async fn send(self, rpc: RequestBuilder) -> Result<(), Error> {
        rpc.method("configManager.setConfig")
            .params(to_value(self).expect("could not serialize SetConfigRequest"))
            .send::<Value>()
            .await
            .map(|_| Ok(()))?
    }
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
        GetConfigRequest::new("General").send::<Self>(rpc).await
    }

    pub async fn set(self, rpc: RequestBuilder) -> Result<(), Error> {
        SetConfigRequest::new("General", self).send(rpc).await
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
        GetConfigRequest::new("NTP").send::<Self>(rpc).await
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
        GetConfigRequest::new("VideoInMode").send::<Self>(rpc).await
    }
}
