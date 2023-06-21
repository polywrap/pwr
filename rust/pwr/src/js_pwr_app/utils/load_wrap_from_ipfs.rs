use std::sync::Arc;

use polywrap_client::{core::{file_reader::SimpleFileReader, wrapper::Wrapper}, wasm::{wasm_wrapper::WasmWrapper, wasm_module::CompiledWasmModule}};

use super::{load_package_from_url, PackageContent};

pub async fn load_wrap_from_ipfs(cid: &str) -> Arc::<dyn Wrapper> {
  let gateway = "https://ipfs.wrappers.io/api/v0/cat?arg=";
  let template_wrap_endpoint = format!("{gateway}{cid}");

  let PackageContent { module, .. } = load_package_from_url(&template_wrap_endpoint);


  let compiled_module = CompiledWasmModule::try_from_bytecode(&module).unwrap();
  let wrap = WasmWrapper::new(compiled_module, Arc::new(SimpleFileReader::new()));
  Arc::new(wrap)
}
