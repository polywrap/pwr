use std::{
    fs::{self, File},
    io::{self, Write},
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};
mod utils;
use clap::{arg, value_parser, Command};
use colored::Colorize;
use notify::RecursiveMode;
use notify_debouncer_mini::new_debouncer;
use polywrap_client::{client::PolywrapClient, core::uri::Uri};
use serde::{Deserialize, Serialize};
use serde_json::json;
use utils::*;
use wrap_manifest_schemas::{deserialize::deserialize_wrap_manifest, versions::WrapManifest};
const DEFAULT_TEMPLATE_CID: &str = "QmTzgDRWiSsux4463gz3h9kXfXkLaUq5gzqdJr7cSmG3Hx";
const DEFAULT_JS_ENGINE_CID: &str = "QmQGWzyd6bsbErRgSdNXsDskvy6VTSaLBUYjZK5zDZVZwC";
const DEFAULT_PY_ENGINE_CID: &str = "QmRhaCMunjt6DcgkSrwAxumWMHDK9UZATJgUrPaJJ2Zmb7";
pub enum ScriptLanguage {
    JavaScript,
    Python,
}

pub async fn run_script_pwr_app(args: &[String], language: ScriptLanguage) -> i32 {
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
        .get_matches_from(args);

    if let Some(matches) = matches.subcommand_matches("invoke") {
        let file = matches.get_one::<PathBuf>("file");
        let method = matches.get_one::<String>("method");

        let engine_cid = matches
            .get_one::<String>("engine").map(|x| x.as_str())
            .unwrap_or(match language {
                ScriptLanguage::JavaScript => DEFAULT_JS_ENGINE_CID,
                ScriptLanguage::Python => DEFAULT_PY_ENGINE_CID,
            });

        let template_cid = matches
            .get_one::<String>("template").map(|x| x.as_str())
            .unwrap_or(DEFAULT_TEMPLATE_CID);

        let is_release = matches.get_flag("release");

        return execute_eval_command(file, method, engine_cid, template_cid, is_release).await;
    } else if let Some(matches) = matches.subcommand_matches("build") {
        let file = matches.get_one::<PathBuf>("file").unwrap();
        let output = matches.get_one::<PathBuf>("output");

        let engine_cid = matches
            .get_one::<String>("engine")
            .map(|x| x.as_str())
            .unwrap_or(match language {
                ScriptLanguage::JavaScript => DEFAULT_JS_ENGINE_CID,
                ScriptLanguage::Python => DEFAULT_PY_ENGINE_CID,
            });

        let template_cid = matches
            .get_one::<String>("template")
            .map(|x| x.as_str())
            .unwrap_or(DEFAULT_TEMPLATE_CID);

        return execute_build_command(file, output, engine_cid, template_cid).await;
    } else if let Some(matches) = matches.subcommand_matches("deploy") {
        let file = matches.get_one::<PathBuf>("file");
        let output = matches.get_one::<PathBuf>("output");

        let engine_cid = matches
            .get_one::<String>("engine")
            .map(|x| x.as_str())
            .unwrap_or(match language {
                ScriptLanguage::JavaScript => DEFAULT_JS_ENGINE_CID,
                ScriptLanguage::Python => DEFAULT_PY_ENGINE_CID,
            });

        let template_cid = matches
            .get_one::<String>("template")
            .map(|x| x.as_str())
            .unwrap_or(DEFAULT_TEMPLATE_CID);

        return execute_deploy_command(file, output, engine_cid, template_cid).await;
    } else if let Some(matches) = matches.subcommand_matches("repl") {
        let file = matches.get_one::<PathBuf>("file");

        let engine_cid = matches
            .get_one::<String>("engine")
            .map(|x| x.as_str())
            .unwrap_or(match language {
                ScriptLanguage::JavaScript => DEFAULT_JS_ENGINE_CID,
                ScriptLanguage::Python => DEFAULT_PY_ENGINE_CID,
            });

        let template_cid = matches
            .get_one::<String>("template").map(|x| x.as_str())
            .unwrap_or(DEFAULT_TEMPLATE_CID);

        let is_release = matches.get_flag("release");
        let should_watch = matches.get_flag("watch");

        return execute_repl_command(file, engine_cid, template_cid, is_release, should_watch)
            .await;
    } else {
        println!("Command not found!");
    }

    1
}

