use std::{
    path::Path,
    sync::{Arc, Mutex},
};

use polywrap_client::{
    client::PolywrapClient,
    core::{
        client::ClientConfig,
        invoker::Invoker,
        resolution::{
            uri_resolution_context::{UriPackageOrWrapper, UriResolutionContext},
            uri_resolver::UriResolver,
        },
        uri::Uri,
        wrapper::Wrapper,
    },
    resolvers::resolution_result_cache_resolver::ResolutionResultCacheResolverOptions,
};
use polywrap_client_builder::{
    PolywrapBaseResolver, PolywrapBaseResolverOptions, PolywrapClientConfig,
    PolywrapClientConfigBuilder,
};
use polywrap_client_default_config::{SystemClientConfig, Web3ClientConfig};

use crate::script_pwr_app::DEFAULT_TEMPLATE_CID;

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
                let wrap = create_wrap_from_file(path, DEFAULT_TEMPLATE_CID).map_err(|e| {
                    polywrap_client::core::error::Error::FileReadError(format!(
                        "Error creating wrap from file: {}",
                        e
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

pub fn get_client_with_wraps(wraps: Vec<(Uri, Arc<dyn Wrapper>)>) -> PolywrapClient {
    let mut config = PolywrapClientConfig::default();
    config.add(SystemClientConfig::default().into());
    config.add(Web3ClientConfig::default().into());
    config.add_redirect(
        Uri::try_from("wrap://ipfs/Qmbokxv3S2UFvkM569Gu4XCi4KvVCn138U7xBFCxfGQipo").unwrap(),
        Uri::try_from("wrap://mock/engine").unwrap(),
    );
    config.add_interface_implementation(
        Uri::try_from("wrap://ens/uri-resolver.core.polywrap.eth").unwrap(),
        Uri::try_from("wrap://http/http.wrappers.dev/u/test/polywrap-resolver").unwrap(),
    );

    for wrap in wraps {
        config.add_wrapper(wrap.0, wrap.1);
    }

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

    let client = PolywrapClient::new(config.into());

    client
}
