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
use crate::{
    ArgsStart,
};
use crate::Route;
use crate::WrapperCallback;
use crate::StartResult;
use crate::env::Env;

pub struct Module;

pub trait ModuleTrait {
  fn start(args: ArgsStart, env: Env) -> Result<StartResult, String>;
}
