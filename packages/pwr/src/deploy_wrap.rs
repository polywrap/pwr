use std::fs;

use clap::Command;
use polywrap_client::core::uri::Uri;
use serde::{Deserialize, Serialize};
use wrap_manifest_schemas::deserialize::deserialize_wrap_manifest;
use wrap_utils::{deploy_package_to_ipfs, deploy_uri_to_http};

use crate::{StringError, MapToErrorString};

pub async fn deploy_wrap(args: &[String]) -> Result<i32, StringError> {
    Command::new("deploy")
        .about("deploys a wrap")
        .get_matches_from(args);

    return execute_deploy_command().await;
}

async fn execute_deploy_command() -> Result<i32, StringError> {
    println!("Deploying the WRAP...");

    let output = "./build";

    let cid = deploy_package_to_ipfs(output).await.map_err_str()?;
    println!("WRAP deployed to IPFS: wrap://ipfs/{}", cid);

    let manifest = fs::read(format!("{output}/wrap.info"))?;
    let manifest = deserialize_wrap_manifest(&manifest, None)?;

    deploy_uri_to_http(
        &manifest.name,
        &Uri::try_from("wrap://ipfs/".to_string() + &cid).map_err_str()?,
    )
    .await
    ?;
    println!(
        "WRAP deployed to wrappers.dev registry: wrap://https/http.wrappers.dev/u/test/{}",
        &manifest.name
    );
    println!("WRAP deployed successfully!");

    Ok(0)
}

#[derive(Serialize, Deserialize)]
struct AppArgs {
    args: Vec<String>,
}

// fn msgpack_to_json_pretty(bytes: &[u8]) -> String {
//     let value: rmpv::Value = rmp_serde::from_slice(bytes)?;
//     serde_json::to_string_pretty(&value)?
// }
