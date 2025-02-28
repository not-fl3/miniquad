use super::super::libwayland_client::{wl_fixed_t, wl_interface, wl_message};
use crate::wayland_interface;

pub const CURSOR_SHAPE_MANAGER_GET_POINTER: u32 = 1;
pub const CURSOR_SHAPE_DEVICE_SET_SHAPE: u32 = 1;
pub const RELATIVE_POINTER_MANAGER_GET_RELATIVE_POINTER: u32 = 1;
pub const POINTER_CONSTRAINTS_LOCK_POINTER: u32 = 1;
pub const zwp_pointer_constraints_v1_lifetime_ONESHOT: u32 = 1;
pub const zwp_pointer_constraints_v1_lifetime_PERSISTENT: u32 = 2;

#[rustfmt::skip]
wayland_interface!(
    wp_cursor_shape_manager_v1_interface,
    wp_cursor_shape_manager_v1,
    1,
    [
        (destroy, "", ()),
        (get_pointer, "no", (wp_cursor_shape_device_v1_interface)),
        (get_tablet_tool_v2, "no", (wp_cursor_shape_device_v1_interface))
    ],
    []
);

#[rustfmt::skip]
wayland_interface!(
    wp_cursor_shape_device_v1_interface,
    wp_cursor_shape_device_v1,
    1,
    [
        (destroy, "", ()),
        (set_shape, "uu", ())
    ],
    []
);

#[rustfmt::skip]
wayland_interface!(
    zwp_relative_pointer_manager_v1_interface,
    zwp_relative_pointer_manager_v1,
    1,
    [
        (destroy, "", ()),
        (get_relative_pointer, "no", (zwp_relative_pointer_v1_interface))
    ],
    []
);

#[rustfmt::skip]
wayland_interface!(
    zwp_pointer_constraints_v1_interface,
    zwp_pointer_constraints_v1,
    1,
    [
        (destroy, "", ()),
        (lock_pointer, "noo?ou", (zwp_locked_pointer_v1_interface)),
        (confine_pointer, "noo?ou", (zwp_confined_pointer_v1_interface))
    ],
    []
);

#[rustfmt::skip]
wayland_interface!(
    zwp_locked_pointer_v1_interface,
    zwp_locked_pointer_v1,
    1,
    [
        (destroy, "", ()),
        (set_cursor_position_hint, "ff", ()),
        (set_region, "?o", ())
    ],
    [("locked", ""), ("unlocked", "")]
);

#[rustfmt::skip]
wayland_interface!(
    zwp_confined_pointer_v1_interface,
    zwp_confined_pointer_v1,
    1,
    [
        (destroy, "", ()),
        (set_region, "?o", ())
    ],
    [("confined", ""), ("unconfined", "")]
);

#[rustfmt::skip]
wayland_interface!(
    zwp_relative_pointer_v1_interface,
    zwp_relative_pointer_v1,
    1,
    [
        (destroy, "", ())
    ],
    [("relative_motion", "uuffff")]
);

crate::wl_listener!(
    zwp_relative_pointer_v1_listener,
    zwp_relative_pointer_v1,
    zwp_relative_pointer_v1_dummy,
    fn relative_motion(
        utime_hi: core::ffi::c_uint,
        utime_lo: core::ffi::c_uint,
        dx: wl_fixed_t,
        dy: wl_fixed_t,
        dx_unaccel: wl_fixed_t,
        dy_unaccel: wl_fixed_t,
    ),
);

pub fn translate_cursor(icon: crate::CursorIcon) -> core::ffi::c_uint {
    // https://wayland.app/protocols/cursor-shape-v1#wp_cursor_shape_device_v1:enum:shape
    match icon {
        crate::CursorIcon::Default => 1,
        crate::CursorIcon::Help => 3,
        crate::CursorIcon::Pointer => 4,
        crate::CursorIcon::Wait => 6,
        crate::CursorIcon::Crosshair => 8,
        crate::CursorIcon::Text => 9,
        crate::CursorIcon::Move => 13,
        crate::CursorIcon::NotAllowed => 15,
        crate::CursorIcon::EWResize => 26,
        crate::CursorIcon::NSResize => 27,
        crate::CursorIcon::NESWResize => 28,
        crate::CursorIcon::NWSEResize => 29,
    }
}
