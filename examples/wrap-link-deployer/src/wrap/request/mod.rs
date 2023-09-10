use serde::{Serialize, Deserialize};
use polywrap_msgpack_serde::{
    wrappers::polywrap_json::JSONString,
    wrappers::polywrap_bigint::BigIntWrapper
};
use polywrap_wasm_rs::{
    BigInt,
    BigNumber,
    Map,
    JSON
};
use crate::KeyValuePair;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Request {
    pub headers: Vec<KeyValuePair>,
    pub params: Vec<KeyValuePair>,
    pub query: Vec<KeyValuePair>,
    #[serde(with = "serde_bytes")]
    pub body: Option<Vec<u8>>,
}

impl Request {
    pub fn new() -> Request {
        Request {
            headers: vec![],
            params: vec![],
            query: vec![],
            body: None,
        }
    }
}
