use std::time::Instant;

use serde::Deserialize;
use serde_json::{json, Value};

use super::utils::AuthParam;
use super::{Client, Config, Error, RequestBuilder, Response};

#[derive(Deserialize, Debug)]
pub struct GetCurrentTime {
    pub time: String,
}

#[derive(Deserialize, Debug)]
struct KeepAlive {
    timeout: i32,
}

pub fn get_current_time(rpc: RequestBuilder) -> Result<GetCurrentTime, Error> {
    rpc.method("global.getCurrentTime")
        .send::<GetCurrentTime>()?
        .params()
}

impl Client {
    pub(crate) fn global_first_login(
        &mut self,
        username: &str,
    ) -> Result<(AuthParam, Response<AuthParam>), Error> {
        self.config = Config::default();
        self.rpc_login()
            .method("global.login")
            .params(json!({
                "userName": username,
                "password": "",
                "loginType": "Direct",
                "clientType": "Web3.0",
            }))
            .send_raw::<AuthParam>()?
            .params_as(|params, res| {
                self.config.session = res.session.clone();
                (params, res)
            })
    }

    pub(crate) fn global_second_login(
        &mut self,
        username: &str,
        password: &str,
        login_type: &str,
        authority_type: &str,
    ) -> Result<bool, Error> {
        let res = self
            .rpc_login()
            .method("global.login")
            .params(json!({
                "userName": username,
                "password": password,
                "clientType": "Web3.0",
                "loginType": login_type,
                "authorityType": authority_type,
            }))
            .send::<Value>()?;
        self.config.last_login = Some(Instant::now());
        res.result()
    }

    pub(crate) fn global_keep_alive(&mut self) -> Result<i32, Error> {
        match self
            .rpc()
            .method("global.keepAlive")
            .params(json!({
                "timeout": 300,
                "active": true
            }))
            .send::<KeepAlive>()
        {
            Ok(res) => {
                self.config.last_login = Some(Instant::now());
                res.params_as(|p, _| p.timeout)
            }
            Err(err @ Error::InvalidSession(_)) => {
                self.config = Config::default();
                Err(err)
            }
            Err(err) => Err(err),
        }
    }

    pub(crate) fn global_logout(&mut self) -> Result<bool, Error> {
        let res = self.rpc().method("global.logout").send::<Value>();
        self.config = Config::default();
        match res {
            Ok(res) => Ok(res.result()?),
            Err(Error::InvalidSession(_)) => Ok(false),
            Err(err) => Err(err),
        }
    }
}
