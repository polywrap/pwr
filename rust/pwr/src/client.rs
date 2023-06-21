use std::sync::{Mutex, Arc};

use polywrap_client::{core::{resolution::uri_resolution_context::UriResolutionContext, error::Error, uri::Uri, wrap_loader::WrapLoader, wrapper::GetFileOptions, uri_resolver_handler::UriResolverHandler}, client::PolywrapClient, builder::{PolywrapClientConfig, PolywrapClientConfigBuilder}};
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

impl CoreClientMock {
    pub fn new() -> Self {
        Self {}
    }
}
impl CoreClient for CoreClientMock {
    fn try_resolve_uri(&self, uri: &Uri) -> Result<Uri, Error> {
        Ok(uri.clone())
    }
   
    fn invoke_raw(
        &self,
        uri: &Uri,
        method: &str,
        args: Option<&[u8]>,
        env: Option<&[u8]>,
        resolution_context: Option<Arc<Mutex<UriResolutionContext>>>,
    ) -> Result<Vec<u8>, Error> {
        Ok(vec![])
    }
    
    fn get_manifest(&self, uri: &Uri) -> Result<Vec<u8>, Error> {
        Ok(vec![])
    }
}

pub struct PwrClient(pub PolywrapClient);

impl PwrClient {
    pub fn new() -> Self {
        let mut config = PolywrapClientConfig::default();
        config
            .add(SystemClientConfig::default().into())
            .add(Web3ClientConfig::default().into());

        let client = PwrClient(PolywrapClient::new(config.into()));

        client
    }
}

impl CoreClient for PwrClient {
    fn try_resolve_uri(&self, uri: &Uri) -> Result<Uri, Error> {
        let result = self.0.try_resolve_uri(uri, None);

        match result {
            Ok(result) => Ok(result.uri()),
            Err(e) => Err(e)
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
        self.0.invoke(uri, method, args, env, resolution_context)
    }
    
    fn get_manifest(&self, uri: &Uri) -> Result<Vec<u8>, Error> {
        let wrapper = self.0.load_wrapper(uri, None);

        match wrapper {
            Ok(wrapper) => {
                let manifest = wrapper.get_file(
                    &GetFileOptions {
                        path: String::from("wrap.info"),
                        encoding: None
                    }
                );

                manifest
            },
            Err(e) => Err(e)
        }
    }
}
