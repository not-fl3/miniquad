//! Clipboard implementation for X11
//! Clipboard API on X11 is pretty weird https://www.uninformativ.de/blog/postings/2017-04-02/0/POSTING-en.html
//! so use this with caution.

use crate::x::*;

use crate::{_sapp_x11_display, _sapp_x11_window};

const CurrentTime: libc::c_long = 0 as libc::c_long;
pub(crate) const SelectionRequest: libc::c_int = 30 as libc::c_int;
pub(crate) const SelectionNotify: libc::c_int = 31 as libc::c_int;
const AnyPropertyType: libc::c_long = 0 as libc::c_long;

type Time = libc::c_ulong;

extern "C" {
    pub fn XConvertSelection(
        _: *mut Display,
        _: Atom,
        _: Atom,
        _: Atom,
        _: Window,
        _: Time,
    ) -> libc::c_int;

    pub fn XSetSelectionOwner(_: *mut Display, _: Atom, _: Window, _: Time) -> libc::c_int;
}

pub unsafe fn get_clipboard(
    mut bufname: *const libc::c_char,
    mut fmtname: *const libc::c_char,
) -> Option<String> {
    assert!(_sapp_x11_display as usize != 0 && _sapp_x11_window != 0);

    let mut result = 0 as *mut libc::c_char;
    let mut ressize: libc::c_ulong = 0;
    let mut restail: libc::c_ulong = 0;
    let mut resbits: libc::c_int = 0;
    let mut bufid = XInternAtom(_sapp_x11_display, bufname, false as _);
    let mut fmtid = XInternAtom(_sapp_x11_display, fmtname, false as _);
    let mut propid = XInternAtom(
        _sapp_x11_display,
        b"XSEL_DATA\x00" as *const u8 as *const libc::c_char,
        false as _,
    );
    let mut incrid = XInternAtom(
        _sapp_x11_display,
        b"INCR\x00" as *const u8 as *const libc::c_char,
        false as _,
    );
    let mut event = _XEvent { type_0: 0 };

    // Here we ask X server to ask current clipboard owner to convert selection to UTF8 for us
    // Selection owner is other X11 application and if it will not satisfy our request for any reason -
    // we will wait for appropriate event forever
    // but OK lets believe that all other linux apps will gracefully send us nice utf-8 strings
    XConvertSelection(
        _sapp_x11_display,
        bufid,
        fmtid,
        propid,
        _sapp_x11_window,
        CurrentTime as Time,
    );

    // And now X server will respond with clipboard data
    // But we want "get_clipboard" to be blocking function and get result asap
    // So we just start to wait for the event right here
    // In case that our app already is clipboard owner - we need to handle SelectionNotify for data response
    // and SelectionRequest - to handle this request we just did couple of lines above
    loop {
        XNextEvent(_sapp_x11_display, &mut event);
        if !(event.type_0 != SelectionNotify || event.xselection.selection != bufid) {
            break;
        }
        if event.type_0 == SelectionRequest {
            respond_to_clipboard_request(&mut event as *mut _);
        }
    }
    if event.xselection.property != 0 {
        let read_size = (100 as u32 * std::mem::size_of::<Atom>() as u32) as libc::c_long;
        let mut bytes: Vec<u8> = vec![];
        let mut offset: libc::c_long = 0 as libc::c_long;
        loop {
            XGetWindowProperty(
                _sapp_x11_display,
                _sapp_x11_window,
                propid,
                offset,
                read_size,
                false as _,
                AnyPropertyType as Atom,
                &mut fmtid,
                &mut resbits,
                &mut ressize,
                &mut restail,
                &mut result as *mut *mut libc::c_char as *mut *mut libc::c_uchar,
            );
            if fmtid == incrid {
                XFree(result as *mut libc::c_void);
                panic!("Buffer is too large and INCR reading is not implemented yet.");
            } else {
                let slice = std::slice::from_raw_parts(result as *const _, ressize as _);
                let str_result = bytes.extend(slice);

                XFree(result as *mut libc::c_void);

                if restail == 0 {
                    return std::str::from_utf8(&bytes[..]).map(|s| s.to_owned()).ok();
                } else {
                    offset += read_size;
                }
            }
        }
    }

    return None;
}

// Next message for clipboard request
static mut MESSAGE: Option<String> = None;

/// Claim that our app is X11 clipboard owner
/// Now when some other linux app will ask X11 for clipboard content - it will be redirected to our app
pub unsafe fn claim_clipboard_ownership(mut bufname: *const libc::c_char, message: String) {
    assert!(_sapp_x11_display as usize != 0 && _sapp_x11_window != 0);

    let mut selection = XInternAtom(
        _sapp_x11_display,
        bufname as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    );

    XSetSelectionOwner(
        _sapp_x11_display,
        selection,
        _sapp_x11_window,
        0 as libc::c_int as Time,
    );

    MESSAGE = Some(message);
}

/// this function is supposed to be called from sapp's event loop
/// when XSelectionEvent received.
/// It will parse event and call XSendEvent with event response
pub(crate) unsafe fn respond_to_clipboard_request(event: *const XEvent) {
    assert!((*event).type_0 == 30); // is it really SelectionRequest

    let empty_message = String::new();
    let message = MESSAGE.as_ref().unwrap_or(&empty_message);

    let UTF8 = XInternAtom(
        _sapp_x11_display,
        b"UTF8_STRING\x00" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
    );
    let xselectionrequest = (*event).xselectionrequest;
    let mut ev = XSelectionEvent {
        type_0: crate::clipboard::SelectionNotify,
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
    if xselectionrequest.target == UTF8 {
        XChangeProperty(
            xselectionrequest.display,
            xselectionrequest.requestor,
            xselectionrequest.property,
            UTF8,
            8 as libc::c_int,
            PropModeReplace,
            message.as_bytes().as_ptr() as *const u8 as *const _,
            message.as_bytes().len() as _,
        );

        XSendEvent(
            _sapp_x11_display,
            ev.requestor,
            0 as libc::c_int,
            0 as libc::c_int as libc::c_long,
            &mut ev as *mut XSelectionEvent as *mut XEvent,
        );
    } else {
        // signal X that request is denied
        ev.property = 0 as Atom;

        XSendEvent(
            _sapp_x11_display,
            ev.requestor,
            0 as libc::c_int,
            0 as libc::c_int as libc::c_long,
            &mut ev as *mut XSelectionEvent as *mut XEvent,
        );
    }
}
