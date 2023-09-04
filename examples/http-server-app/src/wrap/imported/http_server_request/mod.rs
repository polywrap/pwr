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
use crate::HttpServerKeyValuePair;
use serde_bytes::ByteBuf;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HttpServerRequest {
    pub headers: Vec<HttpServerKeyValuePair>,
    pub params: Vec<HttpServerKeyValuePair>,
    pub query: Vec<HttpServerKeyValuePair>,
    pub body: Option<ByteBuf>,
}

impl HttpServerRequest {
    pub const URI: &'static str = "wrap://http/http.wrappers.dev/u/test/http-server";

    pub fn new() -> HttpServerRequest {
        HttpServerRequest {
            headers: vec![],
            params: vec![],
            query: vec![],
            body: None,
        }
    }
}
