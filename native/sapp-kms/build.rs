
fn main() {
    println!("cargo:rustc-link-lib=dylib=GLESv2");
    println!("cargo:rustc-link-lib=dylib=EGL");
    println!("cargo:rustc-link-lib=dylib=drm");
    println!("cargo:rustc-link-lib=dylib=gbm");
}
