#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]

pub mod fs;
pub mod gl;
mod rand;

pub use gl::*;
pub use rand::*;

struct SappContext {
    desc: sapp_desc,
    clipboard: Option<String>,
}

impl SappContext {
    unsafe fn init(desc: sapp_desc) {
        let user_data = desc.user_data;
        SAPP_CONTEXT = Some(SappContext {
            desc,
            clipboard: None,
        });
        SAPP_CONTEXT
            .as_mut()
            .unwrap()
            .desc
            .init_userdata_cb
            .unwrap_or_else(|| panic!())(user_data);
    }

    unsafe fn frame(&mut self) {
        let user_data = self.desc.user_data;
        self.desc.frame_userdata_cb.unwrap_or_else(|| panic!())(user_data);
    }

    unsafe fn event(&mut self, event: sapp_event) {
        let user_data = self.desc.user_data;
        self.desc.event_userdata_cb.unwrap_or_else(|| panic!())(&event as *const _, user_data);
    }
}

static mut SAPP_CONTEXT: Option<SappContext> = None;

unsafe fn sapp_context() -> &'static mut SappContext {
    SAPP_CONTEXT.as_mut().unwrap()
}

pub type sapp_event_type = u32;
pub type sapp_mousebutton = i32;
pub type sapp_keycode = u32;

pub const sapp_event_type_SAPP_EVENTTYPE_INVALID: sapp_event_type = 0;
pub const sapp_event_type_SAPP_EVENTTYPE_KEY_DOWN: sapp_event_type = 1;
pub const sapp_event_type_SAPP_EVENTTYPE_KEY_UP: sapp_event_type = 2;
pub const sapp_event_type_SAPP_EVENTTYPE_CHAR: sapp_event_type = 3;
pub const sapp_event_type_SAPP_EVENTTYPE_MOUSE_DOWN: sapp_event_type = 4;
pub const sapp_event_type_SAPP_EVENTTYPE_MOUSE_UP: sapp_event_type = 5;
pub const sapp_event_type_SAPP_EVENTTYPE_MOUSE_SCROLL: sapp_event_type = 6;
pub const sapp_event_type_SAPP_EVENTTYPE_MOUSE_MOVE: sapp_event_type = 7;
pub const sapp_event_type_SAPP_EVENTTYPE_MOUSE_ENTER: sapp_event_type = 8;
pub const sapp_event_type_SAPP_EVENTTYPE_MOUSE_LEAVE: sapp_event_type = 9;
pub const sapp_event_type_SAPP_EVENTTYPE_TOUCHES_BEGAN: sapp_event_type = 10;
pub const sapp_event_type_SAPP_EVENTTYPE_TOUCHES_MOVED: sapp_event_type = 11;
pub const sapp_event_type_SAPP_EVENTTYPE_TOUCHES_ENDED: sapp_event_type = 12;
pub const sapp_event_type_SAPP_EVENTTYPE_TOUCHES_CANCELLED: sapp_event_type = 13;
pub const sapp_event_type_SAPP_EVENTTYPE_RESIZED: sapp_event_type = 14;
pub const sapp_event_type_SAPP_EVENTTYPE_ICONIFIED: sapp_event_type = 15;
pub const sapp_event_type_SAPP_EVENTTYPE_RESTORED: sapp_event_type = 16;
pub const sapp_event_type_SAPP_EVENTTYPE_SUSPENDED: sapp_event_type = 17;
pub const sapp_event_type_SAPP_EVENTTYPE_RESUMED: sapp_event_type = 18;
pub const sapp_event_type_SAPP_EVENTTYPE_UPDATE_CURSOR: sapp_event_type = 19;
pub const sapp_event_type_SAPP_EVENTTYPE_QUIT_REQUESTED: sapp_event_type = 20;
pub const sapp_event_type_SAPP_EVENTTYPE_RAW_DEVICE: sapp_event_type = 21;
pub const sapp_event_type__SAPP_EVENTTYPE_NUM: sapp_event_type = 22;
pub const sapp_event_type__SAPP_EVENTTYPE_FORCE_U32: sapp_event_type = 2147483647;

