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
use crate::InvocationContextResolutionContext;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsGetOwnContext {
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsGetCallerContext {
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InvocationContextModule {}

impl InvocationContextModule {
    pub const URI: &'static str = "https/http.wrappers.dev/u/test/invocation-context";

    pub fn new() -> InvocationContextModule {
        InvocationContextModule {}
    }

    pub fn get_own_context(args: &ArgsGetOwnContext) -> Result<InvocationContextResolutionContext, String> {
        let uri = InvocationContextModule::URI;
        let args = to_vec(args).map_err(|e| e.to_string())?;
        let result = subinvoke::wrap_subinvoke(
            uri,
            "getOwnContext",
            args,
        )?;
        from_slice(result.as_slice()).map_err(|e| e.to_string())
    }

    pub fn get_caller_context(args: &ArgsGetCallerContext) -> Result<Option<InvocationContextResolutionContext>, String> {
        let uri = InvocationContextModule::URI;
        let args = to_vec(args).map_err(|e| e.to_string())?;
        let result = subinvoke::wrap_subinvoke(
            uri,
            "getCallerContext",
            args,
        )?;
        from_slice(result.as_slice()).map_err(|e| e.to_string())
    }
}
