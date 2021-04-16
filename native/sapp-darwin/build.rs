use std::env;

fn main() {
    let target = env::var("TARGET").unwrap_or_else(|e| panic!("{}", e));

    if target.contains("darwin") == false {
        panic!("sapp_darwin support only darwin targets");
    }

    let mut build = cc::Build::new();

    build
        .flag("-fobjc-arc")
        .include("external/sokol")
        .file("src/sokol_app.m")
        .flag("-DSOKOL_GLCORE33")
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-Wno-unused-function");

    build.try_get_compiler().unwrap_or_else(|e| panic!("{}", e));

    let is_debug = env::var("DEBUG").ok().is_some();
    if is_debug {
        build.flag("-D_DEBUG").flag("-DSOKOL_DEBUG");
    }

    build.compile("sokol-app-sys");

    println!("cargo:rustc-link-lib=framework=Cocoa");
    println!("cargo:rustc-link-lib=framework=OpenGL");
}
