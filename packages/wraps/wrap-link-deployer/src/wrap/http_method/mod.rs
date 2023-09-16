use polywrap_wasm_rs::{EnumTypeError};
use serde::{Serialize, Deserialize};
use std::convert::TryFrom;

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum HttpMethod {
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

pub fn sanitize_http_method_value(value: i32) -> Result<(), EnumTypeError> {
    if value < 0 && value >= HttpMethod::_MAX_ as i32 {
        return Err(EnumTypeError::EnumProcessingError(format!("Invalid value for enum 'HttpMethod': {}", value.to_string())));
    }
    Ok(())
}

pub fn get_http_method_value(key: &str) -> Result<HttpMethod, EnumTypeError> {
    match key {
        "GET" => Ok(HttpMethod::GET),
        "POST" => Ok(HttpMethod::POST),
        "PUT" => Ok(HttpMethod::PUT),
        "PATCH" => Ok(HttpMethod::PATCH),
        "DELETE" => Ok(HttpMethod::DELETE),
        "OPTIONS" => Ok(HttpMethod::OPTIONS),
        "_MAX_" => Ok(HttpMethod::_MAX_),
        err => Err(EnumTypeError::EnumProcessingError(format!("Invalid key for enum 'HttpMethod': {}", err)))
    }
}

pub fn get_http_method_key(value: HttpMethod) -> Result<String, EnumTypeError> {
    if sanitize_http_method_value(value as i32).is_ok() {
        match value {
            HttpMethod::GET => Ok("GET".to_string()),
            HttpMethod::POST => Ok("POST".to_string()),
            HttpMethod::PUT => Ok("PUT".to_string()),
            HttpMethod::PATCH => Ok("PATCH".to_string()),
            HttpMethod::DELETE => Ok("DELETE".to_string()),
            HttpMethod::OPTIONS => Ok("OPTIONS".to_string()),
            HttpMethod::_MAX_ => Ok("_MAX_".to_string()),
        }
    } else {
        Err(EnumTypeError::EnumProcessingError(format!("Invalid value for enum 'HttpMethod': {}", (value  as i32).to_string())))
    }
}

impl TryFrom<i32> for HttpMethod {
    type Error = EnumTypeError;

    fn try_from(v: i32) -> Result<HttpMethod, Self::Error> {
        match v {
            x if x == HttpMethod::GET as i32 => Ok(HttpMethod::GET),
            x if x == HttpMethod::POST as i32 => Ok(HttpMethod::POST),
            x if x == HttpMethod::PUT as i32 => Ok(HttpMethod::PUT),
            x if x == HttpMethod::PATCH as i32 => Ok(HttpMethod::PATCH),
            x if x == HttpMethod::DELETE as i32 => Ok(HttpMethod::DELETE),
            x if x == HttpMethod::OPTIONS as i32 => Ok(HttpMethod::OPTIONS),
            x if x == HttpMethod::_MAX_ as i32 => Ok(HttpMethod::_MAX_),
            _ => Err(EnumTypeError::ParseEnumError(format!("Invalid value for enum 'HttpMethod': {}", (v  as i32).to_string()))),
        }
    }
}
