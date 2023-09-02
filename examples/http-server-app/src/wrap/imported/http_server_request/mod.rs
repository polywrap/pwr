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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HttpServerRequest {
    pub params: Vec<HttpServerKeyValuePair>,
    pub query: Vec<HttpServerKeyValuePair>,
    pub body: Option<JSONString>,
}

impl HttpServerRequest {
    pub const URI: &'static str = "wrap://ipfs/QmZVdVcpDovikMED8zDM42PtDGhewuJ18hNy6kqP2Ukqwp";

    pub fn new() -> HttpServerRequest {
        HttpServerRequest {
            params: vec![],
            query: vec![],
            body: None,
        }
    }
}
