use std::fs;
use std::io;
use std::path::Path;

use mago_prelude::Prelude;

pub fn main() -> io::Result<()> {
    println!("cargo:rustc-env=TARGET={}", std::env::var("TARGET").unwrap());
    println!("cargo:rerun-if-changed=crates/prelude/assets");
    println!("cargo:rerun-if-changed=build.rs");

    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR environment variable not set");
    let prelude_file = Path::new(&out_dir).join("prelude.bin");

    let prelude = Prelude::build();
    let prelude_bin = prelude.encode().expect("Failed to encode the prelude");

    fs::write(prelude_file, prelude_bin)?;

    Ok(())
}
