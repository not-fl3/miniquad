extern crate cc;

use std::env;

use cc::{Build, Tool};

fn build_new() -> (Build, Tool) {
    let build = Build::new();
    let tool = build.try_get_compiler().unwrap_or_else(|e| panic!(e));

    (build, tool)
}

fn select_sokol_gfx_renderer(build: &mut Build) {
    if cfg!(target_family = "windows") {
        build.flag("-DSOKOL_GLCORE33");
    } else if cfg!(target_os = "macos") {
        build.flag("-DSOKOL_METAL");
    } else {
        build.flag("-DSOKOL_GLCORE33");
    }
}

fn make_sokol(target: &str) {
    let (mut build, _) = build_new();

    let is_debug = env::var("DEBUG").ok().is_some();
    //
    // include paths
    //
    build
        .include("external/sokol");

    build
        .file("src/sokol_app.c");

    //
    // select sokol_gfx renderer
    //
    select_sokol_gfx_renderer(&mut build);

    //
    // silence some warnings
    //
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

    }

    if is_debug {
        build
            .flag("-D_DEBUG")
            .flag("-DSOKOL_DEBUG");
    }

    build
        .compile("sokol-app-sys");

    if target == "x86_64-unknown-linux-gnu" {
        println!("cargo:rustc-link-lib=dylib=GL");
        println!("cargo:rustc-link-lib=dylib=X11");
    }
}

fn main() {
    let target = env::var("TARGET").unwrap_or_else(|e| panic!(e));

    if target != "wasm32-unknown-unknown" {
        make_sokol(&target);
    }
}
