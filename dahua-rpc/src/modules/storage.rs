use std::vec;

use serde::Deserialize;
use serde_json::Value;

use crate::{utils::de_int_float_to_i64, Error, RequestBuilder};

#[derive(Deserialize, Debug)]
pub struct Storage {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "State")]
    pub state: String,
    #[serde(rename = "Detail")]
    pub detail: Vec<StorageDetail>,
}

#[derive(Deserialize, Debug)]
pub struct StorageDetail {
    #[serde(rename = "Path")]
    pub path: String,
    #[serde(rename = "Type")]
    pub r#type: String,
    #[serde(rename = "TotalBytes")]
    #[serde(deserialize_with = "de_int_float_to_i64")]
    pub total_bytes: i64,
    #[serde(rename = "UsedBytes")]
    #[serde(deserialize_with = "de_int_float_to_i64")]
    pub used_bytes: i64,
    #[serde(rename = "IsError")]
    pub is_error: bool,
}

#[derive(Deserialize, Debug)]
struct Storage1 {
    pub info: Option<Vec<Storage>>,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum GetDeviceAllInfo {
    Storage1(Storage1),
    Storage2(Vec<Storage>),
}

pub async fn get_device_all_info(
    rpc1: RequestBuilder,
    rpc2: RequestBuilder,
) -> Result<Vec<Storage>, Error> {
    rpc1.method("storage.factory.instance")
        .send::<Value>()
        .await
        .map(|r| {
            rpc2.method("storage.getDeviceAllInfo")
                .object(r.result_number())
        })?
        .send::<Option<GetDeviceAllInfo>>()
        .await?
        .params_map(|p, _| match p {
            Some(p) => match p {
                GetDeviceAllInfo::Storage1(storage1) => storage1.info.unwrap_or(vec![]),
                GetDeviceAllInfo::Storage2(storage2) => storage2,
            },
            None => vec![],
        })
}
