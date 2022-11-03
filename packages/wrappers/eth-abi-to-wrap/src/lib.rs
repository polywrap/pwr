pub mod wrap;
use wrap::{*, imported::ArgsLog};
mod transform;
use transform::*;

pub fn main(_: ArgsMain) -> u8 {
  let _ = transform(vec![AbiItem {
    name: "test".to_string(),
    inputs: vec![],
    outputs: vec![],
    constant: false,
    payable: false,
    state_mutability: "nonpayable".to_string(),
    type_of: "function".to_string(),
  }]);  

  0
}

fn log(message: String) {
  LoggerModule::log(&ArgsLog {
    level: LoggerLogLevel::INFO,
    message,
  }).unwrap();
}
