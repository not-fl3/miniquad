use std::env;
use std::process::Command;

fn main() {
    let target = env::var("TARGET").unwrap_or_else(|e| panic!("{}", e));

    if target.contains("darwin") || target.contains("ios") {
        println!("cargo:rustc-link-lib=framework=MetalKit");
    }

    // The arm64 simulator targets (aarch64-apple-ios-sim, aarch64-apple-tvos-sim);
    // metal.rs tells the x86_64 simulators apart by target_arch.
    // `cfg(target_abi = "sim")` needs Rust 1.78 and newer compilers spell it
    // `target_env = "sim"`, so the target name is checked here instead.
    if rustc_minor_version().map_or(false, |minor| minor >= 80) {
        // Older cargo warns about this line; check-cfg is stable since 1.80.
        println!("cargo:rustc-check-cfg=cfg(miniquad_apple_sim)");
    }
    if target.ends_with("-sim") {
        println!("cargo:rustc-cfg=miniquad_apple_sim");
    }
}

fn rustc_minor_version() -> Option<u32> {
    let rustc = env::var_os("RUSTC")?;
    let output = Command::new(rustc).arg("--version").output().ok()?;
    let version = std::str::from_utf8(&output.stdout).ok()?;
    let mut pieces = version.split('.');
    if pieces.next() != Some("rustc 1") {
        return None;
    }
    pieces.next()?.parse().ok()
}
