pub mod entry;
pub mod build_result;
pub use build_result::BuildResult;
pub mod deploy_result;
pub use deploy_result::DeployResult;
pub mod build_and_deploy_result;
pub use build_and_deploy_result::BuildAndDeployResult;
pub mod language;
pub use language::{
    get_language_key,
    get_language_value,
    sanitize_language_value,
    Language
};
pub mod imported;

pub use imported::http_request::HttpRequest;
pub use imported::http_form_data_entry::HttpFormDataEntry;
pub use imported::http_response::HttpResponse;
pub use imported::http_response_type::{
    get_http_response_type_key,
    get_http_response_type_value,
    sanitize_http_response_type_value,
    HttpResponseType
};
pub use imported::http_module::HttpModule;
pub mod module;
pub use module::{
    Module,
    ModuleTrait,
    build_manifest_wrapped,
    ArgsBuildManifest,
    build_module_wrapped,
    ArgsBuildModule,
    deploy_wrapped,
    ArgsDeploy,
    build_and_deploy_wrapped,
    ArgsBuildAndDeploy
};

// Override print!(...) & println!(...) macros
#[macro_export]
macro_rules! println { ($($args:tt)*) => { polywrap_wasm_rs::wrap_debug_log(format!($($args)*).as_str()); } }
#[macro_export]
macro_rules! print { ($($args:tt)*) => { polywrap_wasm_rs::wrap_debug_log(format!($($args)*).as_str()); } }
