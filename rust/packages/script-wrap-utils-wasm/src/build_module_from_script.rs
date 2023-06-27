use wrap_utils_wasm::{load_package_from_url, PackageContent};

use crate::{ScripWrapModuleBuilder, ScriptInfo};

pub fn build_module_from_script<E>(
    script: ScriptInfo,
    get_bytes_from_url: impl Fn(&str) -> Result<Box<[u8]>, E>,
) -> Result<Box<[u8]>, E> {
    let builder = ScripWrapModuleBuilder::new(script);

    let PackageContent { module, .. } =
        load_package_from_url(&builder.template_endpoint, get_bytes_from_url)?;

    Ok(builder.build(&module))
}
