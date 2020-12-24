#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]

// XCode 11.6
// bindgen --no-layout-tests external/sokol/sokol_app.h --opaque-type IMAGE_TLS_DIRECTORY64 -- -isysroot $(xcrun --sdk iphoneos --show-sdk-path) -arch arm64 -D SOKOL_GLES3 -D SOKOL_NO_ENTRY  > src/sokol_app_ios.rs
pub mod sokol_app_ios;
pub use sokol_app_ios::*;

// XCode 11.6
// bindgen --no-layout-tests /Applications/Xcode.app/Contents/Developer/Platforms/iPhoneOS.platform/Developer/SDKs/iPhoneOS.sdk/System/Library/Frameworks/OpenGLES.framework/Headers/ES3/gl.h -- -isysroot $(xcrun --sdk iphoneos --show-sdk-path) -arch arm64 -framework OpenGLES -F /Applications/Xcode.app/Contents/Developer/Platforms/iPhoneOS.platform/Developer/SDKs/iPhoneOS.sdk/System/Library/Frameworks/ -I /Applications/Xcode.app/Contents/Developer/Platforms/iPhoneOS.platform/Developer/SDKs/iPhoneOS.sdk/usr/include > src/gl.rs
pub mod gl;
pub use gl::*;

pub mod rand;
pub use rand::*;

pub mod query_stab;
pub use query_stab::*;

pub unsafe fn sapp_is_elapsed_timer_supported() -> bool {
    return false;
}
