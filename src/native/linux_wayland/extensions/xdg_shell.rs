// xdg-shell.xml

use super::{
    super::libwayland_client::{wl_array, wl_interface, wl_message},
    wayland_protocol::{wl_output_interface, wl_seat_interface},
};
use crate::wayland_interface;

wayland_interface!(
    xdg_wm_base_interface,
    xdg_wm_base,
    3,
    [
        (destroy, "", ()),
        (create_positioner, "n", (xdg_positioner_interface)),
        (get_xdg_surface, "no", (xdg_surface_interface)),
        (pong, "u", ())
    ],
    [("ping", "u")]
);

wayland_interface!(
    xdg_surface_interface,
    xdg_surface,
    3,
    [
        (destroy, "", ()),
        (get_toplevel, "n", (xdg_toplevel_interface)),
        (get_popup, "n?oo", ()),
        (set_window_geometry, "iiii", ()),
        (ack_configure, "u", ())
    ],
    [("configure", "u")]
);

wayland_interface!(
    xdg_toplevel_interface,
    xdg_toplevel,
    3,
    [
        (destroy, "", ()),
        (set_parent, "?o", (xdg_toplevel_interface)),
        (set_title, "s", ()),
        (set_app_id, "s", ()),
        (show_window_menu, "ouii", (xdg_popup_interface)),
        (r#move, "ou", (wl_seat_interface)),
        (resize, "ouu", (wl_seat_interface)),
        (set_max_size, "ii", ()),
        (set_min_size, "ii", ()),
        (set_maximized, "", ()),
        (unset_maximized, "", ()),
        (set_fullscreen, "", (wl_output_interface)),
        (unset_fullscreen, "", ()),
        (set_minimized, "", ())
    ],
    [("configure", "iia"), ("close", "")]
);

wayland_interface!(
    xdg_positioner_interface,
    xdg_positioner,
    3,
    [
        (destroy, "", ()),
        (set_size, "ii", ()),
        (set_anchor_rect, "iiii", ()),
        (set_anchor, "u", ()),
        (set_gravity, "u", ()),
        (set_constraint_adjustment, "u", ()),
        (set_offset, "ii", ()),
        (set_reactive, "3", ()),
        (set_parent_size, "3ii", ()),
        (set_parent_configure, "3u", ())
    ],
    []
);

wayland_interface!(
    xdg_popup_interface,
    xdg_popup,
    3,
    [
        (destroy, "", ()),
        (grab, "ou", (wl_seat_interface)),
        (reposition, "3ou", (xdg_positioner_interface))
    ],
    [("configure", "iiii"), ("popup_done", "")]
);

#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct xdg_wm_base_listener {
    pub ping:
        Option<unsafe extern "C" fn(_: *mut std::ffi::c_void, _: *mut xdg_wm_base, _: u32) -> ()>,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct xdg_surface_listener {
    pub configure:
        Option<unsafe extern "C" fn(_: *mut std::ffi::c_void, _: *mut xdg_surface, _: u32) -> ()>,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct xdg_toplevel_listener {
    pub configure: Option<
        unsafe extern "C" fn(
            _: *mut std::ffi::c_void,
            _: *mut xdg_toplevel,
            _: i32,
            _: i32,
            _: *mut wl_array,
        ) -> (),
    >,
    pub close: Option<unsafe extern "C" fn(_: *mut std::ffi::c_void, _: *mut xdg_toplevel) -> ()>,
}
