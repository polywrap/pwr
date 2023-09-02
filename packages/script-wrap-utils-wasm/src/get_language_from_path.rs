use std::path::Path;

use crate::{ScriptLanguage, StringError, OkOrErrorString};

pub fn get_language_from_path(script_path: &str) -> Result<ScriptLanguage, StringError> {
    let language = match Path::new(&script_path).extension() {
        Some(ext) => match ext.to_str().ok_or_str("Error reading file extension")? {
            "js" => ScriptLanguage::JavaScript,
            "py" => ScriptLanguage::Python,
            ext => {
                return Err(StringError::new(format!(
                    "File {} has an unsupported extension: {ext}",
                    script_path
                )))
            }
        },
        None => return Err(StringError::new(format!("File {} has no extension", script_path))),
    };

    Ok(language)
}
