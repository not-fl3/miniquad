// zwp-text-input-unstable-v3 protocol bindings
// https://wayland.app/protocols/text-input-unstable-v3

use super::super::libwayland_client::{wl_interface, wl_message, wl_surface};
use super::wayland_protocol::wl_surface_interface;
use crate::wayland_interface;

#[rustfmt::skip]
wayland_interface!(
    zwp_text_input_manager_v3_interface,
    zwp_text_input_manager_v3,
    1,
    [
        (destroy, "", ()),
        (get_text_input, "no", (zwp_text_input_v3_interface))
    ],
    []
);

#[rustfmt::skip]
wayland_interface!(
    zwp_text_input_v3_interface,
    zwp_text_input_v3,
    1,
    [
        (destroy, "", ()),
        (enable, "", ()),
        (disable, "", ()),
        (set_surrounding_text, "sii", ()),
        (set_text_change_cause, "u", ()),
        (set_content_type, "uu", ()),
        (set_cursor_rectangle, "iiii", ()),
        (commit, "", ())
    ],
    [
        (enter, "o", (wl_surface_interface)),
        (leave, "o", (wl_surface_interface)),
        (preedit_string, "?sii", ()),
        (commit_string, "?s", ()),
        (delete_surrounding_text, "uu", ()),
        (done, "u", ())
    ]
);

crate::wl_listener!(
    zwp_text_input_v3_listener,
    zwp_text_input_v3,
    zwp_text_input_v3_dummy,
    fn enter(surface: *mut wl_surface),
    fn leave(surface: *mut wl_surface),
    fn preedit_string(
        text: *const core::ffi::c_char,
        cursor_begin: core::ffi::c_int,
        cursor_end: core::ffi::c_int,
    ),
    fn commit_string(text: *const core::ffi::c_char),
    fn delete_surrounding_text(
        before_length: core::ffi::c_uint,
        after_length: core::ffi::c_uint,
    ),
    fn done(serial: core::ffi::c_uint),
);
