pub mod wrap;
use polywrap_wasm_rs::JSON::json;
use serde::{Serialize, Deserialize};
use wrap::{*, imported::{ArgsLog, ArgsResolve, ArgsCreate}};

#[derive(Serialize, Deserialize)]
struct ResolveResponse {
  path: String
}

pub fn main(args: ArgsMain) -> u8 {
    let port = if args.args.len() > 0 {
        args.args[0].parse::<u32>().unwrap()
    } else {
        8080
    };

    IpfsModule::create(&ArgsCreate {}).unwrap();

    HttpServerModule::start(&imported::http_server_module::ArgsStart {
        port,
        request_timeout: 10000, 
        routes: vec![
            HttpServerRoute {
                path: "/".to_string(),
                http_method: HttpServerHttpMethod::GET,
                handler: HttpServerWrapperCallback {
                    uri: "wrap://ens/ipfs-gateway.pwr-app.eth".to_string(),
                    method: "routeHome".to_string()
                }
            },
            HttpServerRoute {
                path: "/api/v0/resolve".to_string(),
                http_method: HttpServerHttpMethod::GET,
                handler: HttpServerWrapperCallback {
                    uri: "wrap://ens/ipfs-gateway.pwr-app.eth".to_string(),
                    method: "routeResolve".to_string()
                }
            }
        ], 
        on_start: Some(
            HttpServerWrapperCallback {
                uri: "wrap://ens/ipfs-gateway.pwr-app.eth".to_string(),
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

pub fn route_resolve(args: ArgsRouteResolve) -> HttpServerResponse {
    log("Resolve route");

    let name = args.request.query.iter().find(|kv| kv.key == "arg").unwrap().value.clone();

    log("/api/v0/resolve ".to_string() + &name);

    let result = IpfsModule::resolve(&ArgsResolve {
        name,
        options: Some(
            IpfsAbortOptions {
              timeout: Some(15000),
            }
        )
    });

    match result {
        Ok(result) => {
            log("/api/v0/resolve Ok ".to_string() + &result);
            to_json_response(ResolveResponse {
                path: result
            })
        }
        Err(err) => {
            log("/api/v0/resolve Error".to_string());
            to_error_response(err)
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
