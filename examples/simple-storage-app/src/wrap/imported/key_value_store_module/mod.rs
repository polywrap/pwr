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
    subinvoke, wrap_debug_log
};
use serde_bytes::ByteBuf;
use crate::KeyValueStoreKeyValuePair;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsSet {
    pub key: String,
    pub value: ByteBuf,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsGet {
    pub key: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsRemove {
    pub key: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsHas {
    pub key: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsKeys {
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsValues {
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsEntries {
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct KeyValueStoreModule {}

impl KeyValueStoreModule {
    pub const URI: &'static str = "wrap://http/http.wrappers.dev/u/test/key-value-store";

    pub fn new() -> KeyValueStoreModule {
        KeyValueStoreModule {}
    }

    pub fn set(args: &ArgsSet) -> Result<bool, String> {
        let uri = KeyValueStoreModule::URI;
        let args = to_vec(args).map_err(|e| e.to_string())?;
        let result = subinvoke::wrap_subinvoke(
            uri,
            "set",
            args,
        )?;
        from_slice(result.as_slice()).map_err(|e| e.to_string())
    }

    pub fn get(args: &ArgsGet) -> Result<Option<ByteBuf>, String> {
        let uri = KeyValueStoreModule::URI;
        let args = to_vec(args).map_err(|e| e.to_string())?;
        let result = subinvoke::wrap_subinvoke(
            uri,
            "get",
            args,
        )?;
        from_slice(result.as_slice()).map_err(|e| e.to_string())
    }

    pub fn remove(args: &ArgsRemove) -> Result<bool, String> {
        let uri: &str = KeyValueStoreModule::URI;
        let args = to_vec(args).map_err(|e| e.to_string())?;
        let result = subinvoke::wrap_subinvoke(
            uri,
            "remove",
            args,
        )?;
        from_slice(result.as_slice()).map_err(|e| e.to_string())
    }

    pub fn has(args: &ArgsHas) -> Result<bool, String> {
        let uri = KeyValueStoreModule::URI;
        let args = to_vec(args).map_err(|e| e.to_string())?;
        let result = subinvoke::wrap_subinvoke(
            uri,
            "has",
            args,
        )?;
        from_slice(result.as_slice()).map_err(|e| e.to_string())
    }

    pub fn keys(args: &ArgsKeys) -> Result<Vec<String>, String> {
        let uri = KeyValueStoreModule::URI;
        let args = to_vec(args).map_err(|e| e.to_string())?;
        let result = subinvoke::wrap_subinvoke(
            uri,
            "keys",
            args,
        )?;
        from_slice(result.as_slice()).map_err(|e| e.to_string())
    }

    pub fn values(args: &ArgsValues) -> Result<Vec<ByteBuf>, String> {
        let uri = KeyValueStoreModule::URI;
        let args = to_vec(args).map_err(|e| e.to_string())?;
        let result = subinvoke::wrap_subinvoke(
            uri,
            "values",
            args,
        )?;
        from_slice(result.as_slice()).map_err(|e| e.to_string())
    }

    pub fn entries(args: &ArgsEntries) -> Result<Vec<KeyValueStoreKeyValuePair>, String> {
        let uri = KeyValueStoreModule::URI;
        let args = to_vec(args).map_err(|e| e.to_string())?;
        let result = subinvoke::wrap_subinvoke(
            uri,
            "entries",
            args,
        )?;
        from_slice(result.as_slice()).map_err(|e| e.to_string())
    }
}
