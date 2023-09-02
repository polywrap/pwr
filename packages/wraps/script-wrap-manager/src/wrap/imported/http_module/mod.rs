use serde::{Serialize, Deserialize};
use polywrap_msgpack_serde::{
    from_slice,
    to_vec,
    wrappers::polywrap_json::JSONString,
    wrappers::polywrap_bigint::BigIntWrapper
};
use polywrap_wasm_rs::{
    BigInt,
    BigNumber,
    Map,
    JSON,
    subinvoke
};
use crate::HttpRequest;
use crate::HttpResponse;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsGet {
    pub url: String,
    pub request: Option<HttpRequest>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsPost {
    pub url: String,
    pub request: Option<HttpRequest>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HttpModule {}

impl HttpModule {
    pub const URI: &'static str = "wrap://ens/wraps.eth:http@1.1.0";

    pub fn new() -> HttpModule {
        HttpModule {}
    }

    pub fn get(args: &ArgsGet) -> Result<Option<HttpResponse>, String> {
        let uri = HttpModule::URI;
        let args = to_vec(args).map_err(|e| e.to_string())?;
        let result = subinvoke::wrap_subinvoke(
            uri,
            "get",
            args,
        )?;
        from_slice(result.as_slice()).map_err(|e| e.to_string())
    }

    pub fn post(args: &ArgsPost) -> Result<Option<HttpResponse>, String> {
        let uri = HttpModule::URI;
        let args = to_vec(args).map_err(|e| e.to_string())?;
        let result = subinvoke::wrap_subinvoke(
            uri,
            "post",
            args,
        )?;
        from_slice(result.as_slice()).map_err(|e| e.to_string())
    }
}
