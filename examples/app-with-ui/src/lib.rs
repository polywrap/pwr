pub mod wrap;

use polywrap_msgpack_serde::{from_slice, to_vec};
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

        KeyValueStoreModule::set(&ArgsSet {
            key: "port".to_string(),
            value: ByteBuf::from(to_vec(&port).unwrap())
        }).unwrap();

        let uri = InvocationContextModule::get_own_context(&ArgsGetOwnContext {})?.origin_uri;

        HttpServerModule::start(&imported::http_server_module::ArgsStart {
            port,
            request_timeout: 10000, 
            routes: vec![
                HttpServerRoute {
                    path: "/".to_string(),
                    http_method: HttpServerHttpMethod::GET,
                    handler: HttpServerWrapperCallback {
                        uri: uri.clone(),
                        method: "routeHome".to_string()
                    }
                },
            ], 
            on_start: Some(
                HttpServerWrapperCallback {
                    uri,
                    method: "onStart".to_string()
                }
            ),
        }).unwrap();
    
        Ok(0)
    }

    fn on_start(_: ArgsOnStart) -> Result<bool, String> {
        let port = KeyValueStoreModule::get(&ArgsGet {
            key: "port".to_string(),
        })?;
        let port = from_slice(port.unwrap_or(ByteBuf::from(vec![])).as_ref()).unwrap_or(DEFAULT_PORT);

        log(format!("Server started at: http://localhost:{}", port));

        Ok(true)
    }

    fn route_home(_: ArgsRouteHome) -> Result<HttpServerResponse, String> {
        let resp = include_str!("../static/index.html");

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
