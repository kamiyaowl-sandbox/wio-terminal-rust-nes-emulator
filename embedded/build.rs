extern crate bindgen;

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

extern crate cc;

fn main() {
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    // build
    println!("cargo:rustc-link-lib=wrapper");
    cc::Build::new()
        .target("thumbv7em-none-eabi")
        .file("wrapper.h")
        .warnings(true)
        .flag("-Wall")
        .flag("-Wextra")
        .compile("wrapper");

    // bindgen
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .ctypes_prefix("cty")
        .generate()
        .expect("Unable to generate bindings");
    bindings.write_to_file(out.join("bindings.rs"))
        .expect("Coultn't write bindings.");

    // memory.x gen
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());

    // Only re-run the build script when memory.x is changed,
    // instead of when any part of the source code changes.
    println!("cargo:rerun-if-changed=memory.x");
}
