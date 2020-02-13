use crate::Context;

#[cfg(not(target_os = "linux"))]
pub fn get(ctx: &mut Context) -> Option<String> {
    None
}

#[cfg(not(target_os = "linux"))]
pub fn set(ctx: &mut Context, data: &str) {}

#[cfg(target_os = "linux")]
pub fn get(_ctx: &mut Context) -> Option<String> {
    use std::ffi::CString;

    let bufname = CString::new("CLIPBOARD").unwrap();
    let fmtname = CString::new("UTF8_STRING").unwrap();

    unsafe { sapp_linux::clipboard::get_clipboard(bufname.as_ptr(), fmtname.as_ptr()) }
}

#[cfg(target_os = "linux")]
pub fn set(_ctx: &mut Context, data: &str) {
    use std::ffi::CString;

    let bufname = CString::new("CLIPBOARD").unwrap();

    unsafe { sapp_linux::clipboard::claim_clipboard_ownership(bufname.as_ptr(), data.to_owned()) };
}
