pub mod wrap;

use polywrap_wasm_rs::{wrap_debug_log, JSON::json};
pub use wrap::*;
use wrap::imported::*;

impl ModuleTrait for Module {
    fn start(
        args: ArgsStart,
        env: Env
    ) -> Result<StartResult, String> {
        log("Deploying to wrap.link...".to_string());
        HttpModule::post(&ArgsPost {
            url: env.wrap_link_url,
            request: Some(HttpRequest {
                body: Some(json!(DeployBody {
                    name: env.wrap_name,
                    uri: env.wrap_uri,
                    request_timeout: 10000,
                    on_start: args.on_start,
                    routes: args.routes,
                }).to_string()),
                headers: None,
                form_data: None,
                response_type: HttpResponseType::TEXT,
                timeout: None,
                url_params: None,
            }),
        }).unwrap();
        
        Ok(StartResult { ok: true })
    }
}

fn log<S: Into<String>>(message: S) {
    wrap_debug_log(&message.into());
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct DeployBody {
    pub name: String,
    pub uri: String,
    pub request_timeout: u32,
    pub routes: Vec<Route>,
    pub on_start: Option<WrapperCallback>,
}