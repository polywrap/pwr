use std::sync::Arc;

use polywrap_client::core::wrapper::Wrapper;

use crate::{create_wrap_from_script, StringError};

use super::get_script_info_from_file::get_script_info_from_file;

pub fn create_wrap_from_file(script_path: &str) -> Result<Arc<dyn Wrapper>, StringError> {
    let wrap = create_wrap_from_script(get_script_info_from_file(script_path)?)?;

    Ok(wrap)
}