pub const sapp_keycode_SAPP_KEYCODE_INVALID: sapp_keycode = 0;
pub const sapp_keycode_SAPP_KEYCODE_SPACE: sapp_keycode = 32;
pub const sapp_keycode_SAPP_KEYCODE_APOSTROPHE: sapp_keycode = 39;
pub const sapp_keycode_SAPP_KEYCODE_COMMA: sapp_keycode = 44;
pub const sapp_keycode_SAPP_KEYCODE_MINUS: sapp_keycode = 45;
pub const sapp_keycode_SAPP_KEYCODE_PERIOD: sapp_keycode = 46;
pub const sapp_keycode_SAPP_KEYCODE_SLASH: sapp_keycode = 47;
pub const sapp_keycode_SAPP_KEYCODE_0: sapp_keycode = 48;
pub const sapp_keycode_SAPP_KEYCODE_1: sapp_keycode = 49;
pub const sapp_keycode_SAPP_KEYCODE_2: sapp_keycode = 50;
pub const sapp_keycode_SAPP_KEYCODE_3: sapp_keycode = 51;
pub const sapp_keycode_SAPP_KEYCODE_4: sapp_keycode = 52;
pub const sapp_keycode_SAPP_KEYCODE_5: sapp_keycode = 53;
pub const sapp_keycode_SAPP_KEYCODE_6: sapp_keycode = 54;
pub const sapp_keycode_SAPP_KEYCODE_7: sapp_keycode = 55;
pub const sapp_keycode_SAPP_KEYCODE_8: sapp_keycode = 56;
pub const sapp_keycode_SAPP_KEYCODE_9: sapp_keycode = 57;
pub const sapp_keycode_SAPP_KEYCODE_SEMICOLON: sapp_keycode = 59;
pub const sapp_keycode_SAPP_KEYCODE_EQUAL: sapp_keycode = 61;
pub const sapp_keycode_SAPP_KEYCODE_A: sapp_keycode = 65;
pub const sapp_keycode_SAPP_KEYCODE_B: sapp_keycode = 66;
pub const sapp_keycode_SAPP_KEYCODE_C: sapp_keycode = 67;
pub const sapp_keycode_SAPP_KEYCODE_D: sapp_keycode = 68;
pub const sapp_keycode_SAPP_KEYCODE_E: sapp_keycode = 69;
pub const sapp_keycode_SAPP_KEYCODE_F: sapp_keycode = 70;
pub const sapp_keycode_SAPP_KEYCODE_G: sapp_keycode = 71;
pub const sapp_keycode_SAPP_KEYCODE_H: sapp_keycode = 72;
pub const sapp_keycode_SAPP_KEYCODE_I: sapp_keycode = 73;
pub const sapp_keycode_SAPP_KEYCODE_J: sapp_keycode = 74;
pub const sapp_keycode_SAPP_KEYCODE_K: sapp_keycode = 75;
pub const sapp_keycode_SAPP_KEYCODE_L: sapp_keycode = 76;
pub const sapp_keycode_SAPP_KEYCODE_M: sapp_keycode = 77;
pub const sapp_keycode_SAPP_KEYCODE_N: sapp_keycode = 78;
pub const sapp_keycode_SAPP_KEYCODE_O: sapp_keycode = 79;
pub const sapp_keycode_SAPP_KEYCODE_P: sapp_keycode = 80;
pub const sapp_keycode_SAPP_KEYCODE_Q: sapp_keycode = 81;
pub const sapp_keycode_SAPP_KEYCODE_R: sapp_keycode = 82;
pub const sapp_keycode_SAPP_KEYCODE_S: sapp_keycode = 83;
pub const sapp_keycode_SAPP_KEYCODE_T: sapp_keycode = 84;
pub const sapp_keycode_SAPP_KEYCODE_U: sapp_keycode = 85;
pub const sapp_keycode_SAPP_KEYCODE_V: sapp_keycode = 86;
pub const sapp_keycode_SAPP_KEYCODE_W: sapp_keycode = 87;
pub const sapp_keycode_SAPP_KEYCODE_X: sapp_keycode = 88;
pub const sapp_keycode_SAPP_KEYCODE_Y: sapp_keycode = 89;
pub const sapp_keycode_SAPP_KEYCODE_Z: sapp_keycode = 90;
pub const sapp_keycode_SAPP_KEYCODE_LEFT_BRACKET: sapp_keycode = 91;
pub const sapp_keycode_SAPP_KEYCODE_BACKSLASH: sapp_keycode = 92;
pub const sapp_keycode_SAPP_KEYCODE_RIGHT_BRACKET: sapp_keycode = 93;
pub const sapp_keycode_SAPP_KEYCODE_GRAVE_ACCENT: sapp_keycode = 96;
pub const sapp_keycode_SAPP_KEYCODE_WORLD_1: sapp_keycode = 161;
pub const sapp_keycode_SAPP_KEYCODE_WORLD_2: sapp_keycode = 162;
pub const sapp_keycode_SAPP_KEYCODE_ESCAPE: sapp_keycode = 256;
pub const sapp_keycode_SAPP_KEYCODE_ENTER: sapp_keycode = 257;
pub const sapp_keycode_SAPP_KEYCODE_TAB: sapp_keycode = 258;
pub const sapp_keycode_SAPP_KEYCODE_BACKSPACE: sapp_keycode = 259;
pub const sapp_keycode_SAPP_KEYCODE_INSERT: sapp_keycode = 260;
pub const sapp_keycode_SAPP_KEYCODE_DELETE: sapp_keycode = 261;
pub const sapp_keycode_SAPP_KEYCODE_RIGHT: sapp_keycode = 262;
pub const sapp_keycode_SAPP_KEYCODE_LEFT: sapp_keycode = 263;
pub const sapp_keycode_SAPP_KEYCODE_DOWN: sapp_keycode = 264;
pub const sapp_keycode_SAPP_KEYCODE_UP: sapp_keycode = 265;
pub const sapp_keycode_SAPP_KEYCODE_PAGE_UP: sapp_keycode = 266;
pub const sapp_keycode_SAPP_KEYCODE_PAGE_DOWN: sapp_keycode = 267;
pub const sapp_keycode_SAPP_KEYCODE_HOME: sapp_keycode = 268;
pub const sapp_keycode_SAPP_KEYCODE_END: sapp_keycode = 269;
pub const sapp_keycode_SAPP_KEYCODE_CAPS_LOCK: sapp_keycode = 280;
pub const sapp_keycode_SAPP_KEYCODE_SCROLL_LOCK: sapp_keycode = 281;
pub const sapp_keycode_SAPP_KEYCODE_NUM_LOCK: sapp_keycode = 282;
pub const sapp_keycode_SAPP_KEYCODE_PRINT_SCREEN: sapp_keycode = 283;
pub const sapp_keycode_SAPP_KEYCODE_PAUSE: sapp_keycode = 284;
pub const sapp_keycode_SAPP_KEYCODE_F1: sapp_keycode = 290;
pub const sapp_keycode_SAPP_KEYCODE_F2: sapp_keycode = 291;
pub const sapp_keycode_SAPP_KEYCODE_F3: sapp_keycode = 292;
pub const sapp_keycode_SAPP_KEYCODE_F4: sapp_keycode = 293;
pub const sapp_keycode_SAPP_KEYCODE_F5: sapp_keycode = 294;
pub const sapp_keycode_SAPP_KEYCODE_F6: sapp_keycode = 295;
pub const sapp_keycode_SAPP_KEYCODE_F7: sapp_keycode = 296;
pub const sapp_keycode_SAPP_KEYCODE_F8: sapp_keycode = 297;
pub const sapp_keycode_SAPP_KEYCODE_F9: sapp_keycode = 298;
pub const sapp_keycode_SAPP_KEYCODE_F10: sapp_keycode = 299;
pub const sapp_keycode_SAPP_KEYCODE_F11: sapp_keycode = 300;
pub const sapp_keycode_SAPP_KEYCODE_F12: sapp_keycode = 301;
pub const sapp_keycode_SAPP_KEYCODE_F13: sapp_keycode = 302;
pub const sapp_keycode_SAPP_KEYCODE_F14: sapp_keycode = 303;
pub const sapp_keycode_SAPP_KEYCODE_F15: sapp_keycode = 304;
pub const sapp_keycode_SAPP_KEYCODE_F16: sapp_keycode = 305;
pub const sapp_keycode_SAPP_KEYCODE_F17: sapp_keycode = 306;
pub const sapp_keycode_SAPP_KEYCODE_F18: sapp_keycode = 307;
pub const sapp_keycode_SAPP_KEYCODE_F19: sapp_keycode = 308;
pub const sapp_keycode_SAPP_KEYCODE_F20: sapp_keycode = 309;
pub const sapp_keycode_SAPP_KEYCODE_F21: sapp_keycode = 310;
pub const sapp_keycode_SAPP_KEYCODE_F22: sapp_keycode = 311;
pub const sapp_keycode_SAPP_KEYCODE_F23: sapp_keycode = 312;
pub const sapp_keycode_SAPP_KEYCODE_F24: sapp_keycode = 313;
pub const sapp_keycode_SAPP_KEYCODE_F25: sapp_keycode = 314;
pub const sapp_keycode_SAPP_KEYCODE_KP_0: sapp_keycode = 320;
pub const sapp_keycode_SAPP_KEYCODE_KP_1: sapp_keycode = 321;
pub const sapp_keycode_SAPP_KEYCODE_KP_2: sapp_keycode = 322;
pub const sapp_keycode_SAPP_KEYCODE_KP_3: sapp_keycode = 323;
pub const sapp_keycode_SAPP_KEYCODE_KP_4: sapp_keycode = 324;
pub const sapp_keycode_SAPP_KEYCODE_KP_5: sapp_keycode = 325;
pub const sapp_keycode_SAPP_KEYCODE_KP_6: sapp_keycode = 326;
pub const sapp_keycode_SAPP_KEYCODE_KP_7: sapp_keycode = 327;
pub const sapp_keycode_SAPP_KEYCODE_KP_8: sapp_keycode = 328;
pub const sapp_keycode_SAPP_KEYCODE_KP_9: sapp_keycode = 329;
pub const sapp_keycode_SAPP_KEYCODE_KP_DECIMAL: sapp_keycode = 330;
pub const sapp_keycode_SAPP_KEYCODE_KP_DIVIDE: sapp_keycode = 331;
pub const sapp_keycode_SAPP_KEYCODE_KP_MULTIPLY: sapp_keycode = 332;
pub const sapp_keycode_SAPP_KEYCODE_KP_SUBTRACT: sapp_keycode = 333;
pub const sapp_keycode_SAPP_KEYCODE_KP_ADD: sapp_keycode = 334;
pub const sapp_keycode_SAPP_KEYCODE_KP_ENTER: sapp_keycode = 335;
pub const sapp_keycode_SAPP_KEYCODE_KP_EQUAL: sapp_keycode = 336;
pub const sapp_keycode_SAPP_KEYCODE_LEFT_SHIFT: sapp_keycode = 340;
pub const sapp_keycode_SAPP_KEYCODE_LEFT_CONTROL: sapp_keycode = 341;
pub const sapp_keycode_SAPP_KEYCODE_LEFT_ALT: sapp_keycode = 342;
pub const sapp_keycode_SAPP_KEYCODE_LEFT_SUPER: sapp_keycode = 343;
pub const sapp_keycode_SAPP_KEYCODE_RIGHT_SHIFT: sapp_keycode = 344;
pub const sapp_keycode_SAPP_KEYCODE_RIGHT_CONTROL: sapp_keycode = 345;
pub const sapp_keycode_SAPP_KEYCODE_RIGHT_ALT: sapp_keycode = 346;
pub const sapp_keycode_SAPP_KEYCODE_RIGHT_SUPER: sapp_keycode = 347;
pub const sapp_keycode_SAPP_KEYCODE_MENU: sapp_keycode = 348;

