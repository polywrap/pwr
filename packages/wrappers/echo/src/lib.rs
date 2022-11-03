pub mod wrap;
use wrap::{*, imported::ArgsLog};

pub fn main(args: ArgsMain) -> u8 {
  LoggerModule::log(&ArgsLog {
    level: LoggerLogLevel::INFO,
    message: args.args.join(" "),
  }).unwrap();

  0
}
