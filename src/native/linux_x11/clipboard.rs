#![allow(non_upper_case_globals, non_snake_case)]

//! Clipboard implementation for X11
//!
//! Clipboard API on X11 is pretty weird
//! <https://www.uninformativ.de/blog/postings/2017-04-02/0/POSTING-en.html>
//! so use this with caution.

use super::libx11::*;

unsafe fn get_clipboard(
    libx11: &mut LibX11,
    display: *mut Display,
    window: Window,
    format: Atom,
) -> Option<Vec<u8>> {
    // We ask X server to ask current clipboard owner to convert selection to the desired format
    // Selection owner is other X11 application and if it will not satisfy our request for any reason -
    // we will wait for appropriate event forever
    // but OK lets believe that all other linux apps will gracefully send us nice utf-8 strings
    (libx11.XConvertSelection)(
        display,
        libx11.extensions.clipboard,
        format,
        libx11.extensions.xsel_data,
        window,
        CurrentTime as Time,
    );

    // And now X server will respond with clipboard data
    // But we want "get_clipboard" to be blocking function and get result asap
    // So we just start to wait for the event right here
    // In case that our app already is clipboard owner - we need to handle SelectionNotify for data response
    // and SelectionRequest - to handle this request we just did couple of lines above
    let mut event = XEvent { type_0: 0 };
    loop {
        (libx11.XNextEvent)(display, &mut event);
        if event.type_0 == SelectionNotify
            && event.xselection.selection == libx11.extensions.clipboard
        {
            // The property should be XSEL_DATA, otherwise we return None
            return (event.xselection.property == libx11.extensions.xsel_data)
                .then(|| get_property_bytes(libx11, display, window, event.xselection.property));
        }
        if event.type_0 == SelectionRequest {
            respond_to_clipboard_request(libx11, display, &mut event as *mut _);
        }
    }
}

/// Get the bytes from the window property
/// INCR protocol is implemented; will block until all data are received
/// Useful for getting data following a SelectionNotify event
//
//  X11 has the INCR protocol to transfer data in several batches
//  https://www.x.org/releases/X11R7.7/doc/xorg-docs/icccm/icccm.html#INCR_Properties
//
//  The data are sequentially written to the given property on the window
//  After each read, we delete the property to notify X that we are ready to receive more data
//  Used both for the clipboard and the drag-n-drop protocol
pub(crate) unsafe fn get_property_bytes(
    libx11: &mut LibX11,
    display: *mut Display,
    window: Window,
    property: Atom,
) -> Vec<u8> {
    let mut bytes = Vec::new();
    let mut buf = std::ptr::null_mut::<libc::c_char>();
    let mut size: libc::c_ulong = 0;
    let mut actual_type = 0 as Atom;
    let mut actual_format: libc::c_int = 0;
    let mut _size_after: libc::c_ulong = 0;
    let mut incr_mode = false;
    loop {
        (libx11.XGetWindowProperty)(
            display,
            window,
            property,
            0 as _,
            libc::c_long::MAX,
            false as _,
            AnyPropertyType,
            &mut actual_type,
            &mut actual_format,
            &mut size,
            &mut _size_after,
            &mut buf as *mut *mut libc::c_char as _,
        );

        if actual_type == libx11.extensions.incr {
            // we are in INCR mode
            incr_mode = true;
        } else if size == 0 {
            // no more data to read
            (libx11.XFree)(buf as *mut libc::c_void);
            return bytes;
        } else {
            let n_bits = match actual_format {
                8 => std::mem::size_of::<libc::c_char>(),
                16 => std::mem::size_of::<libc::c_int>(),
                32 => std::mem::size_of::<libc::c_long>(),
                _ => unreachable!(),
            };
            bytes.extend(std::slice::from_raw_parts(
                buf as *const _,
                n_bits * size as usize,
            ));

            // if not in INCR mode, then we've got all the data
            if !incr_mode {
                (libx11.XFree)(buf as *mut libc::c_void);
                return bytes;
            }
        }

        if incr_mode {
            (libx11.XDeleteProperty)(display, window, property);
            // wait until we get a new batch
            let mut event = XEvent { type_0: 0 };
            loop {
                (libx11.XNextEvent)(display, &mut event);
                if event.type_0 == PropertyNotify && event.xproperty.state == PropertyNewValue {
                    break;
                }
            }
        }
    }
}

// Next message for clipboard request
static mut MESSAGE: Option<String> = None;

/// Claim that our app is X11 clipboard owner
/// Now when some other linux app will ask X11 for clipboard content - it will be redirected to our app
unsafe fn claim_clipboard_ownership(
    libx11: &mut LibX11,
    display: *mut Display,
    window: Window,
    message: String,
) {
    (libx11.XSetSelectionOwner)(
        display,
        libx11.extensions.clipboard,
        window,
        CurrentTime as Time,
    );

    MESSAGE = Some(message);
}

/// this function is supposed to be called from sapp's event loop
/// when XSelectionEvent received.
/// It will parse event and call XSendEvent with event response
pub(crate) unsafe fn respond_to_clipboard_request(
    libx11: &mut LibX11,
    display: *mut Display,
    event: *const XEvent,
) {
    assert!((*event).type_0 == SelectionRequest); // is it really SelectionRequest

    let empty_message = String::new();
    let message = MESSAGE.as_ref().unwrap_or(&empty_message);

    let utf8_string = libx11.extensions.utf8_string;
    let xselectionrequest = (*event).xselectionrequest;
    let mut ev = XSelectionEvent {
        type_0: SelectionNotify,
        serial: 0,
        send_event: 0,
        display: xselectionrequest.display,
        requestor: xselectionrequest.requestor,
        selection: xselectionrequest.selection,
        target: xselectionrequest.target,
        property: xselectionrequest.property,
        time: xselectionrequest.time,
    };

    // only UTF8 requests are supported
    if xselectionrequest.target == utf8_string {
        (libx11.XChangeProperty)(
            xselectionrequest.display,
            xselectionrequest.requestor,
            xselectionrequest.property,
            utf8_string,
            8 as libc::c_int,
            PropModeReplace,
            message.as_bytes().as_ptr(),
            message.len() as _,
        );

        (libx11.XSendEvent)(
            display,
            ev.requestor,
            false as _,
            NoEventMask,
            &mut ev as *mut XSelectionEvent as *mut XEvent,
        );
    } else {
        // signal X that request is denied
        ev.property = 0 as Atom;

        (libx11.XSendEvent)(
            display,
            ev.requestor,
            false as _,
            NoEventMask,
            &mut ev as *mut XSelectionEvent as *mut XEvent,
        );
    }
}

pub struct X11Clipboard {
    libx11: LibX11,
    display: *mut Display,
    window: Window,
}
unsafe impl Send for X11Clipboard {}
unsafe impl Sync for X11Clipboard {}

impl X11Clipboard {
    pub fn new(libx11: LibX11, display: *mut Display, window: Window) -> X11Clipboard {
        X11Clipboard {
            libx11,
            display,
            window,
        }
    }
}
impl crate::native::Clipboard for X11Clipboard {
    fn get(&mut self) -> Option<String> {
        let utf8_string = self.libx11.extensions.utf8_string;
        let bytes =
            unsafe { get_clipboard(&mut self.libx11, self.display, self.window, utf8_string)? };
        Some(std::str::from_utf8(&bytes).ok()?.to_string())
    }

    fn set(&mut self, data: &str) {
        unsafe {
            claim_clipboard_ownership(&mut self.libx11, self.display, self.window, data.to_owned());
        };
    }
}