pub const sapp_mousebutton_SAPP_MOUSEBUTTON_INVALID: sapp_mousebutton = -1;
pub const sapp_mousebutton_SAPP_MOUSEBUTTON_LEFT: sapp_mousebutton = 0;
pub const sapp_mousebutton_SAPP_MOUSEBUTTON_RIGHT: sapp_mousebutton = 1;
pub const sapp_mousebutton_SAPP_MOUSEBUTTON_MIDDLE: sapp_mousebutton = 2;

pub const SAPP_MODIFIER_SHIFT: u32 = 1 << 0;
pub const SAPP_MODIFIER_CTRL: u32 = 1 << 1;
pub const SAPP_MODIFIER_ALT: u32 = 1 << 2;
pub const SAPP_MODIFIER_SUPER: u32 = 1 << 3;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sapp_event {
    pub frame_count: u64,
    pub type_: sapp_event_type,
    pub key_code: sapp_keycode,
    pub char_code: u32,
    pub key_repeat: bool,
    pub modifiers: u32,
    pub mouse_button: sapp_mousebutton,
    pub mouse_x: f32,
    pub mouse_y: f32,
    pub mouse_dx: f32,
    pub mouse_dy: f32,
    pub scroll_x: f32,
    pub scroll_y: f32,
    pub num_touches: ::std::os::raw::c_int,
    pub touches: [sapp_touchpoint; 8usize],
    pub window_width: ::std::os::raw::c_int,
    pub window_height: ::std::os::raw::c_int,
    pub framebuffer_width: ::std::os::raw::c_int,
    pub framebuffer_height: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sapp_touchpoint {
    pub identifier: usize,
    pub pos_x: f32,
    pub pos_y: f32,
    pub changed: bool,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sapp_desc {
    pub init_cb: ::std::option::Option<unsafe extern "C" fn()>,
    pub frame_cb: ::std::option::Option<unsafe extern "C" fn()>,
    pub cleanup_cb: ::std::option::Option<unsafe extern "C" fn()>,
    pub event_cb: ::std::option::Option<unsafe extern "C" fn(arg1: *const sapp_event)>,
    pub fail_cb: ::std::option::Option<unsafe extern "C" fn(arg1: *const ::std::os::raw::c_char)>,
    pub user_data: *mut ::std::os::raw::c_void,
    pub init_userdata_cb:
        ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
    pub frame_userdata_cb:
        ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
    pub cleanup_userdata_cb:
        ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
    pub event_userdata_cb: ::std::option::Option<
        unsafe extern "C" fn(arg1: *const sapp_event, arg2: *mut ::std::os::raw::c_void),
    >,
    pub fail_userdata_cb: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *const ::std::os::raw::c_char,
            arg2: *mut ::std::os::raw::c_void,
        ),
    >,
    pub width: ::std::os::raw::c_int,
    pub height: ::std::os::raw::c_int,
    pub sample_count: ::std::os::raw::c_int,
    pub swap_interval: ::std::os::raw::c_int,
    pub high_dpi: bool,
    pub fullscreen: bool,
    pub alpha: bool,
    pub window_title: *const ::std::os::raw::c_char,
    pub user_cursor: bool,
    pub html5_canvas_name: *const ::std::os::raw::c_char,
    pub html5_canvas_resize: bool,
    pub html5_preserve_drawing_buffer: bool,
    pub html5_premultiplied_alpha: bool,
    pub html5_ask_leave_site: bool,
    pub ios_keyboard_resizes_canvas: bool,
    pub gl_force_gles2: bool,
}

pub unsafe fn sapp_run(desc: *const sapp_desc) -> ::std::os::raw::c_int {
    {
        use std::ffi::CString;
        use std::panic;

        panic::set_hook(Box::new(|info| {
            let msg = CString::new(format!("{:?}", info)).unwrap_or_else(|_| {
                CString::new(format!("MALFORMED ERROR MESSAGE {:?}", info.location())).unwrap()
            });
            console_log(msg.as_ptr());
        }));
    }

    init_opengl();

    SappContext::init(*desc);

    0
}

pub unsafe fn sapp_width() -> ::std::os::raw::c_int {
    canvas_width()
}

pub unsafe fn sapp_height() -> ::std::os::raw::c_int {
    canvas_height()
}

#[no_mangle]
extern "C" {
    pub fn init_opengl();
    pub fn canvas_width() -> i32;
    pub fn canvas_height() -> i32;
    pub fn console_debug(msg: *const ::std::os::raw::c_char);
    pub fn console_log(msg: *const ::std::os::raw::c_char);
    pub fn console_info(msg: *const ::std::os::raw::c_char);
    pub fn console_warn(msg: *const ::std::os::raw::c_char);
    pub fn console_error(msg: *const ::std::os::raw::c_char);

    pub fn sapp_set_clipboard(clipboard: *const i8, len: usize);

    /// call "requestPointerLock" and "exitPointerLock" internally.
    /// Will hide cursor and will disable mouse_move events, but instead will
    /// will make inifinite mouse field for raw_device_input event.
    /// Notice that this function will works only from "engaging" event callbacks - from
    /// "mouse_down"/"key_down" event handler functions.
    pub fn sapp_set_cursor_grab(grab: bool);
}

/// Do nothing on wasm - cursor will be hidden by "sapp_set_cursor_grab" anyway.
pub unsafe fn sapp_show_mouse(_shown: bool) {}

pub unsafe fn sapp_high_dpi() -> bool {
    false
}

pub unsafe fn sapp_dpi_scale() -> f32 {
    1.
}

#[no_mangle]
pub extern "C" fn allocate_vec_u8(len: usize) -> *mut u8 {
    let mut string = vec![0u8; len];
    let ptr = string.as_mut_ptr();
    std::mem::forget(string);
    ptr
}

#[no_mangle]
pub extern "C" fn on_clipboard_paste(msg: *mut u8, len: usize) {
    let msg = unsafe { String::from_raw_parts(msg, len, len) };

    unsafe { sapp_context().clipboard = Some(msg) };
}

pub fn clipboard_get() -> Option<String> {
    unsafe { sapp_context().clipboard.clone() }
}

pub fn clipboard_set(data: &str) {
    let len = data.len();
    let data = std::ffi::CString::new(data).unwrap();
    unsafe { sapp_set_clipboard(data.as_ptr(), len) };
}

#[no_mangle]
pub extern "C" fn frame() {
    unsafe {
        sapp_context().frame();
    }
}

#[no_mangle]
pub extern "C" fn mouse_move(x: i32, y: i32) {
    let mut event: sapp_event = unsafe { std::mem::zeroed() };

    event.type_ = sapp_event_type_SAPP_EVENTTYPE_MOUSE_MOVE;
    event.mouse_x = x as f32;
    event.mouse_y = y as f32;
    unsafe {
        sapp_context().event(event);
    }
}

#[no_mangle]
pub extern "C" fn raw_mouse_move(dx: i32, dy: i32) {
    let mut event: sapp_event = unsafe { std::mem::zeroed() };

    event.type_ = sapp_event_type_SAPP_EVENTTYPE_RAW_DEVICE;
    event.mouse_dx = dx as f32;
    event.mouse_dy = dy as f32;
    unsafe {
        sapp_context().event(event);
    }
}

#[no_mangle]
pub extern "C" fn mouse_down(x: i32, y: i32, btn: i32) {
    let mut event: sapp_event = unsafe { std::mem::zeroed() };

    event.type_ = sapp_event_type_SAPP_EVENTTYPE_MOUSE_DOWN;
    event.mouse_button = btn;
    event.mouse_x = x as f32;
    event.mouse_y = y as f32;
    unsafe {
        sapp_context().event(event);
    }
}

#[no_mangle]
pub extern "C" fn mouse_up(x: i32, y: i32, btn: i32) {
    let mut event: sapp_event = unsafe { std::mem::zeroed() };

    event.type_ = sapp_event_type_SAPP_EVENTTYPE_MOUSE_UP;
    event.mouse_button = btn;
    event.mouse_x = x as f32;
    event.mouse_y = y as f32;
    unsafe {
        sapp_context().event(event);
    }
}

#[no_mangle]
pub extern "C" fn mouse_wheel(delta_x: i32, delta_y: i32) {
    let mut event: sapp_event = unsafe { std::mem::zeroed() };

    event.type_ = sapp_event_type_SAPP_EVENTTYPE_MOUSE_SCROLL;
    event.scroll_x = delta_x as f32;
    event.scroll_y = delta_y as f32;
    unsafe {
        sapp_context().event(event);
    }
}

#[no_mangle]
pub extern "C" fn key_down(key: u32, modifiers: u32, repeat: bool) {
    let mut event: sapp_event = unsafe { std::mem::zeroed() };

    event.type_ = sapp_event_type_SAPP_EVENTTYPE_KEY_DOWN;
    event.key_code = key;
    event.modifiers = modifiers;
    event.key_repeat = repeat;
    unsafe {
        sapp_context().event(event);
    }
}

#[no_mangle]
pub extern "C" fn key_press(key: u32) {
    let mut event: sapp_event = unsafe { std::mem::zeroed() };

    event.type_ = sapp_event_type_SAPP_EVENTTYPE_CHAR;
    event.char_code = key;
    unsafe {
        sapp_context().event(event);
    }
}

#[no_mangle]
pub extern "C" fn key_up(key: u32) {
    let mut event: sapp_event = unsafe { std::mem::zeroed() };

    event.type_ = sapp_event_type_SAPP_EVENTTYPE_KEY_UP;
    event.key_code = key;
    unsafe {
        sapp_context().event(event);
    }
}

#[no_mangle]
pub extern "C" fn resize(width: i32, height: i32) {
    let mut event: sapp_event = unsafe { std::mem::zeroed() };

    event.type_ = sapp_event_type_SAPP_EVENTTYPE_RESIZED;
    event.window_width = width;
    event.window_height = height;
    unsafe {
        sapp_context().event(event);
    }
}

#[no_mangle]
pub extern "C" fn touch(event_type: u32, id: u32, x: f32, y: f32) {
    let mut event: sapp_event = unsafe { std::mem::zeroed() };

    event.type_ = event_type as u32;
    event.num_touches = 1;
    event.touches[0].identifier = id as usize;
    event.touches[0].pos_x = x;
    event.touches[0].pos_y = y;
    event.touches[0].changed = true;
    unsafe {
        sapp_context().event(event);
    }
}
