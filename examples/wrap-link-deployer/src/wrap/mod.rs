pub mod entry;
pub mod wrapper_callback;
pub use wrapper_callback::WrapperCallback;
pub mod request;
pub use request::Request;
pub mod response;
pub use response::Response;
pub mod route;
pub use route::Route;
pub mod start_result;
pub use start_result::StartResult;
pub mod key_value_pair;
pub use key_value_pair::KeyValuePair;
pub mod http_method;
pub use http_method::{
    get_http_method_key,
    get_http_method_value,
    sanitize_http_method_value,
    HttpMethod
};
pub mod env;
pub use env::Env;
pub mod imported;

pub use imported::http_response::HttpResponse;
pub use imported::http_request::HttpRequest;
pub use imported::http_form_data_entry::HttpFormDataEntry;
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
    start_wrapped,
    ArgsStart
};

// Override print!(...) & println!(...) macros
#[macro_export]
macro_rules! println { ($($args:tt)*) => { polywrap_wasm_rs::wrap_debug_log(format!($($args)*).as_str()); } }
#[macro_export]
macro_rules! print { ($($args:tt)*) => { polywrap_wasm_rs::wrap_debug_log(format!($($args)*).as_str()); } }
