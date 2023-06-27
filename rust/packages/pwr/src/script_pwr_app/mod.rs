mod get_client;
use get_client::get_client;

use clap::{arg, value_parser, Command};
use colored::Colorize;
use notify::RecursiveMode;
use notify_debouncer_mini::new_debouncer;
use polywrap_client::{client::PolywrapClient, core::uri::Uri};
use script_wrap_utils::{create_wrap_from_file, get_script_info_from_file};
use script_wrap_utils_wasm::{
    build_module_from_script, ScriptLanguage, DEFAULT_JS_ENGINE_URI, DEFAULT_PY_ENGINE_URI,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{
    fs::{self, File},
    io::{self, Write},
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};
use wrap_manifest_schemas::{deserialize::deserialize_wrap_manifest, versions::WrapManifest};
use wrap_utils::{deploy_package_to_ipfs, deploy_uri_to_http, get_bytes_from_url};

use crate::{StringError, OkOrErrorString, MapToErrorString};

pub async fn run_script_pwr_app(args: &[String], language: ScriptLanguage) -> Result<i32, StringError> {
    let matches = Command::new("script")
        .subcommand(
            Command::new("invoke")
                .about("invokes a method from a script")
                .arg(
                    arg!(-f --file <FILE> "Path to script file")
                        .required(true)
                        .value_parser(value_parser!(PathBuf)),
                )
                .arg(arg!(-m --method <METHOD> "Method to invoke").required(false))
                .arg(
                    arg!(-e --engine <ENGINE> "IPFS CID of the engine wrap to use").required(false),
                )
                .arg(
                    arg!(-t --template <TEMPLATE> "IPFS CID of the template wrap to use")
                        .required(false),
                )
                .arg(arg!(-r --release "Release").required(false)),
        )
        .subcommand(
            Command::new("build")
                .about("builds a into a wrap")
                .arg(
                    arg!(-f --file <FILE> "File to build")
                        .required(true)
                        .value_parser(value_parser!(PathBuf)),
                )
                .arg(
                    arg!(-o --output <OUTPUT> "Directory for the build artifacts")
                        .required(false)
                        .value_parser(value_parser!(PathBuf)),
                )
                .arg(
                    arg!(-e --engine <ENGINE> "IPFS CID of the engine wrap to use").required(false),
                )
                .arg(
                    arg!(-t --template <TEMPLATE> "IPFS CID of the template wrap to use")
                        .required(false),
                ),
        )
        .subcommand(
            Command::new("deploy")
                .about("deploys a wrap")
                .arg(
                    arg!(-f --file <FILE> "File to deploy")
                        .required(false)
                        .value_parser(value_parser!(PathBuf)),
                )
                .arg(
                    arg!(-e --engine <ENGINE> "IPFS CID of the engine wrap to use").required(false),
                )
                .arg(
                    arg!(-t --template <TEMPLATE> "IPFS CID of the template wrap to use")
                        .required(false),
                ),
        )
        .subcommand(
            Command::new("repl")
                .about("Starts the repl")
                .arg(
                    arg!(-f --file <FILE> "File to input")
                        .required(false)
                        .value_parser(value_parser!(PathBuf)),
                )
                .arg(
                    arg!(-e --engine <ENGINE> "IPFS CID of the engine wrap to use").required(false),
                )
                .arg(
                    arg!(-t --template <TEMPLATE> "IPFS CID of the template wrap to use")
                        .required(false),
                )
                .arg(arg!(-r --release "Release").required(false))
                .arg(arg!(-w --watch "Watch the file for changes").required(false)),
        )
        .subcommand(
            Command::new("new")
                .about("Creates a new script WRAP file")
                .arg(
                    arg!(-f --file <FILE> "File to input")
                        .required(true)
                        .value_parser(value_parser!(PathBuf)),
                ),
        )
        .get_matches_from(args);

    if let Some(matches) = matches.subcommand_matches("invoke") {
        let file = matches.get_one::<PathBuf>("file");
        let method = matches.get_one::<String>("method");

        let engine_uri = matches
            .get_one::<String>("engine")
            .map(|x| x.as_str())
            .unwrap_or(match language {
                ScriptLanguage::JavaScript => DEFAULT_JS_ENGINE_URI,
                ScriptLanguage::Python => DEFAULT_PY_ENGINE_URI,
            });

        let template_cid = matches.get_one::<String>("template").map(|x| x.as_str());

        let is_release = matches.get_flag("release");

        execute_eval_command(
            file,
            method,
            &Uri::try_from(engine_uri)?,
            template_cid,
            is_release,
        )
        .await
    } else if let Some(matches) = matches.subcommand_matches("build") {
        let file = matches.get_one::<PathBuf>("file").easy_err()?;
        let output = matches.get_one::<PathBuf>("output");

        let engine_uri = matches
            .get_one::<String>("engine")
            .map(|x| x.as_str())
            .unwrap_or(match language {
                ScriptLanguage::JavaScript => DEFAULT_JS_ENGINE_URI,
                ScriptLanguage::Python => DEFAULT_PY_ENGINE_URI,
            });

        let template_cid = matches.get_one::<String>("template").map(|x| x.as_str());

        execute_build_command(file, output, &Uri::try_from(engine_uri)?).await
    } else if let Some(matches) = matches.subcommand_matches("deploy") {
        let file = matches.get_one::<PathBuf>("file");
        let output = matches.get_one::<PathBuf>("output");

        let engine_uri = matches
            .get_one::<String>("engine")
            .map(|x| x.as_str())
            .unwrap_or(match language {
                ScriptLanguage::JavaScript => DEFAULT_JS_ENGINE_URI,
                ScriptLanguage::Python => DEFAULT_PY_ENGINE_URI,
            });

        let template_cid = matches.get_one::<String>("template").map(|x| x.as_str());

        execute_deploy_command(
            file,
            output,
            &Uri::try_from(engine_uri)?,
            template_cid,
        )
        .await
    } else if let Some(matches) = matches.subcommand_matches("repl") {
        let file = matches.get_one::<PathBuf>("file");

        let engine_uri = matches
            .get_one::<String>("engine")
            .map(|x| x.as_str())
            .unwrap_or(match language {
                ScriptLanguage::JavaScript => DEFAULT_JS_ENGINE_URI,
                ScriptLanguage::Python => DEFAULT_PY_ENGINE_URI,
            });

        let template_cid = matches.get_one::<String>("template").map(|x| x.as_str());

        let is_release = matches.get_flag("release");
        let should_watch = matches.get_flag("watch");

        execute_repl_command(
            file,
            &Uri::try_from(engine_uri)?,
            template_cid,
            is_release,
            should_watch,
        )
        .await
    } else if let Some(matches) = matches.subcommand_matches("new") {
        let file = matches.get_one::<PathBuf>("file").easy_err()?;

        execute_new_command(file, language).await
    } else {
        println!("Command not found!");
        Ok(1)
    }
}

async fn execute_eval_command(
    file: Option<&PathBuf>,
    method: Option<&String>,
    engine_uri: &Uri,
    template_cid: Option<&str>,
    is_release: bool,
) -> Result<i32, StringError> {
    println!("VM loading...");
    let client = Arc::new(get_client());
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

        let mut args = input
            .split(' ')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        if let Some(method) = &method {
            args.insert(0, method.to_string());
        }
        if let Some(file) = &file {
            args.insert(0, file.to_string_lossy().into_owned());
        }

        if !is_release {
            eval_with_args(&args, client.clone(), engine_uri).await;
        } else {
            deploy_with_args(&args, engine_uri, client.clone()).await;
        }
    }
}

