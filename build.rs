
extern crate git2;

use std::env;
use std::process::Command;
use git2::Repository;

const REPO_URL: &str = "https://github.com/iris-edu/libmseed";
const BUILD_DIR: &str = "src/libmseed";
const GIT_REF: &str = "refs/remotes/origin/2.x"; // libmseed v3 broke the ABI

fn main() {
    let repo = match Repository::clone(REPO_URL, BUILD_DIR) {
        Ok(_repo) => _repo,
        Err(e) => match e.code() {
            git2::ErrorCode::Exists =>
                Repository::open(BUILD_DIR)
                    .expect("Failed to open existing repo"),
            _ => panic!("Failed to update/clone repo: {}", e),
        }
    };
    let branch = repo.revparse_single(GIT_REF).unwrap();
    repo.reset(&branch, git2::ResetType::Hard, None).unwrap();

    let path = std::fs::canonicalize(BUILD_DIR).unwrap();
    env::set_current_dir(&path).unwrap();
    Command::new("make").output().expect("make failed");

    println!("cargo:rustc-link-search={}", path.to_str().unwrap());
    println!("cargo:rustc-link-lib={}", "mseed");
}
