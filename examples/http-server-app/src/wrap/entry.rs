use crate::{
    main_wrapped,
    on_start_wrapped,
    route_home_wrapped,
    route_with_param_wrapped,
    route_with_query_wrapped,
    route_post_wrapped
};
use polywrap_wasm_rs::{
    abort,
    invoke,
    InvokeArgs,
};

#[no_mangle]
pub extern "C" fn _wrap_invoke(method_size: u32, args_size: u32, env_size: u32) -> bool {
    // Ensure the abort handler is properly setup
    abort::wrap_abort_setup();

    let args: InvokeArgs = invoke::wrap_invoke_args(method_size, args_size);
    let result: Vec<u8>;

    match args.method.as_str() {
        "main" => {
            result = main_wrapped(args.args.as_slice(), env_size);
        }
        "onStart" => {
            result = on_start_wrapped(args.args.as_slice(), env_size);
        }
        "routeHome" => {
            result = route_home_wrapped(args.args.as_slice(), env_size);
        }
        "routeWithParam" => {
            result = route_with_param_wrapped(args.args.as_slice(), env_size);
        }
        "routeWithQuery" => {
            result = route_with_query_wrapped(args.args.as_slice(), env_size);
        }
        "routePost" => {
            result = route_post_wrapped(args.args.as_slice(), env_size);
        }
        _ => {
            invoke::wrap_invoke_error(format!("Could not find invoke function {}", args.method));
            return false;
        }
    };
    invoke::wrap_invoke_result(result);
    return true;
}
