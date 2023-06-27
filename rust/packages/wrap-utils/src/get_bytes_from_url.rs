pub fn get_bytes_from_url(url: &str) -> Result<Box<[u8]>, reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    let response = client.get(url).send()?;

    let bytes = response.bytes()?;
    Ok(bytes.to_vec().into_boxed_slice())
}
