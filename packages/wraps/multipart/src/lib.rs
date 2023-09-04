mod wrap;
use std::io::{self, Read};

use multipart::server::Multipart;
use serde_bytes::ByteBuf;
use wrap::*;

use getrandom::register_custom_getrandom;

fn custom_getrandom(_: &mut [u8]) -> Result<(), getrandom::Error> {
    return Ok(());
}

register_custom_getrandom!(custom_getrandom);

impl ModuleTrait for Module {
    fn get_files(args: ArgsGetFiles) -> Result<Vec<FileInfo>, String> {
        let boundary = args.headers.iter()
            .find(|x| x.key.to_lowercase() == "Content-Type".to_lowercase())
            .and_then(|x| get_boundary_from_content_type(&x.value))
            .unwrap();
        let raw_body = args.body.to_vec();

        let mut multipart = Multipart::with_body(io::Cursor::new(raw_body), boundary);

        let mut files: Vec<FileInfo> = Vec::new();

        while let Some(mut field) = multipart.read_entry().unwrap() {
            if let Some(filename) = field.headers.filename.clone() {
                let mut buffer: Vec<u8> = Vec::new();
                field.data.read_to_end(&mut buffer).unwrap();

                files.push(FileInfo {
                    name: filename,
                    content: ByteBuf::from(buffer),
                });
            }
        }

        Ok(files)    
    }
}

fn get_boundary_from_content_type(content_type: &str) -> Option<String> {
    let mut boundary = None;
    let parts: Vec<&str> = content_type.split(';').collect();

    for part in parts {
        let part = part.trim();
        if part.starts_with("boundary=") {
            boundary = Some(part[9..].to_string());
            break;
        }
    }

    boundary
}