use std::sync::Arc;

use polywrap_client::{
    core::{file_reader::SimpleFileReader, wrapper::Wrapper},
    wasm::{wasm_module::CompiledWasmModule, wasm_wrapper::WasmWrapper},
};
use script_wrap_utils_wasm::{build_module_from_script, ScriptInfo};
use wrap_utils::get_bytes_from_url;

pub fn create_wrap_from_script(script: ScriptInfo) -> Result<Arc<dyn Wrapper>, String> {
    let module = build_module_from_script(script, get_bytes_from_url).map_err(|e| e.to_string())?;

    let compiled_module = CompiledWasmModule::try_from_bytecode(&module).unwrap();
    let wrap = WasmWrapper::new(compiled_module, Arc::new(SimpleFileReader::new()));

    Ok(Arc::new(wrap))
}
