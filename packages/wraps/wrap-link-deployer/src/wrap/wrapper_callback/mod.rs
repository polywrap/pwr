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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WrapperCallback {
    pub uri: String,
    pub method: String,
}

impl WrapperCallback {
    pub fn new() -> WrapperCallback {
        WrapperCallback {
            uri: String::new(),
            method: String::new(),
        }
    }
}
