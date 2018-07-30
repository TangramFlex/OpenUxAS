use std::process::Command;
use std::env;
use std::path::PathBuf;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    let mut meson_build = PathBuf::new();
    meson_build.push(out_dir);
    meson_build.push("build");
    
    let st = Command::new("meson")
        .args(&[meson_build.to_str().unwrap(), "-Dforce_dep_download=true"])
        .status()
        .unwrap();
    assert!(st.success());

    let st = Command::new("ninja")
        .args(&["all", "-j2"])
        .current_dir(&meson_build)
        .status()
        .unwrap();
    assert!(st.success());

    link_dep(&meson_build, "", "uxas");
    link_dep(&meson_build, "/src/Services", "services");
    link_dep(&meson_build, "/src/Tasks", "tasks");
    link_dep(&meson_build, "/src/LMCP", "lmcp");
    link_dep(&meson_build, "/src/Communications", "uxas_messages");
    link_dep(&meson_build, "/3rd/zeromq-4.2.3", "zeromq");
    link_dep(&meson_build, "/3rd/zyre-2.0.0", "zyre");
    link_dep(&meson_build, "/3rd/czmq-4.0.2", "czmq");
    link_dep(&meson_build, "/src/Utilities", "utilities");
    link_dep(&meson_build, "/3rd/SQLiteCpp-1.3.1", "sqlitecpp");
    link_dep(&meson_build, "/3rd/sqlite-amalgamation-3120200", "sqlite3");
    link_dep(&meson_build, "/3rd/PugiXML", "pugixml");
    link_dep(&meson_build, "/src/VisilibityLib", "visilibity");
    link_dep(&meson_build, "/3rd/serial-1.2.1", "serial");
    link_dep(&meson_build, "/src/Plans", "plans");
    link_dep(&meson_build, "/3rd/TinyGPS", "tinygps");
    link_dep(&meson_build, "/3rd/minizip-5b7b3e5c4bf1ce8fd4d3da2057760972832a972f", "minizip");
    link_dep(&meson_build, "/3rd/zlib-1.2.8", "zlib");
    link_dep(&meson_build, "/src/DPSS", "dpss");
    println!("cargo:rustc-link-lib=boost_filesystem");
    println!("cargo:rustc-link-lib=boost_regex");
    println!("cargo:rustc-link-lib=boost_system");
    plat::link_glu();
    plat::link_cpp();
}

fn link_dep(base: &PathBuf, path: &str, libname: &str) {
    println!("cargo:rustc-link-search=native={}{}", base.to_str().unwrap(), path);
    println!("cargo:rustc-link-lib=static={}", libname);
}

#[cfg(target_os = "linux")]
mod plat {
    pub fn link_glu() {
        println!("cargo:rustc-link-lib=GLU");
    }

    pub fn link_cpp() {
        println!("cargo:rustc-link-lib=stdc++");
    }
}

#[cfg(target_os = "macos")]
mod plat {
    pub fn link_glu() {
        println!("cargo:rustc-link-lib=framework=OpenGL");
    }

    // on Mac, libstdc++ is a weird old GNU version that doesn't
    // support C++11, so we use the clang's libc++ instead
    pub fn link_cpp() {
        println!("cargo:rustc-link-lib=c++");
    }
}
