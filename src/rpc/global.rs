use serde::Deserialize;
use serde_json::{json, Value};

use super::utils::AuthParam;
use super::{Error, RequestBuilder, Response};

#[derive(Deserialize, Debug)]
pub struct GetCurrentTime {
    pub time: String,
}

#[derive(Deserialize, Debug)]
struct KeepAlive {
    timeout: i32,
}

pub async fn get_current_time(rpc: RequestBuilder) -> Result<GetCurrentTime, Error> {
    rpc.method("global.getCurrentTime")
        .send::<GetCurrentTime>()
        .await?
        .params()
}

pub(crate) async fn first_login(
    rpc_login: RequestBuilder,
    username: &str,
) -> Result<(AuthParam, Response<AuthParam>), Error> {
    rpc_login
        .method("global.login")
        .params(json!({
            "userName": username,
            "password": "",
            "loginType": "Direct",
            "clientType": "Web3.0",
        }))
        .send_raw::<AuthParam>()
        .await?
        .params_map(|params, res| (params, res))
}

pub(crate) async fn second_login(
    rpc_login: RequestBuilder,
    username: &str,
    password: &str,
    login_type: &str,
    authority_type: &str,
) -> Result<bool, Error> {
    rpc_login
        .method("global.login")
        .params(json!({
            "userName": username,
            "password": password,
            "clientType": "Web3.0",
            "loginType": login_type,
            "authorityType": authority_type,
        }))
        .send::<Value>()
        .await?
        .result()
}

pub(crate) async fn logout(rpc: RequestBuilder) -> Result<bool, Error> {
    rpc.method("global.logout").send::<Value>().await?.result()
}

pub(crate) async fn keep_alive(rpc: RequestBuilder) -> Result<i32, Error> {
    rpc.method("global.keepAlive")
        .params(json!({
            "timeout": 300,
            "active": true
        }))
        .send::<KeepAlive>()
        .await?
        .params_map(|p, _| p.timeout)
}
