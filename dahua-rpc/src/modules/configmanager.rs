use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{to_value, Value};

use crate::{Error, RequestBuilder};

fn is_zero(num: &i32) -> bool {
    *num == 0
}

#[derive(Serialize, Debug)]
pub struct GetConfigRequest {
    pub name: &'static str,
    #[serde(skip_serializing_if = "is_zero")]
    pub channel: i32,
}

#[derive(Deserialize, Debug)]
pub struct GetConfigResponse<T> {
    pub table: Option<T>,
}

impl GetConfigRequest {
    pub fn new(name: &'static str, channel: i32) -> Self {
        Self { name, channel }
    }

    pub async fn send<T>(self, rpc: RequestBuilder, method: &'static str) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        rpc.method(method)
            .params(to_value(self).expect("could not serialize GetConfigRequest"))
            .send::<GetConfigResponse<T>>()
            .await?
            .params_map(|p, _| match p.table {
                Some(s) => Ok(s),
                None => Err(Error::no_params()),
            })?
    }

    pub async fn get<T>(self, rpc: RequestBuilder) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        self.send(rpc, "configManager.getConfig").await
    }

    pub async fn get_default<T>(self, rpc: RequestBuilder) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        self.send(rpc, "configManager.getDefault").await
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
    pub fn new(name: &'static str, channel: i32, table: T) -> Self {
        Self {
            name,
            channel,
            table,
        }
    }

    pub async fn set(self, rpc: RequestBuilder) -> Result<(), Error> {
        rpc.method("configManager.setConfig")
            .params(to_value(self).expect("could not serialize SetConfigRequest"))
            .send::<Value>()
            .await
            .map(|_| Ok(()))?
    }
}
