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
pub struct Response {
    pub headers: Option<Vec<KeyValuePair>>,
    #[serde(with = "serde_bytes")]
    pub body: Option<Vec<u8>>,
    #[serde(rename = "statusCode")]
    pub status_code: u16,
}

impl Response {
    pub fn new() -> Response {
        Response {
            headers: None,
            body: None,
            status_code: 0,
        }
    }
}
