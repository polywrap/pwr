use reqwest::multipart;
use serde::{Deserialize, Serialize};
use std::path::Path;
use tokio::fs;

use crate::{StringError, OkOrErrorString};

#[derive(Debug, Serialize, Deserialize)]
struct AddedIpfsFile {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Hash")]
    hash: String,
    #[serde(rename = "Size")]
    size: u32,
}
pub async fn deploy_package_to_ipfs(path: &str) -> Result<String, StringError> {
    let path = Path::new(path);

    let mut dir = fs::read_dir(path).await?;
    let mut form = multipart::Form::new();
    while let Some(entry) = dir.next_entry().await? {
        let file_path = entry.path();
        if file_path.is_file() {
            // Strip the parent path
            let file_name = file_path
                .strip_prefix(path)
                .to_owned()
                .expect("Failed to strip prefix")
                .to_string_lossy()
                .into_owned();

            let a = file_path.to_string_lossy().into_owned();
            let part = multipart::Part::bytes(fs::read(a).await?)
                .file_name(file_name)
                .mime_str("application/x-tar")?;

            form = form.part("files", part);
        }
    }

    // Send the request
    let client = reqwest::Client::new();
    let resp = client
        .post("https://ipfs.wrappers.io/api/v0/add")
        .multipart(form)
        .send()
        .await?;

    if resp.status() != 200 {
        println!("{:?}", resp);
        return Err(StringError::new("Failed to upload to IPFS"));
    }
    let body = resp.text().await?;
    let body = body.split('\n').collect::<Vec<&str>>();
    // find the item that starts with "added"
    let body = body
        .iter()
        .map(|x| serde_json::from_str::<AddedIpfsFile>(x))
        .find_map(|x| if let Ok(x) = x { 
                if x.name.is_empty() { Some(x) } else { None }
            } else { None }
        )
        .ok_or_str("Failed to upload to IPFS: hash not found")?;

    Ok(body.hash)
}
