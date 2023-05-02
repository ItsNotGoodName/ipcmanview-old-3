use std::time::{Duration, Instant};

pub use chrono;
pub use reqwest;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;

pub mod cookie;
pub mod file;
pub mod login;
pub mod modules;
mod utils;

pub fn recommended_reqwest_client_builder() -> reqwest::ClientBuilder {
    reqwest::Client::builder()
        .no_deflate()
        .timeout(Duration::from_secs(10))
}

#[derive(thiserror::Error, Default, Debug)]
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
    #[default]
    Unknown,
}

#[derive(Deserialize, Debug)]
pub struct ResponseError {
    pub code: i32,
    #[serde(default)]
    pub message: String,
    #[serde(skip_deserializing)]
    pub kind: ResponseKind,
}

#[derive(thiserror::Error, Clone, Copy, Debug)]
pub enum LoginError {
    #[error("Client is closed")]
    Closed,
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
    // Only occurs on login requests
    #[error("{0:?}")]
    Login(LoginError),
    // Request could not be made for any reason
    #[error("Request: {0}")]
    Request(String),
    // Response could not be deserialized
    #[error("Parse: {0}")]
    Parse(String),
    // Response contains an error field
    #[error("{0:?}")]
    Response(ResponseError),
    // No session or the server says the session is invalid
    #[error("Session: {0}")]
    Session(String),
}

impl Error {
    fn no_params() -> Error {
        Error::Parse("No 'params' field".to_string())
    }

    fn no_session() -> Error {
        Error::Session("No session".to_string())
    }

    fn from_response_error(mut error: ResponseError) -> Error {
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
#[serde(untagged)]
enum ResponseResult {
    Bool(bool),
    Num(i64),
}

#[derive(Deserialize, Default, Debug)]
#[serde(untagged)]
enum ResponseSession {
    Str(String),
    Num(i64),
    #[default]
    None,
}

#[derive(Deserialize, Debug)]
pub struct Response<T = Value> {
    #[serde(default)]
    pub id: i32,
    #[serde(default)]
    session: ResponseSession,
    pub error: Option<ResponseError>,
    pub params: Option<T>,
    result: ResponseResult,
}

impl<T> Response<T> {
    pub fn params(self) -> Result<T, Error> {
        self.params.ok_or(Error::no_params())
    }

    pub fn params_map<F, O: FnOnce(T, Response<T>) -> F>(mut self, op: O) -> Result<F, Error> {
        match self.params {
            Some(params) => {
                self.params = None;
                Ok(op(params, self))
            }
            None => Err(Error::no_params()),
        }
    }

    pub fn session(&self) -> String {
        match self.session {
            ResponseSession::Str(ref session) => session.clone(),
            ResponseSession::Num(number) => number.to_string(),
            ResponseSession::None => "".to_string(),
        }
    }

    pub fn result(self) -> bool {
        match self.result {
            ResponseResult::Bool(result) => result,
            ResponseResult::Num(result) => result != 0,
        }
    }

    pub fn result_number(self) -> i64 {
        match self.result {
            ResponseResult::Bool(true) => 1,
            ResponseResult::Bool(false) => 0,
            ResponseResult::Num(result) => result,
        }
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
pub enum State {
    #[default]
    Logout,
    Login(Instant),
    Error(LoginError),
}

#[derive(Default, Debug)]
pub struct Connection {
    last_id: i32,
    pub session: String,
}

impl Connection {
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
    pub connection: Connection,
    pub state: State,
}

impl Client {
    pub fn new(client: reqwest::Client, ip: String, username: String, password: String) -> Client {
        Client {
            client,
            ip,
            username,
            password,
            state: State::default(),
            connection: Connection::default(),
        }
    }

    pub fn rpc(&mut self) -> RequestBuilder {
        RequestBuilder::new(
            self.connection.next_id(),
            format!("http://{}/RPC2", self.ip),
            self.client.clone(),
            self.connection.session.clone(),
        )
        .require_session()
    }

    fn rpc_login(&mut self) -> RequestBuilder {
        RequestBuilder::new(
            self.connection.next_id(),
            format!("http://{}/RPC2_Login", self.ip),
            self.client.clone(),
            self.connection.session.clone(),
        )
    }

    fn transition(&mut self, state: State) {
        match (&self.state, &state) {
            // Successful login
            (State::Logout, State::Login(_)) => {}
            // Successful keep alive
            (State::Login(_), State::Login(_)) => {}
            _ => self.connection = Connection::default(),
        }

        self.state = state;
    }
}
