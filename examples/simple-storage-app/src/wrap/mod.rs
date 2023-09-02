pub mod entry;
pub mod imported;

pub use imported::key_value_store_key_value_pair::KeyValueStoreKeyValuePair;
pub use imported::key_value_store_module::KeyValueStoreModule;
pub mod module;
pub use module::{
    Module,
    ModuleTrait,
    main_wrapped,
    ArgsMain
};

// Override print!(...) & println!(...) macros
#[macro_export]
macro_rules! println { ($($args:tt)*) => { polywrap_wasm_rs::wrap_debug_log(format!($($args)*).as_str()); } }
#[macro_export]
macro_rules! print { ($($args:tt)*) => { polywrap_wasm_rs::wrap_debug_log(format!($($args)*).as_str()); } }