async fn execute_build_command(file: &PathBuf, output: Option<&PathBuf>, _engine_uri: &Uri) -> Result<i32, StringError> {
    println!("Building the WRAP...");

    let script = get_script_info_from_file(&file.to_string_lossy())?;
    let module = build_module_from_script(script, get_bytes_from_url)?;

    let default_output = PathBuf::from("./build");
    let output = output.unwrap_or(&default_output);

    if !Path::exists(output) {
        fs::create_dir(output)?;
    }

    let wrap_name = Path::new(&file).file_stem().easy_err()?.to_str().easy_err()?;
    println!("WRAP name: {}", wrap_name);
    let manifest = WrapManifest {
        name: wrap_name.to_string(),
        type_: "wasm".to_string(),
        version: "0.1".to_string(),
        abi: wrap_manifest_schemas::versions::WrapManifest01Abi {
            ..Default::default()
        },
    };
    let manifest = rmp_serde::to_vec_named(&manifest)?;

    let mut file = File::create("./build/wrap.info")?;
    file.write_all(&manifest)?;

    let mut file = File::create("./build/wrap.wasm")?;
    file.write_all(&module)?;

    println!("WRAP built successfully!");
    Ok(0)
}

async fn execute_deploy_command(
    file: Option<&PathBuf>,
    output: Option<&PathBuf>,
    engine_uri: &Uri,
    template_cid: Option<&str>,
) -> Result<i32, StringError> {
    if file.is_some() {
        execute_build_command(file.easy_err()?, output, engine_uri).await;
    }

    println!("Deploying the WRAP...");

    let output = output
        .unwrap_or(&PathBuf::from("./build"))
        .to_string_lossy()
        .into_owned();

    let cid = deploy_package_to_ipfs(&output).await?;
    println!("WRAP deployed to IPFS: wrap://ipfs/{}", cid);

    let manifest = fs::read(format!("{output}/wrap.info"))?;
    let manifest = deserialize_wrap_manifest(&manifest, None)?;

    deploy_uri_to_http(
        &manifest.name,
        &Uri::try_from("wrap://ipfs/".to_string() + &cid)?,
    )
    .await
    ?;
    println!(
        "WRAP deployed to wrappers.dev registry: wrap://http/http.wrappers.dev/u/test/{}",
        &manifest.name
    );
    println!("WRAP deployed successfully!");

    Ok(0)
}
async fn read_file_and_eval(
    file: Option<&PathBuf>,
    engine_uri: &Uri,
    _template_cid: Option<&str>,
    client: Arc<PolywrapClient>,
) -> Result<String, StringError> {
    if let Some(file) = &file {
        if Path::exists(file) {
            let total_input = fs::read_to_string(file)?;

            if !total_input.is_empty() {
                println!("Evaluating file: {:?}...", file);
                invoke_eval(&total_input, vec![], engine_uri, client.clone()).await;
            }
        }
    }

    Ok("".to_string())
}

