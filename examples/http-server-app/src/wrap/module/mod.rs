pub mod wrapped;
pub use wrapped::{
    main_wrapped,
    ArgsMain,
    on_start_wrapped,
    ArgsOnStart,
    route_home_wrapped,
    ArgsRouteHome,
    route_with_param_wrapped,
    ArgsRouteWithParam,
    route_with_query_wrapped,
    ArgsRouteWithQuery,
    route_post_wrapped,
    ArgsRoutePost
};

pub mod module;
pub use module::*;
