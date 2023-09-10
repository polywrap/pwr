pub mod wrap;

use polywrap_wasm_rs::wrap_debug_log;
use serde_bytes::ByteBuf;
pub use wrap::*;
use wrap::imported::*;

const DEFAULT_PORT: u16 = 8080;

impl ModuleTrait for Module {
    fn main(
        args: ArgsMain,
    ) -> Result<i32, String> {      
        let port = if args.args.len() > 0 {
            args.args[0].parse::<u16>().unwrap_or(DEFAULT_PORT)
        } else {
            DEFAULT_PORT
        };
    
        log(format!("Starting server at {port}"));

        HttpServerModule::start(&imported::http_server_module::ArgsStart {
            port,
            request_timeout: 10000, 
            routes: vec![
                HttpServerRoute {
                    path: "/".to_string(),
                    http_method: HttpServerHttpMethod::GET,
                    handler: HttpServerWrapperCallback {
                        uri: "https/http.wrappers.dev/u/test/advanced-server-app".to_string(),
                        method: "routeHome".to_string()
                    }
                },
            ], 
            on_start: Some(
                HttpServerWrapperCallback {
                    uri: "https/http.wrappers.dev/u/test/advanced-server-app".to_string(),
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
        log("Home route with counter");

        let result = KeyValueStoreModule::get(&ArgsGet {
            key: "counter".to_string(),
        }).unwrap();

        let counter = result.unwrap_or(ByteBuf::from(vec![0])).to_vec()[0];

        KeyValueStoreModule::set(&ArgsSet {
            key: "counter".to_string(),
            value: ByteBuf::from(vec![counter + 1])
        }).unwrap();

        let resp = format!("Counter: {}", counter);
        Ok(HttpServerResponse {
            status_code: 200,
            headers: Some(vec![HttpServerKeyValuePair {
                key: "Content-Type".to_string(),
                value: "text/html".to_string(),
            }]),
            body: Some(ByteBuf::from(resp.as_bytes().to_vec())),
        })    
    }
}

fn log<S: Into<String>>(message: S) {
    wrap_debug_log(&message.into());
}
