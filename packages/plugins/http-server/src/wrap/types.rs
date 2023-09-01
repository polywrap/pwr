#![allow(unused_imports)]
#![allow(non_camel_case_types)]

// NOTE: This is an auto-generated file.
//       All modifications will be overwritten.
use polywrap_core::{invoker::Invoker, uri::Uri};
use polywrap_plugin::error::PluginError;
use polywrap_msgpack_serde::{
  to_vec,
  from_slice,
  JSON,
  serde_bytes::ByteBuf,
  JSONString,
  BigNumber
};
use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};

pub type BigInt = String;

// Env START //

// Env END //

// Objects START //

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WrapperCallback {
    pub uri: String,
    pub method: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Request {
    pub params: Vec<KeyValuePair>,
    pub query: Vec<KeyValuePair>,
    pub body: Option<JSONString>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Response {
    pub headers: Option<Vec<KeyValuePair>>,
    pub data: Option<ByteBuf>,
    #[serde(rename = "statusCode")]
    pub status_code: u32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Route {
    pub path: String,
    #[serde(rename = "httpMethod")]
    pub http_method: HttpMethod,
    pub handler: WrapperCallback,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StartResult {
    pub ok: bool,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct KeyValuePair {
    pub key: String,
    pub value: String,
}
// Objects END //

// Enums START //

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    OPTIONS,
    _MAX_
}
// Enums END //

// Imported objects START //

// Imported objects END //

// Imported envs START //

// Imported envs END //

// Imported enums START //

// Imported enums END //

// Imported Modules START //

// Imported Modules END //
