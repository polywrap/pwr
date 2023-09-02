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
use crate::module::{ModuleTrait, Module};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsMain {
    pub args: Vec<String>,
}

pub fn main_wrapped(args: &[u8], env_size: u32) -> Vec<u8> {
    match from_slice::<ArgsMain>(args) {
        Ok(args) => {
            let result = Module::main(ArgsMain {
                args: args.args,
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
