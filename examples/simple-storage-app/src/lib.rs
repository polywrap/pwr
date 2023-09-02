pub mod wrap;
use polywrap_msgpack_serde::{from_slice, to_vec};
use polywrap_wasm_rs::wrap_debug_log;
use serde_bytes::ByteBuf;
pub use wrap::*;
use wrap::imported::{ArgsGet, ArgsSet};

impl ModuleTrait for Module {
    fn main(
        args: ArgsMain,
    ) -> Result<i32, String> {
        // Get first argument as command
        let command = args.args.get(0).unwrap_or(&"".to_string()).clone();

        // Get second argument as key
        let key = args.args.get(1).unwrap_or(&"".to_string()).clone();

        // Get third argument as value
        let value = args.args.get(2).unwrap_or(&"".to_string()).clone();

        match command.as_str() {
            "get" => {
                let args = ArgsGet {
                    key,
                };

                let result = KeyValueStoreModule::get(&args);

                if let Err(e) = result {
                    log(format!("Error: {}", e));
                    return Ok(1);
                }

                match result.unwrap() {
                    Some(value) => {
                        let value = String::from_utf8(value.to_vec()).unwrap();
                        log(format!("Value: {}", value));
                    },
                    None => {
                        log("Value not found".to_string());
                    }
                }
            },
            "set" => {
                let args = ArgsSet {
                    key,
                    value: ByteBuf::from(value.into_bytes()),
                };

                let result = KeyValueStoreModule::set(&args).unwrap();

                log(format!("Set: {}", result));
            },
            _ => {
                log("Invalid command".to_string());
                return Ok(1);
            }
        }

        return Ok(0);
    }
}

fn log<S: Into<String>>(message: S) {
    wrap_debug_log(&message.into());
}