async fn execute_repl_command(
    file: Option<&PathBuf>,
    engine_uri: &Uri,
    template_cid: Option<&str>,
    is_release: bool,
    should_watch: bool,
) -> Result<i32, StringError> {
    println!("REPL loading...");
    let client = Arc::new(get_client());

    if let Some(file) = file {
        if !Path::exists(file) {
            println!("Creating file: {:?}", file);
            File::create(file)?;
            println!("Created.");
        }
    }

    println!("REPL loaded.");

    if should_watch {
        if let Some(file) = file {
            println!("Watching file: {:?}", file);
            read_file_and_eval(Some(file), engine_uri, template_cid, client.clone()).await;
            watch(file, engine_uri, template_cid, client.clone()).await;
            return Ok(0);
        } else {
            write_err("File not specified");

            return Ok(1);
        }
    }

    let mut total_input = read_file_and_eval(file, engine_uri, template_cid, client.clone()).await?;

    loop {
        let input = {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            let input = input.trim(); // Remove whitespace
            input.to_string()
        };

        if !is_release {
            total_input = if let Some(file) = &file {
                fs::read_to_string(file)?
            } else {
                total_input
            };

            match input.as_str() {
                "" => {
                    if !total_input.is_empty() {
                        invoke_eval(&total_input, vec![], engine_uri, client.clone()).await?;
                    }

                    continue;
                }
                _ => {}
            };

            let new_total_input = total_input.clone() + "\n" + &input;
            let result = invoke_eval(&new_total_input, vec![], engine_uri, client.clone()).await?;

            if result == 0 {
                total_input = new_total_input;
                if let Some(file) = &file {
                    let mut file = File::create(file)?;
                    file.write_all(total_input.as_bytes())?;
                }
            }
        } else {
            panic!("Repl not yet supported in release mode");
        }
    }
}

async fn execute_new_command(file: &PathBuf, language: ScriptLanguage) -> Result<i32, StringError> {
    if !Path::exists(file) {
        println!("Creating file: {:?}", file);
        File::create(file)?;
        println!("Created.");
    } else {
        write_err("File already exists");

        return Ok(1);
    }

    match language {
        ScriptLanguage::JavaScript => {
            let mut file = File::create(file)?;
            file.write_all(include_bytes!("./templates/javascript.js"))
                ?;
        }
        ScriptLanguage::Python => {
            let mut file = File::create(file)?;
            file.write_all(include_bytes!("./templates/python.py"))
                ?;
        }
    }

    Ok(0)
}

