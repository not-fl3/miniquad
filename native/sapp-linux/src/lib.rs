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

pub mod gl;
mod rand;
mod x;
mod x_cursor;
mod xi_input;

pub mod clipboard;

pub use gl::*;
pub use rand::*;

use crate::x::*;

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
pub const sapp_mousebutton_SAPP_MOUSEBUTTON_INVALID: sapp_mousebutton = -1;

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

/// opcode from XQueryExtension("XInputExtension")
static mut _sapp_xi_extension_opcode: i32 = -1;

static mut _sapp_empty_cursor: x_cursor::Cursor = 0;

pub type GLXContext = *mut __GLXcontext;
pub type PFNGLXDESTROYCONTEXTPROC =
    Option<unsafe extern "C" fn(_: *mut Display, _: GLXContext) -> ()>;
pub type GLXWindow = XID;
pub type PFNGLXDESTROYWINDOWPROC =
    Option<unsafe extern "C" fn(_: *mut Display, _: GLXWindow) -> ()>;
pub type PFNGLXSWAPBUFFERSPROC =
    Option<unsafe extern "C" fn(_: *mut Display, _: GLXDrawable) -> ()>;
pub type GLXDrawable = XID;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub state: CARD32,
    pub icon: Window,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _sapp_x11_codepair {
    pub keysym: u16,
    pub ucs: u16,
}
impl _sapp_x11_codepair {
    const fn new(keysym: u16, ucs: u16) -> _sapp_x11_codepair {
        _sapp_x11_codepair { keysym, ucs }
    }
}
pub type PFNGLXMAKECURRENTPROC =
    Option<unsafe extern "C" fn(_: *mut Display, _: GLXDrawable, _: GLXContext) -> libc::c_int>;
pub type PFNGLXSWAPINTERVALMESAPROC = Option<unsafe extern "C" fn(_: libc::c_int) -> libc::c_int>;
pub type PFNGLXSWAPINTERVALEXTPROC =
    Option<unsafe extern "C" fn(_: *mut Display, _: GLXDrawable, _: libc::c_int) -> ()>;
pub type GLXFBConfig = *mut __GLXFBConfig;
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
pub type PFNGLXGETFBCONFIGATTRIBPROC = Option<
    unsafe extern "C" fn(
        _: *mut Display,
        _: GLXFBConfig,
        _: libc::c_int,
        _: *mut libc::c_int,
    ) -> libc::c_int,
>;
pub type PFNGLXGETFBCONFIGSPROC = Option<
    unsafe extern "C" fn(_: *mut Display, _: libc::c_int, _: *mut libc::c_int) -> *mut GLXFBConfig,
>;
pub type PFNGLXGETCLIENTSTRINGPROC =
    Option<unsafe extern "C" fn(_: *mut Display, _: libc::c_int) -> *const libc::c_char>;
pub type PFNGLXCREATEWINDOWPROC = Option<
    unsafe extern "C" fn(
        _: *mut Display,
        _: GLXFBConfig,
        _: Window,
        _: *const libc::c_int,
    ) -> GLXWindow,
>;
pub type PFNGLXCREATECONTEXTATTRIBSARBPROC = Option<
    unsafe extern "C" fn(
        _: *mut Display,
        _: GLXFBConfig,
        _: GLXContext,
        _: libc::c_int,
        _: *const libc::c_int,
    ) -> GLXContext,
>;
pub type PFNGLXGETVISUALFROMFBCONFIGPROC =
    Option<unsafe extern "C" fn(_: *mut Display, _: GLXFBConfig) -> *mut XVisualInfo>;
pub type PFNGLXQUERYEXTENSIONSSTRINGPROC =
    Option<unsafe extern "C" fn(_: *mut Display, _: libc::c_int) -> *const libc::c_char>;
pub type __GLXextproc = Option<unsafe extern "C" fn() -> ()>;
pub type PFNGLXGETPROCADDRESSPROC = Option<unsafe extern "C" fn(_: *const GLubyte) -> __GLXextproc>;
pub type PFNGLXQUERYVERSIONPROC = Option<
    unsafe extern "C" fn(_: *mut Display, _: *mut libc::c_int, _: *mut libc::c_int) -> libc::c_int,
>;
pub type PFNGLXQUERYEXTENSIONPROC = Option<
    unsafe extern "C" fn(_: *mut Display, _: *mut libc::c_int, _: *mut libc::c_int) -> libc::c_int,
>;
pub type PFNGLXCREATENEWCONTEXTPROC = Option<
    unsafe extern "C" fn(
        _: *mut Display,
        _: GLXFBConfig,
        _: libc::c_int,
        _: GLXContext,
        _: libc::c_int,
    ) -> GLXContext,
>;

pub unsafe extern "C" fn _sapp_x11_create_window(mut visual: *mut Visual, mut depth: libc::c_int) {
    _sapp_x11_colormap = XCreateColormap(_sapp_x11_display, _sapp_x11_root, visual, AllocNone);
    let mut wa = XSetWindowAttributes {
        background_pixmap: 0,
        background_pixel: 0,
        border_pixmap: 0,
        border_pixel: 0,
        bit_gravity: 0,
        win_gravity: 0,
        backing_store: 0,
        backing_planes: 0,
        backing_pixel: 0,
        save_under: 0,
        event_mask: 0,
        do_not_propagate_mask: 0,
        override_redirect: 0,
        colormap: 0,
        cursor: 0,
    };
    memset(
        &mut wa as *mut XSetWindowAttributes as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<XSetWindowAttributes>() as libc::c_ulong,
    );
    let wamask = (CWBorderPixel | CWColormap | CWEventMask) as u32;
    wa.colormap = _sapp_x11_colormap;
    wa.border_pixel = 0 as libc::c_int as libc::c_ulong;
    wa.event_mask = StructureNotifyMask
        | KeyPressMask
        | KeyReleaseMask
        | PointerMotionMask
        | ButtonPressMask
        | ButtonReleaseMask
        | ExposureMask
        | FocusChangeMask
        | VisibilityChangeMask
        | EnterWindowMask
        | LeaveWindowMask
        | PropertyChangeMask;
    _sapp_x11_grab_error_handler();

    _sapp_x11_window = XCreateWindow(
        _sapp_x11_display,
        _sapp_x11_root,
        0 as libc::c_int,
        0 as libc::c_int,
        _sapp.window_width as libc::c_uint,
        _sapp.window_height as libc::c_uint,
        0 as libc::c_int as libc::c_uint,
        depth,
        InputOutput as libc::c_uint,
        visual,
        wamask as libc::c_ulong,
        &mut wa,
    );
    _sapp_x11_release_error_handler();
    if _sapp_x11_window == 0 {
        _sapp_fail(b"X11: Failed to create window\x00" as *const u8 as *const libc::c_char);
    }

    _sapp_xi_extension_opcode = xi_input::query_xi_extension()
        .unwrap_or_else(|| panic!("Failed to initialize XInputExtension"));

    _sapp_empty_cursor = x_cursor::create_empty_cursor();

    let mut protocols: [Atom; 1] = [_sapp_x11_WM_DELETE_WINDOW];
    XSetWMProtocols(
        _sapp_x11_display,
        _sapp_x11_window,
        protocols.as_mut_ptr(),
        1 as libc::c_int,
    );
    let mut hints = XAllocSizeHints();
    (*hints).flags |= PWinGravity;
    (*hints).win_gravity = StaticGravity;
    XSetWMNormalHints(_sapp_x11_display, _sapp_x11_window, hints);
    XFree(hints as *mut libc::c_void);
    _sapp_x11_update_window_title();
    _sapp_x11_query_window_size();
}

pub unsafe extern "C" fn _sapp_strcpy(
    mut src: *const libc::c_char,
    mut dst: *mut libc::c_char,
    mut max_len: libc::c_int,
) {
    assert!(
        !src.is_null() && !dst.is_null() && max_len > 0 as libc::c_int,
        "src && dst && (max_len > 0)"
    );
    let end: *mut libc::c_char =
        &mut *dst.offset((max_len - 1 as libc::c_int) as isize) as *mut libc::c_char;
    let mut c = 0 as libc::c_int as libc::c_char;
    let mut i = 0 as libc::c_int;
    while i < max_len {
        c = *src;
        if c as libc::c_int != 0 as libc::c_int {
            src = src.offset(1)
        }
        let fresh0 = dst;
        dst = dst.offset(1);
        *fresh0 = c;
        i += 1
    }
    if c as libc::c_int != 0 as libc::c_int {
        *end = 0 as libc::c_int as libc::c_char
    };
}
pub unsafe extern "C" fn _sapp_init_state(mut desc: *const sapp_desc) {
    memset(
        &mut _sapp as *mut _sapp_state as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<_sapp_state>() as libc::c_ulong,
    );
    _sapp.desc = *desc;
    _sapp.first_frame = true;
    _sapp.window_width = if _sapp.desc.width == 0 as libc::c_int {
        640 as libc::c_int
    } else {
        _sapp.desc.width
    };
    _sapp.window_height = if _sapp.desc.height == 0 as libc::c_int {
        480 as libc::c_int
    } else {
        _sapp.desc.height
    };
    _sapp.framebuffer_width = _sapp.window_width;
    _sapp.framebuffer_height = _sapp.window_height;
    _sapp.sample_count = if _sapp.desc.sample_count == 0 as libc::c_int {
        1 as libc::c_int
    } else {
        _sapp.desc.sample_count
    };
    _sapp.swap_interval = if _sapp.desc.swap_interval == 0 as libc::c_int {
        1 as libc::c_int
    } else {
        _sapp.desc.swap_interval
    };
    _sapp.html5_canvas_name = if _sapp.desc.html5_canvas_name.is_null() {
        b"canvas\x00" as *const u8 as *const libc::c_char
    } else {
        _sapp.desc.html5_canvas_name
    };
    _sapp.html5_ask_leave_site = _sapp.desc.html5_ask_leave_site;
    if !_sapp.desc.window_title.is_null() {
        _sapp_strcpy(
            _sapp.desc.window_title,
            _sapp.window_title.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as libc::c_int,
        );
    } else {
        _sapp_strcpy(
            b"sokol_app\x00" as *const u8 as *const libc::c_char,
            _sapp.window_title.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as libc::c_int,
        );
    }
    _sapp.dpi_scale = 1.0f32;
}
pub unsafe extern "C" fn _sapp_x11_query_system_dpi() {
    let mut rms = XResourceManagerString(_sapp_x11_display);
    if !rms.is_null() {
        let mut db = XrmGetStringDatabase(rms);
        if !db.is_null() {
            let mut value = XrmValue {
                size: 0,
                addr: 0 as *mut libc::c_char,
            };
            let mut type_ = std::ptr::null_mut();
            if XrmGetResource(
                db,
                b"Xft.dpi\x00" as *const u8 as *const libc::c_char,
                b"Xft.Dpi\x00" as *const u8 as *const libc::c_char,
                &mut type_,
                &mut value,
            ) != 0
            {
                if !type_.is_null()
                    && strcmp(type_, b"String\x00" as *const u8 as *const libc::c_char)
                        == 0 as libc::c_int
                {
                    _sapp_x11_dpi = atof(value.addr as *const libc::c_char) as libc::c_float
                }
            }
            XrmDestroyDatabase(db);
        }
    };
}
pub static mut _sapp_x11_dpi: libc::c_float = 0.;
pub unsafe extern "C" fn _sapp_x11_init_extensions() {
    _sapp_x11_UTF8_STRING = XInternAtom(
        _sapp_x11_display,
        b"UTF8_STRING\x00" as *const u8 as *const libc::c_char,
        false as _,
    );
    _sapp_x11_WM_PROTOCOLS = XInternAtom(
        _sapp_x11_display,
        b"WM_PROTOCOLS\x00" as *const u8 as *const libc::c_char,
        false as _,
    );
    _sapp_x11_WM_DELETE_WINDOW = XInternAtom(
        _sapp_x11_display,
        b"WM_DELETE_WINDOW\x00" as *const u8 as *const libc::c_char,
        false as _,
    );
    _sapp_x11_WM_STATE = XInternAtom(
        _sapp_x11_display,
        b"WM_STATE\x00" as *const u8 as *const libc::c_char,
        false as _,
    );
    _sapp_x11_NET_WM_NAME = XInternAtom(
        _sapp_x11_display,
        b"_NET_WM_NAME\x00" as *const u8 as *const libc::c_char,
        false as _,
    );
    _sapp_x11_NET_WM_ICON_NAME = XInternAtom(
        _sapp_x11_display,
        b"_NET_WM_ICON_NAME\x00" as *const u8 as *const libc::c_char,
        false as _,
    );
}
pub static mut _sapp_glx_CreateNewContext: PFNGLXCREATENEWCONTEXTPROC = None;
pub static mut _sapp_glx_QueryExtension: PFNGLXQUERYEXTENSIONPROC = None;
pub static mut _sapp_glx_errorbase: libc::c_int = 0;
pub static mut _sapp_glx_eventbase: libc::c_int = 0;
pub static mut _sapp_glx_QueryVersion: PFNGLXQUERYVERSIONPROC = None;
pub static mut _sapp_glx_major: libc::c_int = 0;
pub static mut _sapp_glx_minor: libc::c_int = 0;
pub static mut _sapp_glx_ARB_framebuffer_sRGB: bool = false;
pub static mut _sapp_glx_EXT_framebuffer_sRGB: bool = false;
pub static mut _sapp_glx_GetProcAddress: PFNGLXGETPROCADDRESSPROC = None;
pub static mut _sapp_glx_GetProcAddressARB: PFNGLXGETPROCADDRESSPROC = None;
pub static mut _sapp_glx_libgl: *mut libc::c_void = 0 as *const libc::c_void as *mut libc::c_void;

