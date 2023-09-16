use std::{sync::Arc, collections::HashMap};

use polywrap_client::Client as PolywrapClient;
use polywrap_client_builder::{ClientConfig as PolywrapClientConfig, ClientConfigBuilder as PolywrapClientConfigBuilder};
use polywrap_client_default_config::{Web3ClientConfig, SystemClientConfig};
use polywrap_key_value_store_plugin::KeyValueStorePlugin;
use polywrap_msgpack_serde::to_vec;
use polywrap_plugin::PluginPackage;
use serde::{Deserialize, Serialize};

use crate::{StringError, constants::{HTTP_SERVER_INT, WRAP_LINK_DEPLOYER, KEY_VALUE_STORE_INT}, utils::get_name_from_wrap};

const DEFAULT_BUILD_DIR: &str = "./build";

pub async fn link_wrap(args: &[String]) -> Result<i32, StringError> {
    println!("Deploying to wrap.link...");

    let name = match args.get(1) {
        Some(name) => name.clone(),
        None => get_name_from_wrap(DEFAULT_BUILD_DIR)?
    };
    let uri = match args.get(2) {
        Some(uri) => uri.clone(),
        None => format!("https/http.wrappers.dev/u/test/{}", name)
    };
    let base_url = match args.get(3) {
        Some(base_url) => base_url.clone(),
        None => "https://wrap.link".to_string()
    };
    let wrap_link_url = format!("{}/deploy", base_url);

    let mut config = PolywrapClientConfig::default();
    config
        .add(SystemClientConfig::default().into())
        .add(Web3ClientConfig::default().into())
        .add_redirect(
            HTTP_SERVER_INT.parse().unwrap(), 
            WRAP_LINK_DEPLOYER.parse().unwrap()
        )
        .add_env(
            WRAP_LINK_DEPLOYER.parse().unwrap(), 
            to_vec(&Env {
                wrap_name: name.clone(),
                wrap_uri: uri.clone(),
                wrap_link_url: wrap_link_url.clone(),
            }).unwrap()
        )
        .add_package(
            KEY_VALUE_STORE_INT.parse().unwrap(), 
            Arc::new(PluginPackage::from(KeyValueStorePlugin { store: HashMap::new() }))
        );

    let client = PolywrapClient::new(config.into());
    
    let result = client.invoke::<i32>(&uri.parse().unwrap(), "main", Some(&to_vec(&AppArgs {
        args: vec![],
    }).unwrap()), None).unwrap();

    println!("WRAP deployed and is available at {base_url}/w/{}", name);
    Ok(result)
}

#[derive(serde::Deserialize, serde::Serialize)]
struct Env {
    wrap_name: String,
    wrap_uri: String,
    wrap_link_url: String,
}

#[derive(Serialize, Deserialize)]
struct AppArgs {
    args: Vec<String>,
}
