use std::fs;
use std::sync::{Mutex, Arc};

use polywrap_client::core::client::ClientConfig;
use polywrap_client::core::resolution::uri_resolution_context::UriResolutionContext;
use polywrap_client::core::uri_resolver_handler::UriResolverHandler;
use polywrap_client::{client::*, core::wrap_loader::WrapLoader};
use polywrap_client::core::error::Error;
use polywrap_client::core::uri::Uri;
use polywrap_client::core::wrapper::GetFileOptions;
use polywrap_client_builder::PolywrapClientConfig;

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

pub struct PwrClient {
    client: PolywrapClient
}

impl PwrClient {
    pub fn new() -> Self {
        // let mut builder = BuilderConfig::new(Some(build_default_config()));
        // builder.add_env(Uri::try_from("wrap://ens/wraps.eth:async-ipfs-uri-resolver-ext@1.0.1").unwrap(), msgpack!({
        //     "provider": "https://ipfs.wrappers.io",
        //     "fallbackProviders": ["https://ipfs.io"],
        //     "retries": { "tryResolveUri": 2, "getFile": 2 },
        // }).to_vec());
        // let config = build_resolver(builder);
        
        let client = PolywrapClient::new(get_config());
        Self {
            client
        }
    }
}

fn get_config() -> ClientConfig {
    let manifest = fs::read("/home/nerfzael/dev/web3api/repos/pwr/packages/wrappers/echo/build/wrap.info").expect("Unable to read file");
    let mut bytes = fs::read("/home/nerfzael/dev/web3api/repos/pwr/packages/wrappers/echo/build/wrap.wasm").expect("Unable to read file");

    PolywrapClientConfig::default().into()
}
impl CoreClient for PwrClient {
    fn try_resolve_uri(&self, uri: &Uri) -> Result<Uri, Error> {
        let result = self.client.try_resolve_uri(uri, None);

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
        self.client.invoke(uri, method, args, env, resolution_context)
    }
    
    fn get_manifest(&self, uri: &Uri) -> Result<Vec<u8>, Error> {
        let wrapper = self.client.load_wrapper(uri, None);

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
