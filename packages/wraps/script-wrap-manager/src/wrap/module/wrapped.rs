use serde::{Deserialize, Serialize};
use polywrap_msgpack_serde::{
    from_slice,
    to_vec,
    wrappers::polywrap_json::JSONString,
    wrappers::polywrap_bigint::BigIntWrapper
};
use polywrap_wasm_rs::{
    BigInt,
    BigNumber,
    Map,
    JSON,
    wrap_load_env
};
use crate::module::{ModuleTrait, Module};
use crate::Language;
use crate::BuildResult;
use crate::DeployResult;
use crate::BuildAndDeployResult;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsBuildManifest {
    pub name: String,
    pub src: String,
    pub language: Language,
}

pub fn build_manifest_wrapped(args: &[u8], env_size: u32) -> Vec<u8> {
    match from_slice::<ArgsBuildManifest>(args) {
        Ok(args) => {
            let result = Module::build_manifest(ArgsBuildManifest {
                name: args.name,
                src: args.src,
                language: args.language,
            });
            match result {
                Ok(res) => {
                    to_vec(&res).unwrap()
                }
                Err(e) => {
                    panic!("{}", e.to_string())
                }
            }
        }
        Err(e) => {
            panic!("{}", e.to_string())
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsBuildModule {
    pub src: String,
    pub language: Language,
}

pub fn build_module_wrapped(args: &[u8], env_size: u32) -> Vec<u8> {
    match from_slice::<ArgsBuildModule>(args) {
        Ok(args) => {
            let result = Module::build_module(ArgsBuildModule {
                src: args.src,
                language: args.language,
            });
            match result {
                Ok(res) => {
                    to_vec(&res).unwrap()
                }
                Err(e) => {
                    panic!("{}", e.to_string())
                }
            }
        }
        Err(e) => {
            panic!("{}", e.to_string())
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsDeploy {
    pub name: String,
    pub src: String,
    pub language: Language,
}

pub fn deploy_wrapped(args: &[u8], env_size: u32) -> Vec<u8> {
    match from_slice::<ArgsDeploy>(args) {
        Ok(args) => {
            let result = Module::deploy(ArgsDeploy {
                name: args.name,
                src: args.src,
                language: args.language,
            });
            match result {
                Ok(res) => {
                    to_vec(&res).unwrap()
                }
                Err(e) => {
                    panic!("{}", e.to_string())
                }
            }
        }
        Err(e) => {
            panic!("{}", e.to_string())
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsBuildAndDeploy {
    pub name: String,
    pub src: String,
    pub language: Language,
}

pub fn build_and_deploy_wrapped(args: &[u8], env_size: u32) -> Vec<u8> {
    match from_slice::<ArgsBuildAndDeploy>(args) {
        Ok(args) => {
            let result = Module::build_and_deploy(ArgsBuildAndDeploy {
                name: args.name,
                src: args.src,
                language: args.language,
            });
            match result {
                Ok(res) => {
                    to_vec(&res).unwrap()
                }
                Err(e) => {
                    panic!("{}", e.to_string())
                }
            }
        }
        Err(e) => {
            panic!("{}", e.to_string())
        }
    }
}
