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
use crate::HttpServerRoute;
use crate::HttpServerWrapperCallback;
use crate::HttpServerStartResult;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsStart {
    pub port: u16,
    #[serde(rename = "requestTimeout")]
    pub request_timeout: u32,
    pub routes: Vec<HttpServerRoute>,
    #[serde(rename = "onStart")]
    pub on_start: Option<HttpServerWrapperCallback>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HttpServerModule {}

impl HttpServerModule {
    pub const URI: &'static str = "wrap://ipfs/QmZVdVcpDovikMED8zDM42PtDGhewuJ18hNy6kqP2Ukqwp";

    pub fn new() -> HttpServerModule {
        HttpServerModule {}
    }

    pub fn start(args: &ArgsStart) -> Result<HttpServerStartResult, String> {
        let uri = HttpServerModule::URI;
        let args = to_vec(args).map_err(|e| e.to_string())?;
        let result = subinvoke::wrap_subinvoke(
            uri,
            "start",
            args,
        )?;
        from_slice(result.as_slice()).map_err(|e| e.to_string())
    }
}
