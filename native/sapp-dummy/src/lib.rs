#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#![allow(improper_ctypes)]
#![allow(
    unused_variables,
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

mod gl;
mod rand;

pub use gl::*;
pub use rand::*;

pub type sapp_event_type = libc::c_uint;
pub const sapp_event_type__SAPP_EVENTTYPE_FORCE_U32: sapp_event_type = 2147483647;
pub const sapp_event_type__SAPP_EVENTTYPE_NUM: sapp_event_type = 22;
pub const sapp_event_type_SAPP_EVENTTYPE_RAW_DEVICE: sapp_event_type = 21;
pub const sapp_event_type_SAPP_EVENTTYPE_QUIT_REQUESTED: sapp_event_type = 20;
pub const sapp_event_type_SAPP_EVENTTYPE_UPDATE_CURSOR: sapp_event_type = 19;
pub const sapp_event_type_SAPP_EVENTTYPE_RESUMED: sapp_event_type = 18;
pub const sapp_event_type_SAPP_EVENTTYPE_SUSPENDED: sapp_event_type = 17;
pub const sapp_event_type_SAPP_EVENTTYPE_RESTORED: sapp_event_type = 16;
pub const sapp_event_type_SAPP_EVENTTYPE_ICONIFIED: sapp_event_type = 15;
pub const sapp_event_type_SAPP_EVENTTYPE_RESIZED: sapp_event_type = 14;
pub const sapp_event_type_SAPP_EVENTTYPE_TOUCHES_CANCELLED: sapp_event_type = 13;
pub const sapp_event_type_SAPP_EVENTTYPE_TOUCHES_ENDED: sapp_event_type = 12;
pub const sapp_event_type_SAPP_EVENTTYPE_TOUCHES_MOVED: sapp_event_type = 11;
pub const sapp_event_type_SAPP_EVENTTYPE_TOUCHES_BEGAN: sapp_event_type = 10;
pub const sapp_event_type_SAPP_EVENTTYPE_MOUSE_LEAVE: sapp_event_type = 9;
pub const sapp_event_type_SAPP_EVENTTYPE_MOUSE_ENTER: sapp_event_type = 8;
pub const sapp_event_type_SAPP_EVENTTYPE_MOUSE_MOVE: sapp_event_type = 7;
pub const sapp_event_type_SAPP_EVENTTYPE_MOUSE_SCROLL: sapp_event_type = 6;
pub const sapp_event_type_SAPP_EVENTTYPE_MOUSE_UP: sapp_event_type = 5;
pub const sapp_event_type_SAPP_EVENTTYPE_MOUSE_DOWN: sapp_event_type = 4;
pub const sapp_event_type_SAPP_EVENTTYPE_CHAR: sapp_event_type = 3;
pub const sapp_event_type_SAPP_EVENTTYPE_KEY_UP: sapp_event_type = 2;
pub const sapp_event_type_SAPP_EVENTTYPE_KEY_DOWN: sapp_event_type = 1;
pub const sapp_event_type_SAPP_EVENTTYPE_INVALID: sapp_event_type = 0;

