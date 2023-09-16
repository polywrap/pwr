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
pub struct InvocationContextResolutionContext {
    #[serde(rename = "originUri")]
    pub origin_uri: String,
    #[serde(rename = "finalUri")]
    pub final_uri: String,
}

impl InvocationContextResolutionContext {
    pub const URI: &'static str = "https/http.wrappers.dev/u/test/invocation-context";

    pub fn new() -> InvocationContextResolutionContext {
        InvocationContextResolutionContext {
            origin_uri: String::new(),
            final_uri: String::new(),
        }
    }
}
