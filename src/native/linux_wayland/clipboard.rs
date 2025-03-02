use super::*;
use crate::{wl_request, wl_request_constructor};

use core::ffi::{c_char, c_int, c_void};

// new clipboard content is available
// this could be fired at any time, so we just store the `data_offer` for later use
pub(super) unsafe extern "C" fn data_device_handle_selection(
    data: *mut c_void,
    data_device: *mut wl_data_device,
    data_offer: *mut wl_data_offer,
) {
    let display: &mut WaylandPayload = &mut *(data as *mut _);
    assert_eq!(data_device, display.data_device);
    CLIPBOARD.get_mut().unwrap().data_offer = (!data_offer.is_null()).then_some(data_offer);
}

use std::sync::OnceLock;
static mut CLIPBOARD: OnceLock<ClipboardContext> = OnceLock::new();

// Contains the owned clipboard and the `data_source` (object in Wayland that indicates clipboard
// ownership). There is a static instance `CLIPBOARD` available globally, initialized when creating
// the `WaylandClipboard`.
#[derive(Debug)]
struct ClipboardContext {
    display: *mut WaylandPayload,
    content: String,
    data_source: Option<*mut wl_data_source>,
    data_offer: Option<*mut wl_data_offer>,
}

impl ClipboardContext {
    unsafe fn get_clipboard(&mut self, mime_type: &str) -> Option<Vec<u8>> {
        self.data_offer.map(|data_offer| {
            let display: &mut WaylandPayload = &mut *self.display;
            let mime_type = std::ffi::CString::new(mime_type).unwrap();
            display
                .client
                .data_offer_receive(display.display, data_offer, mime_type.as_ptr())
        })?
    }

    unsafe fn set(&mut self, data: &str) {
        self.content.clear();
        self.content.push_str(data);
        let display: &mut WaylandPayload = &mut *self.display;
        // Wayland requires that only the window with focus can set the clipboard
        if let Some(serial) = display.keyboard_context.enter_serial {
            let data_source = self.new_data_source();
            // only support copying utf8 strings
            let mime_type = std::ffi::CString::new("UTF8_STRING").unwrap();
            wl_request!(
                display.client,
                data_source,
                WL_DATA_SOURCE_OFFER,
                mime_type.as_ptr()
            );
            wl_request!(
                display.client,
                display.data_device,
                WL_DATA_DEVICE_SET_SELECTION,
                data_source,
                serial
            );
        }
    }

    unsafe fn respond_to_clipboard_request(&mut self, mime_type: &str, fd: c_int) {
        #![allow(clippy::single_match)]
        match mime_type {
            "UTF8_STRING" => {
                libc::write(fd, self.content.as_ptr() as _, self.content.len());
            }
            _ => {}
        }
        libc::close(fd);
    }

    unsafe fn destroy_data_source(&mut self) {
        // since the data_source is constructed by us, we need to dispose of it properly
        if let Some(data_source) = self.data_source {
            let display: &mut WaylandPayload = &mut *self.display;
            wl_request!(display.client, data_source, WL_DATA_SOURCE_DESTROY);
            (display.client.wl_proxy_destroy)(data_source as _);
            self.data_source = None;
        }
    }

    unsafe fn new_data_source(&mut self) -> *mut wl_data_source {
        self.destroy_data_source();
        let display: &mut WaylandPayload = &mut *self.display;
        let data_source: *mut wl_data_source = wl_request_constructor!(
            display.client,
            display.data_device_manager,
            WL_DATA_DEVICE_MANAGER_CREATE_DATA_SOURCE,
            display.client.wl_data_source_interface,
        );
        assert!(!data_source.is_null());
        DATA_SOURCE_LISTENER.send = data_source_handle_send;
        DATA_SOURCE_LISTENER.cancelled = data_source_handle_cancelled;
        (display.client.wl_proxy_add_listener)(
            data_source as _,
            &DATA_SOURCE_LISTENER as *const _ as _,
            self.display as *const _ as _,
        );
        self.data_source = Some(data_source);
        data_source
    }
}

static mut DATA_SOURCE_LISTENER: wl_data_source_listener = wl_data_source_listener::dummy();

// some app (could be ourself) is requesting the owned clipboard
unsafe extern "C" fn data_source_handle_send(
    _data: *mut c_void,
    data_source: *mut wl_data_source,
    mime_type: *const c_char,
    fd: c_int,
) {
    let mime_type = core::ffi::CStr::from_ptr(mime_type).to_str().unwrap();
    let ctx = CLIPBOARD.get_mut().unwrap();
    assert!(ctx.data_source == Some(data_source));
    ctx.respond_to_clipboard_request(mime_type, fd);
}

// the owned clipboard has been replaced by some other app
unsafe extern "C" fn data_source_handle_cancelled(
    _data: *mut c_void,
    data_source: *mut wl_data_source,
) {
    let ctx = CLIPBOARD.get_mut().unwrap();
    assert!(ctx.data_source == Some(data_source));
    ctx.destroy_data_source();
}

pub struct WaylandClipboard {}
unsafe impl Send for WaylandClipboard {}
unsafe impl Sync for WaylandClipboard {}

impl WaylandClipboard {
    pub(super) fn new(display: *mut WaylandPayload) -> Self {
        // initialize the global context
        unsafe {
            CLIPBOARD
                .set(ClipboardContext {
                    display,
                    content: String::new(),
                    data_source: None,
                    data_offer: None,
                })
                .unwrap();
        }
        WaylandClipboard {}
    }
}

impl crate::native::Clipboard for WaylandClipboard {
    fn get(&mut self) -> Option<String> {
        let bytes = unsafe { CLIPBOARD.get_mut().unwrap().get_clipboard("UTF8_STRING")? };
        Some(std::str::from_utf8(&bytes).ok()?.to_string())
    }
    fn set(&mut self, data: &str) {
        unsafe {
            CLIPBOARD.get_mut().unwrap().set(data);
        }
    }
}
