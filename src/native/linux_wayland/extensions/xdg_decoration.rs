// xdg-decoration-unstable-v1.xml

use crate::wayland_interface;

use super::{
    super::libwayland_client::{wl_interface, wl_message},
    xdg_shell::xdg_toplevel_interface,
};

pub const ZXDG_TOPLEVEL_DECORATION_V1_MODE_CLIENT_SIDE: u32 = 1;
pub const ZXDG_TOPLEVEL_DECORATION_V1_MODE_SERVER_SIDE: u32 = 2;

#[rustfmt::skip] 
wayland_interface!(
    zxdg_decoration_manager_v1_interface,
    zxdg_decoration_manager_v1,
    3,
    [
        (destroy, "", ()),
        (get_toplevel_decoration, "no", (zxdg_toplevel_decoration_v1_interface, xdg_toplevel_interface))
    ],
    []
);

#[rustfmt::skip] 
wayland_interface!(
    zxdg_toplevel_decoration_v1_interface,
    zxdg_toplevel_decoration_v1,
    3,
    [
        (destroy, "", ()),
        (set_mode, "u", ()),
        (unset_mode, "", ())],
    [("configure", "u")]
);
