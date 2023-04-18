use std::time::Instant;

use crate::rpc::{utils, Client, Error, LoginError};

const TIMEOUT: u64 = 60;
const WATCH_NET: &str = "WatchNet";

async fn login(client: &mut Client, username: &str, password: &str) -> Result<(), Error> {
    if !client.config.session.is_empty() {
        client.global_logout().await.ok();
    }

    let (first_login, res) = client.global_first_login(username).await?;

    match res.error {
        Some(err) => match err.code {
            268632079 | 401 => {}
            _ => return Err(Error::Response(err)),
        },
        None => return Err(Error::Parse("Bad Error Code".to_string())),
    }

    let login_type = match first_login.encryption.as_str() {
        WATCH_NET => WATCH_NET,
        _ => "Direct",
    };

    let password = utils::get_auth(username, password, &first_login);
    let res = client.global_second_login(username, &password, login_type, &first_login.encryption);

    match res.await {
        Ok(_) => Ok(()),
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

async fn keep_alive(client: &mut Client) -> Result<(), Error> {
    match client.config.last_login {
        Some(last_login) => {
            if Instant::now().duration_since(last_login).as_secs() < TIMEOUT {
                return Ok(());
            }

            match client.global_keep_alive().await {
                Ok(_) => Ok(()),
                Err(err) => Err(err),
            }
        }
        None => Err(Error::no_session()),
    }
}

pub struct User {
    pub username: String,
    pub password: String,
    blocked: bool,
}

impl User {
    pub fn new() -> User {
        User {
            username: "".to_string(),
            password: "".to_string(),
            blocked: true,
        }
    }

    pub fn username(mut self, username: String) -> User {
        self.username = username;
        self
    }

    pub fn password(mut self, password: String) -> User {
        self.password = password;
        self
    }

    pub fn unblock(mut self) -> User {
        self.blocked = false;
        self
    }

    pub async fn login(&mut self, client: &mut Client) -> Result<(), Error> {
        if self.blocked {
            return Err(Error::Login(LoginError::Blocked));
        }

        match login(client, &self.username, &self.password).await {
            Ok(res) => Ok(res),
            Err(err) => {
                if let Error::Login(_) = err {
                    self.blocked = true;
                }
                Err(err)
            }
        }
    }

    pub async fn keep_alive_or_login(&mut self, client: &mut Client) -> Result<(), Error> {
        match keep_alive(client).await {
            Ok(o) => Ok(o),
            Err(err @ Error::Request(_)) => Err(err), // Camera probably unreachable
            Err(_) => self.login(client).await, // Let's just assume that our session is invalid
        }
    }

    pub async fn logout(client: &mut Client) -> Result<(), Error> {
        if client.config.session.is_empty() {
            Ok(())
        } else {
            client.global_logout().await.map(|_| ())
        }
    }
}
