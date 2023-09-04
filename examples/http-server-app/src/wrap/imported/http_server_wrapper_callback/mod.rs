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
pub struct HttpServerWrapperCallback {
    pub uri: String,
    pub method: String,
}

impl HttpServerWrapperCallback {
    pub const URI: &'static str = "wrap://https/http.wrappers.dev/u/test/http-server";

    pub fn new() -> HttpServerWrapperCallback {
        HttpServerWrapperCallback {
            uri: String::new(),
            method: String::new(),
        }
    }
}
