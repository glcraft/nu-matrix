use serde::{Deserialize, Serialize};
use json_rpc_types::{Request, Response};

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
pub enum Method {
    #[default]
    Stop
}

pub type MatrixRequest = Request<bool, Method>;
