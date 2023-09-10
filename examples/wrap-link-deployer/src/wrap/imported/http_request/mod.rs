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
use crate::HttpResponseType;
use crate::HttpFormDataEntry;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HttpRequest {
    pub headers: Option<Map<String, String>>,
    #[serde(rename = "urlParams")]
    pub url_params: Option<Map<String, String>>,
    #[serde(rename = "responseType")]
    pub response_type: HttpResponseType,
    pub body: Option<String>,
    #[serde(rename = "formData")]
    pub form_data: Option<Vec<HttpFormDataEntry>>,
    pub timeout: Option<u32>,
}

impl HttpRequest {
    pub const URI: &'static str = "wrapscan.io/polywrap/http@1.0";

    pub fn new() -> HttpRequest {
        HttpRequest {
            headers: None,
            url_params: None,
            response_type: HttpResponseType::_MAX_,
            body: None,
            form_data: None,
            timeout: None,
        }
    }
}
