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
pub struct ArgsSet {
    pub key: String,
    pub value: ByteBuf,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsGet {
    pub key: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsRemove {
    pub key: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsHas {
    pub key: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsKeys {
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsValues {
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsEntries {
}

pub trait Module: PluginModule {
  fn set(&mut self, args: &ArgsSet, invoker: Arc<dyn Invoker>) -> Result<bool, PluginError>;

  fn get(&mut self, args: &ArgsGet, invoker: Arc<dyn Invoker>) -> Result<Option<ByteBuf>, PluginError>;

  fn remove(&mut self, args: &ArgsRemove, invoker: Arc<dyn Invoker>) -> Result<bool, PluginError>;

  fn has(&mut self, args: &ArgsHas, invoker: Arc<dyn Invoker>) -> Result<bool, PluginError>;

  fn keys(&mut self, args: &ArgsKeys, invoker: Arc<dyn Invoker>) -> Result<Vec<String>, PluginError>;

  fn values(&mut self, args: &ArgsValues, invoker: Arc<dyn Invoker>) -> Result<Vec<ByteBuf>, PluginError>;

  fn entries(&mut self, args: &ArgsEntries, invoker: Arc<dyn Invoker>) -> Result<Vec<KeyValuePair>, PluginError>;
}
