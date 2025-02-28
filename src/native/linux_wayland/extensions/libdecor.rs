#![allow(non_camel_case_types, dead_code)]

use super::super::libwayland_client::*;
use super::xdg_shell::*;
use crate::declare_module;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct libdecor {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct libdecor_frame {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct libdecor_configuration {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct libdecor_state {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct libdecor_error {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct libdecor_interface {
    pub error: unsafe extern "C" fn(*mut libdecor, *mut libdecor_error, *const c_char),
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct libdecor_frame_interface {
    pub configure:
        unsafe extern "C" fn(*mut libdecor_frame, *mut libdecor_configuration, *mut c_void),
    pub close: unsafe extern "C" fn(*mut libdecor_frame, *mut c_void),
    pub commit: unsafe extern "C" fn(*mut libdecor_frame, *mut c_void),
}

use core::ffi::{c_char, c_int, c_void};

declare_module! {
    LibDecor,
    "libdecor-0.so",
    "libdecor-0.so.0",
    ...
    ...
    pub fn libdecor_new(*mut wl_display, *mut libdecor_interface) -> *mut libdecor,
    pub fn libdecor_decorate(
        *mut libdecor,
        *mut wl_surface,
        *mut libdecor_frame_interface,
        *mut c_void
    ) -> *mut libdecor_frame,
    pub fn libdecor_frame_set_app_id(*mut libdecor_frame, *const c_char),
    pub fn libdecor_frame_set_title(*mut libdecor_frame, *const c_char),
    pub fn libdecor_frame_map(*mut libdecor_frame),
    pub fn libdecor_state_new(c_int, c_int) -> *mut libdecor_state,
    pub fn libdecor_frame_commit(
        *mut libdecor_frame,
        *mut libdecor_state,
        *mut libdecor_configuration,
    ),
    pub fn libdecor_state_free(*mut libdecor_state),
    pub fn libdecor_configuration_get_content_size(
        *mut libdecor_configuration,
        *mut libdecor_frame,
        *mut c_int,
        *mut c_int,
    ) -> c_int,
    pub fn libdecor_frame_get_xdg_surface(*mut libdecor_frame) -> *mut xdg_surface,
    pub fn libdecor_frame_get_xdg_toplevel(*mut libdecor_frame) -> *mut xdg_toplevel,
    ...
    ...
}