async fn execute_eval_command(
    file: Option<&PathBuf>,
    method: Option<&String>,
    engine_cid: &str,
    template_cid: &str,
    is_release: bool,
) -> i32 {
    println!("VM loading...");
    let client = Arc::new(get_client_with_wraps(vec![]));
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
            eval_with_args(&args, client.clone(), engine_cid).await;
        } else {
            deploy_with_args(&args, template_cid, engine_cid, client.clone()).await;
        }
    }
}

async fn execute_build_command(
    file: &PathBuf,
    output: Option<&PathBuf>,
    _engine_cid: &str,
    template_cid: &str,
) -> i32 {
    println!("Building the WRAP...");

    let script = get_script_info(&file.to_string_lossy()).unwrap();
    let module = build_wasm_module_from_script(&script, template_cid);

    let default_output = PathBuf::from("./build");
    let output = output.unwrap_or(&default_output);

    if !Path::exists(output) {
        fs::create_dir(output).unwrap();
    }

    let wrap_name = Path::new(&file).file_stem().unwrap().to_str().unwrap();
    println!("WRAP name: {}", wrap_name);
    let manifest = WrapManifest {
        name: wrap_name.to_string(),
        type_: "wasm".to_string(),
        version: "0.1".to_string(),
        abi: wrap_manifest_schemas::versions::WrapManifest01Abi {
            ..Default::default()
        },
    };
    let manifest = rmp_serde::to_vec_named(&manifest).unwrap();

    let mut file = File::create("./build/wrap.info").unwrap();
    file.write_all(&manifest).unwrap();

    let mut file = File::create("./build/wrap.wasm").unwrap();
    file.write_all(&module).unwrap();

    println!("WRAP built successfully!");
    0
}

async fn execute_deploy_command(
    file: Option<&PathBuf>,
    output: Option<&PathBuf>,
    engine_cid: &str,
    template_cid: &str,
) -> i32 {
    if file.is_some() {
        execute_build_command(file.unwrap(), output, engine_cid, template_cid).await;
    }

    println!("Deploying the WRAP...");

    let output = output
        .unwrap_or(&PathBuf::from("./build"))
        .to_string_lossy()
        .into_owned();

    let cid = deploy_package_to_ipfs(&output).await.unwrap();
    println!("WRAP deployed to IPFS: wrap://ipfs/{}", cid);

    let manifest = fs::read(format!("{output}/wrap.info")).unwrap();
    let manifest = deserialize_wrap_manifest(&manifest, None).unwrap();

    deploy_uri_to_http(
        &manifest.name,
        &Uri::try_from("wrap://ipfs/".to_string() + &cid).unwrap(),
    )
    .await
    .unwrap();
    println!(
        "WRAP deployed to wrappers.dev registry: wrap://http/http.wrappers.dev/u/test/{}",
        &manifest.name
    );
    println!("WRAP deployed successfully!");
    
    0
}
async fn read_file_and_eval(
    file: Option<&PathBuf>,
    engine_cid: &str,
    _template_cid: &str,
    client: Arc<PolywrapClient>,
) -> String {
    let total_input = if let Some(file) = &file {
        if Path::exists(file) {
            fs::read_to_string(file).unwrap()
        } else {
            "".to_string()
        }
    } else {
        "".to_string()
    };
    if !total_input.is_empty() {
        invoke_eval(&total_input, vec![], engine_cid, client.clone()).await;
    }

    total_input
}

async fn execute_repl_command(
    file: Option<&PathBuf>,
    engine_cid: &str,
    template_cid: &str,
    is_release: bool,
    should_watch: bool,
) -> i32 {
    println!("REPL loading...");
    let client = Arc::new(get_client_with_wraps(vec![]));

    if should_watch {
        if let Some(file) = file {
            println!("Watching file: {:?}", file);
            read_file_and_eval(Some(file), engine_cid, template_cid, client.clone()).await;
            watch(file, engine_cid, template_cid, client.clone()).await;
            return 0;
        } else {
            write_err("File not specified");

            return 1;
        }
    }

    let mut total_input = read_file_and_eval(file, engine_cid, template_cid, client.clone()).await;

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
                if Path::exists(file) {
                    fs::read_to_string(file).unwrap()
                } else {
                    total_input
                }
            } else {
                total_input
            };

            match input.as_str() {
                "" => {
                    if !total_input.is_empty() {
                        invoke_eval(&total_input, vec![], engine_cid, client.clone()).await;
                    }

                    continue;
                }
                _ => {}
            };

            let new_total_input = total_input.clone() + "\n" + &input;
            let result = invoke_eval(&new_total_input, vec![], engine_cid, client.clone()).await;

            if result == 0 {
                total_input = new_total_input;
                if let Some(file) = &file {
                    let mut file = File::create(file).unwrap();
                    file.write_all(total_input.as_bytes()).unwrap();
                }
            }
        } else {
            panic!("Repl not yet supported in release mode");
        }
    }
}

