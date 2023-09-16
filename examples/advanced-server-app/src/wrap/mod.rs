pub mod entry;
pub mod imported;

pub use imported::http_server_route::HttpServerRoute;
pub use imported::http_server_wrapper_callback::HttpServerWrapperCallback;
pub use imported::http_server_start_result::HttpServerStartResult;
pub use imported::http_server_response::HttpServerResponse;
pub use imported::http_server_key_value_pair::HttpServerKeyValuePair;
pub use imported::http_server_request::HttpServerRequest;
pub use imported::multipart_file_info::MultipartFileInfo;
pub use imported::multipart_key_value_pair::MultipartKeyValuePair;
pub use imported::key_value_store_key_value_pair::KeyValueStoreKeyValuePair;
pub use imported::invocation_context_resolution_context::InvocationContextResolutionContext;
pub use imported::http_server_http_method::{
    get_http_server_http_method_key,
    get_http_server_http_method_value,
    sanitize_http_server_http_method_value,
    HttpServerHttpMethod
};
pub use imported::http_server_module::HttpServerModule;
pub use imported::multipart_module::MultipartModule;
pub use imported::key_value_store_module::KeyValueStoreModule;
pub use imported::invocation_context_module::InvocationContextModule;
pub mod module;
pub use module::{
    Module,
    ModuleTrait,
    main_wrapped,
    ArgsMain,
    on_start_wrapped,
    ArgsOnStart,
    route_home_wrapped,
    ArgsRouteHome
};

// Override print!(...) & println!(...) macros
#[macro_export]
macro_rules! println { ($($args:tt)*) => { polywrap_wasm_rs::wrap_debug_log(format!($($args)*).as_str()); } }
#[macro_export]
macro_rules! print { ($($args:tt)*) => { polywrap_wasm_rs::wrap_debug_log(format!($($args)*).as_str()); } }
