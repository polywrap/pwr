use std::path::Path;

use crate::ScriptLanguage;

pub fn get_language_from_path(script_path: &str) -> Result<ScriptLanguage, String> {
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

    Ok(language)
}
