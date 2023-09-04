use std::{sync::{Arc, Mutex}, collections::HashMap};

use polywrap_client::{
    builder::{PolywrapClientConfig, PolywrapClientConfigBuilder},
    client::PolywrapClient,
    core::{
        error::Error, resolution::uri_resolution_context::UriResolutionContext, uri::Uri,
        uri_resolver_handler::UriResolverHandler, wrap_loader::WrapLoader, wrapper::GetFileOptions,
    }, plugin::Invoker,
};
use polywrap_plugin::package::PluginPackage;
use polywrap_http_server_plugin::HttpServerPlugin;
use polywrap_key_value_store_plugin::KeyValueStorePlugin;
use polywrap_client_default_config::{SystemClientConfig, Web3ClientConfig};

pub trait CoreClient {
    fn try_resolve_uri(&self, uri: &Uri) -> Result<Uri, Error>;
    fn invoke_raw(
        &self,
        uri: &Uri,
        method: &str,
        args: Option<&[u8]>,
        env: Option<&[u8]>,
        resolution_context: Option<Arc<Mutex<UriResolutionContext>>>,
    ) -> Result<Vec<u8>, Error>;
    fn get_manifest(&self, uri: &Uri) -> Result<Vec<u8>, Error>;
}

pub struct CoreClientMock;

impl CoreClient for CoreClientMock {
    fn try_resolve_uri(&self, uri: &Uri) -> Result<Uri, Error> {
        Ok(uri.clone())
    }

    fn invoke_raw(
        &self,
        _uri: &Uri,
        _method: &str,
        _args: Option<&[u8]>,
        _env: Option<&[u8]>,
        _resolution_context: Option<Arc<Mutex<UriResolutionContext>>>,
    ) -> Result<Vec<u8>, Error> {
        Ok(vec![])
    }

    fn get_manifest(&self, _uri: &Uri) -> Result<Vec<u8>, Error> {
        Ok(vec![])
    }
}

pub struct PwrClient(pub Arc<PolywrapClient>);

impl PwrClient {
    pub fn new() -> Self {
        let mut config = PolywrapClientConfig::default();
        config
            .add(SystemClientConfig::default().into())
            .add(Web3ClientConfig::default().into())
            .add_package("wrap://https/http.wrappers.dev/u/test/http-server".parse().unwrap(), Arc::new(PluginPackage::from(HttpServerPlugin {})))
            .add_package("wrap://https/http.wrappers.dev/u/test/key-value-store".parse().unwrap(), Arc::new(PluginPackage::from(KeyValueStorePlugin { store: HashMap::new() })));

        PwrClient(Arc::new(PolywrapClient::new(config.into())))
    }
}

impl CoreClient for PwrClient {
    fn try_resolve_uri(&self, uri: &Uri) -> Result<Uri, Error> {
        let result = self.0.try_resolve_uri(uri, None);

        match result {
            Ok(result) => Ok(result.uri()),
            Err(e) => Err(e),
        }
    }

    fn invoke_raw(
        &self,
        uri: &Uri,
        method: &str,
        args: Option<&[u8]>,
        env: Option<&[u8]>,
        resolution_context: Option<Arc<Mutex<UriResolutionContext>>>,
    ) -> Result<Vec<u8>, Error> {
        self.0.invoke_raw(uri, method, args, env, resolution_context)
    }

    fn get_manifest(&self, uri: &Uri) -> Result<Vec<u8>, Error> {
        let wrapper = self.0.load_wrapper(uri, None);

        match wrapper {
            Ok(wrapper) => wrapper.get_file(&GetFileOptions {
                path: String::from("wrap.info"),
                encoding: None,
            }),
            Err(e) => Err(e),
        }
    }
}
