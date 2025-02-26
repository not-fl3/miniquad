#![allow(non_camel_case_types, dead_code)]

use super::libwayland_client::wl_surface;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct wl_egl_window {
    _unused: [u8; 0],
}

use core::ffi::c_int;
crate::declare_module!(
    LibWaylandEgl,
    "libwayland-egl.so",
    "libwayland-egl.so.1",
    ...
    ...
    pub fn wl_egl_window_create(*mut wl_surface, c_int, c_int) -> *mut wl_egl_window,
    pub fn wl_egl_window_destroy(*mut wl_egl_window),
    pub fn wl_egl_window_resize(*mut wl_egl_window, c_int, c_int, c_int, c_int),
    pub fn wl_egl_window_get_attached_size(*mut wl_egl_window, *mut c_int, *mut c_int),
    ...
    ...
);
