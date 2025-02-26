#![allow(non_camel_case_types, non_upper_case_globals, dead_code)]

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xkb_context {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xkb_keymap {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xkb_state {
    _unused: [u8; 0],
}

use core::ffi::{c_int, c_uint};
crate::declare_module!(
    LibXkbCommon,
    "libxkbcommon.so",
    "libxkbcommon.so.0",
    "libxkbcommon.so.0.0.0",
    "libxkbcommon.so.0.0.0.0",
    ...
    ...
    pub fn xkb_context_new(c_int) -> *mut xkb_context,
    pub fn xkb_context_unref(*mut xkb_context),
    pub fn xkb_keymap_new_from_string(*mut xkb_context, *mut libc::FILE, c_int, c_int) -> *mut xkb_keymap,
    pub fn xkb_keymap_unref(*mut xkb_keymap),
    pub fn xkb_keymap_key_repeats(*mut xkb_keymap, c_uint) -> c_int,
    pub fn xkb_state_new(*mut xkb_keymap) -> *mut xkb_state,
    pub fn xkb_state_unref(*mut xkb_state),
    pub fn xkb_state_key_get_one_sym(*mut xkb_state, c_uint) -> c_uint,
    pub fn xkb_state_update_mask(*mut xkb_state, c_uint, c_uint, c_uint, c_uint, c_uint, c_uint) -> c_int,
    pub fn xkb_keysym_to_utf32(c_uint) -> c_uint,
    ...
    ...
);
