use crate::constants::{ScriptLanguage, DEFAULT_JS_ENGINE_URI, DEFAULT_PY_ENGINE_URI, DEFAULT_TEMPLATE_CID_64_KB, DEFAULT_TEMPLATE_CID_128_KB, DEFAULT_TEMPLATE_CID_256_KB, DEFAULT_TEMPLATE_CID_512_KB, DEFAULT_TEMPLATE_CID_1_MB};

use super::{
    get_script_info::ScriptInfo, load_package_from_url, replace_user_module, PackageContent,
};

pub fn build_wasm_module_from_script(script: &ScriptInfo, _template_cid: Option<&str>) -> Vec<u8> {
    let engine_uri = match script.language {
        ScriptLanguage::JavaScript => DEFAULT_JS_ENGINE_URI,
        ScriptLanguage::Python => DEFAULT_PY_ENGINE_URI,
    };

    //We add 2 to the length to account for the 2 bytes that will be added to the module to separate the user script from the engine uri,
    //and the whole user module from the rest of the wasm module.
    let total_user_module_size = engine_uri.as_bytes().len() + script.code.as_bytes().len() + 2;

    let (template_cid, size) = match total_user_module_size {
        0..=64000 => (DEFAULT_TEMPLATE_CID_64_KB, 64000),
        0..=128000 => (DEFAULT_TEMPLATE_CID_128_KB, 128000),
        0..=256000 => (DEFAULT_TEMPLATE_CID_256_KB, 256000),
        0..=512000 => (DEFAULT_TEMPLATE_CID_512_KB, 512000),
        0..=1000000 => (DEFAULT_TEMPLATE_CID_1_MB, 1000000),
        _ => panic!("Script is too large to be built into a wrap."),
    };

    let gateway = "https://ipfs.wrappers.io/api/v0/cat?arg=";
    let template_wrap_endpoint = format!("{gateway}{template_cid}");
    let PackageContent { mut module, .. } = load_package_from_url(&template_wrap_endpoint);

    replace_user_module(
        &mut module,
        &script.code,
        engine_uri.to_string(),
        size
    );

    module
}
