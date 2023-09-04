use serde::{Deserialize, Serialize};
use polywrap_msgpack_serde::{
    from_slice,
    to_vec,
    wrappers::polywrap_json::JSONString,
    wrappers::polywrap_bigint::BigIntWrapper
};
use polywrap_wasm_rs::{
    BigInt,
    BigNumber,
    Map,
    JSON,
    wrap_load_env
};
use crate::module::{ModuleTrait, Module};
use crate::HttpServerRequest;
use crate::HttpServerResponse;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsMain {
    pub args: Vec<String>,
}

pub fn main_wrapped(args: &[u8], env_size: u32) -> Vec<u8> {
    match from_slice::<ArgsMain>(args) {
        Ok(args) => {
            let result = Module::main(ArgsMain {
                args: args.args,
            });
            match result {
                Ok(res) => {
                    to_vec(&res).unwrap()
                }
                Err(e) => {
                    panic!("{}", e.to_string())
                }
            }
        }
        Err(e) => {
            panic!("{}", e.to_string())
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsOnStart {
}

pub fn on_start_wrapped(args: &[u8], env_size: u32) -> Vec<u8> {
            let result = Module::on_start(ArgsOnStart {
            });
            match result {
                Ok(res) => {
                    to_vec(&res).unwrap()
                }
                Err(e) => {
                    panic!("{}", e.to_string())
                }
            }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsRouteHome {
    pub request: HttpServerRequest,
}

pub fn route_home_wrapped(args: &[u8], env_size: u32) -> Vec<u8> {
    match from_slice::<ArgsRouteHome>(args) {
        Ok(args) => {
            let result = Module::route_home(ArgsRouteHome {
                request: args.request,
            });
            match result {
                Ok(res) => {
                    to_vec(&res).unwrap()
                }
                Err(e) => {
                    panic!("{}", e.to_string())
                }
            }
        }
        Err(e) => {
            panic!("{}", e.to_string())
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsRouteWithParam {
    pub request: HttpServerRequest,
}

pub fn route_with_param_wrapped(args: &[u8], env_size: u32) -> Vec<u8> {
    match from_slice::<ArgsRouteWithParam>(args) {
        Ok(args) => {
            let result = Module::route_with_param(ArgsRouteWithParam {
                request: args.request,
            });
            match result {
                Ok(res) => {
                    to_vec(&res).unwrap()
                }
                Err(e) => {
                    panic!("{}", e.to_string())
                }
            }
        }
        Err(e) => {
            panic!("{}", e.to_string())
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsRouteWithQuery {
    pub request: HttpServerRequest,
}

pub fn route_with_query_wrapped(args: &[u8], env_size: u32) -> Vec<u8> {
    match from_slice::<ArgsRouteWithQuery>(args) {
        Ok(args) => {
            let result = Module::route_with_query(ArgsRouteWithQuery {
                request: args.request,
            });
            match result {
                Ok(res) => {
                    to_vec(&res).unwrap()
                }
                Err(e) => {
                    panic!("{}", e.to_string())
                }
            }
        }
        Err(e) => {
            panic!("{}", e.to_string())
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsRoutePost {
    pub request: HttpServerRequest,
}

pub fn route_post_wrapped(args: &[u8], env_size: u32) -> Vec<u8> {
    match from_slice::<ArgsRoutePost>(args) {
        Ok(args) => {
            let result = Module::route_post(ArgsRoutePost {
                request: args.request,
            });
            match result {
                Ok(res) => {
                    to_vec(&res).unwrap()
                }
                Err(e) => {
                    panic!("{}", e.to_string())
                }
            }
        }
        Err(e) => {
            panic!("{}", e.to_string())
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsRouteUpload {
    pub request: HttpServerRequest,
}

pub fn route_upload_wrapped(args: &[u8], env_size: u32) -> Vec<u8> {
    match from_slice::<ArgsRouteUpload>(args) {
        Ok(args) => {
            let result = Module::route_upload(ArgsRouteUpload {
                request: args.request,
            });
            match result {
                Ok(res) => {
                    to_vec(&res).unwrap()
                }
                Err(e) => {
                    panic!("{}", e.to_string())
                }
            }
        }
        Err(e) => {
            panic!("{}", e.to_string())
        }
    }
}
