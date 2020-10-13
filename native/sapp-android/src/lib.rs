#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code)]
#![allow(improper_ctypes)] // u128 types are not actually used anywhere, so the functions with u128 in signatures will be stripped anyway (I believe)

mod egl;
pub mod gl3;
mod rand;
mod sokol_app_android;

pub use rand::*;
pub use egl::*;
pub use gl3::*;

pub use gl3 as gl;

// workaround for egl::* also contains None on Android
pub use std::option::Option::None;

// bindgen --no-layout-tests external/sokol/sokol_app.h --opaque-type IMAGE_TLS_DIRECTORY64 -- -D DSOKOL_GLES3 -target arm-linux-androideabi > src/sokol_app_android.rs
pub use sokol_app_android::*;

