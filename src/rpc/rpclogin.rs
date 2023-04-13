use std::time::Instant;

use crate::rpc::{self, utils};

#[derive(Debug)]
pub enum Error {
    UserOrPasswordNotValid,
    UserNotValid,
    PasswordNotValid,
    InBlackList,
    HasBeedUsed,
    HasBeenLocked,
    BadFirstLogin,
    Unknown(rpc::Error),
}

const TIMEOUT: u64 = 60;
const WATCH_NET: &str = "WatchNet";

fn login(client: &mut rpc::Client, username: &str, password: &str) -> Result<bool, Error> {
    let (first_login, res) = client
        .global_first_login(username)
        .map_err(|e| Error::Unknown(e))?;

    match res.error {
        Some(err) => match err.code {
            268632079 | 401 => {}
            _ => return Err(Error::Unknown(rpc::Error::Response(err))),
        },
        None => return Err(Error::BadFirstLogin),
    }

    let login_type = match first_login.encryption.as_str() {
        WATCH_NET => WATCH_NET,
        _ => "Direct",
    };

    let res = client.global_second_login(
        username,
        &utils::get_auth(username, password, &first_login),
        login_type,
        &first_login.encryption,
    );

    match res {
        Ok(res) => Ok(res),
        Err(err) => match err {
            rpc::Error::Response(err) if err.code == 268632085 => {
                Err(Error::UserOrPasswordNotValid)
            }
            rpc::Error::Response(err) if err.code == 268632081 => Err(Error::HasBeenLocked),
            rpc::Error::Response(err) if err.message == "UserNotValidt" => Err(Error::UserNotValid),
            rpc::Error::Response(err) if err.message == "PasswordNotValid" => {
                Err(Error::PasswordNotValid)
            }
            rpc::Error::Response(err) if err.message == "InBlackList" => Err(Error::InBlackList),
            rpc::Error::Response(err) if err.message == "HasBeedUsed" => Err(Error::HasBeedUsed),
            rpc::Error::Response(err) if err.message == "HasBeenLocked" => {
                Err(Error::HasBeenLocked)
            }
            _ => Err(Error::Unknown(err)),
        },
    }
}

pub struct Manager {
    pub client: rpc::Client,
    username: String,
    password: String,
    invalid: bool,
}

impl Manager {
    pub fn new(client: rpc::Client, username: String, password: String) -> Manager {
        Manager {
            client,
            username,
            password,
            invalid: false,
        }
    }

    pub fn from(client: rpc::Client) -> Manager {
        Manager {
            client,
            username: "".to_string(),
            password: "".to_string(),
            invalid: true,
        }
    }

    pub fn username(mut self, username: String) -> Manager {
        self.username = username;
        self.invalid = false;
        self
    }

    pub fn password(mut self, password: String) -> Manager {
        self.password = password;
        self.invalid = false;
        self
    }

    pub fn logout(&mut self) -> Result<bool, Error> {
        self.client.global_logout().map_err(|e| Error::Unknown(e))
    }

    pub fn login_or_relogin(&mut self) -> Result<bool, Error> {
        if self.invalid {
            return Err(Error::UserOrPasswordNotValid);
        }

        if !self.client.config.session.is_empty() {
            _ = self.logout();
        }

        match login(&mut self.client, &self.username, &self.password) {
            Ok(res) => Ok(res),
            Err(err @ Error::Unknown(_) | err @ Error::BadFirstLogin) => Err(err),
            Err(err) => {
                self.invalid = true;
                Err(err)
            }
        }
    }

    pub fn keep_alive_or_login(&mut self) -> Result<bool, Error> {
        match self.client.config.last_login {
            Some(last_login) => {
                if Instant::now().duration_since(last_login).as_secs() < TIMEOUT {
                    return Ok(true);
                }

                match self.client.global_keep_alive() {
                    Ok(_) => Ok(true),
                    Err(rpc::Error::InvalidSession(_)) => self.login_or_relogin(),
                    Err(err) => Err(Error::Unknown(err)),
                }
            }
            None => self.login_or_relogin(),
        }
    }

    pub fn rpc<T>(
        &mut self,
        op: fn(r: rpc::RequestBuilder) -> Result<T, rpc::Error>,
    ) -> Result<T, Error> {
        self.keep_alive_or_login()?;
        op(self.client.rpc()).map_err(|e| Error::Unknown(e))
    }
}
