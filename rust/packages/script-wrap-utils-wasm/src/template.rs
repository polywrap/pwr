use crate::constants::{
    DEFAULT_TEMPLATE_CID_128_KB, DEFAULT_TEMPLATE_CID_1_MB, DEFAULT_TEMPLATE_CID_256_KB,
    DEFAULT_TEMPLATE_CID_512_KB, DEFAULT_TEMPLATE_CID_64_KB,
};

#[derive(Debug, Clone, Copy)]
pub enum Size {
    Size64KB = 64000,
    Size128KB = 128000,
    Size256KB = 256000,
    Size512KB = 512000,
    Size1MB = 1000000,
}

pub fn get_template_size(user_module_size: usize) -> Option<Size> {
    let template_size = match user_module_size {
        0..=64000 => Size::Size64KB,
        64001..=128000 => Size::Size128KB,
        128001..=256000 => Size::Size256KB,
        256001..=512000 => Size::Size512KB,
        512001..=1000000 => Size::Size1MB,
        _ => return None,
    };

    Some(template_size)
}

pub fn get_template_endpoint(size: &Size) -> String {
    let template_cid = match size {
        Size::Size64KB => DEFAULT_TEMPLATE_CID_64_KB,
        Size::Size128KB => DEFAULT_TEMPLATE_CID_128_KB,
        Size::Size256KB => DEFAULT_TEMPLATE_CID_256_KB,
        Size::Size512KB => DEFAULT_TEMPLATE_CID_512_KB,
        Size::Size1MB => DEFAULT_TEMPLATE_CID_1_MB,
    };

    let gateway = "https://ipfs.wrappers.io/api/v0/cat?arg=";
    let template_wrap_endpoint = format!("{gateway}{template_cid}");

    template_wrap_endpoint
}
