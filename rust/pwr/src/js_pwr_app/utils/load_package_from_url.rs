pub struct PackageContent {
    pub manifest: Vec<u8>,
    pub module: Vec<u8>,
}

pub fn load_package_from_url(path: &str) -> PackageContent {
    let client = reqwest::blocking::Client::new();

    let manifest_url = format!("{}/wrap.info", path);
    let module_url = format!("{}/wrap.wasm", path);

    let manifest_response = client.get(&manifest_url).send().unwrap();
    let module_response = client.get(&module_url).send().unwrap();

    let manifest_bytes = manifest_response.bytes().unwrap();
    let module_bytes = module_response.bytes().unwrap();

    PackageContent {
      manifest: manifest_bytes.to_vec(),
      module: module_bytes.to_vec(),
    }
}
