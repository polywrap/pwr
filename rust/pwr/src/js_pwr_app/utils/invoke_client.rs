use std::sync::Arc;

use polywrap_client::{client::PolywrapClient, core::{file_reader::SimpleFileReader, wrapper::Wrapper}, wasm::wasm_wrapper::WasmWrapper};

pub fn invoke_client(uri: &str, method: &str, args: &[u8], client: Arc<PolywrapClient>, module: &[u8]) -> Result<Vec<u8>, String> {
  let wrap = WasmWrapper::new(module.to_vec(), Arc::new(SimpleFileReader::new()));

  let result = wrap.invoke(method, Some(args), None, client, None);

  result.map_err(|e| format!("Error invoking method: {}", e))
}
