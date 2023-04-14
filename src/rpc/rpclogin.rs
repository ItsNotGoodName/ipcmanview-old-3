use core::time;
use std::thread;
use std::time::Instant;

use crate::rpc::{self, utils, Error, LoginError};

const TIMEOUT: u64 = 60;
const WATCH_NET: &str = "WatchNet";

fn login(client: &mut rpc::Client, username: &str, password: &str) -> Result<bool, Error> {
    let (first_login, res) = client.global_first_login(username)?;

    match res.error {
        Some(err) => match err.code {
            268632079 | 401 => {}
            _ => return Err(rpc::Error::Response(err)),
        },
        None => return Err(Error::Parse("Bad Error Code".to_string())),
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
        Err(err) => Err(Error::Login(match err {
            Error::Response(err) if err.code == 268632085 => LoginError::UserOrPasswordNotValid,
            Error::Response(err) if err.code == 268632081 => LoginError::HasBeenLocked,
            Error::Response(err) if err.message == "UserNotValidt" => LoginError::UserNotValid,
            Error::Response(err) if err.message == "PasswordNotValid" => {
                LoginError::PasswordNotValid
            }
            Error::Response(err) if err.message == "InBlackList" => LoginError::InBlackList,
            Error::Response(err) if err.message == "HasBeedUsed" => LoginError::HasBeedUsed,
            Error::Response(err) if err.message == "HasBeenLocked" => LoginError::HasBeenLocked,
            _ => return Err(err),
        })),
    }
}

pub struct Manager {
    pub client: rpc::Client,
    pub username: String,
    pub password: String,
    lock: bool,
}

impl Manager {
    pub fn new(client: rpc::Client) -> Manager {
        Manager {
            client,
            username: "".to_string(),
            password: "".to_string(),
            lock: true,
        }
    }

    pub fn username(mut self, username: String) -> Manager {
        self.username = username;
        self
    }

    pub fn password(mut self, password: String) -> Manager {
        self.password = password;
        self
    }

    pub fn unlock(mut self) -> Manager {
        self.lock = false;
        self
    }

    pub fn logout(&mut self) -> Result<bool, Error> {
        self.client.global_logout()
    }

    pub fn login(&mut self) -> Result<bool, Error> {
        if self.lock {
            return Err(Error::Login(LoginError::NotReady));
        }

        if self.client.config.session() {
            _ = self.logout();
        }

        match login(&mut self.client, &self.username, &self.password) {
            Ok(res) => Ok(res),
            Err(err @ Error::Login(_)) => {
                self.lock = true;
                Err(err)
            }
            Err(err) => Err(err),
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
                    Err(err @ Error::Request(_)) => Err(err), // Camera probably unreachable
                    Err(_) => {
                        thread::sleep(time::Duration::from_millis(10)); // TODO find a better way to drop current idle connection
                        self.login()
                    } // Let's just assume that our session is invalid
                }
            }
            None => self.login(),
        }
    }

    pub fn rpc<T>(
        &mut self,
        op: fn(r: rpc::RequestBuilder) -> Result<T, Error>,
    ) -> Result<T, Error> {
        self.keep_alive_or_login()?;
        op(self.client.rpc())
    }
}
