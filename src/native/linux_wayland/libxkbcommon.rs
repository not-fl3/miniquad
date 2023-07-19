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

pub type xkb_context_new = unsafe extern "C" fn(flags: ::std::os::raw::c_int) -> *mut xkb_context;
pub type xkb_context_unref = unsafe extern "C" fn(context: *mut xkb_context);

pub type xkb_keymap_new_from_string = unsafe extern "C" fn(
    context: *mut xkb_context,
    file: *mut libc::FILE,
    format: ::std::os::raw::c_int,
    flags: ::std::os::raw::c_int,
) -> *mut xkb_keymap;
pub type xkb_keymap_unref = unsafe extern "C" fn(keymap: *mut xkb_keymap);

pub type xkb_state_new = unsafe extern "C" fn(keymap: *mut xkb_keymap) -> *mut xkb_state;
pub type xkb_state_unref = unsafe extern "C" fn(state: *mut xkb_state);
pub type xkb_state_key_get_one_sym = unsafe extern "C" fn(state: *mut xkb_state, key: u32) -> u32;
pub type xkb_state_update_mask = unsafe extern "C" fn(
    state: *mut xkb_state,
    depressed_mods: u32,
    latched_mods: u32,
    locked_mods: u32,
    depressed_layout: u32,
    latched_layout: u32,
    locked_layout: u32,
) -> ::std::os::raw::c_int;

#[derive(Clone)]
pub struct LibXkbCommon {
    _module: std::rc::Rc<crate::native::module::Module>,
    pub xkb_context_new: xkb_context_new,
    pub xkb_context_unref: xkb_context_unref,
    pub xkb_keymap_new_from_string: xkb_keymap_new_from_string,
    pub xkb_keymap_unref: xkb_keymap_unref,
    pub xkb_state_new: xkb_state_new,
    pub xkb_state_unref: xkb_state_unref,
    pub xkb_state_key_get_one_sym: xkb_state_key_get_one_sym,
    pub xkb_state_update_mask: xkb_state_update_mask,
}

impl LibXkbCommon {
    pub fn try_load() -> Option<LibXkbCommon> {
        crate::native::module::Module::load("libxkbcommon.so")
            .or_else(|_| crate::native::module::Module::load("libxkbcommon.so.0"))
            .map(|module| LibXkbCommon {
                xkb_context_new: module.get_symbol("xkb_context_new").unwrap(),
                xkb_context_unref: module.get_symbol("xkb_context_unref").unwrap(),
                xkb_keymap_new_from_string: module
                    .get_symbol("xkb_keymap_new_from_string")
                    .unwrap(),
                xkb_keymap_unref: module.get_symbol("xkb_keymap_unref").unwrap(),
                xkb_state_new: module.get_symbol("xkb_state_new").unwrap(),
                xkb_state_unref: module.get_symbol("xkb_state_unref").unwrap(),
                xkb_state_key_get_one_sym: module.get_symbol("xkb_state_key_get_one_sym").unwrap(),
                xkb_state_update_mask: module.get_symbol("xkb_state_update_mask").unwrap(),

                _module: std::rc::Rc::new(module),
            })
            .ok()
    }
}
