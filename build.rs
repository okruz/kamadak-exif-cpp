extern crate cbindgen;

use std::{env, path::PathBuf};

fn main() {
    let cbindgen_output = PathBuf::from(env::var("CBINDGEN_OUTPUT").unwrap());
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let config = cbindgen::Config::from_root_or_default(manifest_dir.clone());
    cbindgen::Builder::new()
        .with_crate(manifest_dir)
        .with_config(config)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(cbindgen_output);
}
