mod wrap;
use std::collections::HashMap;
use std::sync::Arc;

use axum::{Router, async_trait};
use axum::extract::{FromRequest, Path, Query, State, RawBody};
use axum::http::{StatusCode, Request};
use axum::routing::{get, post, options, patch, delete, put};
use http::HeaderMap;
use hyper::{Body, body};
use serde::{Deserialize, Serialize};
use wrap::{*, module::*};
use polywrap_plugin::*;
use polywrap_msgpack_serde::from_slice;
use crate::wrap_info::get_manifest;
use crate::types::*;
use axum::response::Response;
use tokio::task;
#[derive(Debug)]
pub struct HttpServerPlugin;

#[plugin_impl]
impl Module for HttpServerPlugin {
    fn start(
        &mut self,
        args: &ArgsStart,
        invoker: Arc<dyn Invoker>,
    ) -> Result<StartResult, PluginError> {
        
        let mut app = Router::new();

        let deps = Dependencies {
            invoker: invoker.clone(),
        };

        let args = args.clone();
        for route in args.routes.iter() {
            let uri: Uri = route.handler.uri.clone().parse().unwrap();
            let method = route.handler.method.clone();
            let route2 = route.clone();
            let func = move |
                headers: HeaderMap,
                query:  Query<HashMap<String, String>>,
                path:  Path<HashMap<String, String>>,
                deps: State<Dependencies>,
                body: RawBody,
            | {
                handle_request(uri.clone(), method.clone(), headers, query, path, deps, body)
            };

            app = match route2.http_method {
                HttpMethod::GET => {
                    app.route(
                        &route2.path,
                        get(func).with_state(deps.clone()),
                    )
                },
                HttpMethod::POST => {
                    app.route(
                        &route2.path,
                        post(func).with_state(deps.clone()),
                    )
                },
                HttpMethod::PUT => {
                    app.route(
                        &route2.path,
                        put(func).with_state(deps.clone()),
                    )
                },
                HttpMethod::DELETE => {
                    app.route(
                        &route2.path,
                        delete(func).with_state(deps.clone()),
                    )
                },
                HttpMethod::PATCH => {
                    app.route(
                        &route.path,
                        patch(func).with_state(deps.clone()),
                    )
                },
                HttpMethod::OPTIONS => {
                    app.route(
                        &route2.path,
                        options(func).with_state(deps.clone()),
                    )
                },
                _ => {
                    return Err(PluginError::InvocationError {
                        exception: format!("Unsupported HTTP method: {:?}", route2.http_method),
                    });
                }
            };
        }

        let addr = std::net::SocketAddr::from(([127, 0, 0, 1], args.port));
       
        let server = axum::Server::bind(&addr)
            .serve(app.into_make_service());

        if let Some(handler) = args.on_start {
            let _ = deps.invoker.invoke_raw(
                &handler.uri.parse().unwrap(),
                &handler.method,
                None,
                None,
                None
            ).unwrap();
        }
        
        task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(
                server
            )
        }).unwrap();

        Ok(StartResult { ok: true })
    }
}

async fn handle_request(
    uri: Uri,
    method: String,
    headers: HeaderMap,
    Query(query_params): Query<HashMap<String, String>>,
    Path(path_params): Path<HashMap<String, String>>,
    deps: State<Dependencies>,
    RawBody(body): RawBody,
) -> Result<Response<Body>, StatusCode> {
    println!("Query Params {:?}", query_params);
    println!("Path Params {:?}", path_params);
            
    let body: Option<Vec<u8>> = 
        body::to_bytes(body).await.map(|x| x.to_vec()).ok();
        
    let result = deps.invoker.invoke_raw(
        &uri,
        &method,
        Some(&to_vec(&RequestArgs {
            request: crate::types::Request {
                headers: headers.into_iter().map(|(k, v)| KeyValuePair {
                    key: k.map(|x| x.to_string()).unwrap_or("".to_string()),
                    value: v.to_str().unwrap().to_string(),
                }).collect(),
                params: path_params.into_iter().map(|(k, v)| KeyValuePair {
                    key: k,
                    value: v,
                }).collect(),
                query: query_params.into_iter().map(|(k, v)| KeyValuePair {
                    key: k,
                    value: v,
                }).collect(),
                body: body.map(|x| ByteBuf::from(x))
            }
        }).unwrap()),
        None,
        None
    ).unwrap();

    let crate::types::Response { body: response_body, status_code, headers } =  from_slice(&result).unwrap();
    
    let mut builder = Response::builder()
        .status(StatusCode::from_u16(status_code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR));

    for header in headers.unwrap_or(vec![]).into_iter() {
        builder = builder.header(header.key, header.value);
    }

    let bytes: Vec<u8> = response_body.unwrap_or(ByteBuf::from(vec![])).into_vec();

    let response: Response<Body> = builder.body(Body::from(bytes)).unwrap();
    Ok(response)
}

#[derive(Clone, Serialize, Deserialize)]
struct RequestArgs {
    request: crate::types::Request,
}

#[derive(Clone)]
struct Dependencies {
    invoker: Arc<dyn Invoker>,
}

#[derive(thiserror::Error, Debug)]
pub enum HttpPluginError {
    #[error("Error sending request: `{0}`")]
    SendRequestError(String),
}

impl From<HttpPluginError> for PluginError {
    fn from(e: HttpPluginError) -> Self {
        PluginError::InvocationError {
            exception: e.to_string(),
        }
    }
}
