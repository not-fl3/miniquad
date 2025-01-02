#![allow(non_camel_case_types, dead_code)]

use super::libwayland_client::wl_surface;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct wl_egl_window {
    _unused: [u8; 0],
}
pub type wl_egl_window_create = unsafe extern "C" fn(
    surface: *mut wl_surface,
    width: ::core::ffi::c_int,
    height: ::core::ffi::c_int,
) -> *mut wl_egl_window;

pub type wl_egl_window_destroy = unsafe extern "C" fn(egl_window: *mut wl_egl_window);

pub type wl_egl_window_resize = unsafe extern "C" fn(
    egl_window: *mut wl_egl_window,
    width: ::core::ffi::c_int,
    height: ::core::ffi::c_int,
    dx: ::core::ffi::c_int,
    dy: ::core::ffi::c_int,
);

pub type wl_egl_window_get_attached_size = unsafe extern "C" fn(
    egl_window: *mut wl_egl_window,
    width: *mut ::core::ffi::c_int,
    height: *mut ::core::ffi::c_int,
);

pub struct LibWaylandEgl {
    _module: crate::native::module::Module,
    pub wl_egl_window_create: wl_egl_window_create,
    pub wl_egl_window_destroy: wl_egl_window_destroy,
    pub wl_egl_window_resize: wl_egl_window_resize,
    pub wl_egl_window_get_attached_size: wl_egl_window_get_attached_size,
}

impl LibWaylandEgl {
    pub fn try_load() -> Option<LibWaylandEgl> {
        crate::native::module::Module::load("libwayland-egl.so")
            .or_else(|_| crate::native::module::Module::load("libwayland-egl.so.1"))
            .and_then(|module| Ok(LibWaylandEgl {
                wl_egl_window_create: module.get_symbol("wl_egl_window_create")?,
                wl_egl_window_destroy: module.get_symbol("wl_egl_window_destroy")?,
                wl_egl_window_resize: module.get_symbol("wl_egl_window_resize")?,
                wl_egl_window_get_attached_size: module
                    .get_symbol("wl_egl_window_get_attached_size")
                    ?,
                _module: module,
            })) 
            .map_err(|err| {
                eprintln!("failed loading libwayland-egl: {err}");
                err
            })
            .ok()
    }
}
