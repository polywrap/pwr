use polywrap_wasm_rs::{EnumTypeError};
use serde::{Serialize, Deserialize};
use std::convert::TryFrom;

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum Language {
    #[serde(rename = "JavaScript")]
    JavaScript,
    #[serde(rename = "Python")]
    Python,
    _MAX_
}

pub fn sanitize_language_value(value: i32) -> Result<(), EnumTypeError> {
    if value < 0 && value >= Language::_MAX_ as i32 {
        return Err(EnumTypeError::EnumProcessingError(format!("Invalid value for enum 'Language': {}", value.to_string())));
    }
    Ok(())
}

pub fn get_language_value(key: &str) -> Result<Language, EnumTypeError> {
    match key {
        "JavaScript" => Ok(Language::JavaScript),
        "Python" => Ok(Language::Python),
        "_MAX_" => Ok(Language::_MAX_),
        err => Err(EnumTypeError::EnumProcessingError(format!("Invalid key for enum 'Language': {}", err)))
    }
}

pub fn get_language_key(value: Language) -> Result<String, EnumTypeError> {
    if sanitize_language_value(value as i32).is_ok() {
        match value {
            Language::JavaScript => Ok("JavaScript".to_string()),
            Language::Python => Ok("Python".to_string()),
            Language::_MAX_ => Ok("_MAX_".to_string()),
        }
    } else {
        Err(EnumTypeError::EnumProcessingError(format!("Invalid value for enum 'Language': {}", (value  as i32).to_string())))
    }
}

impl TryFrom<i32> for Language {
    type Error = EnumTypeError;

    fn try_from(v: i32) -> Result<Language, Self::Error> {
        match v {
            x if x == Language::JavaScript as i32 => Ok(Language::JavaScript),
            x if x == Language::Python as i32 => Ok(Language::Python),
            x if x == Language::_MAX_ as i32 => Ok(Language::_MAX_),
            _ => Err(EnumTypeError::ParseEnumError(format!("Invalid value for enum 'Language': {}", (v  as i32).to_string()))),
        }
    }
}
