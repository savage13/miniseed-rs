
extern crate git2;

use std::env;
use std::process::Command;
use git2::Repository;


fn main() {
    let url = "https://github.com/iris-edu/libmseed";
    match Repository::clone(url, "src/libmseed") {
        Ok(_repo) => {},
        Err(e) => {
            match e.code() {
                git2::ErrorCode::Exists => {
                    println!("directory exists");
                },
                _ => panic!("Failed to update/clone repo: {}", e),
            }
        }
    };

    let dir = "src/libmseed";
    let path = std::fs::canonicalize(dir).unwrap();
    let _ok = env::set_current_dir(&path).is_ok();
    let _output = Command::new("make").output().expect("make failed");

    println!("cargo:rustc-link-search={}", path.to_str().unwrap());
    println!("cargo:rustc-link-lib={}", "mseed");
}