async fn watch(path: &Path, engine_cid: &str, _template_cid: &str, client: Arc<PolywrapClient>) {
    // setup debouncer
    let (tx, rx) = std::sync::mpsc::channel();

    // No specific tickrate, max debounce time 2 seconds
    let mut debouncer = new_debouncer(Duration::from_millis(200), None, tx).unwrap();

    debouncer
        .watcher()
        .watch(path, RecursiveMode::Recursive)
        .unwrap();

    // print all events, non returning
    for result in rx {
        match result {
            Ok(events) => {
                for _ in events {
                    let total_input = if let Some(file) = &path.to_str() {
                        if Path::exists(path) {
                            fs::read_to_string(file).unwrap()
                        } else {
                            "".to_string()
                        }
                    } else {
                        "".to_string()
                    };
                    if !total_input.is_empty() {
                        invoke_eval(&total_input, vec![], engine_cid, client.clone()).await;
                    }
                }
            }
            Err(errors) => errors
                .iter()
                .for_each(|error| write_warn(format!("Watch error: {:?}", error))),
        }
    }
}

async fn deploy_with_args(
    args: impl AsRef<Vec<String>>,
    template_cid: &str,
    _engine_cid: &str,
    client: Arc<PolywrapClient>,
) -> i32 {
    let user_file = args.as_ref()[0].clone();
    let method = &args.as_ref()[1];

    let user_wrap = create_wrap_from_file(&user_file, template_cid).unwrap();

    let args = {
        let serialization_result = polywrap_msgpack::serialize(&AppArgs {
            args: args.as_ref().iter().skip(2).cloned().collect(),
        });
        

        match serialization_result {
            Ok(args) => args,
            Err(serialize_error) => {
                println!("{:?}", serialize_error);
                return 1;
            }
        }
    };

    let result = user_wrap
        .invoke(method, Some(&args), None, client, None)
        .map_err(|e| format!("Error invoking method: {}", e));

    if let Err(error) = result {
        println!("{:?}", error);
        return 0;
    }

    let result = msgpack_to_json_pretty(&result.unwrap());

    println!("{}", result);

    0
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
    engine_cid: &str,
) -> i32 {
    let user_file = args.as_ref()[0].clone();
    let method = &args.as_ref()[1];

    let args = json!({
        "args": args.as_ref().iter().skip(2).cloned().collect::<Vec<String>>(),
    });

    invoke_eval(
        &fs::read_to_string(user_file).unwrap(),
        vec![
            JsEngineGlobalVar {
                name: "wrap_method".to_string(),
                value: serde_json::to_string(method).unwrap(),
            },
            JsEngineGlobalVar {
                name: "wrap_args".to_string(),
                value: serde_json::to_string(&args).unwrap(),
            },
        ],
        engine_cid,
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
    engine_cid: &str,
    client: Arc<PolywrapClient>,
) -> i32 {
    let result = client.invoke::<JsEngineEvalResult>(
        &Uri::try_from(format!("ipfs/{}", engine_cid)).unwrap(),
        "evalWithGlobals",
        Some(
            &rmp_serde::encode::to_vec_named(&ArgsEvalWithGlobals {
                src: src.to_string(),
                globals,
            })
            .unwrap(),
        ),
        None,
        None,
    );

    let result = result.map_err(|e| format!("Error invoking method: {}", e));

    if let Err(error) = result {
        write_err(format!("Runtime error: {:?}", error));
        return 1;
    }

    let result = result.unwrap();

    if result.value.is_none() {
        if result.error.is_none() {
            write_warn("No value");
        } else {
            let error = result.error.unwrap();
            write_err(format!("Eval error: {:?}", error));
            return 1;
        }

        return 1;
    }

    let value = result.value.unwrap();
    let value = serde_json::from_str::<serde_json::Value>(&value).unwrap();
    let result = serde_json::to_string_pretty(&value).unwrap();

    write_ok(result);

    0
}

#[derive(Serialize, Deserialize)]
struct AppArgs {
    args: Vec<String>,
}

fn msgpack_to_json_pretty(bytes: &[u8]) -> String {
    let value: rmpv::Value = rmp_serde::from_slice(bytes).unwrap();
    serde_json::to_string_pretty(&value).unwrap()
}
