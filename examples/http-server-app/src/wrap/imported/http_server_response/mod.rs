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
use serde_bytes::ByteBuf;
use crate::HttpServerKeyValuePair;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HttpServerResponse {
    pub headers: Option<Vec<HttpServerKeyValuePair>>,
    #[serde(with = "serde_bytes")]
    pub data: Option<ByteBuf>,
    #[serde(rename = "statusCode")]
    pub status_code: u16,
}

impl HttpServerResponse {
    pub const URI: &'static str = "wrap://ipfs/QmZVdVcpDovikMED8zDM42PtDGhewuJ18hNy6kqP2Ukqwp";

    pub fn new() -> HttpServerResponse {
        HttpServerResponse {
            headers: None,
            data: None,
            status_code: 0,
        }
    }
}
