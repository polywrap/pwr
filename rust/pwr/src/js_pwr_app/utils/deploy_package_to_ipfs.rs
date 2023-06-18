use std::path::Path;
use reqwest::multipart;
use serde::{Deserialize, Serialize};
use tokio::fs;

#[derive(Debug, Serialize, Deserialize)]
struct AddedIpfsFile {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Hash")]
    hash: String,
    #[serde(rename = "Size")]
    size: u32,
}
pub async fn deploy_package_to_ipfs(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let path = Path::new(path);

    let mut dir = fs::read_dir(path).await?;
    let mut form = multipart::Form::new();
    while let Some(entry) = dir.next_entry().await? {
        let file_path = entry.path();
        println!("{:?}", file_path);
        if file_path.is_file() {
            // Strip the parent path
            let file_name = file_path.strip_prefix(path).to_owned().expect("Failed to strip prefix").to_string_lossy().into_owned();

            let a = file_path.to_string_lossy().into_owned();
            let part = multipart::Part::bytes(fs::read(a).await?)
              .file_name(file_name)
              .mime_str("application/x-tar")?;

            form = form.part("files", part);
        }
    }

    // Send the request
    let client = reqwest::Client::new();
    let resp = client.post("http://localhost:8081/api/v0/add")
        .multipart(form)
        .send().await?;

    if resp.status() != 200 {
        println!("{:?}", resp);
        return Err("Failed to upload to IPFS".into());
    }
    let body = resp.text().await?;
    let body = body.split("\n").collect::<Vec<&str>>();
    // find the item that starts with "added"
    let cid = body.iter().map(|x| serde_json::from_str::<AddedIpfsFile>(x).unwrap())
      .find(|x| x.name == "")
      .unwrap()
      .hash;

    Ok(cid)
}
