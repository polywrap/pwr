mod app_manager;
mod client;
mod deploy_wrap;
mod logger;
mod prompter;
mod script_pwr_app;
use std::{env, fs, error::Error, fmt::Display};

use polywrap_client::core::uri::Uri;

use app_manager::*;
use client::*;
use logger::*;

use deploy_wrap::*;
use prompter::*;
use script_pwr_app::*;
use script_wrap_utils_wasm::ScriptLanguage;

easy_error_string::use_easy_error_string!();

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() -> Result<(), StringError> {
    let _pwr_dir = get_pwr_dir().map_err(print_and_exit);

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
    .await?;

    std::process::exit(exit_code);
}

async fn internal_main(
    args: &[String],
    all_access_controlled_uris: Vec<String>,
    manager: AppManager,
    client: &impl CoreClient,
    logger: &impl Logger,
    prompter: &impl Prompter,
) -> Result<i32, StringError> {
    logger.debug(format!("Args: {:?}", args))?;

    let uri = &args[0];

    let uri = parse_uri(uri)?;

    logger.debug(format!("Parsed URI: {}", uri))?;

    Ok(match uri.to_string().as_str() {
        "wrap://pwr/js" => run_script_pwr_app(args, ScriptLanguage::JavaScript).await?,
        "wrap://pwr/py" => run_script_pwr_app(args, ScriptLanguage::Python).await?,
        "wrap://pwr/deploy" => deploy_wrap(args).await?,
        "wrap://pwr/version" => {
            println!("Version: {}", VERSION);
            0
        }
        _ => manager.run_app(
            &uri,
            args,
            client,
            prompter,
            logger,
            all_access_controlled_uris,
        )?
    })
}

fn get_pwr_dir() -> Result<String, StringError> {
    let app_dir = dirs::home_dir();

    if app_dir.is_none() {
        return Err(String::from("Error: Could not find home directory").into());
    }

    let app_dir = app_dir.easy_err()?.join(".pwr");

    // Check if the .pwr directory exists
    if !app_dir.exists() {
        fs::create_dir(&app_dir)?;
    }

    let pwr_dir = app_dir.into_os_string().into_string()
        .map_err(|e| e.to_string_lossy().to_string())?;

    Ok(pwr_dir)
}

fn parse_uri(uri: &String) -> Result<Uri, StringError> {
    let uri = if uri.ends_with(".eth") && !uri.starts_with("wrap://ens/") && !uri.starts_with("ens/") {
        Uri::try_from(format!("wrap://ens/{}", uri))?
    } else if uri.starts_with("Qm") {
        Uri::try_from(format!("wrap://ipfs/{}", uri))?
    } else if uri.starts_with("ipfs://") {
        Uri::try_from(format!("wrap://ipfs/{}", &uri["ipfs://".len()..uri.len()]))?
    } else if uri.starts_with('.') || uri.starts_with('/') {
        Uri::try_from(format!("wrap://file/{}", uri))?
    } else if !uri.contains('/') {
        Uri::try_from(format!("wrap://pwr/{}", uri))?
    } else {
        Uri::try_from(uri.clone())?
    };

    Ok(uri)
}

fn print_and_exit<T: ToString>(error: T) {
    println!("{}", error.to_string());
    std::process::exit(1);
}
