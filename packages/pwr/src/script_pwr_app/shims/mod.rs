pub fn get_js_shims() -> String {
    include_str!("./wrap.js").to_string() + "\n" +
    include_str!("./textEncoder.js") + "\n" +
    include_str!("./msgpack.js") + "\n" +
    include_str!("./fs.js") + "\n" +
    include_str!("./axios.js") + "\n" +
    include_str!("./subinvoke.js") + "\n" +
    include_str!("./console.js") + "\n" +
    include_str!("./util.js") + "\n" +
    include_str!("./require.js") + "\n"
} 
