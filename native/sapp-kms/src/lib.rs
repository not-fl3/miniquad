#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals
)]

mod kms;

use kms::{drm_screen_height, drm_screen_width, init, swap_buffers};

mod rand;
pub use rand::*;

mod egl;
pub mod gl3;

pub mod query_stab;
pub use query_stab::*;

pub use egl::*;
pub use gl3::*;

pub use gl3 as gl;

use std::option::Option::None;

static mut _sapp: _sapp_state = _sapp_state {
    valid: false,
    window_width: 0,
    window_height: 0,
    framebuffer_width: 0,
    framebuffer_height: 0,
    sample_count: 0,
    swap_interval: 0,
    dpi_scale: 0.,
    gles2_fallback: false,
    first_frame: false,
    init_called: false,
    cleanup_called: false,
    quit_requested: false,
    quit_ordered: false,
    window_title: String::new(),
    frame_count: 0,
    mouse_x: 0.,
    mouse_y: 0.,
    win32_mouse_tracked: false,
    onscreen_keyboard_shown: false,
    event: sapp_event {
        frame_count: 0,
        type_: sapp_event_type_SAPP_EVENTTYPE_INVALID,
        key_code: sapp_keycode_SAPP_KEYCODE_INVALID,
        char_code: 0,
        key_repeat: false,
        modifiers: 0,
        mouse_button: sapp_mousebutton_SAPP_MOUSEBUTTON_LEFT,
        mouse_x: 0.,
        mouse_y: 0.,
        mouse_dx: 0.,
        mouse_dy: 0.,
        scroll_x: 0.,
        scroll_y: 0.,
        num_touches: 0,
        touches: [sapp_touchpoint {
            identifier: 0,
            pos_x: 0.,
            pos_y: 0.,
            changed: false,
        }; 8],
        window_width: 0,
        window_height: 0,
        framebuffer_width: 0,
        framebuffer_height: 0,
    },
    desc: sapp_desc {
        init_cb: None,
        frame_cb: None,
        cleanup_cb: None,
        event_cb: None,
        fail_cb: None,
        user_data: 0 as *const std::ffi::c_void as *mut std::ffi::c_void,
        init_userdata_cb: None,
        frame_userdata_cb: None,
        cleanup_userdata_cb: None,
        event_userdata_cb: None,
        fail_userdata_cb: None,
        width: 0,
        height: 0,
        sample_count: 0,
        swap_interval: 0,
        high_dpi: false,
        fullscreen: false,
        window_resizable: false,
        alpha: false,
        window_title: 0 as *const _,
        user_cursor: false,
        html5_canvas_name: 0 as *const _,
        html5_canvas_resize: false,
        html5_preserve_drawing_buffer: false,
        html5_premultiplied_alpha: false,
        html5_ask_leave_site: false,
        ios_keyboard_resizes_canvas: false,
        gl_force_gles2: false,
    },
    keycodes: [sapp_keycode_SAPP_KEYCODE_INVALID; 512],
};

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
    pub window_resizable: bool,
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
#[derive(Clone)]
pub struct _sapp_state {
    pub valid: bool,
    pub window_width: i32,
    pub window_height: i32,
    pub framebuffer_width: i32,
    pub framebuffer_height: i32,
    pub sample_count: i32,
    pub swap_interval: i32,
    pub dpi_scale: f32,
    pub gles2_fallback: bool,
    pub first_frame: bool,
    pub init_called: bool,
    pub cleanup_called: bool,
    pub quit_requested: bool,
    pub quit_ordered: bool,
    pub window_title: String,
    pub frame_count: u64,
    pub mouse_x: f32,
    pub mouse_y: f32,
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

unsafe fn init_state(desc: *const sapp_desc) {
    fn _sapp_def<T: PartialEq>(x: T, default: T, zero: T) -> T {
        if x == zero {
            default
        } else {
            x
        }
    }

    _sapp.desc = *desc;
    _sapp.first_frame = true;
    _sapp.window_width = 480;
    _sapp.window_height = 320;
    _sapp.framebuffer_width = _sapp.window_width;
    _sapp.framebuffer_height = _sapp.window_height;
    _sapp.sample_count = _sapp_def(_sapp.desc.sample_count, 1, 0);
    _sapp.swap_interval = _sapp_def(_sapp.desc.swap_interval, 1, 0);
    if _sapp.desc.window_title.is_null() {
        _sapp.window_title = "sokol_app".to_string();
    } else {
        _sapp.window_title = std::ffi::CStr::from_ptr(_sapp.desc.window_title)
            .to_string_lossy()
            .to_string();
    }
    _sapp.dpi_scale = 1.0;
}

unsafe fn init_keytable() {
    // TODO: double check this, there are errors here
    _sapp.keycodes[0x00B] = sapp_keycode_SAPP_KEYCODE_0;
    _sapp.keycodes[0x002] = sapp_keycode_SAPP_KEYCODE_1;
    _sapp.keycodes[0x003] = sapp_keycode_SAPP_KEYCODE_2;
    _sapp.keycodes[0x004] = sapp_keycode_SAPP_KEYCODE_3;
    _sapp.keycodes[0x005] = sapp_keycode_SAPP_KEYCODE_4;
    _sapp.keycodes[0x006] = sapp_keycode_SAPP_KEYCODE_5;
    _sapp.keycodes[0x007] = sapp_keycode_SAPP_KEYCODE_6;
    _sapp.keycodes[0x008] = sapp_keycode_SAPP_KEYCODE_7;
    _sapp.keycodes[0x009] = sapp_keycode_SAPP_KEYCODE_8;
    _sapp.keycodes[0x00A] = sapp_keycode_SAPP_KEYCODE_9;
    _sapp.keycodes[0x01E] = sapp_keycode_SAPP_KEYCODE_A;
    _sapp.keycodes[0x030] = sapp_keycode_SAPP_KEYCODE_B;
    _sapp.keycodes[0x02E] = sapp_keycode_SAPP_KEYCODE_C;
    _sapp.keycodes[0x020] = sapp_keycode_SAPP_KEYCODE_D;
    _sapp.keycodes[0x012] = sapp_keycode_SAPP_KEYCODE_E;
    _sapp.keycodes[0x021] = sapp_keycode_SAPP_KEYCODE_F;
    _sapp.keycodes[0x022] = sapp_keycode_SAPP_KEYCODE_G;
    _sapp.keycodes[0x023] = sapp_keycode_SAPP_KEYCODE_H;
    _sapp.keycodes[0x017] = sapp_keycode_SAPP_KEYCODE_I;
    _sapp.keycodes[0x024] = sapp_keycode_SAPP_KEYCODE_J;
    _sapp.keycodes[0x025] = sapp_keycode_SAPP_KEYCODE_K;
    _sapp.keycodes[0x026] = sapp_keycode_SAPP_KEYCODE_L;
    _sapp.keycodes[0x032] = sapp_keycode_SAPP_KEYCODE_M;
    _sapp.keycodes[0x031] = sapp_keycode_SAPP_KEYCODE_N;
    _sapp.keycodes[0x018] = sapp_keycode_SAPP_KEYCODE_O;
    _sapp.keycodes[0x019] = sapp_keycode_SAPP_KEYCODE_P;
    _sapp.keycodes[0x010] = sapp_keycode_SAPP_KEYCODE_Q;
    _sapp.keycodes[0x013] = sapp_keycode_SAPP_KEYCODE_R;
    _sapp.keycodes[0x01F] = sapp_keycode_SAPP_KEYCODE_S;
    _sapp.keycodes[0x014] = sapp_keycode_SAPP_KEYCODE_T;
    _sapp.keycodes[0x016] = sapp_keycode_SAPP_KEYCODE_U;
    _sapp.keycodes[0x02F] = sapp_keycode_SAPP_KEYCODE_V;
    _sapp.keycodes[0x011] = sapp_keycode_SAPP_KEYCODE_W;
    _sapp.keycodes[0x02D] = sapp_keycode_SAPP_KEYCODE_X;
    _sapp.keycodes[0x015] = sapp_keycode_SAPP_KEYCODE_Y;
    _sapp.keycodes[0x02C] = sapp_keycode_SAPP_KEYCODE_Z;
    _sapp.keycodes[0x028] = sapp_keycode_SAPP_KEYCODE_APOSTROPHE;
    _sapp.keycodes[0x02B] = sapp_keycode_SAPP_KEYCODE_BACKSLASH;
    _sapp.keycodes[0x033] = sapp_keycode_SAPP_KEYCODE_COMMA;
    _sapp.keycodes[0x00D] = sapp_keycode_SAPP_KEYCODE_EQUAL;
    _sapp.keycodes[0x029] = sapp_keycode_SAPP_KEYCODE_GRAVE_ACCENT;
    _sapp.keycodes[0x01A] = sapp_keycode_SAPP_KEYCODE_LEFT_BRACKET;
    _sapp.keycodes[0x00C] = sapp_keycode_SAPP_KEYCODE_MINUS;
    _sapp.keycodes[0x034] = sapp_keycode_SAPP_KEYCODE_PERIOD;
    _sapp.keycodes[0x01B] = sapp_keycode_SAPP_KEYCODE_RIGHT_BRACKET;
    _sapp.keycodes[0x027] = sapp_keycode_SAPP_KEYCODE_SEMICOLON;
    _sapp.keycodes[0x035] = sapp_keycode_SAPP_KEYCODE_SLASH;
    _sapp.keycodes[0x056] = sapp_keycode_SAPP_KEYCODE_WORLD_2;
    _sapp.keycodes[0x00E] = sapp_keycode_SAPP_KEYCODE_BACKSPACE;
    _sapp.keycodes[0x153] = sapp_keycode_SAPP_KEYCODE_DELETE;
    _sapp.keycodes[0x14F] = sapp_keycode_SAPP_KEYCODE_END;
    _sapp.keycodes[0x01C] = sapp_keycode_SAPP_KEYCODE_ENTER;
    _sapp.keycodes[0x001] = sapp_keycode_SAPP_KEYCODE_ESCAPE;
    _sapp.keycodes[0x147] = sapp_keycode_SAPP_KEYCODE_HOME;
    _sapp.keycodes[0x152] = sapp_keycode_SAPP_KEYCODE_INSERT;
    _sapp.keycodes[0x15D] = sapp_keycode_SAPP_KEYCODE_MENU;
    _sapp.keycodes[0x151] = sapp_keycode_SAPP_KEYCODE_PAGE_DOWN;
    _sapp.keycodes[0x149] = sapp_keycode_SAPP_KEYCODE_PAGE_UP;
    _sapp.keycodes[0x045] = sapp_keycode_SAPP_KEYCODE_PAUSE;
    _sapp.keycodes[0x146] = sapp_keycode_SAPP_KEYCODE_PAUSE;
    _sapp.keycodes[0x039] = sapp_keycode_SAPP_KEYCODE_SPACE;
    _sapp.keycodes[0x00F] = sapp_keycode_SAPP_KEYCODE_TAB;
    _sapp.keycodes[0x03A] = sapp_keycode_SAPP_KEYCODE_CAPS_LOCK;
    _sapp.keycodes[0x145] = sapp_keycode_SAPP_KEYCODE_NUM_LOCK;
    _sapp.keycodes[0x046] = sapp_keycode_SAPP_KEYCODE_SCROLL_LOCK;
    _sapp.keycodes[0x03B] = sapp_keycode_SAPP_KEYCODE_F1;
    _sapp.keycodes[0x03C] = sapp_keycode_SAPP_KEYCODE_F2;
    _sapp.keycodes[0x03D] = sapp_keycode_SAPP_KEYCODE_F3;
    _sapp.keycodes[0x03E] = sapp_keycode_SAPP_KEYCODE_F4;
    _sapp.keycodes[0x03F] = sapp_keycode_SAPP_KEYCODE_F5;
    _sapp.keycodes[0x040] = sapp_keycode_SAPP_KEYCODE_F6;
    _sapp.keycodes[0x041] = sapp_keycode_SAPP_KEYCODE_F7;
    _sapp.keycodes[0x042] = sapp_keycode_SAPP_KEYCODE_F8;
    _sapp.keycodes[0x043] = sapp_keycode_SAPP_KEYCODE_F9;
    _sapp.keycodes[0x044] = sapp_keycode_SAPP_KEYCODE_F10;
    _sapp.keycodes[0x057] = sapp_keycode_SAPP_KEYCODE_F11;
    _sapp.keycodes[0x058] = sapp_keycode_SAPP_KEYCODE_F12;
    _sapp.keycodes[0x064] = sapp_keycode_SAPP_KEYCODE_F13;
    _sapp.keycodes[0x065] = sapp_keycode_SAPP_KEYCODE_F14;
    _sapp.keycodes[0x066] = sapp_keycode_SAPP_KEYCODE_F15;
    _sapp.keycodes[0x067] = sapp_keycode_SAPP_KEYCODE_UP;
    _sapp.keycodes[0x068] = sapp_keycode_SAPP_KEYCODE_DOWN;
    _sapp.keycodes[0x069] = sapp_keycode_SAPP_KEYCODE_LEFT;
    _sapp.keycodes[0x06A] = sapp_keycode_SAPP_KEYCODE_RIGHT;
    _sapp.keycodes[0x06B] = sapp_keycode_SAPP_KEYCODE_F20;
    _sapp.keycodes[0x06C] = sapp_keycode_SAPP_KEYCODE_DOWN;
    _sapp.keycodes[0x06D] = sapp_keycode_SAPP_KEYCODE_F22;
    _sapp.keycodes[0x06E] = sapp_keycode_SAPP_KEYCODE_F23;
    _sapp.keycodes[0x076] = sapp_keycode_SAPP_KEYCODE_F24;
    _sapp.keycodes[0x06F] = sapp_keycode_SAPP_KEYCODE_DELETE;
    _sapp.keycodes[0x038] = sapp_keycode_SAPP_KEYCODE_LEFT_ALT;
    _sapp.keycodes[0x01D] = sapp_keycode_SAPP_KEYCODE_LEFT_CONTROL;
    _sapp.keycodes[0x02A] = sapp_keycode_SAPP_KEYCODE_LEFT_SHIFT;
    _sapp.keycodes[0x15B] = sapp_keycode_SAPP_KEYCODE_LEFT_SUPER;
    _sapp.keycodes[0x137] = sapp_keycode_SAPP_KEYCODE_PRINT_SCREEN;
    _sapp.keycodes[0x138] = sapp_keycode_SAPP_KEYCODE_RIGHT_ALT;
    _sapp.keycodes[0x11D] = sapp_keycode_SAPP_KEYCODE_RIGHT_CONTROL;
    _sapp.keycodes[0x036] = sapp_keycode_SAPP_KEYCODE_RIGHT_SHIFT;
    _sapp.keycodes[0x15C] = sapp_keycode_SAPP_KEYCODE_RIGHT_SUPER;
    _sapp.keycodes[0x150] = sapp_keycode_SAPP_KEYCODE_DOWN;
    _sapp.keycodes[0x14B] = sapp_keycode_SAPP_KEYCODE_LEFT;
    _sapp.keycodes[0x14D] = sapp_keycode_SAPP_KEYCODE_RIGHT;
    _sapp.keycodes[0x148] = sapp_keycode_SAPP_KEYCODE_UP;
    _sapp.keycodes[0x052] = sapp_keycode_SAPP_KEYCODE_KP_0;
    _sapp.keycodes[0x04F] = sapp_keycode_SAPP_KEYCODE_KP_1;
    _sapp.keycodes[0x050] = sapp_keycode_SAPP_KEYCODE_KP_2;
    _sapp.keycodes[0x051] = sapp_keycode_SAPP_KEYCODE_KP_3;
    _sapp.keycodes[0x04B] = sapp_keycode_SAPP_KEYCODE_KP_4;
    _sapp.keycodes[0x04C] = sapp_keycode_SAPP_KEYCODE_KP_5;
    _sapp.keycodes[0x04D] = sapp_keycode_SAPP_KEYCODE_KP_6;
    _sapp.keycodes[0x047] = sapp_keycode_SAPP_KEYCODE_KP_7;
    _sapp.keycodes[0x048] = sapp_keycode_SAPP_KEYCODE_KP_8;
    _sapp.keycodes[0x049] = sapp_keycode_SAPP_KEYCODE_KP_9;
    _sapp.keycodes[0x04E] = sapp_keycode_SAPP_KEYCODE_KP_ADD;
    _sapp.keycodes[0x053] = sapp_keycode_SAPP_KEYCODE_KP_DECIMAL;
    _sapp.keycodes[0x135] = sapp_keycode_SAPP_KEYCODE_KP_DIVIDE;
    _sapp.keycodes[0x11C] = sapp_keycode_SAPP_KEYCODE_KP_ENTER;
    _sapp.keycodes[0x037] = sapp_keycode_SAPP_KEYCODE_KP_MULTIPLY;
    _sapp.keycodes[0x04A] = sapp_keycode_SAPP_KEYCODE_KP_SUBTRACT;
}

unsafe fn _sapp_call_init() {
    if _sapp.desc.init_cb.is_some() {
        _sapp.desc.init_cb.expect("non-null function pointer")();
    } else if _sapp.desc.init_userdata_cb.is_some() {
        _sapp
            .desc
            .init_userdata_cb
            .expect("non-null function pointer")(_sapp.desc.user_data);
    }
    _sapp.init_called = true;
}

unsafe fn _sapp_init_event(type_: sapp_event_type) {
    _sapp.event = std::mem::zeroed();
    _sapp.event.type_ = type_;
    _sapp.event.frame_count = _sapp.frame_count;
    _sapp.event.mouse_button = -1;
    _sapp.event.window_width = _sapp.window_width;
    _sapp.event.window_height = _sapp.window_height;
    _sapp.event.framebuffer_width = _sapp.framebuffer_width;
    _sapp.event.framebuffer_height = _sapp.framebuffer_height;
}

unsafe fn _sapp_mouse_event(type_: sapp_event_type, btn: sapp_mousebutton) {
    _sapp_init_event(type_);
    //_sapp.event.modifiers = _sapp_win32_mods();
    _sapp.event.mouse_button = btn;
    _sapp.event.mouse_x = _sapp.mouse_x;
    _sapp.event.mouse_y = _sapp.mouse_y;
    _sapp_call_event(&_sapp.event);
}

unsafe fn _sapp_key_event(type_: sapp_event_type, vk: u32, repeat: bool) {
    if vk < _sapp.keycodes.len() as _ {
        _sapp_init_event(type_);
        //_sapp.event.modifiers = _sapp_win32_mods();
        _sapp.event.key_code = _sapp.keycodes[vk as usize];
        _sapp.event.key_repeat = repeat;
        _sapp_call_event(&_sapp.event);
    }
}

unsafe fn _sapp_scroll_event(x: f32, y: f32) {
    _sapp_init_event(sapp_event_type_SAPP_EVENTTYPE_MOUSE_SCROLL);
    //_sapp.event.modifiers = _sapp_win32_mods();
    _sapp.event.scroll_x = -x / 10.0;
    _sapp.event.scroll_y = y / 10.0;
    _sapp_call_event(&_sapp.event);
}

unsafe fn _sapp_char_event(c: u32, repeat: bool) {
    _sapp_init_event(sapp_event_type_SAPP_EVENTTYPE_CHAR);
    //_sapp.event.modifiers = _sapp_win32_mods();
    _sapp.event.char_code = c;
    _sapp.event.key_repeat = repeat;
    _sapp_call_event(&_sapp.event);
}

unsafe fn _sapp_call_event(e: *const sapp_event) {
    if !_sapp.cleanup_called {
        if _sapp.desc.event_cb.is_some() {
            _sapp.desc.event_cb.expect("non-null function pointer")(e);
        } else if _sapp.desc.event_userdata_cb.is_some() {
            _sapp
                .desc
                .event_userdata_cb
                .expect("non-null function pointer")(e, _sapp.desc.user_data);
        }
    };
}

unsafe fn _sapp_call_frame() {
    if _sapp.init_called as i32 != 0 && !_sapp.cleanup_called {
        if _sapp.desc.frame_cb.is_some() {
            _sapp.desc.frame_cb.expect("non-null function pointer")();
        } else if _sapp.desc.frame_userdata_cb.is_some() {
            _sapp
                .desc
                .frame_userdata_cb
                .expect("non-null function pointer")(_sapp.desc.user_data);
        }
    };
}

unsafe fn _sapp_frame() {
    if _sapp.first_frame {
        _sapp.first_frame = false;
        _sapp_call_init();
    }
    _sapp_call_frame();
    _sapp.frame_count = _sapp.frame_count.wrapping_add(1);
}

#[no_mangle]
pub unsafe extern "C" fn sapp_run(desc: *const sapp_desc) -> libc::c_int {
    init();

    init_state(desc);
    init_keytable();

    _sapp.window_width = drm_screen_width() as _;
    _sapp.window_height = drm_screen_height() as _;

    loop {
        _sapp_frame();
        swap_buffers();
    }
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
pub unsafe extern "C" fn sapp_show_mouse(_shown: bool) {}

#[no_mangle]
pub unsafe extern "C" fn sapp_keyboard_shown() -> bool {
    false
}
#[no_mangle]
pub unsafe extern "C" fn sapp_show_keyboard(_shown: bool) {}
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
    _sapp.window_height
}
#[no_mangle]
pub unsafe extern "C" fn sapp_width() -> libc::c_int {
    _sapp.window_width
}
#[no_mangle]
pub unsafe extern "C" fn sapp_isvalid() -> bool {
    true
}
