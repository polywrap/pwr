use std::fs;

use crate::{StringError, VERSION};

pub async fn migrate(_: &[String]) -> Result<i32, StringError> {
    println!("Nothing to migrate.");
    println!("Version: {}", VERSION);

    Ok(0)
}
