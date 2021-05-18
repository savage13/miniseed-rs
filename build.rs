

extern crate git2;
extern crate bindgen;

use std::path::{Path, PathBuf};
use git2::Repository;
use std::env;
use std::process::Command;

const REPO_URL: &str = "https://github.com/iris-edu/libmseed";
const BUILD_DIR: &str = "src/libmseed";
const GIT_REF: &str = "refs/remotes/origin/2.x"; // libmseed v3 broke the ABI

fn fetch_libmseed(dir: &str) {
    println!("fetching libmseed");
    let repo = match Repository::clone(REPO_URL, dir) {
        Ok( creepo ) => { creepo },
        Err(e) => panic!("Failed to update/clone repo: {}", e),
    };

    let branch = repo.revparse_single(GIT_REF).unwrap();
    repo.reset(&branch, git2::ResetType::Hard, None).unwrap();
}

fn make_libmseed(dir: &str) {
    let path = std::fs::canonicalize(dir).unwrap();
    let _ok = env::set_current_dir(&path).is_ok();

    let _output = Command::new("make").output().expect("make failed");
    let path = std::fs::canonicalize("../..").unwrap();
    let _ok = env::set_current_dir(&path).is_ok();
}

fn main() {

    if !Path::new(BUILD_DIR).is_dir() {
        fetch_libmseed(BUILD_DIR);
    }

    make_libmseed(BUILD_DIR);

    println!("cargo:rustc-link-lib=static=mseed");
    println!("cargo:rustc-link-search=native=../libmseed/");
    println!("Generate");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-I../libmseed")
        .allowlist_type("MS.*")
        .allowlist_var("MS_.*")
        .allowlist_var("HPT.*")
        .allowlist_function("ms_.*")
        .allowlist_function("msr_.*")
        .allowlist_function("mst_.*")
        .generate()
        .expect("Unable to generate bindings");
    println!("version: {:?}", bindgen::clang_version());
    println!("Path");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("couldn't write bindings");
}

    
