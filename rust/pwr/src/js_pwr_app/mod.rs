use std::{fs, io, sync::Arc};
mod utils;
use polywrap_client::{client::PolywrapClient, core::uri::Uri};
use utils::*;
use serde::{Deserialize, Serialize};

pub async fn run_js_pwr_app(
    args: &[String], 
) -> i32 {
    println!("Running JS PWR App");

    if args.len() == 1 {
        return run_eval_vm().await;
    }

    if args.len() == 2 {
        return run_eval_vm_for_file(args[1].clone()).await;
    }

    if args.len() == 3 {
        return run_eval_vm_for_file_and_method(args[1].clone(), args[2].clone()).await;
    }

    let client = Arc::new(get_client_with_wraps(vec![
        (Uri::try_from("mock/engine").unwrap(), load_wrap_from_ipfs("QmZwhcANeoZCn9An61d4uPfLtNznxyz85TsBf5AcqHeWVk").await),
    ]));

    return deploy_with_args(args.iter().skip(1).cloned().collect::<Vec<String>>().as_slice(), client.clone()).await;
} 

async fn run_vm() -> i32 {
    loop {
        let input = {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            let input = input.trim(); // Remove whitespace
            input.to_string()
        };

        match input.as_str() {
            "exit" => break,
            "deploy" => {
            return deploy().await;
            },
            _ => {
            println!("Invalid command");
            }
        }
    }

    return 0;
}

async fn run_eval_vm() -> i32 {
    println!("VM loading...");
    let client = Arc::new(get_client_with_wraps(vec![
        (Uri::try_from("mock/engine").unwrap(), load_wrap_from_ipfs("QmZwhcANeoZCn9An61d4uPfLtNznxyz85TsBf5AcqHeWVk").await),
    ]));
    loop {
        println!("VM running");
        let input = {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            let input = input.trim(); // Remove whitespace
            input.to_string()
        };

        deploy_with_args(input.split(" ").map(|s| s.to_string()).collect::<Vec<String>>().as_slice(), client.clone()).await;
    }
}

async fn run_eval_vm_for_file(file: impl AsRef<str>) -> i32 {
    println!("VM loading...");
    let client = Arc::new(get_client_with_wraps(vec![
        (Uri::try_from("mock/engine").unwrap(), load_wrap_from_ipfs("QmZwhcANeoZCn9An61d4uPfLtNznxyz85TsBf5AcqHeWVk").await),
    ]));
    loop {
        println!("VM running");
        let input = {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            let input = input.trim(); // Remove whitespace
            input.to_string()
        };

        deploy_with_args((file.as_ref().to_string() + " " + &input).split(" ").map(|s| s.to_string()).collect::<Vec<String>>().as_slice(), client.clone()).await;
    }
}

async fn run_eval_vm_for_file_and_method(file: impl AsRef<str>, method: impl AsRef<str>) -> i32 {
    println!("VM loading...");
    let client = Arc::new(get_client_with_wraps(vec![
        (Uri::try_from("mock/engine").unwrap(), load_wrap_from_ipfs("QmZwhcANeoZCn9An61d4uPfLtNznxyz85TsBf5AcqHeWVk").await),
    ]));
    loop {
        println!("VM running");
        let input = {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            let input = input.trim(); // Remove whitespace
            input.to_string()
        };

        deploy_with_args((file.as_ref().to_string() + " " + method.as_ref() + " " + &input).split(" ").map(|s| s.to_string()).collect::<Vec<String>>().as_slice(), client.clone()).await;
    }
}

async fn deploy() -> i32 {
    let input = {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim(); // Remove whitespace
        input.to_string()
    };

    let args = input.split(" ").map(|s| s.to_string()).collect::<Vec<String>>();

    let client = Arc::new(get_client_with_wraps(vec![
        (Uri::try_from("mock/engine").unwrap(), load_wrap_from_ipfs("QmZwhcANeoZCn9An61d4uPfLtNznxyz85TsBf5AcqHeWVk").await),
    ]));
    deploy_with_args(&args, client.clone()).await
}


async fn deploy_with_args(args: &[String], client: Arc<PolywrapClient>) -> i32 {
    let user_file = args[0].clone();
    let gateway = "https://ipfs.wrappers.io/api/v0/cat?arg=";
    let template_wrap_endpoint = format!("{gateway}QmcQ1UorqusoobdroMsSwrg1fnPnRyKLwiEHH2hEvNAfaH");

    let user_code = fs::read_to_string(user_file).unwrap();

    let PackageContent { mut module, .. } = load_package_from_url(&template_wrap_endpoint).await;

    replace_user_module(&mut module, &user_code);

    let method = args[1].clone();

    let args = {
        let serialization_result = polywrap_msgpack::serialize(&AppArgs {
          args: args.iter().skip(2).cloned().collect(),
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