unsafe fn _sapp_glx_getprocaddr<T: Sized>(procname: &[u8]) -> T {
    let procname = procname.as_ptr() as _;

    if _sapp_glx_GetProcAddress.is_some() {
        return ::std::mem::transmute_copy::<_, T>(&_sapp_glx_GetProcAddress
            .expect("non-null function pointer")(
            procname
        ));
    } else if _sapp_glx_GetProcAddressARB.is_some() {
        return ::std::mem::transmute_copy::<_, T>(&_sapp_glx_GetProcAddressARB
            .expect("non-null function pointer")(
            procname
        ));
    } else {
        return ::std::mem::transmute_copy::<_, T>(&dlsym(_sapp_glx_libgl, procname as _));
    };
}
pub unsafe extern "C" fn _sapp_glx_has_ext(
    mut ext: *const libc::c_char,
    mut extensions: *const libc::c_char,
) -> bool {
    assert!(!ext.is_null());

    let mut start = extensions;
    loop {
        let mut where_0: *const libc::c_char = strstr(start, ext);
        if where_0.is_null() {
            return false;
        }
        let mut terminator = where_0.offset(strlen(ext) as isize);
        if where_0 == start
            || *where_0.offset(-(1 as libc::c_int as isize)) as libc::c_int == ' ' as i32
        {
            if *terminator as libc::c_int == ' ' as i32
                || *terminator as libc::c_int == '\u{0}' as i32
            {
                break;
            }
        }
        start = terminator
    }
    return true;
}
pub unsafe fn _sapp_glx_extsupported(
    mut ext: &[u8],
    mut extensions: *const libc::c_char,
) -> bool {
    if !extensions.is_null() {
        return _sapp_glx_has_ext(ext.as_ptr() as _, extensions);
    } else {
        return false;
    };
}
pub static mut _sapp_glx_QueryExtensionsString: PFNGLXQUERYEXTENSIONSSTRINGPROC = None;
pub unsafe extern "C" fn _sapp_glx_init() {
    let mut sonames: [*const libc::c_char; 3] = [
        b"libGL.so.1\x00" as *const u8 as *const libc::c_char,
        b"libGL.so\x00" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut i = 0 as libc::c_int;
    while !sonames[i as usize].is_null() {
        _sapp_glx_libgl = dlopen(sonames[i as usize], RTLD_LAZY | RTLD_GLOBAL);
        if !_sapp_glx_libgl.is_null() {
            break;
        }
        i += 1
    }
    if _sapp_glx_libgl.is_null() {
        _sapp_fail(b"GLX: failed to load libGL\x00" as *const u8 as *const libc::c_char);
    }

    unsafe fn load<T: Sized>(lib: *mut libc::c_void, symbol: &[u8]) -> T {
        ::std::mem::transmute_copy::<*mut libc::c_void, T>(&dlsym(lib, symbol.as_ptr() as _))
    }
    _sapp_glx_GetFBConfigs = load(_sapp_glx_libgl, b"glXGetFBConfigs\x00");
    _sapp_glx_GetFBConfigAttrib = load(_sapp_glx_libgl, b"glXGetFBConfigAttrib\x00");
    _sapp_glx_GetClientString = load(_sapp_glx_libgl, b"glXGetClientString\x00");
    _sapp_glx_QueryExtension = load(_sapp_glx_libgl, b"glXQueryExtension\x00");
    _sapp_glx_QueryVersion = load(_sapp_glx_libgl, b"glXQueryVersion\x00");
    _sapp_glx_DestroyContext = load(_sapp_glx_libgl, b"glXDestroyContext\x00");
    _sapp_glx_MakeCurrent = load(_sapp_glx_libgl, b"glXMakeCurrent\x00");
    _sapp_glx_SwapBuffers = load(_sapp_glx_libgl, b"glXSwapBuffers\x00");
    _sapp_glx_QueryExtensionsString = load(_sapp_glx_libgl, b"glXQueryExtensionsString\x00");
    _sapp_glx_CreateNewContext = load(_sapp_glx_libgl, b"glXCreateNewContext\x00");
    _sapp_glx_CreateWindow = load(_sapp_glx_libgl, b"glXCreateWindow\x00");
    _sapp_glx_DestroyWindow = load(_sapp_glx_libgl, b"glXDestroyWindow\x00");
    _sapp_glx_GetProcAddress = load(_sapp_glx_libgl, b"glXGetProcAddress\x00");
    _sapp_glx_GetProcAddressARB = load(_sapp_glx_libgl, b"glXGetProcAddressARB\x00");
    _sapp_glx_GetVisualFromFBConfig = load(_sapp_glx_libgl, b"glXGetVisualFromFBConfig\x00");
    if _sapp_glx_GetFBConfigs.is_none()
        || _sapp_glx_GetFBConfigAttrib.is_none()
        || _sapp_glx_GetClientString.is_none()
        || _sapp_glx_QueryExtension.is_none()
        || _sapp_glx_QueryVersion.is_none()
        || _sapp_glx_DestroyContext.is_none()
        || _sapp_glx_MakeCurrent.is_none()
        || _sapp_glx_SwapBuffers.is_none()
        || _sapp_glx_QueryExtensionsString.is_none()
        || _sapp_glx_CreateNewContext.is_none()
        || _sapp_glx_CreateWindow.is_none()
        || _sapp_glx_DestroyWindow.is_none()
        || _sapp_glx_GetProcAddress.is_none()
        || _sapp_glx_GetProcAddressARB.is_none()
        || _sapp_glx_GetVisualFromFBConfig.is_none()
    {
        _sapp_fail(
            b"GLX: failed to load required entry points\x00" as *const u8 as *const libc::c_char,
        );
    }
    if _sapp_glx_QueryExtension.expect("non-null function pointer")(
        _sapp_x11_display,
        &mut _sapp_glx_errorbase,
        &mut _sapp_glx_eventbase,
    ) == 0
    {
        _sapp_fail(b"GLX: GLX extension not found\x00" as *const u8 as *const libc::c_char);
    }
    if _sapp_glx_QueryVersion.expect("non-null function pointer")(
        _sapp_x11_display,
        &mut _sapp_glx_major,
        &mut _sapp_glx_minor,
    ) == 0
    {
        _sapp_fail(b"GLX: Failed to query GLX version\x00" as *const u8 as *const libc::c_char);
    }
    if _sapp_glx_major == 1 as libc::c_int && _sapp_glx_minor < 3 as libc::c_int {
        _sapp_fail(b"GLX: GLX version 1.3 is required\x00" as *const u8 as *const libc::c_char);
    }
    let mut exts = _sapp_glx_QueryExtensionsString.expect("non-null function pointer")(
        _sapp_x11_display,
        _sapp_x11_screen,
    );
    if _sapp_glx_extsupported(b"GLX_EXT_swap_control\x00", exts) {
        _sapp_glx_SwapIntervalEXT =
            _sapp_glx_getprocaddr::<PFNGLXSWAPINTERVALEXTPROC>(b"glXSwapIntervalEXT\x00");
        _sapp_glx_EXT_swap_control = _sapp_glx_SwapIntervalEXT.is_some()
    }
    if _sapp_glx_extsupported(b"GLX_MESA_swap_control\x00", exts) {
        _sapp_glx_SwapIntervalMESA = _sapp_glx_getprocaddr(b"glXSwapIntervalMESA\x00");
        _sapp_glx_MESA_swap_control = _sapp_glx_SwapIntervalMESA.is_some()
    }
    _sapp_glx_ARB_multisample = _sapp_glx_extsupported(b"GLX_ARB_multisample\x00", exts);
    _sapp_glx_ARB_framebuffer_sRGB = _sapp_glx_extsupported(b"GLX_ARB_framebuffer_sRGB\x00", exts);
    _sapp_glx_EXT_framebuffer_sRGB = _sapp_glx_extsupported(b"GLX_EXT_framebuffer_sRGB\x00", exts);
    if _sapp_glx_extsupported(b"GLX_ARB_create_context\x00", exts) {
        _sapp_glx_CreateContextAttribsARB =
            _sapp_glx_getprocaddr(b"glXCreateContextAttribsARB\x00");
        _sapp_glx_ARB_create_context = _sapp_glx_CreateContextAttribsARB.is_some()
    }
    _sapp_glx_ARB_create_context_profile =
        _sapp_glx_extsupported(b"GLX_ARB_create_context_profile\x00", exts);
}
pub static mut _sapp_glx_GetVisualFromFBConfig: PFNGLXGETVISUALFROMFBCONFIGPROC = None;
pub unsafe extern "C" fn _sapp_glx_choose_visual(
    mut visual: *mut *mut Visual,
    mut depth: *mut libc::c_int,
) {
    let mut native = _sapp_glx_choosefbconfig();
    if native.is_null() {
        _sapp_fail(
            b"GLX: Failed to find a suitable GLXFBConfig\x00" as *const u8 as *const libc::c_char,
        );
    }
    let mut result = _sapp_glx_GetVisualFromFBConfig.expect("non-null function pointer")(
        _sapp_x11_display,
        native,
    );
    if result.is_null() {
        _sapp_fail(
            b"GLX: Failed to retrieve Visual for GLXFBConfig\x00" as *const u8
                as *const libc::c_char,
        );
    }
    *visual = (*result).visual;
    *depth = (*result).depth;
    XFree(result as *mut libc::c_void);
}
pub static mut _sapp_x11_root: Window = 0;
pub static mut _sapp_x11_NET_WM_NAME: Atom = 0;
pub static mut _sapp_x11_NET_WM_ICON_NAME: Atom = 0;
pub static mut _sapp_x11_UTF8_STRING: Atom = 0;
pub unsafe extern "C" fn _sapp_x11_update_window_title() {
    Xutf8SetWMProperties(
        _sapp_x11_display,
        _sapp_x11_window,
        _sapp.window_title.as_mut_ptr(),
        _sapp.window_title.as_mut_ptr(),
        std::ptr::null_mut(),
        0 as libc::c_int,
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
    );
    XChangeProperty(
        _sapp_x11_display,
        _sapp_x11_window,
        _sapp_x11_NET_WM_NAME,
        _sapp_x11_UTF8_STRING,
        8 as libc::c_int,
        PropModeReplace,
        _sapp.window_title.as_mut_ptr() as *mut libc::c_uchar,
        strlen(_sapp.window_title.as_mut_ptr()) as libc::c_int,
    );
    XChangeProperty(
        _sapp_x11_display,
        _sapp_x11_window,
        _sapp_x11_NET_WM_ICON_NAME,
        _sapp_x11_UTF8_STRING,
        8 as libc::c_int,
        PropModeReplace,
        _sapp.window_title.as_mut_ptr() as *mut libc::c_uchar,
        strlen(_sapp.window_title.as_mut_ptr()) as libc::c_int,
    );
    XFlush(_sapp_x11_display);
}
pub unsafe extern "C" fn _sapp_x11_query_window_size() {
    let mut attribs = XWindowAttributes {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
        border_width: 0,
        depth: 0,
        visual: 0 as *mut Visual,
        root: 0,
        class: 0,
        bit_gravity: 0,
        win_gravity: 0,
        backing_store: 0,
        backing_planes: 0,
        backing_pixel: 0,
        save_under: 0,
        colormap: 0,
        map_installed: 0,
        map_state: 0,
        all_event_masks: 0,
        your_event_mask: 0,
        do_not_propagate_mask: 0,
        override_redirect: 0,
        screen: 0 as *mut Screen,
    };
    XGetWindowAttributes(_sapp_x11_display, _sapp_x11_window, &mut attribs);
    _sapp.window_width = attribs.width;
    _sapp.window_height = attribs.height;
    _sapp.framebuffer_width = _sapp.window_width;
    _sapp.framebuffer_height = _sapp.framebuffer_height;
}
pub static mut _sapp_glx_ARB_create_context: bool = false;
pub static mut _sapp_glx_ARB_create_context_profile: bool = false;
pub static mut _sapp_x11_error_code: libc::c_uchar = 0;
pub unsafe extern "C" fn _sapp_x11_error_handler(
    mut display: *mut Display,
    mut event: *mut XErrorEvent,
) -> libc::c_int {
    _sapp_x11_error_code = (*event).error_code;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn _sapp_x11_grab_error_handler() {
    _sapp_x11_error_code = Success as libc::c_uchar;
    XSetErrorHandler(Some(
        _sapp_x11_error_handler
            as unsafe extern "C" fn(_: *mut Display, _: *mut XErrorEvent) -> libc::c_int,
    ));
}
pub static mut _sapp_glx_CreateContextAttribsARB: PFNGLXCREATECONTEXTATTRIBSARBPROC = None;
pub const GLX_CONTEXT_MAJOR_VERSION_ARB: libc::c_int = 0x2091 as libc::c_int;
pub const GLX_CONTEXT_MINOR_VERSION_ARB: libc::c_int = 0x2092 as libc::c_int;
pub const GLX_CONTEXT_PROFILE_MASK_ARB: libc::c_int = 0x9126 as libc::c_int;
pub const GLX_CONTEXT_CORE_PROFILE_BIT_ARB: libc::c_int = 0x1 as libc::c_int;
pub const GLX_CONTEXT_FLAGS_ARB: libc::c_int = 0x2094 as libc::c_int;
pub const GLX_CONTEXT_FORWARD_COMPATIBLE_BIT_ARB: libc::c_int = 0x2 as libc::c_int;
pub unsafe extern "C" fn _sapp_x11_release_error_handler() {
    XSync(_sapp_x11_display, false as _);
    XSetErrorHandler(None);
}
pub static mut _sapp_glx_CreateWindow: PFNGLXCREATEWINDOWPROC = None;
pub static mut _sapp_glx_GetClientString: PFNGLXGETCLIENTSTRINGPROC = None;
pub const GLX_VENDOR: libc::c_int = 1 as libc::c_int;
pub static mut _sapp_glx_GetFBConfigs: PFNGLXGETFBCONFIGSPROC = None;
pub static mut _sapp_x11_screen: libc::c_int = 0;
pub const GLX_RENDER_TYPE: libc::c_int = 0x8011 as libc::c_int;
pub const GLX_RGBA_BIT: libc::c_int = 0x1 as libc::c_int;
pub const GLX_DRAWABLE_TYPE: libc::c_int = 0x8010 as libc::c_int;
pub const GLX_WINDOW_BIT: libc::c_int = 0x1 as libc::c_int;
pub const GLX_RED_SIZE: libc::c_int = 8 as libc::c_int;
pub const GLX_GREEN_SIZE: libc::c_int = 9 as libc::c_int;
pub const GLX_BLUE_SIZE: libc::c_int = 10 as libc::c_int;
pub const GLX_ALPHA_SIZE: libc::c_int = 11 as libc::c_int;
pub const GLX_DEPTH_SIZE: libc::c_int = 12 as libc::c_int;
pub const GLX_STENCIL_SIZE: libc::c_int = 13 as libc::c_int;
pub const GLX_DOUBLEBUFFER: libc::c_int = 5 as libc::c_int;
pub static mut _sapp_glx_ARB_multisample: bool = false;
pub static mut _sapp_glx_GetFBConfigAttrib: PFNGLXGETFBCONFIGATTRIBPROC = None;
pub unsafe extern "C" fn _sapp_glx_attrib(
    mut fbconfig: GLXFBConfig,
    mut attrib: libc::c_int,
) -> libc::c_int {
    let mut value: libc::c_int = 0;
    _sapp_glx_GetFBConfigAttrib.expect("non-null function pointer")(
        _sapp_x11_display,
        fbconfig,
        attrib,
        &mut value,
    );
    return value;
}
pub const GLX_SAMPLES: libc::c_int = 0x186a1 as libc::c_int;
pub fn _sapp_gl_init_fbconfig() -> _sapp_gl_fbconfig {
    _sapp_gl_fbconfig {
        red_bits: -(1 as libc::c_int),
        green_bits: -(1 as libc::c_int),
        blue_bits: -(1 as libc::c_int),
        alpha_bits: -(1 as libc::c_int),
        depth_bits: -(1 as libc::c_int),
        stencil_bits: -(1 as libc::c_int),
        samples: -(1 as libc::c_int),
        doublebuffer: false,
        handle: 0,
    }
}
pub unsafe extern "C" fn _sapp_gl_choose_fbconfig(
    mut desired: *const _sapp_gl_fbconfig,
    mut alternatives: *const _sapp_gl_fbconfig,
    mut count: libc::c_uint,
) -> *const _sapp_gl_fbconfig {
    let mut i: i32 = 0;
    let mut missing: i32 = 0;
    let mut least_missing: i32 = 1000000;
    let mut color_diff: i32 = 0;
    let mut least_color_diff: i32 = 10000000;
    let mut extra_diff: i32 = 0;
    let mut least_extra_diff: i32 = 10000000;
    let mut current = 0 as *const _sapp_gl_fbconfig;
    let mut closest = std::ptr::null();

    for i in 0..count as i32 {
        current = alternatives.offset(i as isize);

        if (*desired).doublebuffer == (*current).doublebuffer {
            missing = 0;
            if (*desired).alpha_bits > 0 && (*current).alpha_bits == 0 {
                missing += 1;
            }
            if (*desired).depth_bits > 0 && (*current).depth_bits == 0 {
                missing += 1;
            }
            if (*desired).stencil_bits > 0 && (*current).stencil_bits == 0 {
                missing += 1;
            }
            if (*desired).samples > 0 && (*current).samples == 0 {
                // Technically, several multisampling buffers could be
                //  involved, but that's a lower level implentation detail and
                //  not important to us here, so we count them as one

                missing += 1;
            }

            // These polynomials make many small channel size differences matter
            //  less than one large channel size difference
            //  Calculate color channel size difference value

            color_diff = 0;
            if (*desired).red_bits != -1 {
                color_diff += ((*desired).red_bits - (*current).red_bits)
                    * ((*desired).red_bits - (*current).red_bits);
            }
            if (*desired).green_bits != -1 {
                color_diff += ((*desired).green_bits - (*current).green_bits)
                    * ((*desired).green_bits - (*current).green_bits)
            }
            if (*desired).blue_bits != -1 {
                color_diff += ((*desired).blue_bits - (*current).blue_bits)
                    * ((*desired).blue_bits - (*current).blue_bits)
            }

            // Calculate non-color channel size difference value
            extra_diff = 0;
            if (*desired).alpha_bits != -1 {
                extra_diff += ((*desired).alpha_bits - (*current).alpha_bits)
                    * ((*desired).alpha_bits - (*current).alpha_bits)
            }
            if (*desired).depth_bits != -1 {
                extra_diff += ((*desired).depth_bits - (*current).depth_bits)
                    * ((*desired).depth_bits - (*current).depth_bits);
            }
            if (*desired).stencil_bits != -1 {
                extra_diff = ((*desired).stencil_bits - (*current).stencil_bits)
                    * ((*desired).stencil_bits - (*current).stencil_bits);
            }
            if (*desired).samples != -1 {
                extra_diff += ((*desired).samples - (*current).samples)
                    * ((*desired).samples - (*current).samples);
            }
            if missing < least_missing {
                closest = current
            } else if missing == least_missing {
                if color_diff < least_color_diff
                    || color_diff == least_color_diff && extra_diff < least_extra_diff
                {
                    closest = current
                }
            }

            // Figure out if the current one is better than the best one found so far
            //  Least number of missing buffers is the most important heuristic,
            //  then color buffer size match and lastly size match for other buffers

            if current == closest {
                least_missing = missing;
                least_color_diff = color_diff;
                least_extra_diff = extra_diff
            }
        }
    }
    return closest;
}
pub unsafe extern "C" fn _sapp_glx_choosefbconfig() -> GLXFBConfig {
    let mut native_configs = 0 as *mut GLXFBConfig;
    let mut closest = 0 as *const _sapp_gl_fbconfig;
    let mut i: libc::c_int = 0;
    let mut native_count: libc::c_int = 0;
    let mut usable_count: libc::c_int = 0;
    let mut vendor = 0 as *const libc::c_char;
    let mut trust_window_bit = true;
    vendor = _sapp_glx_GetClientString.expect("non-null function pointer")(
        _sapp_x11_display,
        GLX_VENDOR,
    );
    if !vendor.is_null()
        && strcmp(vendor, b"Chromium\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
    {
        trust_window_bit = false
    }
    native_configs = _sapp_glx_GetFBConfigs.expect("non-null function pointer")(
        _sapp_x11_display,
        _sapp_x11_screen,
        &mut native_count,
    );
    if native_configs.is_null() || native_count == 0 {
        _sapp_fail(b"GLX: No GLXFBConfigs returned\x00" as *const u8 as *const libc::c_char);
    }
    let mut usable_configs: Vec<_sapp_gl_fbconfig> = Vec::new();
    usable_count = 0 as libc::c_int;
    let mut current_block_25: u64;
    for i in 0..native_count {
        let n = *native_configs.offset(i as isize);
        let mut u = _sapp_gl_init_fbconfig();
        if 0 == _sapp_glx_attrib(n, GLX_RENDER_TYPE) & GLX_RGBA_BIT {
            continue;
        }
        if 0 == _sapp_glx_attrib(n, GLX_DRAWABLE_TYPE) & GLX_WINDOW_BIT {
            if trust_window_bit {
                continue;
            }
        }

        u.red_bits = _sapp_glx_attrib(n, GLX_RED_SIZE);
        u.green_bits = _sapp_glx_attrib(n, GLX_GREEN_SIZE);
        u.blue_bits = _sapp_glx_attrib(n, GLX_BLUE_SIZE);
        u.alpha_bits = _sapp_glx_attrib(n, GLX_ALPHA_SIZE);
        u.depth_bits = _sapp_glx_attrib(n, GLX_DEPTH_SIZE);
        u.stencil_bits = _sapp_glx_attrib(n, GLX_STENCIL_SIZE);
        if _sapp_glx_attrib(n, GLX_DOUBLEBUFFER) != 0 {
            u.doublebuffer = true
        }
        if _sapp_glx_ARB_multisample {
            u.samples = _sapp_glx_attrib(n, GLX_SAMPLES)
        }
        u.handle = n as libc::c_ulong;
        usable_configs.push(u);
        usable_count += 1
    }

    let mut desired = _sapp_gl_init_fbconfig();
    desired.red_bits = 8;
    desired.green_bits = 8;
    desired.blue_bits = 8;
    desired.alpha_bits = 8;
    desired.depth_bits = 24;
    desired.stencil_bits = 8;
    desired.doublebuffer = true;
    desired.samples = if _sapp.sample_count > 1 {
        _sapp.sample_count
    } else {
        0
    };
    closest = _sapp_gl_choose_fbconfig(
        &mut desired,
        usable_configs.as_mut_ptr(),
        usable_count as libc::c_uint,
    );
    let mut result = 0 as GLXFBConfig;
    if !closest.is_null() {
        result = (*closest).handle as GLXFBConfig
    }
    XFree(native_configs as *mut libc::c_void);
    return result;
}
pub unsafe extern "C" fn _sapp_fail(mut msg: *const libc::c_char) {
    if _sapp.desc.fail_cb.is_some() {
        _sapp.desc.fail_cb.expect("non-null function pointer")(msg);
    } else if _sapp.desc.fail_userdata_cb.is_some() {
        _sapp
            .desc
            .fail_userdata_cb
            .expect("non-null function pointer")(msg, _sapp.desc.user_data);
    } else {
        if msg.is_null() {
            println!("_sapp_fail with empty message");
        } else {
            use std::ffi::CString;

            let rust_msg = CString::from_raw(msg as *mut _);

            println!("{}", rust_msg.to_str().unwrap());
        }
    }
    std::process::exit(0);
}
pub unsafe extern "C" fn _sapp_glx_create_context() {
    let mut native = _sapp_glx_choosefbconfig();
    if native.is_null() {
        _sapp_fail(
            b"GLX: Failed to find a suitable GLXFBConfig (2)\x00" as *const u8
                as *const libc::c_char,
        );
    }
    if !(_sapp_glx_ARB_create_context && _sapp_glx_ARB_create_context_profile) {
        _sapp_fail(
            b"GLX: ARB_create_context and ARB_create_context_profile required\x00" as *const u8
                as *const libc::c_char,
        );
    }
    _sapp_x11_grab_error_handler();
    let attribs: [libc::c_int; 10] = [
        GLX_CONTEXT_MAJOR_VERSION_ARB,
        3,
        GLX_CONTEXT_MINOR_VERSION_ARB,
        3,
        GLX_CONTEXT_PROFILE_MASK_ARB,
        GLX_CONTEXT_CORE_PROFILE_BIT_ARB,
        GLX_CONTEXT_FLAGS_ARB,
        GLX_CONTEXT_FORWARD_COMPATIBLE_BIT_ARB,
        0,
        0,
    ];
    _sapp_glx_ctx = _sapp_glx_CreateContextAttribsARB.expect("non-null function pointer")(
        _sapp_x11_display,
        native,
        std::ptr::null_mut(),
        true as _,
        attribs.as_ptr(),
    );
    if _sapp_glx_ctx.is_null() {
        _sapp_fail(b"GLX: failed to create GL context\x00" as *const u8 as *const libc::c_char);
    }
    _sapp_x11_release_error_handler();
    _sapp_glx_window = _sapp_glx_CreateWindow.expect("non-null function pointer")(
        _sapp_x11_display,
        native,
        _sapp_x11_window,
        std::ptr::null(),
    );
    if _sapp_glx_window == 0 {
        _sapp_fail(b"GLX: failed to create window\x00" as *const u8 as *const libc::c_char);
    };
}
pub unsafe extern "C" fn _sapp_x11_window_visible() -> bool {
    let mut wa = XWindowAttributes {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
        border_width: 0,
        depth: 0,
        visual: 0 as *mut Visual,
        root: 0,
        class: 0,
        bit_gravity: 0,
        win_gravity: 0,
        backing_store: 0,
        backing_planes: 0,
        backing_pixel: 0,
        save_under: 0,
        colormap: 0,
        map_installed: 0,
        map_state: 0,
        all_event_masks: 0,
        your_event_mask: 0,
        do_not_propagate_mask: 0,
        override_redirect: 0,
        screen: 0 as *mut Screen,
    };
    XGetWindowAttributes(_sapp_x11_display, _sapp_x11_window, &mut wa);
    return wa.map_state == IsViewable;
}
pub unsafe extern "C" fn _sapp_x11_show_window() {
    if !_sapp_x11_window_visible() {
        XMapWindow(_sapp_x11_display, _sapp_x11_window);
        XRaiseWindow(_sapp_x11_display, _sapp_x11_window);
        XFlush(_sapp_x11_display);
    };
}

unsafe fn _sapp_x11_set_fullscreen() {
    let mut wm_state = XInternAtom(
        _sapp_x11_display,
        b"_NET_WM_STATE\x00" as *const u8 as *const libc::c_char,
        false as _,
    );
    let wm_fullscreen = XInternAtom(
        _sapp_x11_display,
        b"_NET_WM_STATE_FULLSCREEN\x00" as *const u8 as *const libc::c_char,
        false as _,
    );

    // this is the first method to make window fullscreen
    // hide it, change _NET_WM_STATE_FULLSCREEN property and than show it back
    // someone on stackoverflow mentioned that this is not working on ubuntu/unity though
    {
        XLowerWindow(_sapp_x11_display, _sapp_x11_window);
        XUnmapWindow(_sapp_x11_display, _sapp_x11_window);
        XSync(_sapp_x11_display, false as _);

        let mut atoms: [Atom; 2] = [wm_fullscreen, 0 as _];
        XChangeProperty(
            _sapp_x11_display,
            _sapp_x11_window,
            wm_state,
            4 as _,
            32,
            PropModeReplace,
            atoms.as_mut_ptr() as *mut _ as *mut _,
            1,
        );
        XMapWindow(_sapp_x11_display, _sapp_x11_window);
        XRaiseWindow(_sapp_x11_display, _sapp_x11_window);
        XFlush(_sapp_x11_display);
    }

    // however, this is X, so just in case - the second method
    // send ClientMessage to the window with request to change property to fullscreen
    {
        let mut data = [0isize; 5];

        data[0] = 1;
        data[1] = wm_fullscreen as isize;
        data[2] = 0;

        let mut ev = XClientMessageEvent {
            type_0: 33,
            serial: 0,
            send_event: true as _,
            message_type: wm_state,
            window: _sapp_x11_window,
            display: _sapp_x11_display,
            format: 32,
            data: ClientMessageData { l: std::mem::transmute(data) },
        };
        XSendEvent(
            _sapp_x11_display,
            _sapp_x11_root,
            false as _,
            (1048576 | 131072) as _,
            &mut ev as *mut XClientMessageEvent as *mut XEvent,
        );
    }
}

pub static mut _sapp_glx_EXT_swap_control: bool = false;
pub static mut _sapp_glx_SwapIntervalEXT: PFNGLXSWAPINTERVALEXTPROC = None;
pub static mut _sapp_glx_MESA_swap_control: bool = false;
pub static mut _sapp_glx_SwapIntervalMESA: PFNGLXSWAPINTERVALMESAPROC = None;
pub unsafe extern "C" fn _sapp_glx_swapinterval(mut interval: libc::c_int) {
    _sapp_glx_make_current();
    if _sapp_glx_EXT_swap_control {
        _sapp_glx_SwapIntervalEXT.expect("non-null function pointer")(
            _sapp_x11_display,
            _sapp_glx_window,
            interval,
        );
    } else if _sapp_glx_MESA_swap_control {
        _sapp_glx_SwapIntervalMESA.expect("non-null function pointer")(interval);
    };
}
pub static mut _sapp_glx_MakeCurrent: PFNGLXMAKECURRENTPROC = None;
pub unsafe extern "C" fn _sapp_glx_make_current() {
    _sapp_glx_MakeCurrent.expect("non-null function pointer")(
        _sapp_x11_display,
        _sapp_glx_window,
        _sapp_glx_ctx,
    );
}
pub unsafe extern "C" fn _sapp_x11_char_event(mut chr: u32, mut repeat: bool, mut mods: u32) {
    if _sapp_events_enabled() {
        _sapp_init_event(sapp_event_type_SAPP_EVENTTYPE_CHAR);
        _sapp.event.char_code = chr;
        _sapp.event.key_repeat = repeat;
        _sapp.event.modifiers = mods;
        _sapp_call_event(&mut _sapp.event);
    };
}

pub unsafe extern "C" fn _sapp_x11_raw_device_event(dx: f32, dy: f32) {
    if _sapp_events_enabled() {
        _sapp_init_event(sapp_event_type_SAPP_EVENTTYPE_RAW_DEVICE);
        _sapp.event.mouse_dx = dx;
        _sapp.event.mouse_dy = dy;
        _sapp_call_event(&mut _sapp.event);
    };
}

pub static mut _sapp_x11_keysymtab: [_sapp_x11_codepair; 828] = [
    _sapp_x11_codepair::new(0x01a1, 0x0104),
    _sapp_x11_codepair::new(0x01a2, 0x02d8),
    _sapp_x11_codepair::new(0x01a3, 0x0141),
    _sapp_x11_codepair::new(0x01a5, 0x013d),
    _sapp_x11_codepair::new(0x01a6, 0x015a),
    _sapp_x11_codepair::new(0x01a9, 0x0160),
    _sapp_x11_codepair::new(0x01aa, 0x015e),
    _sapp_x11_codepair::new(0x01ab, 0x0164),
    _sapp_x11_codepair::new(0x01ac, 0x0179),
    _sapp_x11_codepair::new(0x01ae, 0x017d),
    _sapp_x11_codepair::new(0x01af, 0x017b),
    _sapp_x11_codepair::new(0x01b1, 0x0105),
    _sapp_x11_codepair::new(0x01b2, 0x02db),
    _sapp_x11_codepair::new(0x01b3, 0x0142),
    _sapp_x11_codepair::new(0x01b5, 0x013e),
    _sapp_x11_codepair::new(0x01b6, 0x015b),
    _sapp_x11_codepair::new(0x01b7, 0x02c7),
    _sapp_x11_codepair::new(0x01b9, 0x0161),
    _sapp_x11_codepair::new(0x01ba, 0x015f),
    _sapp_x11_codepair::new(0x01bb, 0x0165),
    _sapp_x11_codepair::new(0x01bc, 0x017a),
    _sapp_x11_codepair::new(0x01bd, 0x02dd),
    _sapp_x11_codepair::new(0x01be, 0x017e),
    _sapp_x11_codepair::new(0x01bf, 0x017c),
    _sapp_x11_codepair::new(0x01c0, 0x0154),
    _sapp_x11_codepair::new(0x01c3, 0x0102),
    _sapp_x11_codepair::new(0x01c5, 0x0139),
    _sapp_x11_codepair::new(0x01c6, 0x0106),
    _sapp_x11_codepair::new(0x01c8, 0x010c),
    _sapp_x11_codepair::new(0x01ca, 0x0118),
    _sapp_x11_codepair::new(0x01cc, 0x011a),
    _sapp_x11_codepair::new(0x01cf, 0x010e),
    _sapp_x11_codepair::new(0x01d0, 0x0110),
    _sapp_x11_codepair::new(0x01d1, 0x0143),
    _sapp_x11_codepair::new(0x01d2, 0x0147),
    _sapp_x11_codepair::new(0x01d5, 0x0150),
    _sapp_x11_codepair::new(0x01d8, 0x0158),
    _sapp_x11_codepair::new(0x01d9, 0x016e),
    _sapp_x11_codepair::new(0x01db, 0x0170),
    _sapp_x11_codepair::new(0x01de, 0x0162),
    _sapp_x11_codepair::new(0x01e0, 0x0155),
    _sapp_x11_codepair::new(0x01e3, 0x0103),
    _sapp_x11_codepair::new(0x01e5, 0x013a),
    _sapp_x11_codepair::new(0x01e6, 0x0107),
    _sapp_x11_codepair::new(0x01e8, 0x010d),
    _sapp_x11_codepair::new(0x01ea, 0x0119),
    _sapp_x11_codepair::new(0x01ec, 0x011b),
    _sapp_x11_codepair::new(0x01ef, 0x010f),
    _sapp_x11_codepair::new(0x01f0, 0x0111),
    _sapp_x11_codepair::new(0x01f1, 0x0144),
    _sapp_x11_codepair::new(0x01f2, 0x0148),
    _sapp_x11_codepair::new(0x01f5, 0x0151),
    _sapp_x11_codepair::new(0x01f8, 0x0159),
    _sapp_x11_codepair::new(0x01f9, 0x016f),
    _sapp_x11_codepair::new(0x01fb, 0x0171),
    _sapp_x11_codepair::new(0x01fe, 0x0163),
    _sapp_x11_codepair::new(0x01ff, 0x02d9),
    _sapp_x11_codepair::new(0x02a1, 0x0126),
    _sapp_x11_codepair::new(0x02a6, 0x0124),
    _sapp_x11_codepair::new(0x02a9, 0x0130),
    _sapp_x11_codepair::new(0x02ab, 0x011e),
    _sapp_x11_codepair::new(0x02ac, 0x0134),
    _sapp_x11_codepair::new(0x02b1, 0x0127),
    _sapp_x11_codepair::new(0x02b6, 0x0125),
    _sapp_x11_codepair::new(0x02b9, 0x0131),
    _sapp_x11_codepair::new(0x02bb, 0x011f),
    _sapp_x11_codepair::new(0x02bc, 0x0135),
    _sapp_x11_codepair::new(0x02c5, 0x010a),
    _sapp_x11_codepair::new(0x02c6, 0x0108),
    _sapp_x11_codepair::new(0x02d5, 0x0120),
    _sapp_x11_codepair::new(0x02d8, 0x011c),
    _sapp_x11_codepair::new(0x02dd, 0x016c),
    _sapp_x11_codepair::new(0x02de, 0x015c),
    _sapp_x11_codepair::new(0x02e5, 0x010b),
    _sapp_x11_codepair::new(0x02e6, 0x0109),
    _sapp_x11_codepair::new(0x02f5, 0x0121),
    _sapp_x11_codepair::new(0x02f8, 0x011d),
    _sapp_x11_codepair::new(0x02fd, 0x016d),
    _sapp_x11_codepair::new(0x02fe, 0x015d),
    _sapp_x11_codepair::new(0x03a2, 0x0138),
    _sapp_x11_codepair::new(0x03a3, 0x0156),
    _sapp_x11_codepair::new(0x03a5, 0x0128),
    _sapp_x11_codepair::new(0x03a6, 0x013b),
    _sapp_x11_codepair::new(0x03aa, 0x0112),
    _sapp_x11_codepair::new(0x03ab, 0x0122),
    _sapp_x11_codepair::new(0x03ac, 0x0166),
    _sapp_x11_codepair::new(0x03b3, 0x0157),
    _sapp_x11_codepair::new(0x03b5, 0x0129),
    _sapp_x11_codepair::new(0x03b6, 0x013c),
    _sapp_x11_codepair::new(0x03ba, 0x0113),
    _sapp_x11_codepair::new(0x03bb, 0x0123),
    _sapp_x11_codepair::new(0x03bc, 0x0167),
    _sapp_x11_codepair::new(0x03bd, 0x014a),
    _sapp_x11_codepair::new(0x03bf, 0x014b),
    _sapp_x11_codepair::new(0x03c0, 0x0100),
    _sapp_x11_codepair::new(0x03c7, 0x012e),
    _sapp_x11_codepair::new(0x03cc, 0x0116),
    _sapp_x11_codepair::new(0x03cf, 0x012a),
    _sapp_x11_codepair::new(0x03d1, 0x0145),
    _sapp_x11_codepair::new(0x03d2, 0x014c),
    _sapp_x11_codepair::new(0x03d3, 0x0136),
    _sapp_x11_codepair::new(0x03d9, 0x0172),
    _sapp_x11_codepair::new(0x03dd, 0x0168),
    _sapp_x11_codepair::new(0x03de, 0x016a),
    _sapp_x11_codepair::new(0x03e0, 0x0101),
    _sapp_x11_codepair::new(0x03e7, 0x012f),
    _sapp_x11_codepair::new(0x03ec, 0x0117),
    _sapp_x11_codepair::new(0x03ef, 0x012b),
    _sapp_x11_codepair::new(0x03f1, 0x0146),
    _sapp_x11_codepair::new(0x03f2, 0x014d),
    _sapp_x11_codepair::new(0x03f3, 0x0137),
    _sapp_x11_codepair::new(0x03f9, 0x0173),
    _sapp_x11_codepair::new(0x03fd, 0x0169),
    _sapp_x11_codepair::new(0x03fe, 0x016b),
    _sapp_x11_codepair::new(0x047e, 0x203e),
    _sapp_x11_codepair::new(0x04a1, 0x3002),
    _sapp_x11_codepair::new(0x04a2, 0x300c),
    _sapp_x11_codepair::new(0x04a3, 0x300d),
    _sapp_x11_codepair::new(0x04a4, 0x3001),
    _sapp_x11_codepair::new(0x04a5, 0x30fb),
    _sapp_x11_codepair::new(0x04a6, 0x30f2),
    _sapp_x11_codepair::new(0x04a7, 0x30a1),
    _sapp_x11_codepair::new(0x04a8, 0x30a3),
    _sapp_x11_codepair::new(0x04a9, 0x30a5),
    _sapp_x11_codepair::new(0x04aa, 0x30a7),
    _sapp_x11_codepair::new(0x04ab, 0x30a9),
    _sapp_x11_codepair::new(0x04ac, 0x30e3),
    _sapp_x11_codepair::new(0x04ad, 0x30e5),
    _sapp_x11_codepair::new(0x04ae, 0x30e7),
    _sapp_x11_codepair::new(0x04af, 0x30c3),
    _sapp_x11_codepair::new(0x04b0, 0x30fc),
    _sapp_x11_codepair::new(0x04b1, 0x30a2),
    _sapp_x11_codepair::new(0x04b2, 0x30a4),
    _sapp_x11_codepair::new(0x04b3, 0x30a6),
    _sapp_x11_codepair::new(0x04b4, 0x30a8),
    _sapp_x11_codepair::new(0x04b5, 0x30aa),
    _sapp_x11_codepair::new(0x04b6, 0x30ab),
    _sapp_x11_codepair::new(0x04b7, 0x30ad),
    _sapp_x11_codepair::new(0x04b8, 0x30af),
    _sapp_x11_codepair::new(0x04b9, 0x30b1),
    _sapp_x11_codepair::new(0x04ba, 0x30b3),
    _sapp_x11_codepair::new(0x04bb, 0x30b5),
    _sapp_x11_codepair::new(0x04bc, 0x30b7),
    _sapp_x11_codepair::new(0x04bd, 0x30b9),
    _sapp_x11_codepair::new(0x04be, 0x30bb),
    _sapp_x11_codepair::new(0x04bf, 0x30bd),
    _sapp_x11_codepair::new(0x04c0, 0x30bf),
    _sapp_x11_codepair::new(0x04c1, 0x30c1),
    _sapp_x11_codepair::new(0x04c2, 0x30c4),
    _sapp_x11_codepair::new(0x04c3, 0x30c6),
    _sapp_x11_codepair::new(0x04c4, 0x30c8),
    _sapp_x11_codepair::new(0x04c5, 0x30ca),
    _sapp_x11_codepair::new(0x04c6, 0x30cb),
    _sapp_x11_codepair::new(0x04c7, 0x30cc),
    _sapp_x11_codepair::new(0x04c8, 0x30cd),
    _sapp_x11_codepair::new(0x04c9, 0x30ce),
    _sapp_x11_codepair::new(0x04ca, 0x30cf),
    _sapp_x11_codepair::new(0x04cb, 0x30d2),
    _sapp_x11_codepair::new(0x04cc, 0x30d5),
    _sapp_x11_codepair::new(0x04cd, 0x30d8),
    _sapp_x11_codepair::new(0x04ce, 0x30db),
    _sapp_x11_codepair::new(0x04cf, 0x30de),
    _sapp_x11_codepair::new(0x04d0, 0x30df),
    _sapp_x11_codepair::new(0x04d1, 0x30e0),
    _sapp_x11_codepair::new(0x04d2, 0x30e1),
    _sapp_x11_codepair::new(0x04d3, 0x30e2),
    _sapp_x11_codepair::new(0x04d4, 0x30e4),
    _sapp_x11_codepair::new(0x04d5, 0x30e6),
    _sapp_x11_codepair::new(0x04d6, 0x30e8),
    _sapp_x11_codepair::new(0x04d7, 0x30e9),
    _sapp_x11_codepair::new(0x04d8, 0x30ea),
    _sapp_x11_codepair::new(0x04d9, 0x30eb),
    _sapp_x11_codepair::new(0x04da, 0x30ec),
    _sapp_x11_codepair::new(0x04db, 0x30ed),
    _sapp_x11_codepair::new(0x04dc, 0x30ef),
    _sapp_x11_codepair::new(0x04dd, 0x30f3),
    _sapp_x11_codepair::new(0x04de, 0x309b),
    _sapp_x11_codepair::new(0x04df, 0x309c),
    _sapp_x11_codepair::new(0x05ac, 0x060c),
    _sapp_x11_codepair::new(0x05bb, 0x061b),
    _sapp_x11_codepair::new(0x05bf, 0x061f),
    _sapp_x11_codepair::new(0x05c1, 0x0621),
    _sapp_x11_codepair::new(0x05c2, 0x0622),
    _sapp_x11_codepair::new(0x05c3, 0x0623),
    _sapp_x11_codepair::new(0x05c4, 0x0624),
    _sapp_x11_codepair::new(0x05c5, 0x0625),
    _sapp_x11_codepair::new(0x05c6, 0x0626),
    _sapp_x11_codepair::new(0x05c7, 0x0627),
    _sapp_x11_codepair::new(0x05c8, 0x0628),
    _sapp_x11_codepair::new(0x05c9, 0x0629),
    _sapp_x11_codepair::new(0x05ca, 0x062a),
    _sapp_x11_codepair::new(0x05cb, 0x062b),
    _sapp_x11_codepair::new(0x05cc, 0x062c),
    _sapp_x11_codepair::new(0x05cd, 0x062d),
    _sapp_x11_codepair::new(0x05ce, 0x062e),
    _sapp_x11_codepair::new(0x05cf, 0x062f),
    _sapp_x11_codepair::new(0x05d0, 0x0630),
    _sapp_x11_codepair::new(0x05d1, 0x0631),
    _sapp_x11_codepair::new(0x05d2, 0x0632),
    _sapp_x11_codepair::new(0x05d3, 0x0633),
    _sapp_x11_codepair::new(0x05d4, 0x0634),
    _sapp_x11_codepair::new(0x05d5, 0x0635),
    _sapp_x11_codepair::new(0x05d6, 0x0636),
    _sapp_x11_codepair::new(0x05d7, 0x0637),
    _sapp_x11_codepair::new(0x05d8, 0x0638),
    _sapp_x11_codepair::new(0x05d9, 0x0639),
    _sapp_x11_codepair::new(0x05da, 0x063a),
    _sapp_x11_codepair::new(0x05e0, 0x0640),
    _sapp_x11_codepair::new(0x05e1, 0x0641),
    _sapp_x11_codepair::new(0x05e2, 0x0642),
    _sapp_x11_codepair::new(0x05e3, 0x0643),
    _sapp_x11_codepair::new(0x05e4, 0x0644),
    _sapp_x11_codepair::new(0x05e5, 0x0645),
    _sapp_x11_codepair::new(0x05e6, 0x0646),
    _sapp_x11_codepair::new(0x05e7, 0x0647),
    _sapp_x11_codepair::new(0x05e8, 0x0648),
    _sapp_x11_codepair::new(0x05e9, 0x0649),
    _sapp_x11_codepair::new(0x05ea, 0x064a),
    _sapp_x11_codepair::new(0x05eb, 0x064b),
    _sapp_x11_codepair::new(0x05ec, 0x064c),
    _sapp_x11_codepair::new(0x05ed, 0x064d),
    _sapp_x11_codepair::new(0x05ee, 0x064e),
    _sapp_x11_codepair::new(0x05ef, 0x064f),
    _sapp_x11_codepair::new(0x05f0, 0x0650),
    _sapp_x11_codepair::new(0x05f1, 0x0651),
    _sapp_x11_codepair::new(0x05f2, 0x0652),
    _sapp_x11_codepair::new(0x06a1, 0x0452),
    _sapp_x11_codepair::new(0x06a2, 0x0453),
    _sapp_x11_codepair::new(0x06a3, 0x0451),
    _sapp_x11_codepair::new(0x06a4, 0x0454),
    _sapp_x11_codepair::new(0x06a5, 0x0455),
    _sapp_x11_codepair::new(0x06a6, 0x0456),
    _sapp_x11_codepair::new(0x06a7, 0x0457),
    _sapp_x11_codepair::new(0x06a8, 0x0458),
    _sapp_x11_codepair::new(0x06a9, 0x0459),
    _sapp_x11_codepair::new(0x06aa, 0x045a),
    _sapp_x11_codepair::new(0x06ab, 0x045b),
    _sapp_x11_codepair::new(0x06ac, 0x045c),
    _sapp_x11_codepair::new(0x06ae, 0x045e),
    _sapp_x11_codepair::new(0x06af, 0x045f),
    _sapp_x11_codepair::new(0x06b0, 0x2116),
    _sapp_x11_codepair::new(0x06b1, 0x0402),
    _sapp_x11_codepair::new(0x06b2, 0x0403),
    _sapp_x11_codepair::new(0x06b3, 0x0401),
    _sapp_x11_codepair::new(0x06b4, 0x0404),
    _sapp_x11_codepair::new(0x06b5, 0x0405),
    _sapp_x11_codepair::new(0x06b6, 0x0406),
    _sapp_x11_codepair::new(0x06b7, 0x0407),
    _sapp_x11_codepair::new(0x06b8, 0x0408),
    _sapp_x11_codepair::new(0x06b9, 0x0409),
    _sapp_x11_codepair::new(0x06ba, 0x040a),
    _sapp_x11_codepair::new(0x06bb, 0x040b),
    _sapp_x11_codepair::new(0x06bc, 0x040c),
    _sapp_x11_codepair::new(0x06be, 0x040e),
    _sapp_x11_codepair::new(0x06bf, 0x040f),
    _sapp_x11_codepair::new(0x06c0, 0x044e),
    _sapp_x11_codepair::new(0x06c1, 0x0430),
    _sapp_x11_codepair::new(0x06c2, 0x0431),
    _sapp_x11_codepair::new(0x06c3, 0x0446),
    _sapp_x11_codepair::new(0x06c4, 0x0434),
    _sapp_x11_codepair::new(0x06c5, 0x0435),
    _sapp_x11_codepair::new(0x06c6, 0x0444),
    _sapp_x11_codepair::new(0x06c7, 0x0433),
    _sapp_x11_codepair::new(0x06c8, 0x0445),
    _sapp_x11_codepair::new(0x06c9, 0x0438),
    _sapp_x11_codepair::new(0x06ca, 0x0439),
    _sapp_x11_codepair::new(0x06cb, 0x043a),
    _sapp_x11_codepair::new(0x06cc, 0x043b),
    _sapp_x11_codepair::new(0x06cd, 0x043c),
    _sapp_x11_codepair::new(0x06ce, 0x043d),
    _sapp_x11_codepair::new(0x06cf, 0x043e),
    _sapp_x11_codepair::new(0x06d0, 0x043f),
    _sapp_x11_codepair::new(0x06d1, 0x044f),
    _sapp_x11_codepair::new(0x06d2, 0x0440),
    _sapp_x11_codepair::new(0x06d3, 0x0441),
    _sapp_x11_codepair::new(0x06d4, 0x0442),
    _sapp_x11_codepair::new(0x06d5, 0x0443),
    _sapp_x11_codepair::new(0x06d6, 0x0436),
    _sapp_x11_codepair::new(0x06d7, 0x0432),
    _sapp_x11_codepair::new(0x06d8, 0x044c),
    _sapp_x11_codepair::new(0x06d9, 0x044b),
    _sapp_x11_codepair::new(0x06da, 0x0437),
    _sapp_x11_codepair::new(0x06db, 0x0448),
    _sapp_x11_codepair::new(0x06dc, 0x044d),
    _sapp_x11_codepair::new(0x06dd, 0x0449),
    _sapp_x11_codepair::new(0x06de, 0x0447),
    _sapp_x11_codepair::new(0x06df, 0x044a),
    _sapp_x11_codepair::new(0x06e0, 0x042e),
    _sapp_x11_codepair::new(0x06e1, 0x0410),
    _sapp_x11_codepair::new(0x06e2, 0x0411),
    _sapp_x11_codepair::new(0x06e3, 0x0426),
    _sapp_x11_codepair::new(0x06e4, 0x0414),
    _sapp_x11_codepair::new(0x06e5, 0x0415),
    _sapp_x11_codepair::new(0x06e6, 0x0424),
    _sapp_x11_codepair::new(0x06e7, 0x0413),
    _sapp_x11_codepair::new(0x06e8, 0x0425),
    _sapp_x11_codepair::new(0x06e9, 0x0418),
    _sapp_x11_codepair::new(0x06ea, 0x0419),
    _sapp_x11_codepair::new(0x06eb, 0x041a),
    _sapp_x11_codepair::new(0x06ec, 0x041b),
    _sapp_x11_codepair::new(0x06ed, 0x041c),
    _sapp_x11_codepair::new(0x06ee, 0x041d),
    _sapp_x11_codepair::new(0x06ef, 0x041e),
    _sapp_x11_codepair::new(0x06f0, 0x041f),
    _sapp_x11_codepair::new(0x06f1, 0x042f),
    _sapp_x11_codepair::new(0x06f2, 0x0420),
    _sapp_x11_codepair::new(0x06f3, 0x0421),
    _sapp_x11_codepair::new(0x06f4, 0x0422),
    _sapp_x11_codepair::new(0x06f5, 0x0423),
    _sapp_x11_codepair::new(0x06f6, 0x0416),
    _sapp_x11_codepair::new(0x06f7, 0x0412),
    _sapp_x11_codepair::new(0x06f8, 0x042c),
    _sapp_x11_codepair::new(0x06f9, 0x042b),
    _sapp_x11_codepair::new(0x06fa, 0x0417),
    _sapp_x11_codepair::new(0x06fb, 0x0428),
    _sapp_x11_codepair::new(0x06fc, 0x042d),
    _sapp_x11_codepair::new(0x06fd, 0x0429),
    _sapp_x11_codepair::new(0x06fe, 0x0427),
    _sapp_x11_codepair::new(0x06ff, 0x042a),
    _sapp_x11_codepair::new(0x07a1, 0x0386),
    _sapp_x11_codepair::new(0x07a2, 0x0388),
    _sapp_x11_codepair::new(0x07a3, 0x0389),
    _sapp_x11_codepair::new(0x07a4, 0x038a),
    _sapp_x11_codepair::new(0x07a5, 0x03aa),
    _sapp_x11_codepair::new(0x07a7, 0x038c),
    _sapp_x11_codepair::new(0x07a8, 0x038e),
    _sapp_x11_codepair::new(0x07a9, 0x03ab),
    _sapp_x11_codepair::new(0x07ab, 0x038f),
    _sapp_x11_codepair::new(0x07ae, 0x0385),
    _sapp_x11_codepair::new(0x07af, 0x2015),
    _sapp_x11_codepair::new(0x07b1, 0x03ac),
    _sapp_x11_codepair::new(0x07b2, 0x03ad),
    _sapp_x11_codepair::new(0x07b3, 0x03ae),
    _sapp_x11_codepair::new(0x07b4, 0x03af),
    _sapp_x11_codepair::new(0x07b5, 0x03ca),
    _sapp_x11_codepair::new(0x07b6, 0x0390),
    _sapp_x11_codepair::new(0x07b7, 0x03cc),
    _sapp_x11_codepair::new(0x07b8, 0x03cd),
    _sapp_x11_codepair::new(0x07b9, 0x03cb),
    _sapp_x11_codepair::new(0x07ba, 0x03b0),
    _sapp_x11_codepair::new(0x07bb, 0x03ce),
    _sapp_x11_codepair::new(0x07c1, 0x0391),
    _sapp_x11_codepair::new(0x07c2, 0x0392),
    _sapp_x11_codepair::new(0x07c3, 0x0393),
    _sapp_x11_codepair::new(0x07c4, 0x0394),
    _sapp_x11_codepair::new(0x07c5, 0x0395),
    _sapp_x11_codepair::new(0x07c6, 0x0396),
    _sapp_x11_codepair::new(0x07c7, 0x0397),
    _sapp_x11_codepair::new(0x07c8, 0x0398),
    _sapp_x11_codepair::new(0x07c9, 0x0399),
    _sapp_x11_codepair::new(0x07ca, 0x039a),
    _sapp_x11_codepair::new(0x07cb, 0x039b),
    _sapp_x11_codepair::new(0x07cc, 0x039c),
    _sapp_x11_codepair::new(0x07cd, 0x039d),
    _sapp_x11_codepair::new(0x07ce, 0x039e),
    _sapp_x11_codepair::new(0x07cf, 0x039f),
    _sapp_x11_codepair::new(0x07d0, 0x03a0),
    _sapp_x11_codepair::new(0x07d1, 0x03a1),
    _sapp_x11_codepair::new(0x07d2, 0x03a3),
    _sapp_x11_codepair::new(0x07d4, 0x03a4),
    _sapp_x11_codepair::new(0x07d5, 0x03a5),
    _sapp_x11_codepair::new(0x07d6, 0x03a6),
    _sapp_x11_codepair::new(0x07d7, 0x03a7),
    _sapp_x11_codepair::new(0x07d8, 0x03a8),
    _sapp_x11_codepair::new(0x07d9, 0x03a9),
    _sapp_x11_codepair::new(0x07e1, 0x03b1),
    _sapp_x11_codepair::new(0x07e2, 0x03b2),
    _sapp_x11_codepair::new(0x07e3, 0x03b3),
    _sapp_x11_codepair::new(0x07e4, 0x03b4),
    _sapp_x11_codepair::new(0x07e5, 0x03b5),
    _sapp_x11_codepair::new(0x07e6, 0x03b6),
    _sapp_x11_codepair::new(0x07e7, 0x03b7),
    _sapp_x11_codepair::new(0x07e8, 0x03b8),
    _sapp_x11_codepair::new(0x07e9, 0x03b9),
    _sapp_x11_codepair::new(0x07ea, 0x03ba),
    _sapp_x11_codepair::new(0x07eb, 0x03bb),
    _sapp_x11_codepair::new(0x07ec, 0x03bc),
    _sapp_x11_codepair::new(0x07ed, 0x03bd),
    _sapp_x11_codepair::new(0x07ee, 0x03be),
    _sapp_x11_codepair::new(0x07ef, 0x03bf),
    _sapp_x11_codepair::new(0x07f0, 0x03c0),
    _sapp_x11_codepair::new(0x07f1, 0x03c1),
    _sapp_x11_codepair::new(0x07f2, 0x03c3),
    _sapp_x11_codepair::new(0x07f3, 0x03c2),
    _sapp_x11_codepair::new(0x07f4, 0x03c4),
    _sapp_x11_codepair::new(0x07f5, 0x03c5),
    _sapp_x11_codepair::new(0x07f6, 0x03c6),
    _sapp_x11_codepair::new(0x07f7, 0x03c7),
    _sapp_x11_codepair::new(0x07f8, 0x03c8),
    _sapp_x11_codepair::new(0x07f9, 0x03c9),
    _sapp_x11_codepair::new(0x08a1, 0x23b7),
    _sapp_x11_codepair::new(0x08a2, 0x250c),
    _sapp_x11_codepair::new(0x08a3, 0x2500),
    _sapp_x11_codepair::new(0x08a4, 0x2320),
    _sapp_x11_codepair::new(0x08a5, 0x2321),
    _sapp_x11_codepair::new(0x08a6, 0x2502),
    _sapp_x11_codepair::new(0x08a7, 0x23a1),
    _sapp_x11_codepair::new(0x08a8, 0x23a3),
    _sapp_x11_codepair::new(0x08a9, 0x23a4),
    _sapp_x11_codepair::new(0x08aa, 0x23a6),
    _sapp_x11_codepair::new(0x08ab, 0x239b),
    _sapp_x11_codepair::new(0x08ac, 0x239d),
    _sapp_x11_codepair::new(0x08ad, 0x239e),
    _sapp_x11_codepair::new(0x08ae, 0x23a0),
    _sapp_x11_codepair::new(0x08af, 0x23a8),
    _sapp_x11_codepair::new(0x08b0, 0x23ac),
    _sapp_x11_codepair::new(0x08bc, 0x2264),
    _sapp_x11_codepair::new(0x08bd, 0x2260),
    _sapp_x11_codepair::new(0x08be, 0x2265),
    _sapp_x11_codepair::new(0x08bf, 0x222b),
    _sapp_x11_codepair::new(0x08c0, 0x2234),
    _sapp_x11_codepair::new(0x08c1, 0x221d),
    _sapp_x11_codepair::new(0x08c2, 0x221e),
    _sapp_x11_codepair::new(0x08c5, 0x2207),
    _sapp_x11_codepair::new(0x08c8, 0x223c),
    _sapp_x11_codepair::new(0x08c9, 0x2243),
    _sapp_x11_codepair::new(0x08cd, 0x21d4),
    _sapp_x11_codepair::new(0x08ce, 0x21d2),
    _sapp_x11_codepair::new(0x08cf, 0x2261),
    _sapp_x11_codepair::new(0x08d6, 0x221a),
    _sapp_x11_codepair::new(0x08da, 0x2282),
    _sapp_x11_codepair::new(0x08db, 0x2283),
    _sapp_x11_codepair::new(0x08dc, 0x2229),
    _sapp_x11_codepair::new(0x08dd, 0x222a),
    _sapp_x11_codepair::new(0x08de, 0x2227),
    _sapp_x11_codepair::new(0x08df, 0x2228),
    _sapp_x11_codepair::new(0x08ef, 0x2202),
    _sapp_x11_codepair::new(0x08f6, 0x0192),
    _sapp_x11_codepair::new(0x08fb, 0x2190),
    _sapp_x11_codepair::new(0x08fc, 0x2191),
    _sapp_x11_codepair::new(0x08fd, 0x2192),
    _sapp_x11_codepair::new(0x08fe, 0x2193),
    _sapp_x11_codepair::new(0x09e0, 0x25c6),
    _sapp_x11_codepair::new(0x09e1, 0x2592),
    _sapp_x11_codepair::new(0x09e2, 0x2409),
    _sapp_x11_codepair::new(0x09e3, 0x240c),
    _sapp_x11_codepair::new(0x09e4, 0x240d),
    _sapp_x11_codepair::new(0x09e5, 0x240a),
    _sapp_x11_codepair::new(0x09e8, 0x2424),
    _sapp_x11_codepair::new(0x09e9, 0x240b),
    _sapp_x11_codepair::new(0x09ea, 0x2518),
    _sapp_x11_codepair::new(0x09eb, 0x2510),
    _sapp_x11_codepair::new(0x09ec, 0x250c),
    _sapp_x11_codepair::new(0x09ed, 0x2514),
    _sapp_x11_codepair::new(0x09ee, 0x253c),
    _sapp_x11_codepair::new(0x09ef, 0x23ba),
    _sapp_x11_codepair::new(0x09f0, 0x23bb),
    _sapp_x11_codepair::new(0x09f1, 0x2500),
    _sapp_x11_codepair::new(0x09f2, 0x23bc),
    _sapp_x11_codepair::new(0x09f3, 0x23bd),
    _sapp_x11_codepair::new(0x09f4, 0x251c),
    _sapp_x11_codepair::new(0x09f5, 0x2524),
    _sapp_x11_codepair::new(0x09f6, 0x2534),
    _sapp_x11_codepair::new(0x09f7, 0x252c),
    _sapp_x11_codepair::new(0x09f8, 0x2502),
    _sapp_x11_codepair::new(0x0aa1, 0x2003),
    _sapp_x11_codepair::new(0x0aa2, 0x2002),
    _sapp_x11_codepair::new(0x0aa3, 0x2004),
    _sapp_x11_codepair::new(0x0aa4, 0x2005),
    _sapp_x11_codepair::new(0x0aa5, 0x2007),
    _sapp_x11_codepair::new(0x0aa6, 0x2008),
    _sapp_x11_codepair::new(0x0aa7, 0x2009),
    _sapp_x11_codepair::new(0x0aa8, 0x200a),
    _sapp_x11_codepair::new(0x0aa9, 0x2014),
    _sapp_x11_codepair::new(0x0aaa, 0x2013),
    _sapp_x11_codepair::new(0x0aae, 0x2026),
    _sapp_x11_codepair::new(0x0aaf, 0x2025),
    _sapp_x11_codepair::new(0x0ab0, 0x2153),
    _sapp_x11_codepair::new(0x0ab1, 0x2154),
    _sapp_x11_codepair::new(0x0ab2, 0x2155),
    _sapp_x11_codepair::new(0x0ab3, 0x2156),
    _sapp_x11_codepair::new(0x0ab4, 0x2157),
    _sapp_x11_codepair::new(0x0ab5, 0x2158),
    _sapp_x11_codepair::new(0x0ab6, 0x2159),
    _sapp_x11_codepair::new(0x0ab7, 0x215a),
    _sapp_x11_codepair::new(0x0ab8, 0x2105),
    _sapp_x11_codepair::new(0x0abb, 0x2012),
    _sapp_x11_codepair::new(0x0abc, 0x2329),
    _sapp_x11_codepair::new(0x0abe, 0x232a),
    _sapp_x11_codepair::new(0x0ac3, 0x215b),
    _sapp_x11_codepair::new(0x0ac4, 0x215c),
    _sapp_x11_codepair::new(0x0ac5, 0x215d),
    _sapp_x11_codepair::new(0x0ac6, 0x215e),
    _sapp_x11_codepair::new(0x0ac9, 0x2122),
    _sapp_x11_codepair::new(0x0aca, 0x2613),
    _sapp_x11_codepair::new(0x0acc, 0x25c1),
    _sapp_x11_codepair::new(0x0acd, 0x25b7),
    _sapp_x11_codepair::new(0x0ace, 0x25cb),
    _sapp_x11_codepair::new(0x0acf, 0x25af),
    _sapp_x11_codepair::new(0x0ad0, 0x2018),
    _sapp_x11_codepair::new(0x0ad1, 0x2019),
    _sapp_x11_codepair::new(0x0ad2, 0x201c),
    _sapp_x11_codepair::new(0x0ad3, 0x201d),
    _sapp_x11_codepair::new(0x0ad4, 0x211e),
    _sapp_x11_codepair::new(0x0ad6, 0x2032),
    _sapp_x11_codepair::new(0x0ad7, 0x2033),
    _sapp_x11_codepair::new(0x0ad9, 0x271d),
    _sapp_x11_codepair::new(0x0adb, 0x25ac),
    _sapp_x11_codepair::new(0x0adc, 0x25c0),
    _sapp_x11_codepair::new(0x0add, 0x25b6),
    _sapp_x11_codepair::new(0x0ade, 0x25cf),
    _sapp_x11_codepair::new(0x0adf, 0x25ae),
    _sapp_x11_codepair::new(0x0ae0, 0x25e6),
    _sapp_x11_codepair::new(0x0ae1, 0x25ab),
    _sapp_x11_codepair::new(0x0ae2, 0x25ad),
    _sapp_x11_codepair::new(0x0ae3, 0x25b3),
    _sapp_x11_codepair::new(0x0ae4, 0x25bd),
    _sapp_x11_codepair::new(0x0ae5, 0x2606),
    _sapp_x11_codepair::new(0x0ae6, 0x2022),
    _sapp_x11_codepair::new(0x0ae7, 0x25aa),
    _sapp_x11_codepair::new(0x0ae8, 0x25b2),
    _sapp_x11_codepair::new(0x0ae9, 0x25bc),
    _sapp_x11_codepair::new(0x0aea, 0x261c),
    _sapp_x11_codepair::new(0x0aeb, 0x261e),
    _sapp_x11_codepair::new(0x0aec, 0x2663),
    _sapp_x11_codepair::new(0x0aed, 0x2666),
    _sapp_x11_codepair::new(0x0aee, 0x2665),
    _sapp_x11_codepair::new(0x0af0, 0x2720),
    _sapp_x11_codepair::new(0x0af1, 0x2020),
    _sapp_x11_codepair::new(0x0af2, 0x2021),
    _sapp_x11_codepair::new(0x0af3, 0x2713),
    _sapp_x11_codepair::new(0x0af4, 0x2717),
    _sapp_x11_codepair::new(0x0af5, 0x266f),
    _sapp_x11_codepair::new(0x0af6, 0x266d),
    _sapp_x11_codepair::new(0x0af7, 0x2642),
    _sapp_x11_codepair::new(0x0af8, 0x2640),
    _sapp_x11_codepair::new(0x0af9, 0x260e),
    _sapp_x11_codepair::new(0x0afa, 0x2315),
    _sapp_x11_codepair::new(0x0afb, 0x2117),
    _sapp_x11_codepair::new(0x0afc, 0x2038),
    _sapp_x11_codepair::new(0x0afd, 0x201a),
    _sapp_x11_codepair::new(0x0afe, 0x201e),
    _sapp_x11_codepair::new(0x0ba3, 0x003c),
    _sapp_x11_codepair::new(0x0ba6, 0x003e),
    _sapp_x11_codepair::new(0x0ba8, 0x2228),
    _sapp_x11_codepair::new(0x0ba9, 0x2227),
    _sapp_x11_codepair::new(0x0bc0, 0x00af),
    _sapp_x11_codepair::new(0x0bc2, 0x22a5),
    _sapp_x11_codepair::new(0x0bc3, 0x2229),
    _sapp_x11_codepair::new(0x0bc4, 0x230a),
    _sapp_x11_codepair::new(0x0bc6, 0x005f),
    _sapp_x11_codepair::new(0x0bca, 0x2218),
    _sapp_x11_codepair::new(0x0bcc, 0x2395),
    _sapp_x11_codepair::new(0x0bce, 0x22a4),
    _sapp_x11_codepair::new(0x0bcf, 0x25cb),
    _sapp_x11_codepair::new(0x0bd3, 0x2308),
    _sapp_x11_codepair::new(0x0bd6, 0x222a),
    _sapp_x11_codepair::new(0x0bd8, 0x2283),
    _sapp_x11_codepair::new(0x0bda, 0x2282),
    _sapp_x11_codepair::new(0x0bdc, 0x22a2),
    _sapp_x11_codepair::new(0x0bfc, 0x22a3),
    _sapp_x11_codepair::new(0x0cdf, 0x2017),
    _sapp_x11_codepair::new(0x0ce0, 0x05d0),
    _sapp_x11_codepair::new(0x0ce1, 0x05d1),
    _sapp_x11_codepair::new(0x0ce2, 0x05d2),
    _sapp_x11_codepair::new(0x0ce3, 0x05d3),
    _sapp_x11_codepair::new(0x0ce4, 0x05d4),
    _sapp_x11_codepair::new(0x0ce5, 0x05d5),
    _sapp_x11_codepair::new(0x0ce6, 0x05d6),
    _sapp_x11_codepair::new(0x0ce7, 0x05d7),
    _sapp_x11_codepair::new(0x0ce8, 0x05d8),
    _sapp_x11_codepair::new(0x0ce9, 0x05d9),
    _sapp_x11_codepair::new(0x0cea, 0x05da),
    _sapp_x11_codepair::new(0x0ceb, 0x05db),
    _sapp_x11_codepair::new(0x0cec, 0x05dc),
    _sapp_x11_codepair::new(0x0ced, 0x05dd),
    _sapp_x11_codepair::new(0x0cee, 0x05de),
    _sapp_x11_codepair::new(0x0cef, 0x05df),
    _sapp_x11_codepair::new(0x0cf0, 0x05e0),
    _sapp_x11_codepair::new(0x0cf1, 0x05e1),
    _sapp_x11_codepair::new(0x0cf2, 0x05e2),
    _sapp_x11_codepair::new(0x0cf3, 0x05e3),
    _sapp_x11_codepair::new(0x0cf4, 0x05e4),
    _sapp_x11_codepair::new(0x0cf5, 0x05e5),
    _sapp_x11_codepair::new(0x0cf6, 0x05e6),
    _sapp_x11_codepair::new(0x0cf7, 0x05e7),
    _sapp_x11_codepair::new(0x0cf8, 0x05e8),
    _sapp_x11_codepair::new(0x0cf9, 0x05e9),
    _sapp_x11_codepair::new(0x0cfa, 0x05ea),
    _sapp_x11_codepair::new(0x0da1, 0x0e01),
    _sapp_x11_codepair::new(0x0da2, 0x0e02),
    _sapp_x11_codepair::new(0x0da3, 0x0e03),
    _sapp_x11_codepair::new(0x0da4, 0x0e04),
    _sapp_x11_codepair::new(0x0da5, 0x0e05),
    _sapp_x11_codepair::new(0x0da6, 0x0e06),
    _sapp_x11_codepair::new(0x0da7, 0x0e07),
    _sapp_x11_codepair::new(0x0da8, 0x0e08),
    _sapp_x11_codepair::new(0x0da9, 0x0e09),
    _sapp_x11_codepair::new(0x0daa, 0x0e0a),
    _sapp_x11_codepair::new(0x0dab, 0x0e0b),
    _sapp_x11_codepair::new(0x0dac, 0x0e0c),
    _sapp_x11_codepair::new(0x0dad, 0x0e0d),
    _sapp_x11_codepair::new(0x0dae, 0x0e0e),
    _sapp_x11_codepair::new(0x0daf, 0x0e0f),
    _sapp_x11_codepair::new(0x0db0, 0x0e10),
    _sapp_x11_codepair::new(0x0db1, 0x0e11),
    _sapp_x11_codepair::new(0x0db2, 0x0e12),
    _sapp_x11_codepair::new(0x0db3, 0x0e13),
    _sapp_x11_codepair::new(0x0db4, 0x0e14),
    _sapp_x11_codepair::new(0x0db5, 0x0e15),
    _sapp_x11_codepair::new(0x0db6, 0x0e16),
    _sapp_x11_codepair::new(0x0db7, 0x0e17),
    _sapp_x11_codepair::new(0x0db8, 0x0e18),
    _sapp_x11_codepair::new(0x0db9, 0x0e19),
    _sapp_x11_codepair::new(0x0dba, 0x0e1a),
    _sapp_x11_codepair::new(0x0dbb, 0x0e1b),
    _sapp_x11_codepair::new(0x0dbc, 0x0e1c),
    _sapp_x11_codepair::new(0x0dbd, 0x0e1d),
    _sapp_x11_codepair::new(0x0dbe, 0x0e1e),
    _sapp_x11_codepair::new(0x0dbf, 0x0e1f),
    _sapp_x11_codepair::new(0x0dc0, 0x0e20),
    _sapp_x11_codepair::new(0x0dc1, 0x0e21),
    _sapp_x11_codepair::new(0x0dc2, 0x0e22),
    _sapp_x11_codepair::new(0x0dc3, 0x0e23),
    _sapp_x11_codepair::new(0x0dc4, 0x0e24),
    _sapp_x11_codepair::new(0x0dc5, 0x0e25),
    _sapp_x11_codepair::new(0x0dc6, 0x0e26),
    _sapp_x11_codepair::new(0x0dc7, 0x0e27),
    _sapp_x11_codepair::new(0x0dc8, 0x0e28),
    _sapp_x11_codepair::new(0x0dc9, 0x0e29),
    _sapp_x11_codepair::new(0x0dca, 0x0e2a),
    _sapp_x11_codepair::new(0x0dcb, 0x0e2b),
    _sapp_x11_codepair::new(0x0dcc, 0x0e2c),
    _sapp_x11_codepair::new(0x0dcd, 0x0e2d),
    _sapp_x11_codepair::new(0x0dce, 0x0e2e),
    _sapp_x11_codepair::new(0x0dcf, 0x0e2f),
    _sapp_x11_codepair::new(0x0dd0, 0x0e30),
    _sapp_x11_codepair::new(0x0dd1, 0x0e31),
    _sapp_x11_codepair::new(0x0dd2, 0x0e32),
    _sapp_x11_codepair::new(0x0dd3, 0x0e33),
    _sapp_x11_codepair::new(0x0dd4, 0x0e34),
    _sapp_x11_codepair::new(0x0dd5, 0x0e35),
    _sapp_x11_codepair::new(0x0dd6, 0x0e36),
    _sapp_x11_codepair::new(0x0dd7, 0x0e37),
    _sapp_x11_codepair::new(0x0dd8, 0x0e38),
    _sapp_x11_codepair::new(0x0dd9, 0x0e39),
    _sapp_x11_codepair::new(0x0dda, 0x0e3a),
    _sapp_x11_codepair::new(0x0ddf, 0x0e3f),
    _sapp_x11_codepair::new(0x0de0, 0x0e40),
    _sapp_x11_codepair::new(0x0de1, 0x0e41),
    _sapp_x11_codepair::new(0x0de2, 0x0e42),
    _sapp_x11_codepair::new(0x0de3, 0x0e43),
    _sapp_x11_codepair::new(0x0de4, 0x0e44),
    _sapp_x11_codepair::new(0x0de5, 0x0e45),
    _sapp_x11_codepair::new(0x0de6, 0x0e46),
    _sapp_x11_codepair::new(0x0de7, 0x0e47),
    _sapp_x11_codepair::new(0x0de8, 0x0e48),
    _sapp_x11_codepair::new(0x0de9, 0x0e49),
    _sapp_x11_codepair::new(0x0dea, 0x0e4a),
    _sapp_x11_codepair::new(0x0deb, 0x0e4b),
    _sapp_x11_codepair::new(0x0dec, 0x0e4c),
    _sapp_x11_codepair::new(0x0ded, 0x0e4d),
    _sapp_x11_codepair::new(0x0df0, 0x0e50),
    _sapp_x11_codepair::new(0x0df1, 0x0e51),
    _sapp_x11_codepair::new(0x0df2, 0x0e52),
    _sapp_x11_codepair::new(0x0df3, 0x0e53),
    _sapp_x11_codepair::new(0x0df4, 0x0e54),
    _sapp_x11_codepair::new(0x0df5, 0x0e55),
    _sapp_x11_codepair::new(0x0df6, 0x0e56),
    _sapp_x11_codepair::new(0x0df7, 0x0e57),
    _sapp_x11_codepair::new(0x0df8, 0x0e58),
    _sapp_x11_codepair::new(0x0df9, 0x0e59),
    _sapp_x11_codepair::new(0x0ea1, 0x3131),
    _sapp_x11_codepair::new(0x0ea2, 0x3132),
    _sapp_x11_codepair::new(0x0ea3, 0x3133),
    _sapp_x11_codepair::new(0x0ea4, 0x3134),
    _sapp_x11_codepair::new(0x0ea5, 0x3135),
    _sapp_x11_codepair::new(0x0ea6, 0x3136),
    _sapp_x11_codepair::new(0x0ea7, 0x3137),
    _sapp_x11_codepair::new(0x0ea8, 0x3138),
    _sapp_x11_codepair::new(0x0ea9, 0x3139),
    _sapp_x11_codepair::new(0x0eaa, 0x313a),
    _sapp_x11_codepair::new(0x0eab, 0x313b),
    _sapp_x11_codepair::new(0x0eac, 0x313c),
    _sapp_x11_codepair::new(0x0ead, 0x313d),
    _sapp_x11_codepair::new(0x0eae, 0x313e),
    _sapp_x11_codepair::new(0x0eaf, 0x313f),
    _sapp_x11_codepair::new(0x0eb0, 0x3140),
    _sapp_x11_codepair::new(0x0eb1, 0x3141),
    _sapp_x11_codepair::new(0x0eb2, 0x3142),
    _sapp_x11_codepair::new(0x0eb3, 0x3143),
    _sapp_x11_codepair::new(0x0eb4, 0x3144),
    _sapp_x11_codepair::new(0x0eb5, 0x3145),
    _sapp_x11_codepair::new(0x0eb6, 0x3146),
    _sapp_x11_codepair::new(0x0eb7, 0x3147),
    _sapp_x11_codepair::new(0x0eb8, 0x3148),
    _sapp_x11_codepair::new(0x0eb9, 0x3149),
    _sapp_x11_codepair::new(0x0eba, 0x314a),
    _sapp_x11_codepair::new(0x0ebb, 0x314b),
    _sapp_x11_codepair::new(0x0ebc, 0x314c),
    _sapp_x11_codepair::new(0x0ebd, 0x314d),
    _sapp_x11_codepair::new(0x0ebe, 0x314e),
    _sapp_x11_codepair::new(0x0ebf, 0x314f),
    _sapp_x11_codepair::new(0x0ec0, 0x3150),
    _sapp_x11_codepair::new(0x0ec1, 0x3151),
    _sapp_x11_codepair::new(0x0ec2, 0x3152),
    _sapp_x11_codepair::new(0x0ec3, 0x3153),
    _sapp_x11_codepair::new(0x0ec4, 0x3154),
    _sapp_x11_codepair::new(0x0ec5, 0x3155),
    _sapp_x11_codepair::new(0x0ec6, 0x3156),
    _sapp_x11_codepair::new(0x0ec7, 0x3157),
    _sapp_x11_codepair::new(0x0ec8, 0x3158),
    _sapp_x11_codepair::new(0x0ec9, 0x3159),
    _sapp_x11_codepair::new(0x0eca, 0x315a),
    _sapp_x11_codepair::new(0x0ecb, 0x315b),
    _sapp_x11_codepair::new(0x0ecc, 0x315c),
    _sapp_x11_codepair::new(0x0ecd, 0x315d),
    _sapp_x11_codepair::new(0x0ece, 0x315e),
    _sapp_x11_codepair::new(0x0ecf, 0x315f),
    _sapp_x11_codepair::new(0x0ed0, 0x3160),
    _sapp_x11_codepair::new(0x0ed1, 0x3161),
    _sapp_x11_codepair::new(0x0ed2, 0x3162),
    _sapp_x11_codepair::new(0x0ed3, 0x3163),
    _sapp_x11_codepair::new(0x0ed4, 0x11a8),
    _sapp_x11_codepair::new(0x0ed5, 0x11a9),
    _sapp_x11_codepair::new(0x0ed6, 0x11aa),
    _sapp_x11_codepair::new(0x0ed7, 0x11ab),
    _sapp_x11_codepair::new(0x0ed8, 0x11ac),
    _sapp_x11_codepair::new(0x0ed9, 0x11ad),
    _sapp_x11_codepair::new(0x0eda, 0x11ae),
    _sapp_x11_codepair::new(0x0edb, 0x11af),
    _sapp_x11_codepair::new(0x0edc, 0x11b0),
    _sapp_x11_codepair::new(0x0edd, 0x11b1),
    _sapp_x11_codepair::new(0x0ede, 0x11b2),
    _sapp_x11_codepair::new(0x0edf, 0x11b3),
    _sapp_x11_codepair::new(0x0ee0, 0x11b4),
    _sapp_x11_codepair::new(0x0ee1, 0x11b5),
    _sapp_x11_codepair::new(0x0ee2, 0x11b6),
    _sapp_x11_codepair::new(0x0ee3, 0x11b7),
    _sapp_x11_codepair::new(0x0ee4, 0x11b8),
    _sapp_x11_codepair::new(0x0ee5, 0x11b9),
    _sapp_x11_codepair::new(0x0ee6, 0x11ba),
    _sapp_x11_codepair::new(0x0ee7, 0x11bb),
    _sapp_x11_codepair::new(0x0ee8, 0x11bc),
    _sapp_x11_codepair::new(0x0ee9, 0x11bd),
    _sapp_x11_codepair::new(0x0eea, 0x11be),
    _sapp_x11_codepair::new(0x0eeb, 0x11bf),
    _sapp_x11_codepair::new(0x0eec, 0x11c0),
    _sapp_x11_codepair::new(0x0eed, 0x11c1),
    _sapp_x11_codepair::new(0x0eee, 0x11c2),
    _sapp_x11_codepair::new(0x0eef, 0x316d),
    _sapp_x11_codepair::new(0x0ef0, 0x3171),
    _sapp_x11_codepair::new(0x0ef1, 0x3178),
    _sapp_x11_codepair::new(0x0ef2, 0x317f),
    _sapp_x11_codepair::new(0x0ef3, 0x3181),
    _sapp_x11_codepair::new(0x0ef4, 0x3184),
    _sapp_x11_codepair::new(0x0ef5, 0x3186),
    _sapp_x11_codepair::new(0x0ef6, 0x318d),
    _sapp_x11_codepair::new(0x0ef7, 0x318e),
    _sapp_x11_codepair::new(0x0ef8, 0x11eb),
    _sapp_x11_codepair::new(0x0ef9, 0x11f0),
    _sapp_x11_codepair::new(0x0efa, 0x11f9),
    _sapp_x11_codepair::new(0x0eff, 0x20a9),
    _sapp_x11_codepair::new(0x13a4, 0x20ac),
    _sapp_x11_codepair::new(0x13bc, 0x0152),
    _sapp_x11_codepair::new(0x13bd, 0x0153),
    _sapp_x11_codepair::new(0x13be, 0x0178),
    _sapp_x11_codepair::new(0x20ac, 0x20ac),
    _sapp_x11_codepair::new(0xfe50, '`' as u16),
    _sapp_x11_codepair::new(0xfe51, 0x00b4),
    _sapp_x11_codepair::new(0xfe52, '^' as u16),
    _sapp_x11_codepair::new(0xfe53, '~' as u16),
    _sapp_x11_codepair::new(0xfe54, 0x00af),
    _sapp_x11_codepair::new(0xfe55, 0x02d8),
    _sapp_x11_codepair::new(0xfe56, 0x02d9),
    _sapp_x11_codepair::new(0xfe57, 0x00a8),
    _sapp_x11_codepair::new(0xfe58, 0x02da),
    _sapp_x11_codepair::new(0xfe59, 0x02dd),
    _sapp_x11_codepair::new(0xfe5a, 0x02c7),
    _sapp_x11_codepair::new(0xfe5b, 0x00b8),
    _sapp_x11_codepair::new(0xfe5c, 0x02db),
    _sapp_x11_codepair::new(0xfe5d, 0x037a),
    _sapp_x11_codepair::new(0xfe5e, 0x309b),
    _sapp_x11_codepair::new(0xfe5f, 0x309c),
    _sapp_x11_codepair::new(0xfe63, '/' as u16),
    _sapp_x11_codepair::new(0xfe64, 0x02bc),
    _sapp_x11_codepair::new(0xfe65, 0x02bd),
    _sapp_x11_codepair::new(0xfe66, 0x02f5),
    _sapp_x11_codepair::new(0xfe67, 0x02f3),
    _sapp_x11_codepair::new(0xfe68, 0x02cd),
    _sapp_x11_codepair::new(0xfe69, 0xa788),
    _sapp_x11_codepair::new(0xfe6a, 0x02f7),
    _sapp_x11_codepair::new(0xfe6e, ',' as u16),
    _sapp_x11_codepair::new(0xfe6f, 0x00a4),
    _sapp_x11_codepair::new(0xfe80, 'a' as u16), // XK_dead_a
    _sapp_x11_codepair::new(0xfe81, 'A' as u16), // XK_dead_A
    _sapp_x11_codepair::new(0xfe82, 'e' as u16), // XK_dead_e
    _sapp_x11_codepair::new(0xfe83, 'E' as u16), // XK_dead_E
    _sapp_x11_codepair::new(0xfe84, 'i' as u16), // XK_dead_i
    _sapp_x11_codepair::new(0xfe85, 'I' as u16), // XK_dead_I
    _sapp_x11_codepair::new(0xfe86, 'o' as u16), // XK_dead_o
    _sapp_x11_codepair::new(0xfe87, 'O' as u16), // XK_dead_O
    _sapp_x11_codepair::new(0xfe88, 'u' as u16), // XK_dead_u
    _sapp_x11_codepair::new(0xfe89, 'U' as u16), // XK_dead_U
    _sapp_x11_codepair::new(0xfe8a, 0x0259),
    _sapp_x11_codepair::new(0xfe8b, 0x018f),
    _sapp_x11_codepair::new(0xfe8c, 0x00b5),
    _sapp_x11_codepair::new(0xfe90, '_' as u16),
    _sapp_x11_codepair::new(0xfe91, 0x02c8),
    _sapp_x11_codepair::new(0xfe92, 0x02cc),
    _sapp_x11_codepair::new(0xff80 /*XKB_KEY_KP_Space*/, ' ' as u16),
    _sapp_x11_codepair::new(0xff95 /*XKB_KEY_KP_7*/, 0x0037),
    _sapp_x11_codepair::new(0xff96 /*XKB_KEY_KP_4*/, 0x0034),
    _sapp_x11_codepair::new(0xff97 /*XKB_KEY_KP_8*/, 0x0038),
    _sapp_x11_codepair::new(0xff98 /*XKB_KEY_KP_6*/, 0x0036),
    _sapp_x11_codepair::new(0xff99 /*XKB_KEY_KP_2*/, 0x0032),
    _sapp_x11_codepair::new(0xff9a /*XKB_KEY_KP_9*/, 0x0039),
    _sapp_x11_codepair::new(0xff9b /*XKB_KEY_KP_3*/, 0x0033),
    _sapp_x11_codepair::new(0xff9c /*XKB_KEY_KP_1*/, 0x0031),
    _sapp_x11_codepair::new(0xff9d /*XKB_KEY_KP_5*/, 0x0035),
    _sapp_x11_codepair::new(0xff9e /*XKB_KEY_KP_0*/, 0x0030),
    _sapp_x11_codepair::new(0xffaa /*XKB_KEY_KP_Multiply*/, '*' as u16),
    _sapp_x11_codepair::new(0xffab /*XKB_KEY_KP_Add*/, '+' as u16),
    _sapp_x11_codepair::new(0xffac /*XKB_KEY_KP_Separator*/, ',' as u16),
    _sapp_x11_codepair::new(0xffad /*XKB_KEY_KP_Subtract*/, '-' as u16),
    _sapp_x11_codepair::new(0xffae /*XKB_KEY_KP_Decimal*/, '.' as u16),
    _sapp_x11_codepair::new(0xffaf /*XKB_KEY_KP_Divide*/, '/' as u16),
    _sapp_x11_codepair::new(0xffb0 /*XKB_KEY_KP_0*/, 0x0030),
    _sapp_x11_codepair::new(0xffb1 /*XKB_KEY_KP_1*/, 0x0031),
    _sapp_x11_codepair::new(0xffb2 /*XKB_KEY_KP_2*/, 0x0032),
    _sapp_x11_codepair::new(0xffb3 /*XKB_KEY_KP_3*/, 0x0033),
    _sapp_x11_codepair::new(0xffb4 /*XKB_KEY_KP_4*/, 0x0034),
    _sapp_x11_codepair::new(0xffb5 /*XKB_KEY_KP_5*/, 0x0035),
    _sapp_x11_codepair::new(0xffb6 /*XKB_KEY_KP_6*/, 0x0036),
    _sapp_x11_codepair::new(0xffb7 /*XKB_KEY_KP_7*/, 0x0037),
    _sapp_x11_codepair::new(0xffb8 /*XKB_KEY_KP_8*/, 0x0038),
    _sapp_x11_codepair::new(0xffb9 /*XKB_KEY_KP_9*/, 0x0039),
    _sapp_x11_codepair::new(0xffbd /*XKB_KEY_KP_Equal*/, '=' as u16),
];
pub unsafe extern "C" fn _sapp_x11_keysym_to_unicode(mut keysym: KeySym) -> i32 {
    let mut min = 0 as libc::c_int;
    let mut max = (::std::mem::size_of::<[_sapp_x11_codepair; 828]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<_sapp_x11_codepair>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int;
    let mut mid: libc::c_int = 0;
    if keysym >= 0x20 as libc::c_int as libc::c_ulong
        && keysym <= 0x7e as libc::c_int as libc::c_ulong
        || keysym >= 0xa0 as libc::c_int as libc::c_ulong
            && keysym <= 0xff as libc::c_int as libc::c_ulong
    {
        return keysym as i32;
    }
    if keysym & 0xff000000 as libc::c_uint as libc::c_ulong
        == 0x1000000 as libc::c_int as libc::c_ulong
    {
        return (keysym & 0xffffff as libc::c_int as libc::c_ulong) as i32;
    }
    while max >= min {
        mid = (min + max) / 2 as libc::c_int;
        if (_sapp_x11_keysymtab[mid as usize].keysym as libc::c_ulong) < keysym {
            min = mid + 1 as libc::c_int
        } else if _sapp_x11_keysymtab[mid as usize].keysym as libc::c_ulong > keysym {
            max = mid - 1 as libc::c_int
        } else {
            return _sapp_x11_keysymtab[mid as usize].ucs as i32;
        }
    }
    return -(1 as libc::c_int);
}
pub static mut _sapp_x11_keycodes: [bool; 256] = [false; 256];
pub unsafe extern "C" fn _sapp_x11_key_event(
    mut type_: sapp_event_type,
    mut key: sapp_keycode,
    mut repeat: bool,
    mut mods: u32,
) {
    if _sapp_events_enabled() {
        _sapp_init_event(type_);
        _sapp.event.key_code = key;
        _sapp.event.key_repeat = repeat;
        _sapp.event.modifiers = mods;
        _sapp_call_event(&mut _sapp.event);
    };
}
pub unsafe extern "C" fn _sapp_x11_translate_key(mut scancode: libc::c_int) -> sapp_keycode {
    let mut dummy: libc::c_int = 0;
    let mut keysyms = XGetKeyboardMapping(
        _sapp_x11_display,
        scancode as KeyCode,
        1 as libc::c_int,
        &mut dummy,
    );
    assert!(!keysyms.is_null());

    let mut keysym = *keysyms.offset(0 as libc::c_int as isize);
    XFree(keysyms as *mut libc::c_void);
    match keysym {
        65307 => return sapp_keycode_SAPP_KEYCODE_ESCAPE,
        65289 => return sapp_keycode_SAPP_KEYCODE_TAB,
        65505 => return sapp_keycode_SAPP_KEYCODE_LEFT_SHIFT,
        65506 => return sapp_keycode_SAPP_KEYCODE_RIGHT_SHIFT,
        65507 => return sapp_keycode_SAPP_KEYCODE_LEFT_CONTROL,
        65508 => return sapp_keycode_SAPP_KEYCODE_RIGHT_CONTROL,
        65511 | 65513 => return sapp_keycode_SAPP_KEYCODE_LEFT_ALT,
        65406 | 65027 | 65512 | 65514 => return sapp_keycode_SAPP_KEYCODE_RIGHT_ALT,
        65515 => return sapp_keycode_SAPP_KEYCODE_LEFT_SUPER,
        65516 => return sapp_keycode_SAPP_KEYCODE_RIGHT_SUPER,
        65383 => return sapp_keycode_SAPP_KEYCODE_MENU,
        65407 => return sapp_keycode_SAPP_KEYCODE_NUM_LOCK,
        65509 => return sapp_keycode_SAPP_KEYCODE_CAPS_LOCK,
        65377 => return sapp_keycode_SAPP_KEYCODE_PRINT_SCREEN,
        65300 => return sapp_keycode_SAPP_KEYCODE_SCROLL_LOCK,
        65299 => return sapp_keycode_SAPP_KEYCODE_PAUSE,
        65535 => return sapp_keycode_SAPP_KEYCODE_DELETE,
        65288 => return sapp_keycode_SAPP_KEYCODE_BACKSPACE,
        65293 => return sapp_keycode_SAPP_KEYCODE_ENTER,
        65360 => return sapp_keycode_SAPP_KEYCODE_HOME,
        65367 => return sapp_keycode_SAPP_KEYCODE_END,
        65365 => return sapp_keycode_SAPP_KEYCODE_PAGE_UP,
        65366 => return sapp_keycode_SAPP_KEYCODE_PAGE_DOWN,
        65379 => return sapp_keycode_SAPP_KEYCODE_INSERT,
        65361 => return sapp_keycode_SAPP_KEYCODE_LEFT,
        65363 => return sapp_keycode_SAPP_KEYCODE_RIGHT,
        65364 => return sapp_keycode_SAPP_KEYCODE_DOWN,
        65362 => return sapp_keycode_SAPP_KEYCODE_UP,
        65470 => return sapp_keycode_SAPP_KEYCODE_F1,
        65471 => return sapp_keycode_SAPP_KEYCODE_F2,
        65472 => return sapp_keycode_SAPP_KEYCODE_F3,
        65473 => return sapp_keycode_SAPP_KEYCODE_F4,
        65474 => return sapp_keycode_SAPP_KEYCODE_F5,
        65475 => return sapp_keycode_SAPP_KEYCODE_F6,
        65476 => return sapp_keycode_SAPP_KEYCODE_F7,
        65477 => return sapp_keycode_SAPP_KEYCODE_F8,
        65478 => return sapp_keycode_SAPP_KEYCODE_F9,
        65479 => return sapp_keycode_SAPP_KEYCODE_F10,
        65480 => return sapp_keycode_SAPP_KEYCODE_F11,
        65481 => return sapp_keycode_SAPP_KEYCODE_F12,
        65482 => return sapp_keycode_SAPP_KEYCODE_F13,
        65483 => return sapp_keycode_SAPP_KEYCODE_F14,
        65484 => return sapp_keycode_SAPP_KEYCODE_F15,
        65485 => return sapp_keycode_SAPP_KEYCODE_F16,
        65486 => return sapp_keycode_SAPP_KEYCODE_F17,
        65487 => return sapp_keycode_SAPP_KEYCODE_F18,
        65488 => return sapp_keycode_SAPP_KEYCODE_F19,
        65489 => return sapp_keycode_SAPP_KEYCODE_F20,
        65490 => return sapp_keycode_SAPP_KEYCODE_F21,
        65491 => return sapp_keycode_SAPP_KEYCODE_F22,
        65492 => return sapp_keycode_SAPP_KEYCODE_F23,
        65493 => return sapp_keycode_SAPP_KEYCODE_F24,
        65494 => return sapp_keycode_SAPP_KEYCODE_F25,
        65455 => return sapp_keycode_SAPP_KEYCODE_KP_DIVIDE,
        65450 => return sapp_keycode_SAPP_KEYCODE_KP_MULTIPLY,
        65453 => return sapp_keycode_SAPP_KEYCODE_KP_SUBTRACT,
        65451 => return sapp_keycode_SAPP_KEYCODE_KP_ADD,
        65438 => return sapp_keycode_SAPP_KEYCODE_KP_0,
        65436 => return sapp_keycode_SAPP_KEYCODE_KP_1,
        65433 => return sapp_keycode_SAPP_KEYCODE_KP_2,
        65435 => return sapp_keycode_SAPP_KEYCODE_KP_3,
        65430 => return sapp_keycode_SAPP_KEYCODE_KP_4,
        65437 => return sapp_keycode_SAPP_KEYCODE_KP_5,
        65432 => return sapp_keycode_SAPP_KEYCODE_KP_6,
        65429 => return sapp_keycode_SAPP_KEYCODE_KP_7,
        65431 => return sapp_keycode_SAPP_KEYCODE_KP_8,
        65434 => return sapp_keycode_SAPP_KEYCODE_KP_9,
        65439 => return sapp_keycode_SAPP_KEYCODE_KP_DECIMAL,
        65469 => return sapp_keycode_SAPP_KEYCODE_KP_EQUAL,
        65421 => return sapp_keycode_SAPP_KEYCODE_KP_ENTER,
        97 => return sapp_keycode_SAPP_KEYCODE_A,
        98 => return sapp_keycode_SAPP_KEYCODE_B,
        99 => return sapp_keycode_SAPP_KEYCODE_C,
        100 => return sapp_keycode_SAPP_KEYCODE_D,
        101 => return sapp_keycode_SAPP_KEYCODE_E,
        102 => return sapp_keycode_SAPP_KEYCODE_F,
        103 => return sapp_keycode_SAPP_KEYCODE_G,
        104 => return sapp_keycode_SAPP_KEYCODE_H,
        105 => return sapp_keycode_SAPP_KEYCODE_I,
        106 => return sapp_keycode_SAPP_KEYCODE_J,
        107 => return sapp_keycode_SAPP_KEYCODE_K,
        108 => return sapp_keycode_SAPP_KEYCODE_L,
        109 => return sapp_keycode_SAPP_KEYCODE_M,
        110 => return sapp_keycode_SAPP_KEYCODE_N,
        111 => return sapp_keycode_SAPP_KEYCODE_O,
        112 => return sapp_keycode_SAPP_KEYCODE_P,
        113 => return sapp_keycode_SAPP_KEYCODE_Q,
        114 => return sapp_keycode_SAPP_KEYCODE_R,
        115 => return sapp_keycode_SAPP_KEYCODE_S,
        116 => return sapp_keycode_SAPP_KEYCODE_T,
        117 => return sapp_keycode_SAPP_KEYCODE_U,
        118 => return sapp_keycode_SAPP_KEYCODE_V,
        119 => return sapp_keycode_SAPP_KEYCODE_W,
        120 => return sapp_keycode_SAPP_KEYCODE_X,
        121 => return sapp_keycode_SAPP_KEYCODE_Y,
        122 => return sapp_keycode_SAPP_KEYCODE_Z,
        49 => return sapp_keycode_SAPP_KEYCODE_1,
        50 => return sapp_keycode_SAPP_KEYCODE_2,
        51 => return sapp_keycode_SAPP_KEYCODE_3,
        52 => return sapp_keycode_SAPP_KEYCODE_4,
        53 => return sapp_keycode_SAPP_KEYCODE_5,
        54 => return sapp_keycode_SAPP_KEYCODE_6,
        55 => return sapp_keycode_SAPP_KEYCODE_7,
        56 => return sapp_keycode_SAPP_KEYCODE_8,
        57 => return sapp_keycode_SAPP_KEYCODE_9,
        48 => return sapp_keycode_SAPP_KEYCODE_0,
        32 => return sapp_keycode_SAPP_KEYCODE_SPACE,
        45 => return sapp_keycode_SAPP_KEYCODE_MINUS,
        61 => return sapp_keycode_SAPP_KEYCODE_EQUAL,
        91 => return sapp_keycode_SAPP_KEYCODE_LEFT_BRACKET,
        93 => return sapp_keycode_SAPP_KEYCODE_RIGHT_BRACKET,
        92 => return sapp_keycode_SAPP_KEYCODE_BACKSLASH,
        59 => return sapp_keycode_SAPP_KEYCODE_SEMICOLON,
        39 => return sapp_keycode_SAPP_KEYCODE_APOSTROPHE,
        96 => return sapp_keycode_SAPP_KEYCODE_GRAVE_ACCENT,
        44 => return sapp_keycode_SAPP_KEYCODE_COMMA,
        46 => return sapp_keycode_SAPP_KEYCODE_PERIOD,
        47 => return sapp_keycode_SAPP_KEYCODE_SLASH,
        60 => return sapp_keycode_SAPP_KEYCODE_WORLD_1,
        _ => return sapp_keycode_SAPP_KEYCODE_INVALID,
    };
}
pub unsafe extern "C" fn _sapp_x11_scroll_event(
    mut x: libc::c_float,
    mut y: libc::c_float,
    mut mods: u32,
) {
    if _sapp_events_enabled() {
        _sapp_init_event(sapp_event_type_SAPP_EVENTTYPE_MOUSE_SCROLL);
        _sapp.event.modifiers = mods;
        _sapp.event.scroll_x = x;
        _sapp.event.scroll_y = y;
        _sapp_call_event(&mut _sapp.event);
    };
}
pub unsafe extern "C" fn _sapp_x11_translate_button(mut event: *const XEvent) -> sapp_mousebutton {
    match (*event).xbutton.button {
        1 => return sapp_mousebutton_SAPP_MOUSEBUTTON_LEFT,
        2 => return sapp_mousebutton_SAPP_MOUSEBUTTON_MIDDLE,
        3 => return sapp_mousebutton_SAPP_MOUSEBUTTON_RIGHT,
        _ => return sapp_mousebutton_SAPP_MOUSEBUTTON_INVALID,
    };
}
pub unsafe extern "C" fn _sapp_x11_mouse_event(
    mut type_: sapp_event_type,
    mut btn: sapp_mousebutton,
    mut mods: u32,
) {
    if _sapp_events_enabled() {
        _sapp_init_event(type_);
        _sapp.event.mouse_button = btn;
        _sapp.event.modifiers = mods;
        _sapp.event.mouse_x = _sapp.mouse_x;
        _sapp.event.mouse_y = _sapp.mouse_y;
        _sapp_call_event(&mut _sapp.event);
    };
}
pub unsafe extern "C" fn _sapp_x11_mod(mut x11_mods: libc::c_int) -> u32 {
    let mut mods = 0 as libc::c_int as u32;
    if x11_mods & ShiftMask != 0 {
        mods |= SAPP_MODIFIER_SHIFT as libc::c_int as libc::c_uint
    }
    if x11_mods & ControlMask != 0 {
        mods |= SAPP_MODIFIER_CTRL as libc::c_int as libc::c_uint
    }
    if x11_mods & Mod1Mask != 0 {
        mods |= SAPP_MODIFIER_ALT as libc::c_int as libc::c_uint
    }
    if x11_mods & Mod4Mask != 0 {
        mods |= SAPP_MODIFIER_SUPER as libc::c_int as libc::c_uint
    }
    return mods;
}
pub static mut _sapp_x11_window_state: libc::c_int = 0;
pub unsafe extern "C" fn _sapp_x11_get_window_property(
    mut property: Atom,
    mut type_: Atom,
    mut value: *mut *mut libc::c_uchar,
) -> libc::c_ulong {
    let mut actualType: Atom = 0;
    let mut actualFormat: libc::c_int = 0;
    let mut itemCount: libc::c_ulong = 0;
    let mut bytesAfter: libc::c_ulong = 0;
    XGetWindowProperty(
        _sapp_x11_display,
        _sapp_x11_window,
        property,
        0,
        libc::c_long::max_value(),
        false as _,
        type_,
        &mut actualType,
        &mut actualFormat,
        &mut itemCount,
        &mut bytesAfter,
        value,
    );
    return itemCount;
}
pub static mut _sapp_x11_WM_STATE: Atom = 0;
pub unsafe extern "C" fn _sapp_x11_get_window_state() -> libc::c_int {
    let mut result = WithdrawnState;
    let mut state: *mut C2RustUnnamed_1 = std::ptr::null_mut();
    if _sapp_x11_get_window_property(
        _sapp_x11_WM_STATE,
        _sapp_x11_WM_STATE,
        &mut state as *mut *mut C2RustUnnamed_1 as *mut *mut libc::c_uchar,
    ) >= 2
    {
        result = (*state).state as libc::c_int
    }
    if !state.is_null() {
        XFree(state as *mut libc::c_void);
    }
    return result;
}
pub static mut _sapp_x11_WM_PROTOCOLS: Atom = 0;
pub static mut _sapp_x11_WM_DELETE_WINDOW: Atom = 0;
pub unsafe extern "C" fn _sapp_x11_process_event(mut event: *mut XEvent) {
    match (*event).type_0 {
        2 => {
            let mut keycode = (*event).xkey.keycode as libc::c_int;
            let key = _sapp_x11_translate_key(keycode);
            let mut repeat = _sapp_x11_keycodes[(keycode & 0xff as libc::c_int) as usize];
            _sapp_x11_keycodes[(keycode & 0xff as libc::c_int) as usize] = true;
            let mods = _sapp_x11_mod((*event).xkey.state as libc::c_int);
            if key as libc::c_uint
                != sapp_keycode_SAPP_KEYCODE_INVALID as libc::c_int as libc::c_uint
            {
                _sapp_x11_key_event(sapp_event_type_SAPP_EVENTTYPE_KEY_DOWN, key, repeat, mods);
            }
            let mut keysym: KeySym = 0;
            XLookupString(
                &mut (*event).xkey,
                std::ptr::null_mut(),
                0 as libc::c_int,
                &mut keysym,
                std::ptr::null_mut(),
            );
            let mut chr = _sapp_x11_keysym_to_unicode(keysym);
            if chr > 0 as libc::c_int {
                _sapp_x11_char_event(chr as u32, repeat, mods);
            }
        }
        3 => {
            let mut keycode_0 = (*event).xkey.keycode as libc::c_int;
            let key_0 = _sapp_x11_translate_key(keycode_0);
            _sapp_x11_keycodes[(keycode_0 & 0xff as libc::c_int) as usize] = false;
            if key_0 as libc::c_uint
                != sapp_keycode_SAPP_KEYCODE_INVALID as libc::c_int as libc::c_uint
            {
                let mods_0 = _sapp_x11_mod((*event).xkey.state as libc::c_int);
                _sapp_x11_key_event(sapp_event_type_SAPP_EVENTTYPE_KEY_UP, key_0, false, mods_0);
            }
        }
        4 => {
            let btn = _sapp_x11_translate_button(event);
            let mods_1 = _sapp_x11_mod((*event).xbutton.state as libc::c_int);
            if btn as libc::c_int != sapp_mousebutton_SAPP_MOUSEBUTTON_INVALID as libc::c_int {
                _sapp_x11_mouse_event(sapp_event_type_SAPP_EVENTTYPE_MOUSE_DOWN, btn, mods_1);
            } else {
                match (*event).xbutton.button {
                    4 => {
                        _sapp_x11_scroll_event(0.0f32, 1.0f32, mods_1);
                    }
                    5 => {
                        _sapp_x11_scroll_event(0.0f32, -1.0f32, mods_1);
                    }
                    6 => {
                        _sapp_x11_scroll_event(1.0f32, 0.0f32, mods_1);
                    }
                    7 => {
                        _sapp_x11_scroll_event(-1.0f32, 0.0f32, mods_1);
                    }
                    _ => {}
                }
            }
        }
        5 => {
            let btn_0 = _sapp_x11_translate_button(event);
            if btn_0 as libc::c_int != sapp_mousebutton_SAPP_MOUSEBUTTON_INVALID as libc::c_int {
                _sapp_x11_mouse_event(
                    sapp_event_type_SAPP_EVENTTYPE_MOUSE_UP,
                    btn_0,
                    _sapp_x11_mod((*event).xbutton.state as libc::c_int),
                );
            }
        }
        7 => {
            _sapp_x11_mouse_event(
                sapp_event_type_SAPP_EVENTTYPE_MOUSE_ENTER,
                sapp_mousebutton_SAPP_MOUSEBUTTON_INVALID,
                _sapp_x11_mod((*event).xcrossing.state as libc::c_int),
            );
        }
        8 => {
            _sapp_x11_mouse_event(
                sapp_event_type_SAPP_EVENTTYPE_MOUSE_LEAVE,
                sapp_mousebutton_SAPP_MOUSEBUTTON_INVALID,
                _sapp_x11_mod((*event).xcrossing.state as libc::c_int),
            );
        }
        6 => {
            _sapp.mouse_x = (*event).xmotion.x as libc::c_float;
            _sapp.mouse_y = (*event).xmotion.y as libc::c_float;
            _sapp_x11_mouse_event(
                sapp_event_type_SAPP_EVENTTYPE_MOUSE_MOVE,
                sapp_mousebutton_SAPP_MOUSEBUTTON_INVALID,
                _sapp_x11_mod((*event).xmotion.state as libc::c_int),
            );
        }
        22 => {
            if (*event).xconfigure.width != _sapp.window_width
                || (*event).xconfigure.height != _sapp.window_height
            {
                _sapp.window_width = (*event).xconfigure.width;
                _sapp.window_height = (*event).xconfigure.height;
                _sapp.framebuffer_width = _sapp.window_width;
                _sapp.framebuffer_height = _sapp.window_height;
                _sapp_x11_app_event(sapp_event_type_SAPP_EVENTTYPE_RESIZED);
            }
        }
        28 => {
            if (*event).xproperty.state == PropertyNewValue {
                if (*event).xproperty.atom == _sapp_x11_WM_STATE {
                    let state = _sapp_x11_get_window_state();
                    if state != _sapp_x11_window_state {
                        _sapp_x11_window_state = state;
                        if state == IconicState {
                            _sapp_x11_app_event(sapp_event_type_SAPP_EVENTTYPE_ICONIFIED);
                        } else if state == NormalState {
                            _sapp_x11_app_event(sapp_event_type_SAPP_EVENTTYPE_RESTORED);
                        }
                    }
                }
            }
        }
        33 => {
            if (*event).xclient.message_type == _sapp_x11_WM_PROTOCOLS {
                let protocol = (*event).xclient.data.l[0 as libc::c_int as usize] as Atom;
                if protocol == _sapp_x11_WM_DELETE_WINDOW {
                    _sapp.quit_requested = true
                }
            }
        }
        // SelectionRequest
        30 => {
            // some other app is waiting for clibpoard content
            // need to make appropriate XSelectionEvent - response for this request
            // only UTF8_STRING request is actually supported
            crate::clipboard::respond_to_clipboard_request(event);
        }
        // SelectionClear
        29 => {}
        17 => {}

        // GenericEvent
        35 if (*event).xcookie.extension == _sapp_xi_extension_opcode => {
            if (*event).xcookie.evtype == crate::xi_input::XI_RawMotion {
                let (dx, dy) = crate::xi_input::read_cookie(&mut (*event).xcookie);
                _sapp_x11_raw_device_event(dx as f32, dy as f32);
            }
        }
        _ => {}
    };
}
pub unsafe extern "C" fn _sapp_call_init() {
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
pub unsafe extern "C" fn _sapp_call_frame() {
    if _sapp.init_called as libc::c_int != 0 && !_sapp.cleanup_called {
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
pub unsafe extern "C" fn _sapp_frame() {
    if _sapp.first_frame {
        _sapp.first_frame = false;
        _sapp_call_init();
    }
    _sapp_call_frame();
    _sapp.frame_count = _sapp.frame_count.wrapping_add(1);
}
pub static mut _sapp_glx_SwapBuffers: PFNGLXSWAPBUFFERSPROC = None;
pub unsafe extern "C" fn _sapp_glx_swap_buffers() {
    _sapp_glx_SwapBuffers.expect("non-null function pointer")(_sapp_x11_display, _sapp_glx_window);
}
pub unsafe extern "C" fn _sapp_events_enabled() -> bool {
    return (_sapp.desc.event_cb.is_some() || _sapp.desc.event_userdata_cb.is_some())
        && _sapp.init_called as libc::c_int != 0;
}
pub unsafe extern "C" fn _sapp_init_event(mut type_: sapp_event_type) {
    memset(
        &mut _sapp.event as *mut sapp_event as *mut libc::c_void,
        0,
        ::std::mem::size_of::<sapp_event>() as libc::c_ulong,
    );
    _sapp.event.type_ = type_;
    _sapp.event.frame_count = _sapp.frame_count;
    _sapp.event.mouse_button = sapp_mousebutton_SAPP_MOUSEBUTTON_INVALID;
    _sapp.event.window_width = _sapp.window_width;
    _sapp.event.window_height = _sapp.window_height;
    _sapp.event.framebuffer_width = _sapp.framebuffer_width;
    _sapp.event.framebuffer_height = _sapp.framebuffer_height;
}
pub unsafe extern "C" fn _sapp_call_event(mut e: *const sapp_event) {
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
pub unsafe extern "C" fn _sapp_x11_app_event(mut type_: sapp_event_type) {
    if _sapp_events_enabled() {
        _sapp_init_event(type_);
        _sapp_call_event(&mut _sapp.event);
    };
}
pub unsafe extern "C" fn _sapp_call_cleanup() {
    if !_sapp.cleanup_called {
        if _sapp.desc.cleanup_cb.is_some() {
            _sapp.desc.cleanup_cb.expect("non-null function pointer")();
        } else if _sapp.desc.cleanup_userdata_cb.is_some() {
            _sapp
                .desc
                .cleanup_userdata_cb
                .expect("non-null function pointer")(_sapp.desc.user_data);
        }
        _sapp.cleanup_called = true
    };
}
pub static mut _sapp_glx_DestroyWindow: PFNGLXDESTROYWINDOWPROC = None;
pub static mut _sapp_glx_window: GLXWindow = 0;
pub static mut _sapp_glx_DestroyContext: PFNGLXDESTROYCONTEXTPROC = None;
pub static mut _sapp_glx_ctx: GLXContext = 0 as *const __GLXcontext as *mut __GLXcontext;
pub unsafe extern "C" fn _sapp_glx_destroy_context() {
    if _sapp_glx_window != 0 {
        _sapp_glx_DestroyWindow.expect("non-null function pointer")(
            _sapp_x11_display,
            _sapp_glx_window,
        );
        _sapp_glx_window = 0 as libc::c_int as GLXWindow
    }
    if !_sapp_glx_ctx.is_null() {
        _sapp_glx_DestroyContext.expect("non-null function pointer")(
            _sapp_x11_display,
            _sapp_glx_ctx,
        );
        _sapp_glx_ctx = 0 as GLXContext
    };
}
pub static mut _sapp_x11_window: Window = 0;
pub static mut _sapp_x11_colormap: Colormap = 0;
pub unsafe extern "C" fn _sapp_x11_destroy_window() {
    if _sapp_x11_window != 0 {
        XUnmapWindow(_sapp_x11_display, _sapp_x11_window);
        XDestroyWindow(_sapp_x11_display, _sapp_x11_window);
        _sapp_x11_window = 0 as libc::c_int as Window
    }
    if _sapp_x11_colormap != 0 {
        XFreeColormap(_sapp_x11_display, _sapp_x11_colormap);
        _sapp_x11_colormap = 0 as libc::c_int as Colormap
    }
    XFlush(_sapp_x11_display);
}
pub static mut _sapp_x11_display: *mut Display = 0 as *const Display as *mut Display;

#[no_mangle]
pub unsafe extern "C" fn sapp_run(mut desc: *const sapp_desc) {
    _sapp_init_state(desc);
    _sapp_x11_window_state = NormalState;
    XInitThreads();
    XrmInitialize();
    _sapp_x11_display = XOpenDisplay(std::ptr::null());
    if _sapp_x11_display.is_null() {
        _sapp_fail(b"XOpenDisplay() failed!\n\x00" as *const u8 as *const libc::c_char);
    }
    _sapp_x11_screen = (*(_sapp_x11_display as _XPrivDisplay)).default_screen;
    _sapp_x11_root = (*(*(_sapp_x11_display as _XPrivDisplay))
        .screens
        .offset((*(_sapp_x11_display as _XPrivDisplay)).default_screen as isize))
    .root;
    XkbSetDetectableAutoRepeat(_sapp_x11_display, true as _, std::ptr::null_mut());

    // because X11 Xft.dpi may be not presented on the linux system at all
    // and _sapp_x11_query_system_dpi will keep _sapp_x11_dpi as 0
    // this hack make final dpi as 1.0 wich probably makes sense for systems without dpi (hm)
    _sapp_x11_dpi = 96.0f32;

    _sapp_x11_query_system_dpi();
    _sapp.dpi_scale = _sapp_x11_dpi / 96.0f32;
    _sapp_x11_init_extensions();
    _sapp_glx_init();
    let mut visual = 0 as *mut Visual;
    let mut depth = 0 as libc::c_int;
    _sapp_glx_choose_visual(&mut visual, &mut depth);
    _sapp_x11_create_window(visual, depth);
    _sapp_glx_create_context();
    _sapp.valid = true;
    _sapp_x11_show_window();
    if (*desc).fullscreen {
        _sapp_x11_set_fullscreen();
    }
    _sapp_glx_swapinterval(_sapp.swap_interval);
    XFlush(_sapp_x11_display);

    while !_sapp.quit_ordered {
        _sapp_glx_make_current();
        let mut count = XPending(_sapp_x11_display);
        loop {
            let fresh1 = count;
            count = count - 1;
            if !(fresh1 != 0) {
                break;
            }
            let mut event = _XEvent { type_0: 0 };
            XNextEvent(_sapp_x11_display, &mut event);
            _sapp_x11_process_event(&mut event);
        }
        _sapp_frame();
        _sapp_glx_swap_buffers();
        XFlush(_sapp_x11_display);
        if _sapp.quit_requested as libc::c_int != 0 && !_sapp.quit_ordered {
            _sapp_x11_app_event(sapp_event_type_SAPP_EVENTTYPE_QUIT_REQUESTED);
            if _sapp.quit_requested {
                _sapp.quit_ordered = true
            }
        }
    }
    _sapp_call_cleanup();
    _sapp_glx_destroy_context();
    _sapp_x11_destroy_window();
    XCloseDisplay(_sapp_x11_display);
}
#[no_mangle]
pub unsafe extern "C" fn sapp_frame_count() -> u64 {
    return _sapp.frame_count;
}
#[no_mangle]
pub unsafe extern "C" fn sapp_quit() {
    _sapp.quit_ordered = true;
}
#[no_mangle]
pub unsafe extern "C" fn sapp_cancel_quit() {
    _sapp.quit_requested = false;
}
#[no_mangle]
pub unsafe extern "C" fn sapp_request_quit() {
    _sapp.quit_requested = true;
}
#[no_mangle]
pub unsafe extern "C" fn sapp_query_desc() -> sapp_desc {
    return _sapp.desc;
}
#[no_mangle]
pub unsafe extern "C" fn sapp_userdata() -> *mut libc::c_void {
    return _sapp.desc.user_data;
}
#[no_mangle]
pub unsafe extern "C" fn sapp_mouse_shown() -> bool {
    return false;
}

pub unsafe extern "C" fn sapp_set_cursor_grab(mut grab: bool) {
    XUngrabPointer(_sapp_x11_display, 0);

    if grab {
        XGrabPointer(
            _sapp_x11_display,
            _sapp_x11_window,
            true as _,
            (ButtonPressMask
                | ButtonReleaseMask
                | EnterWindowMask
                | LeaveWindowMask
                | PointerMotionMask
                | PointerMotionHintMask
                | Button1MotionMask
                | Button2MotionMask
                | Button3MotionMask
                | Button4MotionMask
                | Button5MotionMask
                | ButtonMotionMask
                | KeymapStateMask) as libc::c_uint,
            GrabModeAsync,
            GrabModeAsync,
            _sapp_x11_window,
            0,
            0, // CurrentTime
        );
    }

    XFlush(_sapp_x11_display);
}

#[no_mangle]
pub unsafe extern "C" fn sapp_show_mouse(mut shown: bool) {
    if shown {
        x_cursor::set_cursor(0);
    } else {
        x_cursor::set_cursor(_sapp_empty_cursor);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sapp_keyboard_shown() -> bool {
    return _sapp.onscreen_keyboard_shown;
}
#[no_mangle]
pub unsafe extern "C" fn sapp_show_keyboard(mut shown: bool) {}
#[no_mangle]
pub unsafe extern "C" fn sapp_dpi_scale() -> libc::c_float {
    return _sapp.dpi_scale;
}
#[no_mangle]
pub unsafe extern "C" fn sapp_high_dpi() -> bool {
    return _sapp.desc.high_dpi && _sapp.dpi_scale > 1.5f32;
}
#[no_mangle]
pub unsafe extern "C" fn sapp_height() -> libc::c_int {
    return if _sapp.framebuffer_height > 0 {
        _sapp.framebuffer_height
    } else {
        1
    };
}
#[no_mangle]
pub unsafe extern "C" fn sapp_width() -> libc::c_int {
    return if _sapp.framebuffer_width > 0 {
        _sapp.framebuffer_width
    } else {
        1
    };
}
pub static mut _sapp: _sapp_state = _sapp_state {
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
    html5_canvas_name: 0 as *const libc::c_char,
    html5_ask_leave_site: false,
    window_title: [0; 128],
    window_title_wide: [0; 128],
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
        user_data: 0 as *const libc::c_void as *mut libc::c_void,
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
        alpha: false,
        window_title: 0 as *const libc::c_char,
        user_cursor: false,
        html5_canvas_name: 0 as *const libc::c_char,
        html5_canvas_resize: false,
        html5_preserve_drawing_buffer: false,
        html5_premultiplied_alpha: false,
        html5_ask_leave_site: false,
        ios_keyboard_resizes_canvas: false,
        gl_force_gles2: false,
    },
    keycodes: [sapp_keycode_SAPP_KEYCODE_INVALID; 512],
};
#[no_mangle]
pub unsafe extern "C" fn sapp_isvalid() -> bool {
    return _sapp.valid;
}
