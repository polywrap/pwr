use polywrap_client::core::uri::Uri;
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
pub async fn deploy_uri_to_http(
    package_name_and_version: &str,
    uri: &Uri,
) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let resp = client
        .post("https://http.wrappers.dev/u/test/".to_string() + package_name_and_version)
        .json(&serde_json::json!({
            "uri": uri.to_string(),
        }))
        .header(WRAPPERS_REGISTRY_KEY_HEADER, WRAPPERS_TEST_ACCOUNT_API_KEY)
        .send()
        .await?;

    if resp.status() != 200 {
        println!("{:?}", resp);
        return Err("Failed to upload to IPFS".into());
    }

    Ok(package_name_and_version.to_string())
}
