use crate::StringError;

pub fn replace_user_module(
    module: &mut [u8],
    user_code: &str,
    engine_url: &str,
    padded_user_module_size: usize,
) -> Result<(), StringError> {
    // 1 + 0 bytes until padded_user_module_size
    let mut target = vec![0u8; padded_user_module_size];
    target[0] = b'1';

    // Define the replacement sequence.
    let mut injected_bytes = engine_url.as_bytes().to_vec();
    injected_bytes.push(0);
    injected_bytes.extend_from_slice(user_code.as_bytes());

    // Pad my_var with zeroes if it is shorter than target.
    while injected_bytes.len() < target.len() {
        injected_bytes.push(0);
    }

    let mut has_replaced = false;
    for i in 0..(module.len() - target.len() + 1) {
        if &module[i..i + target.len()] == target.as_slice() {
            // If a match is found, replace the window with injected_bytes.
            module[i..i + target.len()].clone_from_slice(&injected_bytes);
            has_replaced = true;
            break;
        }
    }

    match has_replaced {
        true => Ok(()),
        false => {
            Err(StringError::new(
                "Could not find the target sequence in the module. Must be an error in the template wrap.",
            ))
        }
    }
}
