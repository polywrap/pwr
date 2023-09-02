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
pub struct BuildAndDeployResult {
    pub uri: Option<String>,
    #[serde(with = "serde_bytes")]
    pub manifest: Option<Vec<u8>>,
    #[serde(with = "serde_bytes")]
    pub module: Option<Vec<u8>>,
    pub error: Option<String>,
}

impl BuildAndDeployResult {
    pub fn new() -> BuildAndDeployResult {
        BuildAndDeployResult {
            uri: None,
            manifest: None,
            module: None,
            error: None,
        }
    }
}
