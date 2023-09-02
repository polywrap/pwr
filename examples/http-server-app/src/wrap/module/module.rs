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
    ArgsOnStart,
    ArgsRouteHome,
};
use crate::HttpServerRequest;
use crate::HttpServerResponse;

pub struct Module;

pub trait ModuleTrait {
  fn main(args: ArgsMain) -> Result<i32, String>;

  fn on_start(args: ArgsOnStart) -> Result<bool, String>;

  fn route_home(args: ArgsRouteHome) -> Result<HttpServerResponse, String>;
}
