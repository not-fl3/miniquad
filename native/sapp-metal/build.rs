use std::env;

fn main() {
    let target = env::var("TARGET").unwrap_or_else(|e| panic!(e));

    if target.contains("apple") == false {
        panic!("sapp_metal support only Apple targets");
    }

    let mut build = cc::Build::new();

    build
        .flag("-fobjc-arc")
        .include("external/sokol")
        .file("src/sokol_app.m")
        .flag("-DSOKOL_METAL")
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-Wno-unused-function");

    build.try_get_compiler().unwrap_or_else(|e| panic!(e));

    let is_debug = env::var("DEBUG").ok().is_some();
    if is_debug {
        build.flag("-D_DEBUG").flag("-DSOKOL_DEBUG");
    }

    build.compile("sokol-app-sys");

    println!("cargo:rustc-link-lib=framework=Metal");
    println!("cargo:rustc-link-lib=framework=MetalKit");

    #[cfg(target_os = "macos")]
    {
        println!("cargo:rustc-link-lib=framework=Cocoa");
        println!("cargo:rustc-link-lib=framework=Quartz");
    }

    #[cfg(target_os = "ios")]
    {
        println!("cargo:rustc-link-lib=framework=UIKit");
    }

}
