pub mod wrap;
use polywrap_wasm_rs::JSON::json;
use serde::{Serialize, Deserialize};
use wrap::{*, imported::{ArgsLog, ArgsStart}};

use crate::wrap::imported::ArgsInvoke;

#[derive(Serialize, Deserialize)]
struct ClientInvokeBody {
  uri: String,
  method: String,
  args: Vec<u8>,
}

pub fn main(args: ArgsMain) -> u8 {
    let port = if args.args.len() > 0 {
        args.args[0].parse::<u32>().unwrap()
    } else {
        8080
    };

    HttpServerModule::start(&ArgsStart {
        port,
        request_timeout: 10000, 
        routes: vec![
            HttpServerRoute {
                path: "/".to_string(),
                http_method: HttpServerHttpMethod::GET,
                handler: HttpServerWrapperCallback {
                    uri: "wrap://ens/server.pwr-app.eth".to_string(),
                    method: "routeHome".to_string()
                }
            },
            HttpServerRoute {
                path: "/client/invoke".to_string(),
                http_method: HttpServerHttpMethod::POST,
                handler: HttpServerWrapperCallback {
                    uri: "wrap://ens/server.pwr-app.eth".to_string(),
                    method: "routeClientInvoke".to_string()
                }
            }
        ], 
        on_start: Some(
            HttpServerWrapperCallback {
                uri: "wrap://ens/server.pwr-app.eth".to_string(),
                method: "onStart".to_string()
            }
        ),
    }).unwrap();

    0
}

pub fn on_start(_: ArgsOnStart) -> bool {
    log("Server started".to_string());

    true
}

pub fn route_home(_: ArgsRouteHome) -> HttpServerResponse {
    log("Home route");

    HttpServerResponse {
        status_code: 200,
        headers: Some(vec![HttpServerKeyValuePair {
            key: "Content-Type".to_string(),
            value: "text/html".to_string(),
        }]),
        data: Some("Hello world!".as_bytes().to_vec()),
    }    
}

pub fn route_client_invoke(args: ArgsRouteClientInvoke) -> HttpServerResponse {
    log("Client invoke route");

    if args.request.body.is_none() {
        return to_error_response("Missing body".to_string());
    }

    let ClientInvokeBody { uri, method, args } = serde_json::from_str(
        &args.request.body
            .unwrap()
            .to_string()
    ).unwrap();

    log("/client/invoke ".to_string() + &uri + " " + &method);

    let result = WrapClientModule::invoke(&ArgsInvoke {
        uri,
        method,
        args: Some(args),
    });

    match result {
        Ok(result) => {
            if result.data.is_none() {
                return to_error_response("No data returned from client invoke".to_string());
            }
          
            to_json_response(WrapClientInvocationResult {
                data: result.data,
                error: None,
            })
        }
        Err(err) => {
            to_json_response(WrapClientInvocationResult {
                data: None,
                error: Some(err),
            })
        }
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
            json!(data)
                .to_string()
                .as_bytes()
                .to_vec()
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
        data: Some(message.as_bytes().to_vec()),
    }
}

fn log<S: Into<String>>(message: S) {
    LoggerModule::log(&ArgsLog {
        level: LoggerLogLevel::INFO,
        message: message.into(),
    }).unwrap();
}
