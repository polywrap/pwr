use crate::script_pwr_app::{ScriptLanguage, DEFAULT_JS_ENGINE_CID, DEFAULT_PY_ENGINE_CID};

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
            ScriptLanguage::JavaScript => format!("ipfs/{DEFAULT_JS_ENGINE_CID}"),
            ScriptLanguage::Python => format!("ipfs/{DEFAULT_PY_ENGINE_CID}"),
        },
    );

    module
}
