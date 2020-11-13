use std::env;

fn main() {
    let target = env::var("TARGET").unwrap_or_else(|e| panic!(e));

    if target.contains("linux") == false {
        panic!("sapp_linux support only linux target");
    }

    println!("cargo:rustc-link-lib=dylib=wayland-client");
    println!("cargo:rustc-link-lib=dylib=wayland-egl");
    println!("cargo:rustc-link-lib=dylib=EGL");
    println!("cargo:rustc-link-lib=dylib=GL");
}
