extern crate pkg;

use std::env;

use pkg::build::git_commit;

fn main() {
    set_version();

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=.git/logs/HEAD");
}

/// Overrides the `CARGO_PKG_VERSION` environment variable, with a long version
/// string containing the hash and date of the latest git commit.
fn set_version() {
    let version = env::var("CARGO_PKG_VERSION")
        .expect("missing cargo version (CARGO_PKG_VERSION) environment variable");

    let long_version = match git_commit() {
        Some(commit) => {
            // Only take the first 7 characters of the hash.
            let hash = &commit.hash()[..7];
            let date = commit.date();

            format!("{} ({} {})", version, hash, date)
        }
        None => version,
    };

    println!("cargo:rustc-env=CARGO_PKG_VERSION={}", long_version);
}
