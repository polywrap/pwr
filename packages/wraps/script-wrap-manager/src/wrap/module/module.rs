use polywrap_msgpack_serde::{
    wrappers::polywrap_json::JSONString,
    wrappers::polywrap_bigint::BigIntWrapper
};
use polywrap_wasm_rs::{
    BigInt,
    BigNumber,
    Map,
    JSON
};
use crate::{
    ArgsBuildManifest,
    ArgsBuildModule,
    ArgsDeploy,
    ArgsBuildAndDeploy,
};
use crate::Language;
use crate::BuildResult;
use crate::DeployResult;
use crate::BuildAndDeployResult;

pub struct Module;

pub trait ModuleTrait {
  fn build_manifest(args: ArgsBuildManifest) -> Result<BuildResult, String>;

  fn build_module(args: ArgsBuildModule) -> Result<BuildResult, String>;

  fn deploy(args: ArgsDeploy) -> Result<DeployResult, String>;

  fn build_and_deploy(args: ArgsBuildAndDeploy) -> Result<BuildAndDeployResult, String>;
}
