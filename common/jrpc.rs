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
    #[serde(rename = "jsonrpc")]
    pub version: Version,
    #[serde(flatten)]
    pub method: Method,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
}

impl Request {
    pub fn new(method: Method, id: Option<i64>) -> Self {
        Self {
            version: Version::V2,
            method,
            id,
        }
    }
}