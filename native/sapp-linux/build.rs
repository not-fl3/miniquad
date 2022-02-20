use std::env;

fn main() {
    let target = env::var("TARGET").unwrap_or_else(|e| panic!("{}", e));

    if target.contains("linux") == false
       && target.contains("dragonfly") == false
       && target.contains("freebsd") == false
       && target.contains("netbsd") == false
       && target.contains("openbsd") == false
    {
        panic!("sapp_linux support only linux target");
    }

    println!("cargo:rustc-link-lib=dylib=GL");
    println!("cargo:rustc-link-lib=dylib=X11");
    println!("cargo:rustc-link-lib=dylib=Xi");
}
