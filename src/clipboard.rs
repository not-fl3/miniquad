//! OS clipboard abstraction

use crate::Context;

#[cfg(target_os = "linux")]
mod linux {
    use crate::Context;

    pub fn get(_ctx: &mut Context) -> Option<String> {
        None
    }

    pub fn set(_ctx: &mut Context, _data: &str) {}
}

#[cfg(target_arch = "wasm32")]
mod wasm {
    use crate::Context;

    pub fn get(_ctx: &mut Context) -> Option<String> {
        sapp_wasm::clipboard_get()
    }

    pub fn set(_ctx: &mut Context, data: &str) {
        sapp_wasm::clipboard_set(data);
    }
}

#[cfg(target_os = "windows")]
mod windows {
    use crate::Context;

    pub fn get(_ctx: &mut Context) -> Option<String> {
        unsafe { sapp_windows::clipboard::get_clipboard_text() }
    }

    pub fn set(_ctx: &mut Context, data: &str) {
        unsafe { sapp_windows::clipboard::set_clipboard_text(data) };
    }
}

#[cfg(not(any(target_os = "linux", target_os = "windows", target_arch = "wasm32")))]
mod dummy {
    use crate::Context;

    pub fn get(_ctx: &mut Context) -> Option<String> {
        None
    }

    pub fn set(_ctx: &mut Context, _data: &str) {}
}

#[cfg(not(any(target_os = "linux", target_os = "windows", target_arch = "wasm32")))]
use dummy as clipboard;
#[cfg(target_os = "linux")]
use linux as clipboard;
#[cfg(target_arch = "wasm32")]
use wasm as clipboard;
#[cfg(target_os = "windows")]
use windows as clipboard;

/// Get current OS clipboard value
pub fn get(ctx: &mut Context) -> Option<String> {
    clipboard::get(ctx)
}

/// Save value to OS clipboard
pub fn set(ctx: &mut Context, data: &str) {
    clipboard::set(ctx, data);
}
