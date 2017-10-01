extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=static=mseed");
    println!("cargo:rustc-link-search=native=../libmseed/");
    println!("Generate");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-I../libmseed")
        .whitelisted_type("MS.*")
        .whitelisted_var("MS_.*")
        .whitelisted_var("HPT.*")
        .whitelisted_function("ms_.*")
        .whitelisted_function("msr_.*")
        .whitelisted_function("mst_.*")
        .generate()
        .expect("Unable to generate bindings");
    println!("version: {:?}", bindgen::clang_version());
    println!("Path");
    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    println!("Write");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Unable to write bindings");

    println!("Done");
}
