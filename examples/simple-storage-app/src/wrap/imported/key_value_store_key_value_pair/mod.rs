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
use serde_bytes::ByteBuf;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct KeyValueStoreKeyValuePair {
    pub key: String,
    pub value: ByteBuf,
}

impl KeyValueStoreKeyValuePair {
    pub const URI: &'static str = "wrap://https/http.wrappers.dev/u/test/key-value-store";

    pub fn new() -> KeyValueStoreKeyValuePair {
        KeyValueStoreKeyValuePair {
            key: String::new(),
            value: ByteBuf::from(vec![]),
        }
    }
}
