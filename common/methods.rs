use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
#[serde(tag = "method", content = "parameters")]
pub enum Method {
    #[default]
    Stop,
    NewIdentity(u32, u32),
}

impl Method {
    fn is_notification(&self) -> bool {
        matches!(self, Method::Stop)
    }
}
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Response {
    NewIdentity{
        id: u64,
    }
}