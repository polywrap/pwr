use std::{fs, sync::Arc, collections::HashMap};

use clap::Command;
use polywrap_client::PolywrapClient;
use polywrap_client_builder::{PolywrapClientConfig, PolywrapClientConfigBuilder};
use polywrap_client_default_config::{Web3ClientConfig, SystemClientConfig};
use polywrap_key_value_store_plugin::KeyValueStorePlugin;
use polywrap_msgpack_serde::to_vec;
use polywrap_plugin::{PluginPackage, Invoker};
use serde::{Deserialize, Serialize};

use crate::StringError;

pub async fn link_wrap(args: &[String]) -> Result<i32, StringError> {
    return execute_link_command(args[1].clone(), args[2].clone(), args[3].clone()).await;
}

#[derive(serde::Deserialize, serde::Serialize)]
struct Env {
    wrap_name: String,
    wrap_uri: String,
    wrap_link_url: String,
}

async fn execute_link_command(name: String, uri: String, wrap_link_url: String) -> Result<i32, StringError> {
    println!("Deploying to wrap.link...");

    let mut config = PolywrapClientConfig::default();
    config
        .add(SystemClientConfig::default().into())
        .add(Web3ClientConfig::default().into())
        .add_redirect("wrap://https/http.wrappers.dev/u/test/http-server".parse().unwrap(), "wrap://https/http.wrappers.dev/u/test/wrap-link-deployer".parse().unwrap())
        .add_env("wrap://https/http.wrappers.dev/u/test/wrap-link-deployer".parse().unwrap(), to_vec(&Env {
            wrap_name: name.clone(),
            wrap_uri: uri.clone(),
            wrap_link_url: wrap_link_url.clone(),
        }).unwrap())
        .add_package("wrap://https/http.wrappers.dev/u/test/key-value-store".parse().unwrap(), Arc::new(PluginPackage::from(KeyValueStorePlugin { store: HashMap::new() })));

    let client = PolywrapClient::new(config.into());
    
    let result = client.invoke::<i32>(&uri.parse().unwrap(), "main", Some(&to_vec(&AppArgs {
        args: vec![],
    }).unwrap()), None, None).unwrap();
    println!("WRAP deployed successfully!");

    Ok(result)
}

#[derive(Serialize, Deserialize)]
struct AppArgs {
    args: Vec<String>,
}

// fn msgpack_to_json_pretty(bytes: &[u8]) -> String {
//     let value: rmpv::Value = rmp_serde::from_slice(bytes)?;
//     serde_json::to_string_pretty(&value)?
// }
