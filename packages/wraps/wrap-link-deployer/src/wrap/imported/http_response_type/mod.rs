use polywrap_wasm_rs::EnumTypeError;
use serde::{Serialize, Deserialize};
use std::convert::TryFrom;

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum HttpResponseType {
    #[serde(rename = "TEXT")]
    TEXT,
    #[serde(rename = "BINARY")]
    BINARY,
    _MAX_
}

pub fn sanitize_http_response_type_value(value: i32) -> Result<(), EnumTypeError> {
    if value < 0 && value >= HttpResponseType::_MAX_ as i32 {
        return Err(EnumTypeError::EnumProcessingError(format!("Invalid value for enum 'HttpResponseType': {}", value.to_string())));
    }
    Ok(())
}

pub fn get_http_response_type_value(key: &str) -> Result<HttpResponseType, EnumTypeError> {
    match key {
        "TEXT" => Ok(HttpResponseType::TEXT),
        "BINARY" => Ok(HttpResponseType::BINARY),
        "_MAX_" => Ok(HttpResponseType::_MAX_),
        err => Err(EnumTypeError::EnumProcessingError(format!("Invalid key for enum 'HttpResponseType': {}", err)))
    }
}

pub fn get_http_response_type_key(value: HttpResponseType) -> Result<String, EnumTypeError> {
    if sanitize_http_response_type_value(value as i32).is_ok() {
        match value {
            HttpResponseType::TEXT => Ok("TEXT".to_string()),
            HttpResponseType::BINARY => Ok("BINARY".to_string()),
            HttpResponseType::_MAX_ => Ok("_MAX_".to_string()),
        }
    } else {
        Err(EnumTypeError::EnumProcessingError(format!("Invalid value for enum 'HttpResponseType': {}", (value  as i32).to_string())))
    }
}

impl TryFrom<i32> for HttpResponseType {
    type Error = EnumTypeError;

    fn try_from(v: i32) -> Result<HttpResponseType, Self::Error> {
        match v {
            x if x == HttpResponseType::TEXT as i32 => Ok(HttpResponseType::TEXT),
            x if x == HttpResponseType::BINARY as i32 => Ok(HttpResponseType::BINARY),
            x if x == HttpResponseType::_MAX_ as i32 => Ok(HttpResponseType::_MAX_),
            _ => Err(EnumTypeError::ParseEnumError(format!("Invalid value for enum 'HttpResponseType': {}", (v  as i32).to_string()))),
        }
    }
}
