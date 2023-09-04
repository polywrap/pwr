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
pub struct HttpServerResponse {
    pub headers: Option<Vec<HttpServerKeyValuePair>>,
    pub body: Option<ByteBuf>,
    #[serde(rename = "statusCode")]
    pub status_code: u16,
}

impl HttpServerResponse {
    pub const URI: &'static str = "wrap://https/http.wrappers.dev/u/test/http-server";

    pub fn new() -> HttpServerResponse {
        HttpServerResponse {
            headers: None,
            body: None,
            status_code: 0,
        }
    }
}
