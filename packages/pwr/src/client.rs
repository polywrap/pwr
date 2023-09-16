use std::{sync::{Arc, Mutex}, collections::HashMap};

use polywrap_client::{
    builder::{ClientConfig as PolywrapClientConfig, ClientConfigBuilder as PolywrapClientConfigBuilder},
    client::Client as PolywrapClient,
    core::{
        error::Error, uri::Uri,
        wrapper::GetFileOptions,
    },
};
use polywrap_msgpack_serde::to_vec;
use polywrap_plugin::{package::PluginPackage, wrap_loader::WrapLoader, uri_resolver_handler::UriResolverHandler, Invoker, InvokerContext, uri};
use polywrap_http_server_plugin::HttpServerPlugin;
use polywrap_key_value_store_plugin::KeyValueStorePlugin;
use polywrap_client_default_config::{SystemClientConfig, Web3ClientConfig};
use serde::Serialize;

pub trait CoreClient {
    fn try_resolve_uri(&self, uri: &Uri) -> Result<Uri, Error>;
    fn invoke_raw(
        &self,
        uri: &Uri,
        method: &str,
        args: Option<&[u8]>,
        context: Option<InvokerContext>
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
        _context: Option<InvokerContext>
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
            .add_package("wrap://https/http.wrappers.dev/u/test/key-value-store".parse().unwrap(), Arc::new(PluginPackage::from(KeyValueStorePlugin { store: HashMap::new() })))
            .add_env(
                uri!("wrapscan.io/polywrap/async-ipfs-uri-resolver@1.0"),
                to_vec(&IpfsEnv {
                    provider: "https://ipfs.io".to_string(),
                    fallback_providers: vec![],
                    retries: Retries {
                        try_resolve_uri: 2,
                        get_file: 2,
                    },
                })
                .unwrap(),
            );

        PwrClient(Arc::new(PolywrapClient::new(config.into())))
    }
}

#[derive(Serialize)]
pub struct IpfsEnv {
    provider: String,
    #[serde(rename = "fallbackProviders")]
    fallback_providers: Vec<String>,
    retries: Retries,
}

#[derive(Serialize)]
pub struct Retries {
    #[serde(rename = "tryResolveUri")]
    try_resolve_uri: u8,
    #[serde(rename = "getFile")]
    get_file: u8,
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
        context: Option<InvokerContext>
    ) -> Result<Vec<u8>, Error> {
        self.0.invoke_raw(uri, method, args, context)
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
