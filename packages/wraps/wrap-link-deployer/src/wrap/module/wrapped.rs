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
use crate::Route;
use crate::WrapperCallback;
use crate::StartResult;
use crate::Env;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsStart {
    pub port: u16,
    #[serde(rename = "requestTimeout")]
    pub request_timeout: u32,
    pub routes: Vec<Route>,
    #[serde(rename = "onStart")]
    pub on_start: Option<WrapperCallback>,
}

pub fn start_wrapped(args: &[u8], env_size: u32) -> Vec<u8> {
    if env_size == 0 {
        panic!("Environment is not set, and it is required by method 'start'");
    }

    let env_buf = wrap_load_env(env_size);
    let env = from_slice::<Env>(&env_buf).unwrap();

    match from_slice::<ArgsStart>(args) {
        Ok(args) => {
            let result = Module::start(ArgsStart {
                port: args.port,
                request_timeout: args.request_timeout,
                routes: args.routes,
                on_start: args.on_start,
            }, env);
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
