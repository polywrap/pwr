pub mod entry;
pub mod file_info;
pub use file_info::FileInfo;
pub mod key_value_pair;
pub use key_value_pair::KeyValuePair;

pub mod module;
pub use module::{
    Module,
    ModuleTrait,
    get_files_wrapped,
    ArgsGetFiles
};

// Override print!(...) & println!(...) macros
#[macro_export]
macro_rules! println { ($($args:tt)*) => { polywrap_wasm_rs::wrap_debug_log(format!($($args)*).as_str()); } }
#[macro_export]
macro_rules! print { ($($args:tt)*) => { polywrap_wasm_rs::wrap_debug_log(format!($($args)*).as_str()); } }
