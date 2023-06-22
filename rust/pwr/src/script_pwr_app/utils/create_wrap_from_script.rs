use std::sync::Arc;

use polywrap_client::{core::{file_reader::SimpleFileReader, wrapper::Wrapper}, wasm::{wasm_wrapper::WasmWrapper, wasm_module::CompiledWasmModule}};

use super::{get_script_info::ScriptInfo, build_wasm_module_from_script};

pub fn create_wrap_from_script(script: &ScriptInfo, template_cid: &str) -> Result<Arc<dyn Wrapper>, String> {
  let module = build_wasm_module_from_script(script, template_cid);

  let compiled_module = CompiledWasmModule::try_from_bytecode(&module).unwrap();
  let wrap = WasmWrapper::new(compiled_module, Arc::new(SimpleFileReader::new()));

  return Ok(Arc::new(wrap));
}
