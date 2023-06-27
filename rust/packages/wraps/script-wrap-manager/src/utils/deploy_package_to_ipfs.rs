use crate::{
    wrap::imported::HttpFormDataEntry, ArgsPost, HttpModule, HttpRequest, HttpResponseType,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct AddedIpfsFile {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Hash")]
    hash: String,
    #[serde(rename = "Size")]
    size: u32,
}

pub fn deploy_package_to_ipfs(manifest: &[u8], module: &[u8]) -> Result<String, String> {
    let mut form_data: Vec<HttpFormDataEntry> = Vec::new();
    form_data.push(HttpFormDataEntry {
        name: "files".to_string(),
        value: Some(base64::encode(&manifest)),
        file_name: Some("wrap.info".to_string()),
        _type: Some("binary".to_string()),
    });
    form_data.push(HttpFormDataEntry {
        name: "files".to_string(),
        value: Some(base64::encode(&module)),
        file_name: Some("wrap.wasm".to_string()),
        _type: Some("binary".to_string()),
    });

    let args = ArgsPost {
        url: "https://ipfs.wrappers.io/api/v0/add".to_string(),
        request: Some(HttpRequest {
            response_type: HttpResponseType::TEXT,
            headers: None,
            url_params: None,
            body: None,
            timeout: None,
            form_data: Some(form_data),
        }),
    };
    let result = HttpModule::post(&args)?.ok_or("Unexpected response type")?;

    let body = result.body.unwrap();
    let body = body
        .split("\n")
        .filter(|x| x.len() > 0)
        .map(|x| serde_json::from_str::<AddedIpfsFile>(x).unwrap())
        .find(|x| x.name.len() == 0)
        .unwrap()
        .hash;

    Ok(body)
}
