pub fn replace_user_module(module: &mut [u8], user_code: &str) {
    let target = include_str!("./index.js").as_bytes().to_vec();
    // Define the replacement sequence.
    let mut my_var: Vec<u8> = user_code.as_bytes().to_vec();

    // Pad my_var with zeroes if it is shorter than target.
    while my_var.len() < target.len() {
        my_var.push(0);
    }

    for i in 0..(module.len() - target.len() + 1) {
        if &module[i..i+target.len()] == target.as_slice() {
            // If a match is found, replace the window with my_var.
            module[i..i+target.len()].clone_from_slice(&my_var);
            break;
        }
    }
}
