use crate::{
    wrap::imported::{HttpFormDataEntry, ArgsPost}, HttpModule, HttpRequest, HttpResponseType, StringError, OkOrErrorString, MapToErrorString,
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

pub fn deploy_package_to_ipfs(manifest: &[u8], module: &[u8]) -> Result<String, StringError> {
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
    let result = HttpModule::post(&args)
        .map_err_str()?
        .ok_or_str("Unexpected response type")?;

    let body = result.body.ok_or_str("Failed to upload to IPFS: body not defined")?;
    let body = body
        .split("\n")
        .filter(|x| x.len() > 0)
        .map(|x| serde_json::from_str::<AddedIpfsFile>(x))
        .find_map(|x| if let Ok(x) = x { 
                if x.name.is_empty() { Some(x) } else { None }
            } else { None }
        )
        .ok_or_str("Failed to upload to IPFS: hash not found")?;

    Ok(body.hash)
}
