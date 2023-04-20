use std::time::Instant;

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;

use self::utils::{de_int_bool_to_i64, de_number_string_to_string};

// RPC Modules
pub mod global;
pub mod license;
pub mod magicbox;
pub mod mediafilefind;

pub mod rpccookie;
pub mod rpcfile;
pub mod rpclogin;

pub mod utils;

#[derive(thiserror::Error, Debug)]
pub enum ResponseKind {
    #[error("InvalidRequest")]
    InvalidRequest,
    #[error("MethodNotFound")]
    MethodNotFound,
    #[error("InterfaceNotFound")]
    InterfaceNotFound,
    #[error("NoData")]
    NoData,
    #[error("Unknown")]
    Unknown,
}

impl Default for ResponseKind {
    fn default() -> Self {
        ResponseKind::Unknown
    }
}

#[derive(Deserialize, Debug)]
pub struct ResponseError {
    pub code: i32,
    #[serde(default)]
    pub message: String,
    #[serde(skip_deserializing)]
    pub kind: ResponseKind,
}

#[derive(thiserror::Error, Debug)]
pub enum LoginError {
    #[error("Client is closed")]
    Closed,
    #[error("Blocked to prevent account lock")]
    Blocked,
    #[error("User or password not valid")]
    UserOrPasswordNotValid,
    #[error("User not valid")]
    UserNotValid,
    #[error("Password not valid")]
    PasswordNotValid,
    #[error("User in blackList")]
    InBlackList,
    #[error("User has be used")]
    HasBeedUsed,
    #[error("User locked")]
    HasBeenLocked,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    // This can only occurs on login requests.
    #[error("Login: {0}")]
    Login(LoginError),
    // The request could not be made for any reason.
    #[error("Request: {0}")]
    Request(String),
    //  The response could not be deserialized.
    #[error("Parse: {0}")]
    Parse(String),
    // The response contains an error field.
    #[error("{0:?}")]
    Response(ResponseError),
    // No session or the server says your session is invalid.
    #[error("Session: {0}")]
    Session(String),
}

impl Error {
    pub fn no_params() -> Error {
        Error::Parse("No 'params' field".to_string())
    }

    pub fn no_session() -> Error {
        Error::Session("No session".to_string())
    }

    pub fn from_response_error(mut error: ResponseError) -> Error {
        error.kind = match error.code {
            287637505 | 287637504 => return Error::Session(error.message),
            268894209 => ResponseKind::InvalidRequest,
            268894210 => ResponseKind::MethodNotFound,
            268632064 => ResponseKind::InterfaceNotFound,
            285409284 => ResponseKind::NoData,
            _ => ResponseKind::Unknown,
        };
        Error::Response(error)
    }
}

#[derive(Deserialize, Debug)]
pub struct Response<T = Value> {
    #[serde(default)]
    pub id: i32,
    #[serde(default, deserialize_with = "de_number_string_to_string")]
    pub session: String,
    pub error: Option<ResponseError>,
    pub params: Option<T>,
    #[serde(deserialize_with = "de_int_bool_to_i64")]
    pub result: i64,
}

impl<T> Response<T> {
    pub fn params(self) -> Result<T, Error> {
        self.params.ok_or(Error::no_params())
    }

    pub fn params_as<F, O: FnOnce(T, Response<T>) -> F>(mut self, op: O) -> Result<F, Error> {
        match self.params {
            Some(params) => {
                self.params = None;
                Ok(op(params, self))
            }
            None => Err(Error::no_params()),
        }
    }

    pub fn result(self) -> Result<bool, Error> {
        Ok(self.result != 0)
    }

    pub fn result_number(self) -> Result<i64, Error> {
        Ok(self.result)
    }
}

#[derive(Serialize, Debug)]
pub struct Request {
    pub id: i32,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub session: String,
    pub method: &'static str,
    pub params: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<i64>,
}

pub struct RequestBuilder {
    req: Request,
    url: String,
    client: reqwest::Client,
    require_session: bool,
}

impl RequestBuilder {
    pub fn new(id: i32, url: String, client: reqwest::Client, session: String) -> RequestBuilder {
        RequestBuilder {
            req: Request {
                id,
                session,
                method: "",
                params: serde_json::Value::Null,
                object: None,
            },
            url,
            client,
            require_session: false,
        }
    }

    pub fn require_session(mut self) -> RequestBuilder {
        self.require_session = true;
        self
    }

    pub fn params(mut self, params: serde_json::Value) -> RequestBuilder {
        self.req.params = params;
        self
    }

    pub fn object(mut self, object: i64) -> RequestBuilder {
        self.req.object = Some(object);
        self
    }

    pub fn method(mut self, method: &'static str) -> RequestBuilder {
        self.req.method = method;
        self
    }

    pub async fn send_raw<T: DeserializeOwned>(self) -> Result<Response<T>, Error> {
        if self.require_session && self.req.session.is_empty() {
            return Err(Error::no_session());
        }
        self.client
            .post(&self.url)
            .json(&self.req)
            .send()
            .await
            .map_err(|e| Error::Request(e.to_string()))?
            .json::<Response<T>>()
            .await
            .map_err(|e| Error::Parse(e.to_string()))
    }

    pub async fn send<T: DeserializeOwned>(self) -> Result<Response<T>, Error> {
        let res = self.send_raw().await?;
        match res.error {
            Some(err) => Err(Error::from_response_error(err)),
            None => Ok(res),
        }
    }
}

#[derive(Default, Debug)]
pub struct Config {
    last_id: i32,
    pub session: String,
    pub last_login: Option<Instant>,
}

impl Config {
    pub fn next_id(&mut self) -> i32 {
        self.last_id += 1;
        self.last_id
    }
}

pub struct Client {
    client: reqwest::Client,
    pub ip: String,
    pub username: String,
    pub password: String,
    pub blocked: bool,
    pub closed: bool,
    pub config: Config,
}

impl Client {
    pub fn new(client: reqwest::Client, ip: String, username: String, password: String) -> Client {
        Client {
            client,
            ip,
            username,
            password,
            blocked: false,
            closed: false,
            config: Config::default(),
        }
    }

    pub fn rpc(&mut self) -> RequestBuilder {
        RequestBuilder::new(
            self.config.next_id(),
            format!("http://{}/RPC2", self.ip),
            self.client.clone(),
            self.config.session.clone(),
        )
        .require_session()
    }

    fn rpc_login(&mut self) -> RequestBuilder {
        RequestBuilder::new(
            self.config.next_id(),
            format!("http://{}/RPC2_Login", self.ip),
            self.client.clone(),
            self.config.session.clone(),
        )
    }
}
