use crate::wrap::imported::{http_request::HttpRequest, ArgsGet, HttpModule, HttpResponseType};

pub fn get_bytes_from_url(url: &str) -> Box<[u8]> {
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
    });

    match result {
        Ok(response) => match response {
            Some(response) => base64::decode(response.body.unwrap())
                .unwrap()
                .into_boxed_slice(),
            _ => panic!("Unexpected response type"),
        },
        Err(error) => panic!("Error: {}", error),
    }
}
