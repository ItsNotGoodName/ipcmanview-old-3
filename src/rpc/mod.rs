use std::time::Instant;

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;

use self::utils::{de_int_bool_to_i64, de_number_string_to_string};

// RPC Modules
pub mod global;
pub mod license;
pub mod magicbox;
pub mod mediafilefind;

pub mod rpclogin;

pub mod utils;

#[derive(Deserialize, Debug)]
pub struct ResponseError {
    pub code: i32,
    pub message: String,
}

#[derive(Debug)]
pub enum LoginError {
    Lock,
    UserOrPasswordNotValid,
    UserNotValid,
    PasswordNotValid,
    InBlackList,
    HasBeedUsed,
    HasBeenLocked,
}

// TODO implement anyerror
#[derive(Debug)]
pub enum Error {
    Response(ResponseError),
    Login(LoginError),
    Request(String),
    Parse(String),
    InvalidSession(String),
    InvalidRequest(String),
    MethodNotFound(String),
    InterfaceNotFound(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::error::Error for Error {}

impl Error {
    pub fn no_params() -> Error {
        Error::Parse("No 'params' Field".to_string())
    }

    pub fn no_session() -> Error {
        Error::InvalidSession("No Session".to_string())
    }

    pub fn from_response_error(response_error: ResponseError) -> Error {
        match response_error {
            ResponseError {
                code: 287637505 | 287637504,
                message,
            } => Error::InvalidSession(message),
            ResponseError {
                code: 268894209,
                message,
            } => Error::InvalidRequest(message),
            ResponseError {
                code: 268894210,
                message,
            } => Error::MethodNotFound(message),
            ResponseError {
                code: 268632064,
                message,
            } => Error::InterfaceNotFound(message),
            _ => Error::Response(response_error),
        }
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

    pub fn session(&self) -> bool {
        !self.session.is_empty()
    }
}

pub struct RequestBuilder {
    req: Request,
    url: String,
    client: reqwest::Client,
    require_session: bool,
}

impl RequestBuilder {
    pub fn new(
        config: &mut Config,
        url: String,
        client: reqwest::Client,
        require_session: bool,
    ) -> RequestBuilder {
        RequestBuilder {
            req: Request {
                id: config.next_id(),
                session: config.session.clone(),
                method: "",
                params: serde_json::Value::Null,
                object: None,
            },
            url,
            client,
            require_session,
        }
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
        if self.require_session && self.req.session == "" {
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

pub struct Client {
    pub config: Config,
    pub ip: String,
    url: String,
    url_rpc: String,
    url_rpc_login: String,
    client: reqwest::Client,
}

impl Client {
    pub fn new(ip: String, client: reqwest::Client) -> Client {
        let url = format!("http://{ip}");
        let url_rpc = format!("{url}/RPC2");
        let url_rpc_login = format!("{url}/RPC2_Login");
        Client {
            config: Config::default(),
            ip,
            url,
            url_rpc,
            url_rpc_login,
            client,
        }
    }

    pub fn rpc(&mut self) -> RequestBuilder {
        RequestBuilder::new(
            &mut self.config,
            self.url_rpc.clone(),
            self.client.clone(),
            true,
        )
    }

    fn rpc_login(&mut self) -> RequestBuilder {
        RequestBuilder::new(
            &mut self.config,
            self.url_rpc_login.clone(),
            self.client.clone(),
            false,
        )
    }

    pub fn cookie(&self) -> String {
        format!("WebClientSessionID={session}; DWebClientSessionID={session}; DhWebClientSessionID={session}", session=self.config.session)
    }

    pub fn url_rpc_load(&self, file_path: &str) -> String {
        format!("{}/RPC2_Login{}", self.url, file_path)
    }
}
