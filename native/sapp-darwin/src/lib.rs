#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]

// bindgen --no-layout-tests external/sokol/sokol_app.h --opaque-type IMAGE_TLS_DIRECTORY64 -- -D SOKOL_GLCORE33 -D SOKOL_NO_ENTRY -target x86_64-apple-darwin > src/sokol_app_darwin.rs
pub mod sokol_app_darwin;
pub use sokol_app_darwin::*;
