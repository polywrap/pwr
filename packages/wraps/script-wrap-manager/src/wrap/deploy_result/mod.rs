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
pub struct DeployResult {
    pub uri: Option<String>,
    pub error: Option<String>,
}

impl DeployResult {
    pub fn new() -> DeployResult {
        DeployResult {
            uri: None,
            error: None,
        }
    }
}
