#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#![allow(improper_ctypes)] // u128 types are not actually used anywhere, so the functions with u128 in signatures will be stripped anyway (I believe)

#[cfg(target_os = "linux")]
pub mod sokol_app_linux;
#[cfg(target_os = "linux")]
pub use crate::sokol_app_linux as sokol_app;

// bindgen --no-layout-tests external/sokol/sokol_app.h -- -D SOKOL_GLCORE33 -D SOKOL_IMPL -D SOKOL_NO_ENTRY -target x86_64-pc-windows-gnu > src/sokol_app_win.rs
#[cfg(target_os = "windows")]
pub mod sokol_app_win;
#[cfg(target_os = "windows")]
pub use crate::sokol_app_win as sokol_app;

// bindgen --no-layout-tests external/sokol/sokol_app.h -- -I. -I/usr/lib/emscripten/system/include -I/usr/lib/emscripten/system/include/libc -I/usr/lib/emscripten/system/lib/libc/musl/arch/emscripten/ -D SOKOL_GLES3 -D SOKOL_IMPL -D SOKOL_NO_ENTRY -D __EMSCRIPTEN__ -target wasm32-unknown-emscripten -fvisibility=default > src/sokol_app_wasm.rs
#[cfg(target_arch = "wasm32")]
pub mod sokol_app_wasm;
#[cfg(target_arch = "wasm32")]
pub use crate::sokol_app_wasm as sokol_app;
