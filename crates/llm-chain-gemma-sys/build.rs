#![allow(clippy::uninlined_format_args)]

extern crate cc;

use std::env;

fn main() {
    #[cfg(target_os = "windows")]
    {
        // Gemma.cpp does not support MSBuild at this point --
        // it does support clang-cl though. At this time, Windows
        // is out of the support because of this.
        // See: https://github.com/google/gemma.cpp/pull/6
        cc::Build::new()
            .cpp(true)
            .file("src/bindings_win.cc")
            .std("c++17")
            .compile("bindings");
        return;
    }
    let target = env::var("TARGET").unwrap();
    // Link C++ standard library
    if let Some(cpp_stdlib) = get_cpp_link_stdlib(&target) {
        println!("cargo:rustc-link-lib=dylib={}", cpp_stdlib);
        println!("cargo:rustc-link-arg=-l{}", cpp_stdlib);
    }
    // Link macOS Accelerate framework for matrix calculations
    if target.contains("apple") {
        println!("cargo:rustc-link-lib=framework=Accelerate");
    }
    println!("cargo:rustc-link-search={}", env::var("OUT_DIR").unwrap());
    println!("cargo:rustc-link-lib=static=gemma");
    println!("cargo:rustc-link-lib=static=hwy");
    println!("cargo:rustc-link-lib=static=hwy_contrib");
    println!("cargo:rustc-link-lib=static=sentencepiece");
    println!("cargo:rustc-link-lib=static=bindings");
    println!("cargo:rerun-if-changed=wrapper.h");

    // stop if we're on docs.rs
    if env::var("DOCS_RS").is_ok() {
        return;
    }

    // Run cmake to generate build files.
    env::set_current_dir("gemma.cpp").expect("Unable to change directory to gemma.cpp");
    env::set_current_dir("build").expect("Unable to change directory to gemma.cpp build");

    env::set_var("CXXFLAGS", "-fPIC");
    env::set_var("CFLAGS", "-fPIC");

    let mut code = std::process::Command::new("cmake");
    let code = code
        .arg("..")
        .arg("-DCMAKE_BUILD_TYPE=Release")
        .arg("-DBUILD_SHARED_LIBS=OFF")
        .arg("-DWEIGHT_TYPE=hwy::bfloat16_t")
        .arg("-DSPM_ENABLE_SHARED=OFF");
    let code = code.status().expect("Failed to generate build script");
    if code.code() != Some(0) {
        panic!("Failed to generate build script");
    }

    // Build binary.
    #[allow(clippy::suspicious_command_arg_space)]
    let code = std::process::Command::new("cmake")
        .arg("--build")
        .arg(".")
        .arg("--config")
        .arg("Release")
        .arg("--target")
        .arg("libgemma")
        .status()
        .expect("Failed to build lib");
    if code.code() != Some(0) {
        panic!("Failed to build lib");
    }

    // move libllama.a to where Cargo expects it (OUT_DIR)
    std::fs::copy(
        "libgemma.a",
        format!("{}/libgemma.a", env::var("OUT_DIR").unwrap()),
    )
    .expect("Failed to copy lib");

    std::fs::copy(
        "_deps/highway-build/libhwy.a",
        format!("{}/libhwy.a", env::var("OUT_DIR").unwrap()),
    )
    .expect("Failed to copy libhwy.a");

    std::fs::copy(
        "_deps/highway-build/libhwy_contrib.a",
        format!("{}/libhwy_contrib.a", env::var("OUT_DIR").unwrap()),
    )
    .expect("Failed to copy libhwy_contrib.a");

    std::fs::copy(
        "_deps/sentencepiece-build/src/libsentencepiece.a",
        format!("{}/libsentencepiece.a", env::var("OUT_DIR").unwrap()),
    )
    .expect("Failed to copy libsentencepiece.a");

    // Finally, build bindings.cc to allow access for gemma.cpp.
    // So far, bindgen does not correctly generate buildable rust file,
    // so I manually wrote bindings.rs for hand-written src/bindings.cc file.
    env::set_current_dir("..").expect("Unlable to change directory back to gemma.cpp");
    env::set_current_dir("..").expect("Unlable to change directory back to crate top");

    cc::Build::new()
        .cpp(true)
        .file("src/bindings.cc")
        .std("c++17")
        .include("./gemma.cpp")
        .include("./gemma.cpp/build/_deps/highway-src")
        .include("./gemma.cpp/build/_deps/sentencepiece-src")
        .compile("bindings");
}

// From https://github.com/alexcrichton/cc-rs/blob/fba7feded71ee4f63cfe885673ead6d7b4f2f454/src/lib.rs#L2462
fn get_cpp_link_stdlib(target: &str) -> Option<&'static str> {
    if target.contains("msvc") {
        None
    } else if target.contains("apple") || target.contains("freebsd") || target.contains("openbsd") {
        Some("c++")
    } else if target.contains("android") {
        Some("c++_shared")
    } else {
        Some("stdc++")
    }
}
