mod wrap;
use std::collections::HashMap;
use std::sync::Arc;

use axum::{Router, async_trait};
use axum::extract::FromRequest;
use axum::http::{StatusCode, Request};
use axum::routing::{get, post, options, patch, delete, put};
use hyper::Body;
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
                PathParams(req): PathParams<Dependencies>,
            | {
                handle_request(uri.clone(), method.clone(), req)
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
       
        task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(
                axum::Server::bind(&addr)
                    .serve(app.into_make_service())
            )
        }).unwrap();
        
        Ok(StartResult { ok: true })
    }
}

async fn handle_request(
    uri: Uri,
    method: String,
    req: (HashMap<String, String>, Dependencies),
) -> Result<Response<Body>, StatusCode> {
    let (params, deps) = req;
    let params: Vec<crate::types::KeyValuePair> =  params.into_iter().map(|(k, v)| {
        crate::types::KeyValuePair {
            key: k,
            value: v,
        }
    }).collect();

            
    let result = deps.invoker.invoke_raw(
        &uri,
        &method,
        Some(&to_vec(&RequestArgs {
            request: crate::types::Request {
                params,
                query: vec![],
                body: None,
            }
        }).unwrap()),
        None,
        None
    ).unwrap();

    let crate::types::Response { data, status_code, headers } =  from_slice(&result).unwrap();
    
    let mut builder = Response::builder()
        .status(StatusCode::from_u16(status_code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR));

    for header in headers.unwrap_or(vec![]).into_iter() {
        builder = builder.header(header.key, header.value);
    }

    let bytes: Vec<u8> = data.unwrap_or(ByteBuf::from(vec![])).into_vec();

    let response: Response<Body> = builder.body(Body::from(bytes)).unwrap();
    Ok(response)
}


struct PathParams<S>((HashMap<String, String>, S));

#[async_trait]
impl<S, B> FromRequest<S, B> for PathParams<S>
where
    for<'a> B: Send + 'a,
    S: Clone + Send + Sync,
{
    type Rejection = std::convert::Infallible;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let mut map = HashMap::new();
        if let Some(route_params) = req.extensions().get::<axum::extract::RawPathParams>().clone() {
            for (name, value) in route_params.iter() {
                map.insert(name.to_string(), value.to_string());
            }
        }

        Ok(PathParams::<S>((map, state.clone())))
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct RequestArgs {
    request: crate::types::Request,
}

#[derive(Clone)]
struct Dependencies {
    invoker: Arc<dyn Invoker>,
}

pub async fn home() -> Result<String, StatusCode> {
    let page = format!("Version: ");

    Ok(page)
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
