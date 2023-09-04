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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MultipartFileInfo {
    pub name: String,
    pub content: ByteBuf,
}

impl MultipartFileInfo {
    pub const URI: &'static str = "wrap://https/http.wrappers.dev/u/test/multipart";

    pub fn new() -> MultipartFileInfo {
        MultipartFileInfo {
            name: String::new(),
            content: ByteBuf::from(vec![]),
        }
    }
}
