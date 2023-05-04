use std::time::Instant;

use crate::{modules::global, utils, Client, Error, LoginError, RequestBuilder, State};

const TIMEOUT: u64 = 60;
const WATCH_NET: &str = "WatchNet";

impl Client {
    pub async fn logout(&mut self) {
        if let State::Login(_) = self.state {
            global::logout(self.rpc_raw()).await.ok();
            self.transition(State::Logout)
        }
    }

    pub async fn login(&mut self) -> Result<(), Error> {
        // Make sure we are in State::Logout
        match self.state {
            State::Logout => {}
            State::Login(_) => {
                global::logout(self.rpc_raw()).await.ok();
                self.transition(State::Logout)
            }
            State::Error(err) => return Err(Error::Login(err)),
        }

        match self.login_procedure().await {
            Ok(o) => {
                self.transition(State::Login(Instant::now()));
                Ok(o)
            }
            Err(err) => {
                if let Error::Login(err) = err {
                    // Block client on a login error
                    self.transition(State::Error(err));
                } else {
                    // Reset connection
                    self.transition(State::Logout);
                }

                Err(err)
            }
        }
    }

    async fn login_procedure(&mut self) -> Result<(), Error> {
        // Do a first login and set our session
        let (first_login, res) = global::first_login(self.rpc_login(), &self.username).await?;
        self.connection.session = res.session();
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

    pub async fn keep_alive_or_login(&mut self) -> Result<(), Error> {
        // Make sure we are login
        if let State::Login(last_login) = self.state {
            // Check if we need to run keep alive
            if Instant::now().duration_since(last_login).as_secs() < TIMEOUT {
                return Ok(());
            }

            // Run keep alive
            match global::keep_alive(self.rpc_raw()).await {
                Ok(_) => {
                    self.transition(State::Login(Instant::now()));
                    Ok(())
                }
                Err(err @ Error::Request(_)) => Err(err), // Camera probably unreachable
                Err(_) => {
                    // Let's just assume that our session is invalid
                    self.transition(State::Logout);

                    match self.login().await {
                        Ok(o) => Ok(o),
                        // Assume error was a connection reset because we did a keep alive and login request on the same connection
                        // This only affects some cameras such as the "SD2A500-GN-A-PV"
                        // TODO: check error kind is OS connection reset
                        Err(Error::Request(_)) => self.login().await,
                        Err(err) => Err(err),
                    }
                }
            }
        } else {
            self.login().await
        }
    }

    pub async fn rpc(&mut self) -> Result<RequestBuilder, Error> {
        self.keep_alive_or_login().await.map(|_| self.rpc_raw())
    }

    pub async fn cookie(&mut self) -> Result<String, Error> {
        self.keep_alive_or_login().await.map(|_| self.cookie_raw())
    }
}
