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
pub struct BuildResult {
    #[serde(with = "serde_bytes")]
    pub data: Option<Vec<u8>>,
    pub error: Option<String>,
}

impl BuildResult {
    pub fn new() -> BuildResult {
        BuildResult {
            data: None,
            error: None,
        }
    }
}
