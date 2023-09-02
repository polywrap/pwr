pub mod wrap;
use polywrap_wasm_rs::wrap_debug_log;
pub use wrap::*;

impl ModuleTrait for Module {
    fn main(
        args: ArgsMain,
    ) -> Result<i32, String> {
        wrap_debug_log(&format!("These are the CLI arguments, {}!", args.args.join(" ")));

        return Ok(0);
    }
}
