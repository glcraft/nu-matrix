use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
#[serde(tag = "method", content = "parameters")]
pub enum Method {
    #[default]
    Stop,
    Substract(i32, i32),
}

impl Method {
    fn is_notification(&self) -> bool {
        matches!(self, Method::Stop)
    }
}