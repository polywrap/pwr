#![allow(unused_imports)]
#![allow(non_camel_case_types)]

// NOTE: This is an auto-generated file.
//       All modifications will be overwritten.
use polywrap_core::{invoker::Invoker, uri::Uri};
use polywrap_plugin::error::PluginError;
use polywrap_msgpack_serde::{
  to_vec,
  from_slice,
  JSON,
  serde_bytes::ByteBuf,
  JSONString,
  BigNumber
};
use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};
use std::sync::Arc;

pub type BigInt = String;

// Env START //

// Env END //

// Objects START //

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct KeyValuePair {
    pub key: String,
    pub value: ByteBuf,
}
// Objects END //

// Enums START //

// Enums END //

// Imported objects START //

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InvocationContextResolutionContext {
    #[serde(rename = "originUri")]
    pub origin_uri: String,
    #[serde(rename = "finalUri")]
    pub final_uri: String,
}
// Imported objects END //

// Imported envs START //

// Imported envs END //

// Imported enums START //

// Imported enums END //

// Imported Modules START //

// URI: "https/http.wrappers.dev/u/test/invocation-context" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InvocationContextModuleArgsGetOwnContext {
}

// URI: "https/http.wrappers.dev/u/test/invocation-context" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InvocationContextModuleArgsGetCallerContext {
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InvocationContextModule {}

impl InvocationContextModule {
    pub const URI: &'static str = "https/http.wrappers.dev/u/test/invocation-context";

    pub fn new() -> InvocationContextModule {
        InvocationContextModule {}
    }

    pub fn get_own_context(args: &InvocationContextModuleArgsGetOwnContext, invoker: Arc<dyn Invoker>) -> Result<InvocationContextResolutionContext, PluginError> {
        let uri = InvocationContextModule::URI;
    
        let serialized_args = to_vec(args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let uri = Uri::try_from(uri).unwrap();
        let result = invoker.invoke_raw(
            &uri,
            "getOwnContext",
            opt_args,
            None,
        )
        .map_err(|e| PluginError::InvocationError { exception: e.to_string() })?;

        Ok(from_slice(result.as_slice())?)
    }

    pub fn get_caller_context(args: &InvocationContextModuleArgsGetCallerContext, invoker: Arc<dyn Invoker>) -> Result<Option<InvocationContextResolutionContext>, PluginError> {
        let uri = InvocationContextModule::URI;
       
        let serialized_args = to_vec(args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let uri = Uri::try_from(uri).unwrap();
        let result = invoker.invoke_raw(
            &uri,
            "getCallerContext",
            opt_args,
            None,
        )
        .map_err(|e| PluginError::InvocationError { exception: e.to_string() })?;

        Ok(Some(from_slice(result.as_slice())?))
    }
}
// Imported Modules END //
