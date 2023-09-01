use std::fmt::Display;

use wrap_utils_wasm::{load_package_from_url, PackageContent};

use crate::{ScripWrapModuleBuilder, ScriptInfo, StringError};

#[derive(Debug, thiserror::Error)]
pub enum BuildModuleFromScriptError<E> where E: Display {
    #[error("Error loading package from url: {0}")]
    LoadPackageFromUrlError(E),
    #[error("Error building module: {0}")]
    BuildError(String),
}

impl<E> From<StringError> for BuildModuleFromScriptError<E> where E: Display {
    fn from(s: StringError) -> Self {
        BuildModuleFromScriptError::BuildError(s.to_string())
    }
}

pub fn build_module_from_script<E> (
    script: ScriptInfo,
    get_bytes_from_url: impl Fn(&str) -> Result<Box<[u8]>, E>,
) -> Result<Box<[u8]>, BuildModuleFromScriptError<E>> where E: Display {
    let builder = ScripWrapModuleBuilder::new(script);

    let PackageContent { module, .. } =
        load_package_from_url(&builder.template_endpoint, get_bytes_from_url)
        .map_err(|e| BuildModuleFromScriptError::LoadPackageFromUrlError(e))?;

    Ok(builder.build(&module)?)
}
