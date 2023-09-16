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
pub struct Env {
    pub wrap_name: String,
    pub wrap_uri: String,
    pub wrap_link_url: String,
}

impl Env {
    pub fn new() -> Env {
        Env {
            wrap_name: String::new(),
            wrap_uri: String::new(),
            wrap_link_url: String::new(),
        }
    }
}
