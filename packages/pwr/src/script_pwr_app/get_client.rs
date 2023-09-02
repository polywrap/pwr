use std::{
    path::Path,
    sync::{Arc, Mutex},
};

use polywrap_client::{
    client::PolywrapClient,
    core::{
        client::ClientConfig,
        invoker::Invoker,
        macros::uri,
        resolution::{
            uri_resolution_context::{UriPackageOrWrapper, UriResolutionContext},
            uri_resolver::UriResolver,
        },
        uri::Uri,
    },
    resolvers::resolution_result_cache_resolver::ResolutionResultCacheResolverOptions,
};
use polywrap_client_builder::{
    PolywrapBaseResolver, PolywrapBaseResolverOptions, PolywrapClientConfig,
    PolywrapClientConfigBuilder,
};
use polywrap_client_default_config::{SystemClientConfig, Web3ClientConfig};

use super::create_wrap_from_file;

#[derive(Debug)]
struct LocalResolver {}

impl LocalResolver {
    pub fn new() -> Self {
        Self {}
    }
}

impl UriResolver for LocalResolver {
    fn try_resolve_uri(
        &self,
        uri: &Uri,
        _client: Arc<dyn Invoker>,
        _resolution_context: Arc<Mutex<UriResolutionContext>>,
    ) -> Result<UriPackageOrWrapper, polywrap_client::core::error::Error> {
        if uri.authority() == "script" {
            let path = uri.path();

            if Path::new(path).extension().is_none() {
                return Ok(UriPackageOrWrapper::Uri(uri.clone()));
            }

            if Path::new(path).exists() {
                let wrap = create_wrap_from_file(path).map_err(|e| {
                    polywrap_client::core::error::Error::FileReadError(format!(
                        "Error creating wrap from file: {}",
                        e.to_string()
                    ))
                })?;

                return Ok(UriPackageOrWrapper::Wrapper(uri.clone(), wrap));
            } else {
                return Err(polywrap_client::core::error::Error::FileReadError(
                    "File does not exist: ".to_string() + path,
                ));
            }
        }

        Ok(UriPackageOrWrapper::Uri(uri.clone()))
    }
}

pub fn get_client() -> PolywrapClient {
    let mut config = PolywrapClientConfig::default();
    config.add(SystemClientConfig::default().into());
    config.add(Web3ClientConfig::default().into());
    config.add_redirect(
        uri!("ens/wraps.eth:http-uri-resolver-ext@1.0.1"),
        // TODO: remove this once the latest version of the http-uri-resolver-ext is published
        uri!("ipfs/QmansMm6hUBYs7D7EW1zA7BFBnDBGGgCM2jyVTWuDmMVNx"),
    );
    // TODO: re-enable this when the resolver is re-written in rust (for performance)
    //   config.add_interface_implementation(
    //       uri!("wrap://ens/uri-resolver.core.polywrap.eth"),
    //       uri!("wrap://http/http.wrappers.dev/u/test/polywrap-resolver"),
    //   );

    config.add_resolver(Arc::new(LocalResolver::new()));

    let config = ClientConfig {
        resolver: PolywrapBaseResolver::new(PolywrapBaseResolverOptions {
            static_resolver: config.build_static_resolver(),
            dynamic_resolvers: config.resolvers,
            cache_resolver_options: Some(ResolutionResultCacheResolverOptions {
                skip_cache: Some(|uri| uri.authority() != "ipfs"),
                ..Default::default()
            }),
            ..Default::default()
        }),
        envs: config.envs,
        interfaces: config.interfaces,
    };

    PolywrapClient::new(config)
}
