mod app_manager;
mod client;
mod deploy_wrap;
mod logger;
mod prompter;
mod script_pwr_app;
use std::{env, fmt::Display, fs};

use polywrap_client::core::uri::Uri;

use app_manager::*;
use client::*;
use logger::*;

use deploy_wrap::*;
use prompter::*;
use script_pwr_app::*;
use script_wrap_utils_wasm::ScriptLanguage;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() {
    let _pwr_dir = get_pwr_dir().map_err(print_and_exit).unwrap();

    let args: Vec<String> = env::args().collect();
    let args = &args[1..];

    let client = PwrClient::new();
    let manager = AppManager::new();
    let prompter = PrompterMock::new();
    let logger = LoggerMock::new();

    let all_access_controlled_uris: Vec<String> = vec![];

    let exit_code = internal_main(
        args,
        all_access_controlled_uris,
        manager,
        &client,
        &logger,
        &prompter,
    )
    .await;

    std::process::exit(exit_code);
}

pub async fn internal_main(
    args: &[String],
    all_access_controlled_uris: Vec<String>,
    manager: AppManager,
    client: &impl CoreClient,
    logger: &impl Logger,
    prompter: &impl Prompter,
) -> i32 {
    logger.debug(format!("Args: {:?}", args)).unwrap();

    let uri = &args[0];

    let uri = parse_uri(uri);

    logger.debug(format!("Parsed URI: {}", uri)).unwrap();

    match uri.to_string().as_str() {
        "wrap://pwr/js" => return run_script_pwr_app(args, ScriptLanguage::JavaScript).await,
        "wrap://pwr/py" => return run_script_pwr_app(args, ScriptLanguage::Python).await,
        "wrap://pwr/deploy" => return deploy_wrap(args).await,
        "wrap://pwr/version" => {
            println!("Version: {}", VERSION);
            return 0;
        }
        _ => {}
    }

    manager.run_app(
        &uri,
        args,
        client,
        prompter,
        logger,
        all_access_controlled_uris,
    )
}

fn get_pwr_dir() -> Result<String, String> {
    let app_dir = dirs::home_dir();

    if app_dir.is_none() {
        return Err(String::from("Error: Could not find home directory"));
    }

    let app_dir = app_dir.unwrap().join(".pwr");

    // Check if the .pwr directory exists
    if !app_dir.exists() {
        fs::create_dir(&app_dir).unwrap();
    }

    let pwr_dir = app_dir.into_os_string().into_string().unwrap();

    Ok(pwr_dir)
}

fn parse_uri(uri: &String) -> Uri {
    if uri.ends_with(".eth") && !uri.starts_with("wrap://ens/") && !uri.starts_with("ens/") {
        Uri::try_from(format!("wrap://ens/{}", uri)).unwrap()
    } else if uri.starts_with("Qm") {
        return Uri::try_from(format!("wrap://ipfs/{}", uri)).unwrap();
    } else if uri.starts_with("ipfs://") {
        return Uri::try_from(format!("wrap://ipfs/{}", &uri["ipfs://".len()..uri.len()])).unwrap();
    } else if uri.starts_with('.') || uri.starts_with('/') {
        return Uri::try_from(format!("wrap://file/{}", uri)).unwrap();
    } else if !uri.contains('/') {
        return Uri::try_from(format!("wrap://pwr/{}", uri)).unwrap();
    } else {
        return Uri::try_from(uri.clone()).unwrap();
    }
}

fn print_and_exit<T: Display>(error: T) {
    println!("{}", error);
    std::process::exit(1);
}
