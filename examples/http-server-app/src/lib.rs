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
                        uri: "https/http.wrappers.dev/u/test/http-server-pwr-app".to_string(),
                        method: "routeHome".to_string()
                    }
                },
                HttpServerRoute {
                    path: "/with-param/:param".to_string(),
                    http_method: HttpServerHttpMethod::GET,
                    handler: HttpServerWrapperCallback {
                        uri: "https/http.wrappers.dev/u/test/http-server-pwr-app".to_string(),
                        method: "routeWithParam".to_string()
                    }
                },
                HttpServerRoute {
                    path: "/with-query".to_string(),
                    http_method: HttpServerHttpMethod::GET,
                    handler: HttpServerWrapperCallback {
                        uri: "https/http.wrappers.dev/u/test/http-server-pwr-app".to_string(),
                        method: "routeWithQuery".to_string()
                    }
                },
                HttpServerRoute {
                    path: "/post".to_string(),
                    http_method: HttpServerHttpMethod::POST,
                    handler: HttpServerWrapperCallback {
                        uri: "https/http.wrappers.dev/u/test/http-server-pwr-app".to_string(),
                        method: "routePost".to_string()
                    }
                },
                HttpServerRoute {
                    path: "/upload".to_string(),
                    http_method: HttpServerHttpMethod::POST,
                    handler: HttpServerWrapperCallback {
                        uri: "https/http.wrappers.dev/u/test/http-server-pwr-app".to_string(),
                        method: "routeUpload".to_string()
                    }
                },
            ], 
            on_start: Some(
                HttpServerWrapperCallback {
                    uri: "https/http.wrappers.dev/u/test/http-server-pwr-app".to_string(),
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

        let resp = format!("Hello world!");
        Ok(HttpServerResponse {
            status_code: 200,
            headers: Some(vec![HttpServerKeyValuePair {
                key: "Content-Type".to_string(),
                value: "text/html".to_string(),
            }]),
            body: Some(ByteBuf::from(resp.as_bytes().to_vec())),
        })    
    }
    
    fn route_with_param(args: ArgsRouteWithParam) -> Result<HttpServerResponse, String> {
        log("Route with param");

        let resp = format!("{:?}", args);
        Ok(HttpServerResponse {
            status_code: 200,
            headers: Some(vec![HttpServerKeyValuePair {
                key: "Content-Type".to_string(),
                value: "text/html".to_string(),
            }]),
            body: Some(ByteBuf::from(resp.as_bytes().to_vec())),
        })    
    }

    fn route_with_query(args: ArgsRouteWithQuery) -> Result<HttpServerResponse, String> {
        log("Route with query");

        let resp = format!("{:?}", args);
        Ok(HttpServerResponse {
            status_code: 200,
            headers: Some(vec![HttpServerKeyValuePair {
                key: "Content-Type".to_string(),
                value: "text/html".to_string(),
            }]),
            body: Some(ByteBuf::from(resp.as_bytes().to_vec())),
        })    
    }

    fn route_post(args: ArgsRoutePost) -> Result<HttpServerResponse, String> {
        log(format!("Route post, body {:?}", args));

        Ok(HttpServerResponse {
            status_code: 200,
            headers: Some(vec![HttpServerKeyValuePair {
                key: "Content-Type".to_string(),
                value: "text/html".to_string(),
            }]),
            body: args.request.body.map(|x| ByteBuf::from(x.to_vec())),
        })    
    }

    fn route_upload(args: ArgsRouteUpload) -> Result<HttpServerResponse, String> {
        log(format!("Route upload"));
        
        let files = MultipartModule::get_files(&ArgsGetFiles {
            headers: args.request.headers.iter().map(|x| MultipartKeyValuePair {
                key: x.key.clone(),
                value: x.value.clone()
            }).collect::<Vec<MultipartKeyValuePair>>(),
            body: args.request.body.unwrap()
        }).unwrap();

        log(format!("Found the following files: {}", files.iter().map(|x| x.name.clone()).collect::<Vec<String>>().join(", ")));

        Ok(HttpServerResponse {
            status_code: 200,
            headers: Some(vec![
                HttpServerKeyValuePair {
                    key: "Content-Type".to_string(),
                    value: "text/html".to_string(),
                },
                HttpServerKeyValuePair {
                    key: "Content-Disposition".to_string(),
                    value: format!("attachment; filename=\"{}\"", files[0].name),
                }
            ]),
            body: Some(files[0].content.clone()),
        })    
    }
}

fn log<S: Into<String>>(message: S) {
    wrap_debug_log(&message.into());
}
