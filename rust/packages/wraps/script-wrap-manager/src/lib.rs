mod wrap;
use script_wrap_utils_wasm::{ScriptInfo, ScriptLanguage, build_module_from_script};
use serde::{Deserialize, Serialize};
use wrap::{imported::*, module::serialization::ArgsDeploy, *};
mod utils;
use utils::*;

easy_error_string::use_easy_error_string!();

impl ModuleTrait for Module {
    fn build_manifest(args: ArgsBuildManifest) -> Result<BuildResult, String> {
        let manifest = build_wrap_manifest(&args.name);

        Ok(BuildResult {
            data: Some(manifest),
            error: None,
        })
    }

    fn build_module(args: ArgsBuildModule) -> Result<BuildResult, String> {
        let lang = match args.language {
            Language::JavaScript => ScriptLanguage::JavaScript,
            Language::Python => ScriptLanguage::Python,
            _ => return Err(String::from("Invalid language")),
        };

        let script = ScriptInfo {
            code: args.src,
            language: lang,
        };

        let module = build_module_from_script(script, get_bytes_from_url).map_err_str()?;

        Ok(BuildResult {
            data: Some(module.to_vec()),
            error: None,
        })
    }

    fn deploy(args: ArgsDeploy) -> Result<DeployResult, String> {
        let lang = match args.language {
            Language::JavaScript => ScriptLanguage::JavaScript,
            Language::Python => ScriptLanguage::Python,
            _ => return Err(String::from("Invalid language")),
        };

        let script = ScriptInfo {
            code: args.src,
            language: lang,
        };

        let manifest = build_wrap_manifest(&args.name);

        let module = build_module_from_script(script, get_bytes_from_url).map_err_str()?;

        let result = deploy_package_to_ipfs(&manifest, &module)?;

        Ok(DeployResult {
            uri: Some(format!("wrap://ipfs/{}", result)),
            error: None,
        })
    }
}

#[derive(Serialize, Deserialize)]
struct WrapManifest01Abi;
#[derive(Serialize, Deserialize)]
struct WrapManifest01 {
    abi: WrapManifest01Abi,
    name: String,
    type_: String,
    version: String,
}

// fn build_module_from_script(script: ScriptInfo) -> Result<Box<[u8]>, StringError> {
//     let builder = ScripWrapModuleBuilder::new(script);

//     let PackageContent { module, .. } =
//         load_package_from_url::<()>(&builder.template_endpoint, |url| {
//             Ok(get_bytes_from_url(&url)?)
//         })
//         .unwrap();

//     Ok(builder.build(&module)?)
// }

fn build_wrap_manifest(wrap_name: &str) -> Vec<u8> {
    // let manifest = WrapManifest01 {
    //     abi: WrapManifest01Abi,
    //     name: wrap_name.to_string(),
    //     type_: "wasm".to_string(),
    //     version: "0.1".to_string(),
    // };

    // let manifest = rmp_serde::to_vec_named(&manifest).unwrap();

    // manifest

    let manifest = include_bytes!("./wrap.info");
    manifest.to_vec()
}
