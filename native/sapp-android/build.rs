extern crate cc;

use cc::{Build, Tool};

use std::env;

fn build_new() -> (Build, Tool) {
    let build = Build::new();
    let tool = build.try_get_compiler().unwrap_or_else(|e| panic!(e));

    (build, tool)
}

fn build_android() {
    let (mut build, _) = build_new();

    let is_debug = env::var("DEBUG").ok().is_some();

    build.include("external/sokol");
    build.file("src/sokol_app.c");
    build.flag("-DSOKOL_GLES3");

    build
        .flag("-Wno-unused-parameter")
        .flag_if_supported("-Wno-unused-function");


    if is_debug {
        build.flag("-D_DEBUG").flag("-DSOKOL_DEBUG");
    }

    build.compile("sokol-app-sys");
}

fn main() {
    let target = env::var("TARGET").unwrap_or_else(|e| panic!(e));

    if target.contains("android") == false {
        panic!("Trying to build sapp_android not for android!");
    }

    build_android();
}
