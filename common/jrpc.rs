use serde::{Deserialize, Serialize};
use super::methods::Method;

#[derive(Serialize, Deserialize, Default, Debug)]
pub enum Version {
    #[default]
    #[serde(rename = "2.0")]
    V2
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Request {
    pub version: Version,
    #[serde(flatten)]
    pub method: Method,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    pub session: u32,
}

impl Request {
    pub fn new(session: u32, method: Method, id: Option<i64>) -> Self {
        Self {
            version: Version::V2,
            method,
            id,
            session
        }
    }
}

#[derive(Debug, Clone)]
pub enum Error {
    Parse,
    InvalidRequest,
    MethodNotFound,
    InvalidParams,
    InternalError,
    ServerError{
        code: i32,
        message: String
    },
    ApplicationError{
        code: i32,
        message: String
    }
}

impl Error {
    pub fn code(&self) -> i32 {
        match self {
            Error::Parse => -32700,
            Error::InvalidRequest => -32600,
            Error::MethodNotFound => -32601,
            Error::InvalidParams => -32602,
            Error::InternalError => -32603,
            Error::ServerError{code, ..} | Error::ApplicationError{code, ..} => *code,
        }
    }
}
impl ToString for Error {
    fn to_string(&self) -> String {
        match self {
            Error::Parse => "Parse error".into(),
            Error::InvalidRequest => "Invalid Request".into(),
            Error::MethodNotFound => "Method not found".into(),
            Error::InvalidParams => "Invalid params".into(),
            Error::InternalError => "Internal error".into(),
            Error::ServerError{message, ..} => format!("Server error: {}", message),
            Error::ApplicationError{message, ..} => format!("Application error: {}", message),
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(_: serde_json::Error) -> Self {
        Error::Parse
    }
}
impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        JRPCError::from(self.clone()).serialize(serializer)
    }
}
impl<'a> serde::Deserialize<'a> for Error {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        let err = JRPCError::deserialize(deserializer)?;
        match err.code {
            -32700 => Ok(Error::Parse),
            -32600 => Ok(Error::InvalidRequest),
            -32601 => Ok(Error::MethodNotFound),
            -32602 => Ok(Error::InvalidParams),
            -32603 => Ok(Error::InternalError),
            -32099..=-32000 => Ok(Error::ServerError{code: err.code, message: err.message}),
            _ => Ok(Error::ApplicationError{code: err.code, message: err.message}),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct JRPCError {
    code: i32,
    message: String,
}

impl From<Error> for JRPCError {
    fn from(err: Error) -> Self {
        JRPCError {
            code: err.code(),
            message: err.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Result<T> {
    #[serde(rename = "result")]
    Ok(T),
    #[serde(rename = "error")]
    Err(Error),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response<T> {
    #[serde(rename = "jsonrpc")]
    pub version: Version,
    pub id: i64,
    #[serde(flatten)]
    pub result: Result<T>,
}