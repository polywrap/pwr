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
use serde_bytes::ByteBuf;
use crate::MultipartKeyValuePair;
use crate::MultipartFileInfo;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsGetFiles {
    pub headers: Vec<MultipartKeyValuePair>,
    pub body: ByteBuf,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MultipartModule {}

impl MultipartModule {
    pub const URI: &'static str = "wrap://http/http.wrappers.dev/u/test/multipart";

    pub fn new() -> MultipartModule {
        MultipartModule {}
    }

    pub fn get_files(args: &ArgsGetFiles) -> Result<Vec<MultipartFileInfo>, String> {
        let uri = MultipartModule::URI;
        let args = to_vec(args).map_err(|e| e.to_string())?;
        let result = subinvoke::wrap_subinvoke(
            uri,
            "getFiles",
            args,
        )?;
        from_slice(result.as_slice()).map_err(|e| e.to_string())
    }
}
