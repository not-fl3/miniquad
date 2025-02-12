#![allow(non_upper_case_globals, non_snake_case)]

// Drag-n-Drop specification at
// https://freedesktop.org/wiki/Specifications/XDND/
// C++ implementation example
// https://www.codeproject.com/Tips/5387630/How-to-handle-X11-Drag-n-Drop-events

// The basic logic is similar to the mouse events
// mouse_down   => on_enter
// mouse_motion => on_pos
// mouse_up     => on_drop

use super::libx11::*;

#[derive(Default)]
pub struct X11DnD {
    source: Window,        // source window
    version: libc::c_long, // version of XDnD, should be 5
    format: Atom,          // we want UTF8_STRING for the paths of the dropped files
}

impl super::X11Display {
    pub(super) unsafe fn init_drag_n_drop(&mut self) {
        // Add the window property XdndAware to inform X that we accept drag-n-drop
        (self.libx11.XChangeProperty)(
            self.display,
            self.window,
            self.libx11.extensions.xdnd_aware,
            4 as _,
            32,
            PropModeReplace,
            [5].as_ptr() as _, // XDnD version number
            1,
        );
    }
}

impl X11DnD {
    pub unsafe fn on_enter(
        &mut self,
        libx11: &mut LibX11,
        display: *mut Display,
        _window: Window,
        data: ClientMessageData,
    ) {
        // Store the relevant info
        self.source = data.l[0] as _;
        self.version = data.l[1] >> 24;

        // Then we need to determine the format (or type) to get the data
        // We want UTF8_STRING
        // The available ones depend on the source window and their number can vary
        self.format = 0;
        let formats: Vec<Atom> = if data.l[1] & 1 == 0 {
            // If there are fewer than 3, they are already in the client message we got
            vec![data.l[2] as _, data.l[3] as _, data.l[4] as _]
        } else {
            // Otherwise we need to read them from the window property
            let bytes = super::clipboard::get_property_bytes(
                libx11,
                display,
                self.source,
                libx11.extensions.xdnd_type_list,
            );
            // Realign the bytes into Atoms
            bytes.align_to::<Atom>().1.to_vec()
        };

        for format in formats {
            if format == libx11.extensions.utf8_string {
                self.format = format;
                break;
            }
        }
    }

    pub unsafe fn on_position(
        &mut self,
        libx11: &mut LibX11,
        display: *mut Display,
        window: Window,
        _data: ClientMessageData,
    ) {
        if self.version <= 5 {
            // We need to send back a client message of type XdndStatus
            let mut reply = XClientMessageEvent {
                type_0: ClientMessage,
                serial: 0,
                send_event: true as _,
                message_type: libx11.extensions.xdnd_status,
                window: self.source,
                display,
                format: 32,
                data: ClientMessageData {
                    l: [window as _, 0, 0, 0, 0],
                },
            };

            // The source window supports the desired format
            if self.format != 0 {
                // Notify that we can receive the drop
                reply.data.l[1] = 1;

                // Notify that we accept the XdndActionCopy action
                if self.version >= 2 {
                    reply.data.l[4] = libx11.extensions.xdnd_action_copy as _;
                }
            }
            (libx11.XSendEvent)(
                display,
                self.source,
                false as _,
                NoEventMask,
                &mut reply as *mut XClientMessageEvent as *mut _,
            );
            (libx11.XFlush)(display);
        }
    }

    pub unsafe fn on_drop(
        &mut self,
        libx11: &mut LibX11,
        display: *mut Display,
        window: Window,
        data: ClientMessageData,
    ) {
        if self.version <= 5 {
            if self.format != 0 {
                // Request to retrieve the data
                // The actual data will then be sent via a SelectionNotify event
                let mut time = CurrentTime;
                if self.version >= 1 {
                    time = data.l[3];
                }
                (libx11.XConvertSelection)(
                    display,
                    libx11.extensions.xdnd_selection,
                    self.format,
                    libx11.extensions.xdnd_selection,
                    window,
                    time as Time,
                );
            } else if self.version >= 2 {
                let mut reply = XClientMessageEvent {
                    type_0: ClientMessage,
                    serial: 0,
                    send_event: true as _,
                    message_type: libx11.extensions.xdnd_finished,
                    window: self.source,
                    display,
                    format: 32,
                    data: ClientMessageData {
                        l: [window as _, 0, 0, 0, 0],
                    },
                };
                (libx11.XSendEvent)(
                    display,
                    self.source,
                    false as _,
                    NoEventMask,
                    &mut reply as *mut XClientMessageEvent as *mut _,
                );
                (libx11.XFlush)(display);
            }
        }
    }
}
