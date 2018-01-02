// build.rs

use std::process::Command;
use std::env;
use std::path::PathBuf;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    let mut meson_build = PathBuf::new();
    meson_build.push(out_dir);
    meson_build.push("build");
    
    Command::new("meson.py").args(&[meson_build.to_str().unwrap()])
                       .status().unwrap();
    Command::new("ninja").args(&["libuxas.a"])
                      .current_dir(&meson_build)
                      .status().unwrap();

    println!("cargo:rustc-link-search=native={}", meson_build.to_str().unwrap());
    println!("cargo:rustc-link-search=native={}/src/Services", meson_build.to_str().unwrap());
    println!("cargo:rustc-link-lib=static=uxas");
    println!("cargo:rustc-link-lib=static=services");
    println!("cargo:rustc-link-lib=stdc++");
}
