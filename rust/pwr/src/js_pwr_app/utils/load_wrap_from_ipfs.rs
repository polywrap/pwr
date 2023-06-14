use std::sync::Arc;

use polywrap_client::{core::{file_reader::SimpleFileReader, wrapper::Wrapper}, wasm::wasm_wrapper::WasmWrapper};

use super::{load_package_from_url, PackageContent};

pub async fn load_wrap_from_ipfs(cid: &str) -> Arc::<dyn Wrapper> {
  let gateway = "https://ipfs.wrappers.io/api/v0/cat?arg=";
  let template_wrap_endpoint = format!("{gateway}{cid}");

  let PackageContent { module, .. } = load_package_from_url(&template_wrap_endpoint).await;

  Arc::new(WasmWrapper::new_compiled(module, Arc::new(SimpleFileReader::new())))
}
