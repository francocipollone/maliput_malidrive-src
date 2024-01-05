use std::env;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

const MALIPUT_MALIDRIVE_PATH: &str = "dep/maliput_malidrive";

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=src/lib.rs");
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let install_dir = out_dir.join("install");
    fs::create_dir_all(&install_dir)?;

    println!("cargo:root={}", install_dir.display());

    println!("cargo:rustc-link-lib=dylib={}", "stdc++");
    println!("cargo:rustc-link-arg=-l{}", "stdc++");
    println!("cargo:rustc-link-search={}", env::var("OUT_DIR").unwrap());

    // build lib
    env::set_current_dir(MALIPUT_MALIDRIVE_PATH)
        .unwrap_or_else(|_| panic!("Unable to change directory to {}", MALIPUT_MALIDRIVE_PATH));

    let code = std::process::Command::new("bazel")
        .arg(format!("--output_base={}", install_dir.display()))
        .arg("build")
        .arg(format!("--symlink_prefix={}", install_dir.join("bazel-").display()))
        .arg("//...")
        .status()
        .expect("Failed to generate build script");
    if code.code() != Some(0) {
        panic!("Failed to generate build script");
    }
    let bazel_bin_dir = install_dir.join("bazel-bin");

    println!("cargo:rustc-env=INSTALL_DIR={}", bazel_bin_dir.display());

    //---Header files---
    let virtual_includes_path = bazel_bin_dir.join("_virtual_includes");
    let mut virtual_includes = Vec::new();
    for entry in fs::read_dir(virtual_includes_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            virtual_includes.push(path);
        }
    }

    // Add all the virtual includes to CXXBRIDGE_DIR.
    for (i, path) in virtual_includes.iter().enumerate() {
        println!("cargo:CXXBRIDGE_DIR{}={}", i, path.display());
    }
    Ok(())
}
