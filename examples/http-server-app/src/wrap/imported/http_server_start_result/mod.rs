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
pub struct HttpServerStartResult {
    pub ok: bool,
}

impl HttpServerStartResult {
    pub const URI: &'static str = "wrap://ipfs/QmZVdVcpDovikMED8zDM42PtDGhewuJ18hNy6kqP2Ukqwp";

    pub fn new() -> HttpServerStartResult {
        HttpServerStartResult {
            ok: false,
        }
    }
}
