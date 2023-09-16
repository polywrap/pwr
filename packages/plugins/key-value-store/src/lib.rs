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
        invoker: Arc<dyn Invoker>,
    ) -> Result<bool, PluginError> {
        let uri = get_caller_uri(invoker)?;
        let key = format!("{}: {}", uri, args.key);

        self.store.insert(key, args.value.clone().to_vec());
        Ok(true)
    }

    fn get(
        &mut self,
        args: &ArgsGet,
        invoker: Arc<dyn Invoker>,
    ) -> Result<Option<ByteBuf>, PluginError> {
        let uri = get_caller_uri(invoker)?;
        let key = format!("{}: {}", uri, args.key);
        
        let value = self.store.get(&key).map(|v| ByteBuf::from(v.clone()));
        Ok(value)
    }

    fn remove(
        &mut self,
        args: &ArgsRemove,
        invoker: Arc<dyn Invoker>,
    ) -> Result<bool, PluginError> {
        let uri = get_caller_uri(invoker)?;
        let key = format!("{}: {}", uri, args.key);

        self.store.remove(&key);
        Ok(true)
    }

    fn has(
        &mut self,
        args: &ArgsHas,
        invoker: Arc<dyn Invoker>,
    ) -> Result<bool, PluginError> {
        let uri = get_caller_uri(invoker)?;
        let key = format!("{}: {}", uri, args.key);

        let has = self.store.contains_key(&key);
        Ok(has)
    }

    fn keys(
        &mut self,
        _: &ArgsKeys,
        invoker: Arc<dyn Invoker>,
    ) -> Result<Vec<String>, PluginError> {
        let uri = get_caller_uri(invoker)?;
    
        let keys = self.store.keys()
            .map(|k| k.clone())
            .filter(|x| x.starts_with(&format!("{}: ", uri)))
            .collect();
        Ok(keys)
    }

    fn values(
        &mut self,
        _: &ArgsValues,
        invoker: Arc<dyn Invoker>,
    ) -> Result<Vec<ByteBuf>, PluginError> {
        let uri = get_caller_uri(invoker)?;
      
        let values = self.store.iter()
            .filter(|x| x.0.starts_with(&format!("{}: ", uri)))
            .map(|x| ByteBuf::from(x.1.clone()))
            .collect();
        Ok(values)
    }

    fn entries(
        &mut self,
        _: &ArgsEntries,
        invoker: Arc<dyn Invoker>,
    ) -> Result<Vec<KeyValuePair>, PluginError> {
        let uri = get_caller_uri(invoker)?;
      
        let entries = self.store.iter()
            .filter(|x| x.0.starts_with(&format!("{}: ", uri)))
            .map(|(k, v)| KeyValuePair {
                key: k.clone(),
                value: ByteBuf::from(v.clone()),
            })
            .collect();
        Ok(entries)
    }
}

fn get_caller_uri(invoker: Arc<dyn Invoker>) -> Result<String, PluginError> {
    let context = InvocationContextModule::get_caller_context(&InvocationContextModuleArgsGetCallerContext {}, invoker)?;

    let origin_uri = context.ok_or(PluginError::InvocationError { exception: "Key value store can not be called directly".to_string() })?
        .origin_uri;

    Ok(origin_uri)
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
