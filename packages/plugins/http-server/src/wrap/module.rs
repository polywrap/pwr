/// NOTE: This is an auto-generated file.
///       All modifications will be overwritten.

use std::sync::Arc;
use polywrap_core::invoker::Invoker;
use polywrap_plugin::{error::PluginError, module::PluginModule};
use polywrap_msgpack_serde::{
  to_vec,
  from_slice,
  JSON,
  serde_bytes::ByteBuf,
  JSONString,
  BigNumber
};
use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};
use super::types::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsStart {
    pub port: u32,
    #[serde(rename = "requestTimeout")]
    pub request_timeout: u32,
    pub routes: Vec<Route>,
    #[serde(rename = "onStart")]
    pub on_start: Option<WrapperCallback>,
}

pub trait Module: PluginModule {
  fn start(&mut self, args: &ArgsStart, invoker: Arc<dyn Invoker>) -> Result<StartResult, PluginError>;
}
