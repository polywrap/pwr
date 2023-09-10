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
pub struct HttpResponse {
    pub status: i32,
    #[serde(rename = "statusText")]
    pub status_text: String,
    pub headers: Option<Map<String, String>>,
    pub body: Option<String>,
}

impl HttpResponse {
    pub const URI: &'static str = "wrapscan.io/polywrap/http@1.0";

    pub fn new() -> HttpResponse {
        HttpResponse {
            status: 0,
            status_text: String::new(),
            headers: None,
            body: None,
        }
    }
}
