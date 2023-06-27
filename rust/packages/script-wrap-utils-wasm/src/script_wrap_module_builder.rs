use crate::{
    constants::{DEFAULT_JS_ENGINE_URI, DEFAULT_PY_ENGINE_URI},
    get_template_endpoint, get_template_size, replace_user_module, template, ScriptInfo,
    ScriptLanguage,
};

pub struct ScripWrapModuleBuilder {
    code: String,
    engine_uri: String,
    template_size: template::Size,
    pub template_endpoint: String,
}

impl ScripWrapModuleBuilder {
    pub fn new(script: ScriptInfo) -> Self {
        let engine_uri = match script.language {
            ScriptLanguage::JavaScript => DEFAULT_JS_ENGINE_URI,
            ScriptLanguage::Python => DEFAULT_PY_ENGINE_URI,
        };

        //We add 2 to the length to account for the 2 bytes that will be added to the module to separate the user script from the engine uri,
        //and the whole user module from the rest of the wasm module.
        let user_module_size = engine_uri.as_bytes().len() + script.code.as_bytes().len() + 2;

        let template_size = get_template_size(user_module_size)
            .expect("User module should be smaller than template module");
        let template_endpoint = get_template_endpoint(&template_size);

        Self {
            code: script.code,
            engine_uri: engine_uri.to_string(),
            template_size,
            template_endpoint,
        }
    }

    pub fn build(&self, template_module: &[u8]) -> Box<[u8]> {
        let mut module = template_module.to_vec();

        replace_user_module(
            &mut module,
            &self.code,
            &self.engine_uri,
            self.template_size as usize,
        );

        module.into_boxed_slice()
    }
}
