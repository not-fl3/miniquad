extern crate cc;

use cc::{Build, Tool};

use std::env;

fn build_new() -> (Build, Tool) {
    let build = Build::new();
    let tool = build.try_get_compiler().unwrap_or_else(|e| panic!(e));

    (build, tool)
}

fn build_windows(target: &str) {
    let (mut build, _) = build_new();

    let is_debug = env::var("DEBUG").ok().is_some();

    build.include("external/sokol");
    build.file("src/sokol_app.c");
    build.flag("-DSOKOL_GLCORE33");

    build
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-Wno-unused-function");

    //
    // x86_64-pc-windows-gnu: additional compile/link flags
    //
    // and remember to https://github.com/rust-lang/rust/issues/47048
    if target == "x86_64-pc-windows-gnu" {
        build
            .flag("-D_WIN32_WINNT=0x0601")
            .flag_if_supported("-Wno-cast-function-type")
            .flag_if_supported("-Wno-sign-compare")
            .flag_if_supported("-Wno-unknown-pragmas");

        println!("cargo:rustc-link-lib=gdi32");
        println!("cargo:rustc-link-lib=ole32");
        println!("cargo:rustc-link-lib=shell32");
    } else {
        println!("cargo:rustc-link-lib=gdi32");
        println!("cargo:rustc-link-lib=user32");        
    }

    if is_debug {
        build.flag("-D_DEBUG").flag("-DSOKOL_DEBUG");
    }

    build.compile("sokol-app-sys");
}

fn main() {
    let target = env::var("TARGET").unwrap_or_else(|e| panic!(e));

    if target.contains("windows") == false{
        panic!("Trying to build sapp_windows not for windows!");
        }

    build_windows(&target);
}
