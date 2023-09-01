mod wrap;
use std::sync::Arc;

use wrap::{*, module::*};
use polywrap_plugin::*;
use crate::wrap_info::get_manifest;
use crate::types::*;

#[derive(Debug)]
pub struct HttpServerPlugin;

#[plugin_impl]
impl Module for HttpServerPlugin {
    fn start(
        &mut self,
        args: &ArgsStart,
        _: Arc<dyn Invoker>,
    ) -> Result<StartResult, PluginError> {
        

      
        Ok(StartResult {
            ok: true,
        })
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
