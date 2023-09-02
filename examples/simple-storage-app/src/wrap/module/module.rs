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
    ArgsMain,
};

pub struct Module;

pub trait ModuleTrait {
  fn main(args: ArgsMain) -> Result<i32, String>;
}
