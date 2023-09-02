use polywrap_client::core::uri::Uri;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

const WRAPPERS_TEST_ACCOUNT_API_KEY: &str = "2vdnpfe00gw";
const WRAPPERS_REGISTRY_KEY_HEADER: &str = "x-api-key";

#[derive(Debug, Serialize, Deserialize)]
struct AddedIpfsFile {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Hash")]
    hash: String,
    #[serde(rename = "Size")]
    size: u32,
}

#[derive(thiserror::Error, Debug)]
pub enum DeployUriToHttpError {
    #[error("Failed to deploy URI to HTTP: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Failed to deploy URI to HTTP: {0} code")]
    ResponseError(StatusCode),
}

pub async fn deploy_uri_to_http(
    package_name_and_version: &str,
    uri: &Uri,
) -> Result<String, DeployUriToHttpError> {
    let deployed_uri = "https://http.wrappers.dev/u/test/".to_string() + package_name_and_version;

    let http_client = reqwest::Client::new();

    let resp = http_client
        .post(&deployed_uri)
        .json(&serde_json::json!({
            "uri": uri.to_string(),
        }))
        .header(WRAPPERS_REGISTRY_KEY_HEADER, WRAPPERS_TEST_ACCOUNT_API_KEY)
        .send()
        .await?;

    if resp.status() != 200 {
        println!("{:?}", resp);
        return Err(DeployUriToHttpError::ResponseError(resp.status()));
    }

    Ok(deployed_uri)
}
