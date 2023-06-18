use std::{sync::Arc, fs, path::Path};

use polywrap_client::{core::{file_reader::SimpleFileReader, wrapper::Wrapper}, wasm::{wasm_wrapper::WasmWrapper, wasm_module::CompiledWasmModule}};

use crate::js_pwr_app::{ScriptLanguage, DEFAULT_JS_ENGINE_CID, DEFAULT_PY_ENGINE_CID};

use super::{replace_user_module, load_package_from_url, PackageContent, get_script_info::ScriptInfo, build_wasm_module_from_script};

pub fn create_wrap_from_script(script: &ScriptInfo, template_cid: &str) -> Result<Arc<dyn Wrapper>, String> {
  let module = build_wasm_module_from_script(script, template_cid);

  let compiled_module = CompiledWasmModule::from_byte_code(&module).unwrap();
  let wrap = WasmWrapper::new(compiled_module, Arc::new(SimpleFileReader::new()));

  return Ok(Arc::new(wrap));
}
