use std::fs;

use wrap_manifest_schemas::deserialize::deserialize_wrap_manifest;

use crate::StringError;

pub fn get_name_from_wrap(path: &str) -> Result<String, StringError> {
  let manifest = fs::read(format!("{path}/wrap.info"))?;
  let manifest = deserialize_wrap_manifest(&manifest, None)?;

  Ok(manifest.name)
}
