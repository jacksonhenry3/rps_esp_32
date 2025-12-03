use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("exp_table.rs");

    let mut content = String::from("static TABLE: [f32; 2001] = [\n");
    // In build.rs
    for i in 0..=2000 {
        let val = ((i as f32 - 1000.0) * 0.1f32).exp();

        // Check if the calculation resulted in infinity
        if val.is_infinite() {
            // Write the valid Rust constant for infinity
            content.push_str("    f32::INFINITY,\n");
        } else {
            // Write the standard number
            // {:?} preserves full precision better than {:.8}
            content.push_str(&format!("    {:?},\n", val));
        }
    }
    content.push_str("];");

    fs::write(&dest_path, content).unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}
