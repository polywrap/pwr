use serde::{Deserialize, Serialize};
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
    wrap_load_env
};
use serde_bytes::ByteBuf;
use crate::module::{ModuleTrait, Module};
use crate::KeyValuePair;
use crate::FileInfo;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsGetFiles {
    pub headers: Vec<KeyValuePair>,
    pub body: ByteBuf,
}

pub fn get_files_wrapped(args: &[u8], env_size: u32) -> Vec<u8> {
    match from_slice::<ArgsGetFiles>(args) {
        Ok(args) => {
            let result = Module::get_files(ArgsGetFiles {
                headers: args.headers,
                body: args.body,
            });
            match result {
                Ok(res) => {
                    to_vec(&res).unwrap()
                }
                Err(e) => {
                    panic!("{}", e.to_string())
                }
            }
        }
        Err(e) => {
            panic!("{}", e.to_string())
        }
    }
}
