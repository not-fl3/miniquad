#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#![allow(improper_ctypes)] // u128 types are not actually used anywhere, so the functions with u128 in signatures will be stripped anyway (I believe)

// bindgen --no-layout-tests external/sokol/sokol_app.h --opaque-type IMAGE_TLS_DIRECTORY64 -- -D SOKOL_GLCORE33 -D SOKOL_IMPL -D SOKOL_NO_ENTRY -target x86_64-pc-windows-gnu > src/sokol_app_msvc.rs
#[cfg(target_env = "msvc")]
pub mod sokol_app_msvc;

#[cfg(target_env = "msvc")]
pub use sokol_app_msvc::*;

// bindgen --no-layout-tests external/sokol/sokol_app.h -- -D SOKOL_GLCORE33 -D SOKOL_IMPL -D SOKOL_NO_ENTRY -target x86_64-pc-windows-gnu > src/sokol_app_win.rs
#[cfg(target_env = "gnu")]
pub mod sokol_app_gnu;
#[cfg(target_env = "gnu")]
pub use sokol_app_gnu::*;