async fn watch(
    path: &Path,
    engine_uri: &Uri,
    template_cid: Option<&str>,
    client: Arc<PolywrapClient>,
) -> Result<(), StringError> {
    // setup debouncer
    let (tx, rx) = std::sync::mpsc::channel();

    // No specific tickrate, max debounce time 2 seconds
    let mut debouncer = new_debouncer(Duration::from_millis(200), None, tx)?;

    debouncer
        .watcher()
        .watch(path, RecursiveMode::Recursive)?;

    // print all events, non returning
    for result in rx {
        match result {
            Ok(events) => {
                for _ in events {
                    read_file_and_eval(
                        Some(&path.to_owned()),
                        engine_uri,
                        template_cid,
                        client.clone(),
                    )
                    .await?;
                }
            }
            Err(errors) => errors
                .iter()
                .for_each(|error| write_warn(format!("Watch error: {:?}", error))),
        }
    }

    Ok(())
}

async fn deploy_with_args(
    args: impl AsRef<Vec<String>>,
    _engine_uri: &Uri,
    client: Arc<PolywrapClient>,
) -> Result<i32, StringError> {
    let user_file = args.as_ref()[0].clone();
    let method = &args.as_ref()[1];

    let user_wrap = create_wrap_from_file(&user_file)?;

    let args = {
        let serialization_result = polywrap_msgpack::serialize(&AppArgs {
            args: args.as_ref().iter().skip(2).cloned().collect(),
        });

        match serialization_result {
            Ok(args) => args,
            Err(serialize_error) => {
                println!("{:?}", serialize_error);
                return Ok(1);
            }
        }
    };

    let result = user_wrap
        .invoke(method, Some(&args), None, client)
        .map_err(|e| format!("Error invoking method: {}", e));

    if let Err(error) = result {
        println!("{:?}", error);
        return Ok(0);
    }

    let result = msgpack_to_json_pretty(&result.easy_err()?);

    println!("{}", result?);

    Ok(0)
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsEvalWithGlobals {
    pub src: String,
    pub globals: Vec<JsEngineGlobalVar>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct JsEngineGlobalVar {
    pub name: String,
    pub value: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct JsEngineEvalResult {
    pub value: Option<String>,
    pub error: Option<String>,
}

async fn eval_with_args(
    args: impl AsRef<Vec<String>>,
    client: Arc<PolywrapClient>,
    engine_uri: &Uri,
) -> Result<i32, StringError> {
    let user_file = args.as_ref()[0].clone();
    let method = &args.as_ref()[1];

    let args = json!({
        "args": args.as_ref().iter().skip(2).cloned().collect::<Vec<String>>(),
    });

    invoke_eval(
        &fs::read_to_string(user_file)?,
        vec![
            JsEngineGlobalVar {
                name: "__wrap_method".to_string(),
                value: serde_json::to_string(method)?,
            },
            JsEngineGlobalVar {
                name: "__wrap_args".to_string(),
                value: serde_json::to_string(&args)?,
            },
        ],
        engine_uri,
        client,
    )
    .await
}

fn write_ok(str: impl AsRef<str>) {
    println!("{}", str.as_ref().green());
}

fn write_err(str: impl AsRef<str>) {
    println!("{}", str.as_ref().red());
}

fn write_warn(str: impl AsRef<str>) {
    println!("{}", str.as_ref().yellow());
}

async fn invoke_eval(
    src: &str,
    globals: Vec<JsEngineGlobalVar>,
    engine_uri: &Uri,
    client: Arc<PolywrapClient>,
) -> Result<i32, StringError> {
    let result = client.invoke::<JsEngineEvalResult>(
        engine_uri,
        "evalWithGlobals",
        Some(
            &rmp_serde::encode::to_vec_named(&ArgsEvalWithGlobals {
                src: src.to_string(),
                globals,
            })?,
        ),
        None,
        None,
    );

    let result = result.map_err(|e| format!("Error invoking method: {}", e));

    if let Err(error) = result {
        write_err(format!("Runtime error: {:?}", error));
        return Ok(1);
    }

    let result = result?;

    if result.value.is_none() {
        if result.error.is_none() {
            write_warn("No value");
        } else {
            let error = result.error.easy_err()?;
            write_err(format!("Eval error: {:?}", error));
            return Ok(1);
        }

        return Ok(1);
    }

    let value = result.value.easy_err()?;
    let value = serde_json::from_str::<serde_json::Value>(&value)?;
    let result = serde_json::to_string_pretty(&value)?;

    write_ok(result);

    Ok(0)
}

#[derive(Serialize, Deserialize)]
struct AppArgs {
    args: Vec<String>,
}

fn msgpack_to_json_pretty(bytes: &[u8]) -> Result<String, StringError> {
    let value: rmpv::Value = rmp_serde::from_slice(bytes)?;
    let result = serde_json::to_string_pretty(&value)?;

    Ok(result)
}