pub type sapp_keycode = libc::c_uint;
pub const sapp_keycode_SAPP_KEYCODE_MENU: sapp_keycode = 348;
pub const sapp_keycode_SAPP_KEYCODE_RIGHT_SUPER: sapp_keycode = 347;
pub const sapp_keycode_SAPP_KEYCODE_RIGHT_ALT: sapp_keycode = 346;
pub const sapp_keycode_SAPP_KEYCODE_RIGHT_CONTROL: sapp_keycode = 345;
pub const sapp_keycode_SAPP_KEYCODE_RIGHT_SHIFT: sapp_keycode = 344;
pub const sapp_keycode_SAPP_KEYCODE_LEFT_SUPER: sapp_keycode = 343;
pub const sapp_keycode_SAPP_KEYCODE_LEFT_ALT: sapp_keycode = 342;
pub const sapp_keycode_SAPP_KEYCODE_LEFT_CONTROL: sapp_keycode = 341;
pub const sapp_keycode_SAPP_KEYCODE_LEFT_SHIFT: sapp_keycode = 340;
pub const sapp_keycode_SAPP_KEYCODE_KP_EQUAL: sapp_keycode = 336;
pub const sapp_keycode_SAPP_KEYCODE_KP_ENTER: sapp_keycode = 335;
pub const sapp_keycode_SAPP_KEYCODE_KP_ADD: sapp_keycode = 334;
pub const sapp_keycode_SAPP_KEYCODE_KP_SUBTRACT: sapp_keycode = 333;
pub const sapp_keycode_SAPP_KEYCODE_KP_MULTIPLY: sapp_keycode = 332;
pub const sapp_keycode_SAPP_KEYCODE_KP_DIVIDE: sapp_keycode = 331;
pub const sapp_keycode_SAPP_KEYCODE_KP_DECIMAL: sapp_keycode = 330;
pub const sapp_keycode_SAPP_KEYCODE_KP_9: sapp_keycode = 329;
pub const sapp_keycode_SAPP_KEYCODE_KP_8: sapp_keycode = 328;
pub const sapp_keycode_SAPP_KEYCODE_KP_7: sapp_keycode = 327;
pub const sapp_keycode_SAPP_KEYCODE_KP_6: sapp_keycode = 326;
pub const sapp_keycode_SAPP_KEYCODE_KP_5: sapp_keycode = 325;
pub const sapp_keycode_SAPP_KEYCODE_KP_4: sapp_keycode = 324;
pub const sapp_keycode_SAPP_KEYCODE_KP_3: sapp_keycode = 323;
pub const sapp_keycode_SAPP_KEYCODE_KP_2: sapp_keycode = 322;
pub const sapp_keycode_SAPP_KEYCODE_KP_1: sapp_keycode = 321;
pub const sapp_keycode_SAPP_KEYCODE_KP_0: sapp_keycode = 320;
pub const sapp_keycode_SAPP_KEYCODE_F25: sapp_keycode = 314;
pub const sapp_keycode_SAPP_KEYCODE_F24: sapp_keycode = 313;
pub const sapp_keycode_SAPP_KEYCODE_F23: sapp_keycode = 312;
pub const sapp_keycode_SAPP_KEYCODE_F22: sapp_keycode = 311;
pub const sapp_keycode_SAPP_KEYCODE_F21: sapp_keycode = 310;
pub const sapp_keycode_SAPP_KEYCODE_F20: sapp_keycode = 309;
pub const sapp_keycode_SAPP_KEYCODE_F19: sapp_keycode = 308;
pub const sapp_keycode_SAPP_KEYCODE_F18: sapp_keycode = 307;
pub const sapp_keycode_SAPP_KEYCODE_F17: sapp_keycode = 306;
pub const sapp_keycode_SAPP_KEYCODE_F16: sapp_keycode = 305;
pub const sapp_keycode_SAPP_KEYCODE_F15: sapp_keycode = 304;
pub const sapp_keycode_SAPP_KEYCODE_F14: sapp_keycode = 303;
pub const sapp_keycode_SAPP_KEYCODE_F13: sapp_keycode = 302;
pub const sapp_keycode_SAPP_KEYCODE_F12: sapp_keycode = 301;
pub const sapp_keycode_SAPP_KEYCODE_F11: sapp_keycode = 300;
pub const sapp_keycode_SAPP_KEYCODE_F10: sapp_keycode = 299;
pub const sapp_keycode_SAPP_KEYCODE_F9: sapp_keycode = 298;
pub const sapp_keycode_SAPP_KEYCODE_F8: sapp_keycode = 297;
pub const sapp_keycode_SAPP_KEYCODE_F7: sapp_keycode = 296;
pub const sapp_keycode_SAPP_KEYCODE_F6: sapp_keycode = 295;
pub const sapp_keycode_SAPP_KEYCODE_F5: sapp_keycode = 294;
pub const sapp_keycode_SAPP_KEYCODE_F4: sapp_keycode = 293;
pub const sapp_keycode_SAPP_KEYCODE_F3: sapp_keycode = 292;
pub const sapp_keycode_SAPP_KEYCODE_F2: sapp_keycode = 291;
pub const sapp_keycode_SAPP_KEYCODE_F1: sapp_keycode = 290;
pub const sapp_keycode_SAPP_KEYCODE_PAUSE: sapp_keycode = 284;
pub const sapp_keycode_SAPP_KEYCODE_PRINT_SCREEN: sapp_keycode = 283;
pub const sapp_keycode_SAPP_KEYCODE_NUM_LOCK: sapp_keycode = 282;
pub const sapp_keycode_SAPP_KEYCODE_SCROLL_LOCK: sapp_keycode = 281;
pub const sapp_keycode_SAPP_KEYCODE_CAPS_LOCK: sapp_keycode = 280;
pub const sapp_keycode_SAPP_KEYCODE_END: sapp_keycode = 269;
pub const sapp_keycode_SAPP_KEYCODE_HOME: sapp_keycode = 268;
pub const sapp_keycode_SAPP_KEYCODE_PAGE_DOWN: sapp_keycode = 267;
pub const sapp_keycode_SAPP_KEYCODE_PAGE_UP: sapp_keycode = 266;
pub const sapp_keycode_SAPP_KEYCODE_UP: sapp_keycode = 265;
pub const sapp_keycode_SAPP_KEYCODE_DOWN: sapp_keycode = 264;
pub const sapp_keycode_SAPP_KEYCODE_LEFT: sapp_keycode = 263;
pub const sapp_keycode_SAPP_KEYCODE_RIGHT: sapp_keycode = 262;
pub const sapp_keycode_SAPP_KEYCODE_DELETE: sapp_keycode = 261;
pub const sapp_keycode_SAPP_KEYCODE_INSERT: sapp_keycode = 260;
pub const sapp_keycode_SAPP_KEYCODE_BACKSPACE: sapp_keycode = 259;
pub const sapp_keycode_SAPP_KEYCODE_TAB: sapp_keycode = 258;
pub const sapp_keycode_SAPP_KEYCODE_ENTER: sapp_keycode = 257;
pub const sapp_keycode_SAPP_KEYCODE_ESCAPE: sapp_keycode = 256;
pub const sapp_keycode_SAPP_KEYCODE_WORLD_2: sapp_keycode = 162;
pub const sapp_keycode_SAPP_KEYCODE_WORLD_1: sapp_keycode = 161;
pub const sapp_keycode_SAPP_KEYCODE_GRAVE_ACCENT: sapp_keycode = 96;
pub const sapp_keycode_SAPP_KEYCODE_RIGHT_BRACKET: sapp_keycode = 93;
pub const sapp_keycode_SAPP_KEYCODE_BACKSLASH: sapp_keycode = 92;
pub const sapp_keycode_SAPP_KEYCODE_LEFT_BRACKET: sapp_keycode = 91;
pub const sapp_keycode_SAPP_KEYCODE_Z: sapp_keycode = 90;
pub const sapp_keycode_SAPP_KEYCODE_Y: sapp_keycode = 89;
pub const sapp_keycode_SAPP_KEYCODE_X: sapp_keycode = 88;
pub const sapp_keycode_SAPP_KEYCODE_W: sapp_keycode = 87;
pub const sapp_keycode_SAPP_KEYCODE_V: sapp_keycode = 86;
pub const sapp_keycode_SAPP_KEYCODE_U: sapp_keycode = 85;
pub const sapp_keycode_SAPP_KEYCODE_T: sapp_keycode = 84;
pub const sapp_keycode_SAPP_KEYCODE_S: sapp_keycode = 83;
pub const sapp_keycode_SAPP_KEYCODE_R: sapp_keycode = 82;
pub const sapp_keycode_SAPP_KEYCODE_Q: sapp_keycode = 81;
pub const sapp_keycode_SAPP_KEYCODE_P: sapp_keycode = 80;
pub const sapp_keycode_SAPP_KEYCODE_O: sapp_keycode = 79;
pub const sapp_keycode_SAPP_KEYCODE_N: sapp_keycode = 78;
pub const sapp_keycode_SAPP_KEYCODE_M: sapp_keycode = 77;
pub const sapp_keycode_SAPP_KEYCODE_L: sapp_keycode = 76;
pub const sapp_keycode_SAPP_KEYCODE_K: sapp_keycode = 75;
pub const sapp_keycode_SAPP_KEYCODE_J: sapp_keycode = 74;
pub const sapp_keycode_SAPP_KEYCODE_I: sapp_keycode = 73;
pub const sapp_keycode_SAPP_KEYCODE_H: sapp_keycode = 72;
pub const sapp_keycode_SAPP_KEYCODE_G: sapp_keycode = 71;
pub const sapp_keycode_SAPP_KEYCODE_F: sapp_keycode = 70;
pub const sapp_keycode_SAPP_KEYCODE_E: sapp_keycode = 69;
pub const sapp_keycode_SAPP_KEYCODE_D: sapp_keycode = 68;
pub const sapp_keycode_SAPP_KEYCODE_C: sapp_keycode = 67;
pub const sapp_keycode_SAPP_KEYCODE_B: sapp_keycode = 66;
pub const sapp_keycode_SAPP_KEYCODE_A: sapp_keycode = 65;
pub const sapp_keycode_SAPP_KEYCODE_EQUAL: sapp_keycode = 61;
pub const sapp_keycode_SAPP_KEYCODE_SEMICOLON: sapp_keycode = 59;
pub const sapp_keycode_SAPP_KEYCODE_9: sapp_keycode = 57;
pub const sapp_keycode_SAPP_KEYCODE_8: sapp_keycode = 56;
pub const sapp_keycode_SAPP_KEYCODE_7: sapp_keycode = 55;
pub const sapp_keycode_SAPP_KEYCODE_6: sapp_keycode = 54;
pub const sapp_keycode_SAPP_KEYCODE_5: sapp_keycode = 53;
pub const sapp_keycode_SAPP_KEYCODE_4: sapp_keycode = 52;
pub const sapp_keycode_SAPP_KEYCODE_3: sapp_keycode = 51;
pub const sapp_keycode_SAPP_KEYCODE_2: sapp_keycode = 50;
pub const sapp_keycode_SAPP_KEYCODE_1: sapp_keycode = 49;
pub const sapp_keycode_SAPP_KEYCODE_0: sapp_keycode = 48;
pub const sapp_keycode_SAPP_KEYCODE_SLASH: sapp_keycode = 47;
pub const sapp_keycode_SAPP_KEYCODE_PERIOD: sapp_keycode = 46;
pub const sapp_keycode_SAPP_KEYCODE_MINUS: sapp_keycode = 45;
pub const sapp_keycode_SAPP_KEYCODE_COMMA: sapp_keycode = 44;
pub const sapp_keycode_SAPP_KEYCODE_APOSTROPHE: sapp_keycode = 39;
pub const sapp_keycode_SAPP_KEYCODE_SPACE: sapp_keycode = 32;
pub const sapp_keycode_SAPP_KEYCODE_INVALID: sapp_keycode = 0;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sapp_touchpoint {
    pub identifier: libc::c_ulong,
    pub pos_x: libc::c_float,
    pub pos_y: libc::c_float,
    pub changed: bool,
}

