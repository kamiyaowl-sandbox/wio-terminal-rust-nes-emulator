extern crate bindgen;

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());

    // bindgen
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .ctypes_prefix("cty")
        .generate()
        .expect("Unable to generate bindings");
    bindings.write_to_file(out.join("bindings.rs"))
        .expect("Coultn't write bindings.");
    println!("cargo:rustc-link-c-lib=wrapper.h");

    // memory.x gen
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());
}
