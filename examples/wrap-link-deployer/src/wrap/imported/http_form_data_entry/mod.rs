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
pub struct HttpFormDataEntry {
    pub name: String,
    pub value: Option<String>,
    #[serde(rename = "fileName")]
    pub file_name: Option<String>,
    #[serde(rename = "type")]
    pub _type: Option<String>,
}

impl HttpFormDataEntry {
    pub const URI: &'static str = "wrapscan.io/polywrap/http@1.0";

    pub fn new() -> HttpFormDataEntry {
        HttpFormDataEntry {
            name: String::new(),
            value: None,
            file_name: None,
            _type: None,
        }
    }
}
