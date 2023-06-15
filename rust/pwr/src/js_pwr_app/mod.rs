use std::{fs, io, sync::Arc, path::PathBuf};
mod utils;
use clap::{arg, Command, value_parser};
use polywrap_client::{client::PolywrapClient, core::uri::Uri};
use utils::*;
use serde::{Deserialize, Serialize};

const DEFAULT_JS_ENGINE_CID: &str = "QmZwhcANeoZCn9An61d4uPfLtNznxyz85TsBf5AcqHeWVk";
const DEFAULT_TEMPLATE_CID: &str = "QmbRVyK6yGu11iCiUqC2YeRQRo9xMCk2jVKxMQcuuaYgmc";

pub async fn run_js_pwr_app(
    args: &[String], 
) -> i32 {
    println!("Running JS PWR App");

    let matches = Command::new("js") 
        .subcommand(
            Command::new("eval")
                .about("evaluates a file")
                .arg(arg!(-f --file <FILE> "File to input").required(true).value_parser(value_parser!(PathBuf)))
                .arg(arg!(-m --method <METHOD> "Method to execute").required(false))
                .arg(arg!(-e --engine <ENGINE> "IPFS CID of the engine wrap to use").required(false))
                .arg(arg!(-t --template <TEMPLATE> "IPFS CID of the template wrap to use").required(false))
        )
        .get_matches_from(args);

    if let Some(matches) = matches.subcommand_matches("eval") {
        let file = matches.get_one::<PathBuf>("file");
        let method = matches.get_one::<String>("method");
        let engine_cid = matches.get_one::<String>("engine")
            .and_then(|x| Some(x.as_str()))
            .unwrap_or(DEFAULT_JS_ENGINE_CID);
        let template_cid = matches.get_one::<String>("template")
            .and_then(|x| Some(x.as_str()))
            .unwrap_or(DEFAULT_TEMPLATE_CID);

        return run_eval_vm(file, method, &engine_cid, &template_cid).await;
    } else {
        println!("Command not found!");
    }

    return 1;
} 

async fn run_eval_vm(file: Option<&PathBuf>, method: Option<&String>, engine_cid: &str, template_cid: &str) -> i32 {
    println!("VM loading...");
    let client = Arc::new(get_client_with_wraps(vec![
        (Uri::try_from("mock/engine").unwrap(), load_wrap_from_ipfs(engine_cid).await),
    ]));
    loop {
        println!("VM ready");
        let input = {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            let input = input.trim(); // Remove whitespace
            input.to_string()
        };

         let mut args = input.split(" ").map(|s| s.to_string()).collect::<Vec<String>>();
         if let Some(method) = &method {
             args.insert(0, method.to_string());
         }
         if let Some(file) = &file {
             args.insert(0, file.to_string_lossy().into_owned());
         }

        deploy_with_args(&args, template_cid, client.clone()).await;
    }
}

async fn deploy_with_args(args: impl AsRef<Vec<String>>, template_cid: &str, client: Arc<PolywrapClient>) -> i32 {
    let user_file = args.as_ref()[0].clone();
    let method = &args.as_ref()[1];
   
    let gateway = "https://ipfs.wrappers.io/api/v0/cat?arg=";
    let template_wrap_endpoint = format!("{gateway}{template_cid}");

    let user_code = fs::read_to_string(user_file).unwrap();
    let user_code = format!("{user_code}");

    let PackageContent { mut module, .. } = load_package_from_url(&template_wrap_endpoint).await;

    replace_user_module(&mut module, &user_code);

    let args = {
        let serialization_result = polywrap_msgpack::serialize(&AppArgs {
          args: args.as_ref().iter().skip(2).cloned().collect(),
        });
        let args = match serialization_result {
            Ok(args) => args,
            Err(serialize_error) => {
                println!("{:?}", serialize_error);
                return 1;
            }
        };

        args
    };
    let result = invoke_client("mock/test", &method, &args, client, &module);

    if let Err(error) = result {
        println!("{:?}", error);
        return 1;
    }

    let result = msgpack_to_json_pretty(&result.unwrap());

    println!("{}", result);

    return 0;
}


#[derive(Serialize, Deserialize)] 
struct AppArgs {
    args: Vec<String>,
}

fn msgpack_to_json_pretty(bytes: &[u8]) -> String {
    let value: rmpv::Value = rmp_serde::from_slice(&bytes).unwrap();
    serde_json::to_string_pretty(&value).unwrap()
}
