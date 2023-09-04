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
use crate::HttpServerHttpMethod;
use crate::HttpServerWrapperCallback;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HttpServerRoute {
    pub path: String,
    #[serde(rename = "httpMethod")]
    pub http_method: HttpServerHttpMethod,
    pub handler: HttpServerWrapperCallback,
}

impl HttpServerRoute {
    pub const URI: &'static str = "wrap://https/http.wrappers.dev/u/test/http-server";

    pub fn new() -> HttpServerRoute {
        HttpServerRoute {
            path: String::new(),
            http_method: HttpServerHttpMethod::_MAX_,
            handler: HttpServerWrapperCallback::new(),
        }
    }
}