pub type sapp_mousebutton = libc::c_int;
pub const sapp_mousebutton_SAPP_MOUSEBUTTON_MIDDLE: sapp_mousebutton = 2;
pub const sapp_mousebutton_SAPP_MOUSEBUTTON_RIGHT: sapp_mousebutton = 1;
pub const sapp_mousebutton_SAPP_MOUSEBUTTON_LEFT: sapp_mousebutton = 0;
pub const sapp_event_type_SAPP_MOUSEBUTTON_INVALID: sapp_mousebutton = -1;

pub const SAPP_MODIFIER_SHIFT: libc::c_uint = 1 << 0;
pub const SAPP_MODIFIER_CTRL: libc::c_uint = 1 << 1;
pub const SAPP_MODIFIER_ALT: libc::c_uint = 1 << 2;
pub const SAPP_MODIFIER_SUPER: libc::c_uint = 1 << 3;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sapp_event {
    pub frame_count: u64,
    pub type_: sapp_event_type,
    pub key_code: sapp_keycode,
    pub char_code: u32,
    pub key_repeat: bool,
    pub modifiers: u32,
    pub mouse_button: sapp_mousebutton,
    pub mouse_x: libc::c_float,
    pub mouse_y: libc::c_float,
    pub mouse_dx: libc::c_float,
    pub mouse_dy: libc::c_float,
    pub scroll_x: libc::c_float,
    pub scroll_y: libc::c_float,
    pub num_touches: libc::c_int,
    pub touches: [sapp_touchpoint; 8],
    pub window_width: libc::c_int,
    pub window_height: libc::c_int,
    pub framebuffer_width: libc::c_int,
    pub framebuffer_height: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sapp_desc {
    pub init_cb: Option<unsafe extern "C" fn() -> ()>,
    pub frame_cb: Option<unsafe extern "C" fn() -> ()>,
    pub cleanup_cb: Option<unsafe extern "C" fn() -> ()>,
    pub event_cb: Option<unsafe extern "C" fn(_: *const sapp_event) -> ()>,
    pub fail_cb: Option<unsafe extern "C" fn(_: *const libc::c_char) -> ()>,
    pub user_data: *mut libc::c_void,
    pub init_userdata_cb: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
    pub frame_userdata_cb: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
    pub cleanup_userdata_cb: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
    pub event_userdata_cb:
        Option<unsafe extern "C" fn(_: *const sapp_event, _: *mut libc::c_void) -> ()>,
    pub fail_userdata_cb:
        Option<unsafe extern "C" fn(_: *const libc::c_char, _: *mut libc::c_void) -> ()>,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub sample_count: libc::c_int,
    pub swap_interval: libc::c_int,
    pub high_dpi: bool,
    pub fullscreen: bool,
    pub alpha: bool,
    pub window_title: *const libc::c_char,
    pub user_cursor: bool,
    pub html5_canvas_name: *const libc::c_char,
    pub html5_canvas_resize: bool,
    pub html5_preserve_drawing_buffer: bool,
    pub html5_premultiplied_alpha: bool,
    pub html5_ask_leave_site: bool,
    pub ios_keyboard_resizes_canvas: bool,
    pub gl_force_gles2: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _sapp_state {
    pub valid: bool,
    pub window_width: libc::c_int,
    pub window_height: libc::c_int,
    pub framebuffer_width: libc::c_int,
    pub framebuffer_height: libc::c_int,
    pub sample_count: libc::c_int,
    pub swap_interval: libc::c_int,
    pub dpi_scale: libc::c_float,
    pub gles2_fallback: bool,
    pub first_frame: bool,
    pub init_called: bool,
    pub cleanup_called: bool,
    pub quit_requested: bool,
    pub quit_ordered: bool,
    pub html5_canvas_name: *const libc::c_char,
    pub html5_ask_leave_site: bool,
    pub window_title: [libc::c_char; 128],
    pub window_title_wide: [i32; 128],
    pub frame_count: u64,
    pub mouse_x: libc::c_float,
    pub mouse_y: libc::c_float,
    pub win32_mouse_tracked: bool,
    pub onscreen_keyboard_shown: bool,
    pub event: sapp_event,
    pub desc: sapp_desc,
    pub keycodes: [sapp_keycode; 512],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _sapp_gl_fbconfig {
    pub red_bits: libc::c_int,
    pub green_bits: libc::c_int,
    pub blue_bits: libc::c_int,
    pub alpha_bits: libc::c_int,
    pub depth_bits: libc::c_int,
    pub stencil_bits: libc::c_int,
    pub samples: libc::c_int,
    pub doublebuffer: bool,
    pub handle: libc::c_ulong,
}

#[no_mangle]
pub unsafe extern "C" fn sapp_run(mut desc: *const sapp_desc) -> libc::c_int {
    return 0 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn sapp_frame_count() -> u64 {
    0
}
#[no_mangle]
pub unsafe extern "C" fn sapp_quit() {}
#[no_mangle]
pub unsafe extern "C" fn sapp_cancel_quit() {}
#[no_mangle]
pub unsafe extern "C" fn sapp_request_quit() {}
#[no_mangle]
pub unsafe extern "C" fn sapp_query_desc() -> sapp_desc {
    unimplemented!()
}
#[no_mangle]
pub unsafe extern "C" fn sapp_userdata() -> *mut libc::c_void {
    unimplemented!()
}
#[no_mangle]
pub unsafe extern "C" fn sapp_mouse_shown() -> bool {
    return false;
}
pub unsafe extern "C" fn sapp_set_cursor_grab(mut _grab: bool) {}
#[no_mangle]
pub unsafe extern "C" fn sapp_show_mouse(mut shown: bool) {}

#[no_mangle]
pub unsafe extern "C" fn sapp_keyboard_shown() -> bool {
    false
}
#[no_mangle]
pub unsafe extern "C" fn sapp_show_keyboard(mut shown: bool) {}
#[no_mangle]
pub unsafe extern "C" fn sapp_dpi_scale() -> libc::c_float {
    1.0
}
#[no_mangle]
pub unsafe extern "C" fn sapp_high_dpi() -> bool {
    false
}
#[no_mangle]
pub unsafe extern "C" fn sapp_height() -> libc::c_int {
    1
}
#[no_mangle]
pub unsafe extern "C" fn sapp_width() -> libc::c_int {
    1
}
#[no_mangle]
pub unsafe extern "C" fn sapp_isvalid() -> bool {
    true
}
pub unsafe extern "C" fn sapp_is_elapsed_timer_supported() -> bool {
    false
}
