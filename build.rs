

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

    let search_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let search_dir = Path::new(&search_dir).join("src").join("libmseed");

    println!("cargo:rustc-link-lib=static=mseed");
    println!("cargo:rustc-link-search=native={}", search_dir.display());
    println!("Searching for libraries at: {}", search_dir.display());
    println!("Generate bindings.rs");

    let path = env::current_dir().unwrap();
    println!("The current directory is {}", path.display());

    let path : PathBuf = [BUILD_DIR, "libmseed.h"].iter().collect();
    if !path.exists() {
        panic!("libmseed header file: libmseed.h does not exist");
    }

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-I./src/libmseed")
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

    
