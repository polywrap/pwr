use std::{fs, path::Path};

use crate::script_pwr_app::ScriptLanguage;

pub struct ScriptInfo {
    pub code: String,
    pub language: ScriptLanguage,
}

pub fn get_script_info(script_path: &str) -> Result<ScriptInfo, String> {
    let code = fs::read_to_string(script_path).map_err(|e| format!("Error reading file: {}", e))?;
    let language = match Path::new(&script_path).extension() {
        Some(ext) => match ext.to_str().unwrap() {
            "js" => ScriptLanguage::JavaScript,
            "py" => ScriptLanguage::Python,
            ext => {
                return Err(format!(
                    "File {} has an unsupported extension: {ext}",
                    script_path
                ))
            }
        },
        None => return Err(format!("File {} has no extension", script_path)),
    };

    Ok(ScriptInfo { code, language })
}
