use std::time::Instant;

use super::{modules::global, utils, Client, Error, LoginError, State};

const TIMEOUT: u64 = 60;
const WATCH_NET: &str = "WatchNet";

impl Client {
    pub async fn login(&mut self) -> Result<(), Error> {
        // Prevent login when client is closed
        if self.closed {
            return Err(Error::Login(LoginError::Closed));
        }

        // Fail safe to prevent account lock when password is wrong
        if self.blocked {
            return Err(Error::Login(LoginError::Blocked));
        }

        // Session has to be empty to login
        if !self.state.session.is_empty() {
            global::logout(self.rpc()).await.ok();
            self.state = State::default();
        }

        match self.login_procedure().await {
            Ok(o) => Ok(o),
            Err(err) => {
                self.state = State::default();
                // Block client on a login error
                if let Error::Login(_) = err {
                    self.blocked = true;
                }
                Err(err)
            }
        }
    }

    async fn login_procedure(&mut self) -> Result<(), Error> {
        // Do a first login and set our session
        let (first_login, res) = global::first_login(self.rpc_login(), &self.username).await?;
        self.state.session = res.session;
        // Make sure the caemra supports this login procedure
        match res.error {
            Some(err) => match err.code {
                268632079 | 401 => {}
                _ => return Err(Error::Response(err)),
            },
            None => return Err(Error::Parse("No error field in first login".to_string())),
        }

        // Magic
        let login_type = match first_login.encryption.as_str() {
            WATCH_NET => WATCH_NET,
            _ => "Direct",
        };

        // Encrypt password based on the first login and then do a second login
        let password = utils::get_auth(&self.username, &self.password, &first_login);
        let res = global::second_login(
            self.rpc_login(),
            &self.username,
            &password,
            login_type,
            &first_login.encryption,
        );

        match res.await {
            Ok(_) => {
                self.state.last_login = Some(Instant::now());
                Ok(())
            }
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

    async fn keep_alive(&mut self) -> Result<(), Error> {
        match self.state.last_login {
            Some(last_login) => {
                if Instant::now().duration_since(last_login).as_secs() < TIMEOUT {
                    return Ok(());
                }

                match global::keep_alive(self.rpc()).await {
                    Ok(_) => {
                        self.state.last_login = Some(Instant::now());
                        Ok(())
                    }
                    Err(err) => Err(err),
                }
            }
            None => Err(Error::no_session()),
        }
    }

    pub async fn keep_alive_or_login(&mut self) -> Result<(), Error> {
        match self.keep_alive().await {
            Ok(o) => Ok(o),
            Err(err @ Error::Request(_)) => Err(err), // Camera probably unreachable
            Err(_) => self.login().await, // Let's just assume that our session is invalid
        }
    }

    pub async fn logout(&mut self) -> Result<(), Error> {
        if self.state.session.is_empty() {
            Ok(())
        } else {
            let res = global::logout(self.rpc()).await;
            self.state = State::default();
            match res {
                Ok(_) => Ok(()),
                Err(err) => Err(err),
            }
        }
    }
}
