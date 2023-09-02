mod wrap;
use std::collections::HashMap;
use std::sync::Arc;

use wrap::{*, module::*};
use polywrap_plugin::*;
use crate::wrap_info::get_manifest;
use crate::types::*;
#[derive(Debug)]
pub struct KeyValueStorePlugin {
    pub store: HashMap<String, Vec<u8>>,
}

#[plugin_impl]
impl Module for KeyValueStorePlugin {
    fn set(
        &mut self,
        args: &ArgsSet,
        _: Arc<dyn Invoker>,
    ) -> Result<bool, PluginError> {
        self.store.insert(args.key.clone(), args.value.clone().to_vec());
        Ok(true)
    }

    fn get(
        &mut self,
        args: &ArgsGet,
        _: Arc<dyn Invoker>,
    ) -> Result<Option<ByteBuf>, PluginError> {
        let value = self.store.get(&args.key).map(|v| ByteBuf::from(v.clone()));
        println!("get: {:?}", value);
        Ok(value)
    }

    fn remove(
        &mut self,
        args: &ArgsRemove,
        _: Arc<dyn Invoker>,
    ) -> Result<bool, PluginError> {
        let key = args.key.clone();
        self.store.remove(&key);
        Ok(true)
    }

    fn has(
        &mut self,
        args: &ArgsHas,
        _: Arc<dyn Invoker>,
    ) -> Result<bool, PluginError> {
        let has = self.store.contains_key(&args.key);
        Ok(has)
    }

    fn keys(
        &mut self,
        _: &ArgsKeys,
        _: Arc<dyn Invoker>,
    ) -> Result<Vec<String>, PluginError> {
        let keys = self.store.keys().map(|k| k.clone()).collect();
        Ok(keys)
    }

    fn values(
        &mut self,
        _: &ArgsValues,
        _: Arc<dyn Invoker>,
    ) -> Result<Vec<ByteBuf>, PluginError> {
        let values = self.store.values().map(|v| ByteBuf::from(v.clone())).collect();
        Ok(values)
    }

    fn entries(
        &mut self,
        _: &ArgsEntries,
        _: Arc<dyn Invoker>,
    ) -> Result<Vec<KeyValuePair>, PluginError> {
        let entries = self.store.iter().map(|(k, v)| KeyValuePair {
            key: k.clone(),
            value: ByteBuf::from(v.clone()),
        }).collect();
        Ok(entries)
    }

}

#[derive(thiserror::Error, Debug)]
pub enum HttpPluginError {
    #[error("Error sending request: `{0}`")]
    SendRequestError(String),
}

impl From<HttpPluginError> for PluginError {
    fn from(e: HttpPluginError) -> Self {
        PluginError::InvocationError {
            exception: e.to_string(),
        }
    }
}
