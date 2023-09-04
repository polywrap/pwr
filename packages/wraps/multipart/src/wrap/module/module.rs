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
    ArgsGetFiles,
};
use crate::KeyValuePair;
use crate::FileInfo;

pub struct Module;

pub trait ModuleTrait {
  fn get_files(args: ArgsGetFiles) -> Result<Vec<FileInfo>, String>;
}
