use script_wrap_utils_wasm::{get_language_from_path, ScriptInfo};
use std::fs;

pub fn get_script_info_from_file(script_path: &str) -> Result<ScriptInfo, String> {
    let code = fs::read_to_string(script_path).map_err(|e| format!("Error reading file: {}", e))?;

    Ok(ScriptInfo {
        code,
        language: get_language_from_path(script_path)?,
    })
}
