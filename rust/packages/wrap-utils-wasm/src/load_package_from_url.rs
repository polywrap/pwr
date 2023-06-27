use crate::PackageContent;

pub fn load_package_from_url<E>(
    path: &str,
    get_bytes_from_url: impl Fn(&str) -> Result<Box<[u8]>, E>,
) -> Result<PackageContent, E> {
    let manifest_url = format!("{}/wrap.info", path);
    let module_url = format!("{}/wrap.wasm", path);

    let manifest_bytes = get_bytes_from_url(&manifest_url)?;
    let module_bytes = get_bytes_from_url(&module_url)?;

    let result = PackageContent {
        manifest: manifest_bytes,
        module: module_bytes,
    };

    Ok(result)
}
