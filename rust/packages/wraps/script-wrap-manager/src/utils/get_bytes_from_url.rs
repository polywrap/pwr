use crate::{wrap::imported::{http_request::HttpRequest, ArgsGet, HttpModule, HttpResponseType}, StringError, OkOrErrorString, MapToErrorString};

pub fn get_bytes_from_url(url: &str) -> Result<Box<[u8]>, StringError> {
    let result = HttpModule::get(&ArgsGet {
        url: url.to_string(),
        request: Some(HttpRequest {
            response_type: HttpResponseType::BINARY,
            headers: None,
            url_params: None,
            body: None,
            timeout: None,
            form_data: None,
        }),
    }).map_err_str()?;

    let result = result.ok_or_str("Unexpected response type")?;
    let result = base64::decode(result.body.ok_or_str("Body is empty")?)?
            .into_boxed_slice();
   
    Ok(result)
}
