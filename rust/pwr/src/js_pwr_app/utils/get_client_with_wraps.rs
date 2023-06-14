use std::sync::Arc;

use polywrap_client::{client::PolywrapClient, core::{uri::Uri, client::UriRedirect, wrapper::Wrapper}, builder::types::{BuilderConfig, ClientConfigHandler}, msgpack};

pub fn get_client_with_wraps(wraps: Vec<(Uri, Arc<dyn Wrapper>)>) -> PolywrapClient {
  let config = {
      BuilderConfig {
          interfaces: None,
          envs: None,
          wrappers: Some(wraps),
          packages: None,
          redirects: Some(vec![
                UriRedirect::new(Uri::try_from("wrap://ipfs/QmZwhcANeoZCn9An61d4uPfLtNznxyz85TsBf5AcqHeWVk").unwrap(), Uri::try_from("wrap://mock/engine").unwrap()),
          ]),
          resolvers: None,
      }
  };
  let client = PolywrapClient::new(config.build());

  client
}
