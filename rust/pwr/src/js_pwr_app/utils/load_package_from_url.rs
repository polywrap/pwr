use tokio::try_join;

pub struct PackageContent {
    pub manifest: Vec<u8>,
    pub module: Vec<u8>,
}

pub async fn load_package_from_url(path: &str) -> PackageContent {
    let client = reqwest::Client::new();

    let manifest_url = format!("{}/wrap.info", path);
    let module_url = format!("{}/wrap.wasm", path);

    let manifest_future = client.get(&manifest_url).send();
    let module_future = client.get(&module_url).send();

    let (manifest_response, module_response) = try_join!(manifest_future, module_future).unwrap();

    let manifest_bytes = manifest_response.bytes().await.unwrap();
    let module_bytes = module_response.bytes().await.unwrap();

    PackageContent {
      manifest: manifest_bytes.to_vec(),
      module: module_bytes.to_vec(),
    }
}
