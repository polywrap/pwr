pub mod wrap;
use polywrap_wasm_rs::{wrap_debug_log, JSON::json};
use serde::Serialize;
use serde_bytes::ByteBuf;
pub use wrap::*;
use wrap::imported::*;

const DEFAULT_PORT: u16 = 8080;

impl ModuleTrait for Module {
    fn main(
        args: ArgsMain,
    ) -> Result<i32, String> {
        log("Starting server...".to_string());
        
        let port = if args.args.len() > 0 {
            args.args[0].parse::<u16>().unwrap_or(DEFAULT_PORT)
        } else {
            DEFAULT_PORT
        };
    
        HttpServerModule::start(&imported::http_server_module::ArgsStart {
            port,
            request_timeout: 10000, 
            routes: vec![
                HttpServerRoute {
                    path: "/".to_string(),
                    http_method: HttpServerHttpMethod::GET,
                    handler: HttpServerWrapperCallback {
                        uri: "http/http.wrappers.dev/u/test/http-server-pwr-app".to_string(),
                        method: "routeHome".to_string()
                    }
                },
            ], 
            on_start: Some(
                HttpServerWrapperCallback {
                    uri: "http/http.wrappers.dev/u/test/http-server-pwr-app".to_string(),
                    method: "onStart".to_string()
                }
            ),
        }).unwrap();
    
        Ok(0)
    }

    fn on_start(_: ArgsOnStart) -> Result<bool, String> {
        log("Server started".to_string());

        Ok(true)
    }

    fn route_home(_: ArgsRouteHome) -> Result<HttpServerResponse, String> {
        log("Home route");

        Ok(HttpServerResponse {
            status_code: 200,
            headers: Some(vec![HttpServerKeyValuePair {
                key: "Content-Type".to_string(),
                value: "text/html".to_string(),
            }]),
            data: Some(ByteBuf::from("Hello world!".as_bytes().to_vec())),
        })    
    }
}

fn to_json_response<T: Serialize>(data: T) -> HttpServerResponse {
    HttpServerResponse {
        status_code: 200,
        headers: Some(vec![HttpServerKeyValuePair {
            key: "Content-Type".to_string(),
            value: "application/json".to_string(),
        }]),
        data: Some(
            ByteBuf::from(json!(data)
                .to_string()
                .as_bytes()
                .to_vec())
        ),
    }    
}

fn to_error_response(message: String) -> HttpServerResponse {
    HttpServerResponse {
        status_code: 500,
        headers: Some(vec![HttpServerKeyValuePair {
            key: "Content-Type".to_string(),
            value: "text/html".to_string(),
        }]),
        data: Some(ByteBuf::from(message.as_bytes().to_vec())),
    }
}

fn log<S: Into<String>>(message: S) {
    wrap_debug_log(&message.into());
}
