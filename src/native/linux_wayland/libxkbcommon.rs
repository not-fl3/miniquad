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

pub const XKB_STATE_MODS_EFFECTIVE: c_int = 1 << 3;
pub const XKB_MOD_NAME_SHIFT: &str = "Shift";
pub const XKB_MOD_NAME_CTRL: &str = "Control";
pub const XKB_MOD_NAME_ALT: &str = "Mod1";
pub const XKB_MOD_NAME_LOGO: &str = "Mod4";

use core::ffi::{c_char, c_int, c_uint};
pub type xkb_keycode_t = c_uint;
pub type xkb_keysym_t = c_uint;
pub type xkb_mod_index_t = c_uint;
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
    pub fn xkb_keymap_key_repeats(*mut xkb_keymap, xkb_keycode_t) -> c_int,
    pub fn xkb_state_new(*mut xkb_keymap) -> *mut xkb_state,
    pub fn xkb_state_unref(*mut xkb_state),
    pub fn xkb_state_key_get_one_sym(*mut xkb_state, xkb_keycode_t) -> xkb_keysym_t,
    pub fn xkb_keymap_mod_get_index(*mut xkb_keymap, *const c_char) -> xkb_mod_index_t,
    pub fn xkb_state_mod_index_is_active(*mut xkb_state, xkb_mod_index_t, c_int) -> c_int,
    pub fn xkb_state_update_mask(*mut xkb_state, c_uint, c_uint, c_uint, c_uint, c_uint, c_uint) -> c_int,
    pub fn xkb_keysym_to_utf32(xkb_keysym_t) -> c_uint,
    ...
    ...
);

impl LibXkbCommon {
    // The keycodes in Miniquad are obtained without modifiers (for example, `Shift + Key1` is
    // translated to `Key1` and not `Exclam`)
    pub unsafe fn keymap_key_get_sym_without_mod(
        &mut self,
        keymap: *mut xkb_keymap,
        keycode: xkb_keycode_t,
    ) -> xkb_keysym_t {
        let xkb_state = (self.xkb_state_new)(keymap);
        let keysym = (self.xkb_state_key_get_one_sym)(xkb_state, keycode);
        (self.xkb_state_unref)(xkb_state);
        keysym
    }
}

pub mod libxkbcommon_ex {
    use super::*;
    use crate::KeyMods;

    /// In `xkb` the modifier indices are tied to a particular `xkb_keymap` and not hardcoded.
    #[derive(Copy, Clone)]
    pub struct XkbKeymap {
        pub xkb_keymap: *mut xkb_keymap,
        shift: xkb_mod_index_t,
        ctrl: xkb_mod_index_t,
        alt: xkb_mod_index_t,
        logo: xkb_mod_index_t,
    }

    impl Default for XkbKeymap {
        fn default() -> Self {
            XkbKeymap {
                xkb_keymap: std::ptr::null_mut(),
                shift: 0,
                ctrl: 0,
                alt: 0,
                logo: 0,
            }
        }
    }

    impl XkbKeymap {
        pub unsafe fn cache_mod_indices(&mut self, libxkb: &mut LibXkbCommon) {
            let shift = std::ffi::CString::new(XKB_MOD_NAME_SHIFT).unwrap();
            self.shift = (libxkb.xkb_keymap_mod_get_index)(self.xkb_keymap, shift.as_ptr());
            let ctrl = std::ffi::CString::new(XKB_MOD_NAME_CTRL).unwrap();
            self.ctrl = (libxkb.xkb_keymap_mod_get_index)(self.xkb_keymap, ctrl.as_ptr());
            let alt = std::ffi::CString::new(XKB_MOD_NAME_ALT).unwrap();
            self.alt = (libxkb.xkb_keymap_mod_get_index)(self.xkb_keymap, alt.as_ptr());
            let logo = std::ffi::CString::new(XKB_MOD_NAME_LOGO).unwrap();
            self.logo = (libxkb.xkb_keymap_mod_get_index)(self.xkb_keymap, logo.as_ptr());
        }
        pub unsafe fn get_keymods(
            &self,
            libxkb: &mut LibXkbCommon,
            xkb_state: *mut xkb_state,
        ) -> KeyMods {
            let mut mods = KeyMods::default();
            let is_active = libxkb.xkb_state_mod_index_is_active;
            if (is_active)(xkb_state, self.shift, XKB_STATE_MODS_EFFECTIVE) == 1 {
                mods.shift = true;
            }
            if (is_active)(xkb_state, self.ctrl, XKB_STATE_MODS_EFFECTIVE) == 1 {
                mods.ctrl = true;
            }
            if (is_active)(xkb_state, self.alt, XKB_STATE_MODS_EFFECTIVE) == 1 {
                mods.alt = true;
            }
            if (is_active)(xkb_state, self.logo, XKB_STATE_MODS_EFFECTIVE) == 1 {
                mods.logo = true;
            }
            mods
        }
    }
}

pub use libxkbcommon_ex::XkbKeymap;
