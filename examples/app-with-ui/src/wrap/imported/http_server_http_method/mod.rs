use polywrap_wasm_rs::EnumTypeError;
use serde::{Serialize, Deserialize};
use std::convert::TryFrom;

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum HttpServerHttpMethod {
    #[serde(rename = "GET")]
    GET,
    #[serde(rename = "POST")]
    POST,
    #[serde(rename = "PUT")]
    PUT,
    #[serde(rename = "PATCH")]
    PATCH,
    #[serde(rename = "DELETE")]
    DELETE,
    #[serde(rename = "OPTIONS")]
    OPTIONS,
    _MAX_
}

pub fn sanitize_http_server_http_method_value(value: i32) -> Result<(), EnumTypeError> {
    if value < 0 && value >= HttpServerHttpMethod::_MAX_ as i32 {
        return Err(EnumTypeError::EnumProcessingError(format!("Invalid value for enum 'HttpServerHttpMethod': {}", value.to_string())));
    }
    Ok(())
}

pub fn get_http_server_http_method_value(key: &str) -> Result<HttpServerHttpMethod, EnumTypeError> {
    match key {
        "GET" => Ok(HttpServerHttpMethod::GET),
        "POST" => Ok(HttpServerHttpMethod::POST),
        "PUT" => Ok(HttpServerHttpMethod::PUT),
        "PATCH" => Ok(HttpServerHttpMethod::PATCH),
        "DELETE" => Ok(HttpServerHttpMethod::DELETE),
        "OPTIONS" => Ok(HttpServerHttpMethod::OPTIONS),
        "_MAX_" => Ok(HttpServerHttpMethod::_MAX_),
        err => Err(EnumTypeError::EnumProcessingError(format!("Invalid key for enum 'HttpServerHttpMethod': {}", err)))
    }
}

pub fn get_http_server_http_method_key(value: HttpServerHttpMethod) -> Result<String, EnumTypeError> {
    if sanitize_http_server_http_method_value(value as i32).is_ok() {
        match value {
            HttpServerHttpMethod::GET => Ok("GET".to_string()),
            HttpServerHttpMethod::POST => Ok("POST".to_string()),
            HttpServerHttpMethod::PUT => Ok("PUT".to_string()),
            HttpServerHttpMethod::PATCH => Ok("PATCH".to_string()),
            HttpServerHttpMethod::DELETE => Ok("DELETE".to_string()),
            HttpServerHttpMethod::OPTIONS => Ok("OPTIONS".to_string()),
            HttpServerHttpMethod::_MAX_ => Ok("_MAX_".to_string()),
        }
    } else {
        Err(EnumTypeError::EnumProcessingError(format!("Invalid value for enum 'HttpServerHttpMethod': {}", (value  as i32).to_string())))
    }
}

impl TryFrom<i32> for HttpServerHttpMethod {
    type Error = EnumTypeError;

    fn try_from(v: i32) -> Result<HttpServerHttpMethod, Self::Error> {
        match v {
            x if x == HttpServerHttpMethod::GET as i32 => Ok(HttpServerHttpMethod::GET),
            x if x == HttpServerHttpMethod::POST as i32 => Ok(HttpServerHttpMethod::POST),
            x if x == HttpServerHttpMethod::PUT as i32 => Ok(HttpServerHttpMethod::PUT),
            x if x == HttpServerHttpMethod::PATCH as i32 => Ok(HttpServerHttpMethod::PATCH),
            x if x == HttpServerHttpMethod::DELETE as i32 => Ok(HttpServerHttpMethod::DELETE),
            x if x == HttpServerHttpMethod::OPTIONS as i32 => Ok(HttpServerHttpMethod::OPTIONS),
            x if x == HttpServerHttpMethod::_MAX_ as i32 => Ok(HttpServerHttpMethod::_MAX_),
            _ => Err(EnumTypeError::ParseEnumError(format!("Invalid value for enum 'HttpServerHttpMethod': {}", (v  as i32).to_string()))),
        }
    }
}
