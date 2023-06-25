use std::sync::Arc;

use polywrap_client::core::wrapper::Wrapper;

use super::{create_wrap_from_script, get_script_info};

pub fn create_wrap_from_file(
    script_path: &str,
    template_cid: Option<&str>,
) -> Result<Arc<dyn Wrapper>, String> {
    let script_info = get_script_info(script_path)?;

    create_wrap_from_script(&script_info, template_cid)
}
