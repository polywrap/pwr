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
    pub const URI: &'static str = "wrap://ipfs/QmQrq7XuV7v5yANxYk8k42DjH3Vn2QS8DjR9ZXPNn8wdtz";

    pub fn new() -> KeyValueStoreKeyValuePair {
        KeyValueStoreKeyValuePair {
            key: String::new(),
            value: ByteBuf::from(vec![]),
        }
    }
}
