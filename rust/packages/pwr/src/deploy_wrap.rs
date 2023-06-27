use std::fs;

use clap::Command;
use polywrap_client::core::uri::Uri;
use serde::{Deserialize, Serialize};
use wrap_manifest_schemas::deserialize::deserialize_wrap_manifest;
use wrap_utils::{deploy_package_to_ipfs, deploy_uri_to_http};

pub async fn deploy_wrap(args: &[String]) -> i32 {
    Command::new("deploy")
        .about("deploys a wrap")
        .get_matches_from(args);

    return execute_deploy_command().await;
}

async fn execute_deploy_command() -> i32 {
    println!("Deploying the WRAP...");

    let output = "./build";

    let cid = deploy_package_to_ipfs(output).await.unwrap();
    println!("WRAP deployed to IPFS: wrap://ipfs/{}", cid);

    let manifest = fs::read(format!("{output}/wrap.info")).unwrap();
    let manifest = deserialize_wrap_manifest(&manifest, None).unwrap();

    deploy_uri_to_http(
        &manifest.name,
        &Uri::try_from("wrap://ipfs/".to_string() + &cid).unwrap(),
    )
    .await
    .unwrap();
    println!(
        "WRAP deployed to wrappers.dev registry: wrap://http/http.wrappers.dev/u/test/{}",
        &manifest.name
    );
    println!("WRAP deployed successfully!");

    0
}

#[derive(Serialize, Deserialize)]
struct AppArgs {
    args: Vec<String>,
}

// fn msgpack_to_json_pretty(bytes: &[u8]) -> String {
//     let value: rmpv::Value = rmp_serde::from_slice(bytes).unwrap();
//     serde_json::to_string_pretty(&value).unwrap()
// }
