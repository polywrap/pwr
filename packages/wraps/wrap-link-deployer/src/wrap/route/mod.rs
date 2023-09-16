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
use crate::HttpMethod;
use crate::WrapperCallback;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Route {
    pub path: String,
    #[serde(rename = "httpMethod")]
    pub http_method: HttpMethod,
    pub handler: WrapperCallback,
}

impl Route {
    pub fn new() -> Route {
        Route {
            path: String::new(),
            http_method: HttpMethod::_MAX_,
            handler: WrapperCallback::new(),
        }
    }
}
