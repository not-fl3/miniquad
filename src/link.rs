//! Functions to open links

/// If not wasm, then open link in browser. If target is wasm, then link can be opened in the same tab, or in a new tab. But when link is opened in a new tab, browser may block it and ask user permission to do it.
pub fn open(url: &str, new_tab: bool) {
    #[cfg(target_arch = "wasm32")]
    {
        use sapp_wasm::link;
        use std::ffi::CString;

        let url = CString::new(url).unwrap();
        unsafe {
            link::link_open(url.as_ptr(), url.as_bytes().len() as u32, new_tab as u32)
        };
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        drop(new_tab); // used only in wasm
        if let Err(err) = webbrowser::open(url) {
            eprintln!("Failed to open url: {}, url: `{}`", err, url);
        }
    }
}
