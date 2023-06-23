use crate::constants::{ScriptLanguage, DEFAULT_JS_ENGINE_URI, DEFAULT_PY_ENGINE_URI};

use super::{
    get_script_info::ScriptInfo, load_package_from_url, replace_user_module, PackageContent,
};

pub fn build_wasm_module_from_script(script: &ScriptInfo, template_cid: &str) -> Vec<u8> {
    let gateway = "https://ipfs.wrappers.io/api/v0/cat?arg=";
    let template_wrap_endpoint = format!("{gateway}{template_cid}");
    let PackageContent { mut module, .. } = load_package_from_url(&template_wrap_endpoint);

    replace_user_module(
        &mut module,
        &script.code,
        match script.language {
            ScriptLanguage::JavaScript => DEFAULT_JS_ENGINE_URI.to_string(),
            ScriptLanguage::Python => DEFAULT_PY_ENGINE_URI.to_string(),
        },
    );

    module
}
