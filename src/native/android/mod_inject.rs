// Imagine crate A depending on miniquad.
// On android A is being compiled to .so and this .so is being loaded
// by "System.loadLibrary("")" java call.
// extern "C" functions from miniquad are right there, in the .so. But somehow
// they are invisible for JNI unless they are declared in the A itself,
// not in the A's dependencies.
//
// Why? I do not know. Would be nice to find some tracking issue.
//
// But we really need to be able to make miniquad's functions visible to java.
//
// Contents of this file is being copied right to the "main.rs" of the main crate
// by cargo quad. And therefore functions like JAVA_CLASS_PATH_QuadSurface_nativeOnSurfaceCreated are well visible for the JNI
// and they just forward the call to the real implementation inside miniquad
// Note that because it is being injected - we might not have neither miniquad
// or ndk_sys as a crate dependency.. so we cant use anything from them.


#[no_mangle]
pub extern "C" fn quad_main() {
    let _ = super::main();
}
