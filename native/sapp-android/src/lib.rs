#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    dead_code
)]

mod egl;
pub mod gl3;
mod rand;

pub mod query_stab;
pub use query_stab::*;

pub use egl::*;
pub use gl3::*;
pub use rand::*;

pub use gl3 as gl;

// workaround for egl::* also contains None on Android
pub use std::option::Option::None;

use ndk_sys::{AInputQueue, ALooper, ANativeActivity, ANativeWindow};

use std::{
    cell::RefCell,
    os::unix::prelude::RawFd,
    ptr::null_mut,
    sync::{mpsc, Mutex},
    thread, thread_local,
};

use libc::pipe;

pub const SAPP_MAX_TOUCHPOINTS: usize = 8;

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

pub type sapp_event_type = ::std::os::raw::c_uint;
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
pub type sapp_keycode = ::std::os::raw::c_uint;

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct sapp_touchpoint {
    pub identifier: usize,
    pub pos_x: f32,
    pub pos_y: f32,
    pub changed: bool,
}
pub const sapp_mousebutton_SAPP_MOUSEBUTTON_INVALID: sapp_mousebutton = -1;
pub const sapp_mousebutton_SAPP_MOUSEBUTTON_LEFT: sapp_mousebutton = 0;
pub const sapp_mousebutton_SAPP_MOUSEBUTTON_RIGHT: sapp_mousebutton = 1;
pub const sapp_mousebutton_SAPP_MOUSEBUTTON_MIDDLE: sapp_mousebutton = 2;

pub type sapp_mousebutton = ::std::os::raw::c_int;
pub const SAPP_MODIFIER_SHIFT: ::std::os::raw::c_uint = 1;
pub const SAPP_MODIFIER_CTRL: ::std::os::raw::c_uint = 2;
pub const SAPP_MODIFIER_ALT: ::std::os::raw::c_uint = 4;
pub const SAPP_MODIFIER_SUPER: ::std::os::raw::c_uint = 8;
pub type _bindgen_ty_2 = ::std::os::raw::c_uint;

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
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
    pub touches: [sapp_touchpoint; SAPP_MAX_TOUCHPOINTS],
    pub window_width: ::std::os::raw::c_int,
    pub window_height: ::std::os::raw::c_int,
    pub framebuffer_width: ::std::os::raw::c_int,
    pub framebuffer_height: ::std::os::raw::c_int,
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
    pub ios_keyboard_resizes_canvas: bool,
    pub gl_force_gles2: bool,
}

impl Default for sapp_desc {
    fn default() -> sapp_desc {
        sapp_desc {
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
            alpha: false,
            window_title: 0 as *const _,
            user_cursor: false,
            ios_keyboard_resizes_canvas: false,
            gl_force_gles2: false,
        }
    }
}

#[derive(Default)]
struct MainThreadData {
    read_from_main_fd: RawFd,
    write_from_main_fd: RawFd,
    receiver: Option<mpsc::Receiver<()>>,
}

struct AndroidState {
    config: egl::EGLConfig,
    display: egl::EGLDisplay,
    context: egl::EGLContext,
    surface: egl::EGLSurface,

    looper: *mut ALooper,
    window: *mut ANativeWindow,
    input: *mut AInputQueue,

    has_resumed: bool,
    has_focus: bool,
}

impl Default for AndroidState {
    fn default() -> AndroidState {
        AndroidState {
            config: null_mut(),
            display: null_mut(),
            context: null_mut(),
            surface: null_mut(),
            looper: null_mut(),
            window: null_mut(),
            input: null_mut(),
            has_resumed: false,
            has_focus: false,
        }
    }
}
impl AndroidState {
    unsafe fn cleanup(&mut self) {
        if !self.display.is_null() {
            egl::eglMakeCurrent(self.display, null_mut(), null_mut(), null_mut());
            if !self.surface.is_null() {
                console_info(b"Destroying egl surface\0".as_ptr() as _);
                egl::eglDestroySurface(self.display, self.surface);
                self.surface = null_mut();
            }
            if !self.context.is_null() {
                console_info(b"Aestroying egl context\0".as_ptr() as _);
                egl::eglDestroyContext(self.display, self.context);
                self.context = null_mut();
            }
            console_info(b"Terminating egl display\0".as_ptr() as _);
            egl::eglTerminate(self.display);
            self.display = null_mut();
        }
    }
}

#[derive(Default)]
struct UiThreadData {
    read_from_main_fd: RawFd,
    write_from_main_fd: RawFd,
    android_state: AndroidState,
    desc: sapp_desc,
    first_frame: bool,
    frame_count: u64,
    is_thread_stopping: bool,
    sender: Option<mpsc::Sender<()>>,
}

impl UiThreadData {
    unsafe fn init_event(&mut self, event_type: u32) -> sapp_event {
        let shared_state = read_shared_state();

        sapp_event {
            type_: event_type,
            frame_count: self.frame_count,
            window_width: shared_state.window_width,
            window_height: shared_state.window_height,
            framebuffer_width: shared_state.framebuffer_width,
            framebuffer_height: shared_state.framebuffer_height,
            ..Default::default()
        }
    }

    unsafe fn call_event(&mut self, event: sapp_event) {
        if let Some(event_cb) = self.desc.event_cb {
            event_cb(&event);
        } else if let Some(event_userdata_cb) = self.desc.event_userdata_cb {
            event_userdata_cb(&event, self.desc.user_data);
        }
    }
}

fn wait_ui_thread() {
    MAIN_THREAD_DATA.with(|data| {
        let mut data = data.borrow_mut();
        let rx = data.receiver.as_mut().expect("not main thread");

        rx.recv().unwrap()
    })
}

fn notify_main_thread() {
    UI_THREAD_DATA.with(|data| {
        let mut data = data.borrow_mut();
        let tx = data.sender.as_mut().expect("not ui thread");

        tx.send(()).unwrap()
    })
}

/// Part of the state accessible from either main thread, ui thread
/// and all the functions like `sapp_width` - from any user thread
#[derive(Clone, Copy)]
struct SharedState {
    window_width: i32,
    window_height: i32,
    framebuffer_width: i32,
    framebuffer_height: i32,
    dpi_scale: f32,
    activity: *mut ANativeActivity,
    quit_requested: bool,
    quit_ordered: bool,
}

unsafe impl Send for SharedState {}
static mut SHARED_STATE: Option<Mutex<SharedState>> = None;

unsafe fn lock_shared_state() -> std::sync::MutexGuard<'static, SharedState> {
    if SHARED_STATE.is_none() {
        SHARED_STATE = Some(Mutex::new(SharedState {
            window_width: 0,
            window_height: 0,
            framebuffer_width: 0,
            framebuffer_height: 0,
            dpi_scale: 1.,
            activity: null_mut(),
            quit_ordered: false,
            quit_requested: false,
        }))
    }

    SHARED_STATE.as_mut().unwrap().lock().unwrap()
}

unsafe fn read_shared_state() -> SharedState {
    lock_shared_state().clone()
}

#[derive(Debug)]
#[repr(C)]
enum AndroidMessage {
    Create,
    Resume,
    Pause,
    Destroy,
    Focus,
    NoFocus,
    SetInputQueue(*mut AInputQueue),
    SetNativeWindow(*mut ANativeWindow),
}

thread_local! {
    static MAIN_THREAD_DATA: RefCell<MainThreadData> = RefCell::new(Default::default());
    static UI_THREAD_DATA: RefCell<UiThreadData> = RefCell::new(Default::default());

}

// used only once, to pass sapp_desc from main_thread to ui_thread
// will be None most of the time
static mut SAPP_DESC: Option<sapp_desc> = None;

pub unsafe fn sapp_width() -> ::std::os::raw::c_int {
    lock_shared_state().framebuffer_width
}

pub unsafe fn sapp_height() -> ::std::os::raw::c_int {
    lock_shared_state().framebuffer_height
}

pub unsafe fn sapp_high_dpi() -> bool {
    // TODO: maybe check desc.high_dpi?

    lock_shared_state().dpi_scale > 1.5
}

pub unsafe fn sapp_dpi_scale() -> f32 {
    lock_shared_state().dpi_scale
}

pub unsafe fn sapp_set_cursor_grab(_grab: bool) {}
pub unsafe fn sapp_show_mouse(_visible: bool) {}

pub unsafe fn sapp_request_quit() {
    lock_shared_state().quit_requested = true;
}

pub unsafe fn sapp_cancel_quit() {
    lock_shared_state().quit_requested = false;
}

pub unsafe fn sapp_quit() {
    lock_shared_state().quit_ordered = true;
}

#[no_mangle]
pub unsafe extern "C" fn sapp_run(desc: *const sapp_desc) -> ::std::os::raw::c_int {
    SAPP_DESC = Some(*desc);

    {
        use std::ffi::CString;
        use std::panic;

        panic::set_hook(Box::new(|info| {
            let msg = CString::new(format!("{:?}", info)).unwrap_or_else(|_| {
                CString::new(format!("MALFORMED ERROR MESSAGE {:?}", info.location())).unwrap()
            });
            console_error(msg.as_ptr());
        }));
    }

    0
}

extern "C" {
    fn sokol_main();
}

#[no_mangle]
unsafe extern "C" fn sapp_ANativeActivity_onCreate(
    activity: *mut ::std::os::raw::c_void,
    _saved_state: *mut ::std::os::raw::c_void,
    _saved_state_size: ::std::os::raw::c_int,
) {
    console_info(b"ANativeActivity_onCreate\0".as_ptr() as _);

    sokol_main();

    let activity: *mut ANativeActivity = activity as _;

    lock_shared_state().activity = activity;

    let mut pipe_fd: [RawFd; 2] = Default::default();
    if pipe(pipe_fd.as_mut_ptr()) != 0 {
        console_error(b"Could not create thread pipe\0".as_ptr() as _);
        return;
    }
    let read_from_main_fd = pipe_fd[0];
    let write_from_main_fd = pipe_fd[1];

    let (tx, rx) = mpsc::channel();

    MAIN_THREAD_DATA.with(|data| {
        let mut data = data.borrow_mut();
        data.read_from_main_fd = read_from_main_fd;
        data.write_from_main_fd = write_from_main_fd;
        data.receiver = Some(rx);
    });

    thread::spawn(move || sapp_android_loop(tx, read_from_main_fd, write_from_main_fd));

    wait_ui_thread();

    android_msg(AndroidMessage::Create);

    wait_ui_thread();

    let mut callbacks = (*activity).callbacks.as_mut().unwrap();
    (*callbacks).onStart = Some(on_start);
    (*callbacks).onResume = Some(on_resume);
    (*callbacks).onSaveInstanceState = Some(on_save_instance_state);
    (*callbacks).onWindowFocusChanged = Some(on_window_focus_changed);
    (*callbacks).onPause = Some(on_pause);
    (*callbacks).onStop = Some(on_stop);
    (*callbacks).onDestroy = Some(on_destroy);
    (*callbacks).onNativeWindowCreated = Some(on_native_window_created);
    (*callbacks).onNativeWindowDestroyed = Some(on_native_window_destroyed);
    (*callbacks).onInputQueueCreated = Some(on_input_queue_created);
    (*callbacks).onInputQueueDestroyed = Some(on_input_queue_destroyed);
    (*callbacks).onConfigurationChanged = Some(on_config_changed);
    (*callbacks).onLowMemory = Some(on_low_memory);

    console_info(b"NativeActivity successfully created\0".as_ptr() as _);

    auto_hide_nav_bar(activity);
}

fn android_should_update(state: &AndroidState) -> bool {
    let is_in_front = state.has_resumed && state.has_focus;
    let has_surface = state.surface.is_null() == false;

    is_in_front && has_surface
}

unsafe fn android_update_dimensions(
    data: &mut UiThreadData,
    window: *mut ANativeWindow,
    force_update: bool,
) {
    let state = &mut data.android_state;
    assert!(state.display.is_null() == false);
    assert!(state.context.is_null() == false);
    assert!(state.surface.is_null() == false);
    assert!(window.is_null() == false);

    let win_w = ndk_sys::ANativeWindow_getWidth(window);
    let win_h = ndk_sys::ANativeWindow_getHeight(window);

    let mut shared_state = lock_shared_state();

    let win_changed = shared_state.window_width != win_w || shared_state.window_height != win_h;
    shared_state.window_width = win_w;
    shared_state.window_height = win_h;

    if win_changed || force_update {
        if !data.desc.high_dpi {
            let buf_w = win_w / 2;
            let buf_h = win_h / 2;
            let mut format = 0;
            let egl_result = egl::eglGetConfigAttrib(
                state.display,
                state.config,
                egl::EGL_NATIVE_VISUAL_ID as _,
                &mut format,
            );
            assert!(egl_result == 1);
            // NOTE: calling ANativeWindow_setBuffersGeometry() with the same dimensions
            // as the ANativeWindow size results in weird display artefacts, that's
            // why it's only called when the buffer geometry is different from
            // the window size
            let result = ndk_sys::ANativeWindow_setBuffersGeometry(window, buf_w, buf_h, format);
            assert!(result == 0);
        }
    }

    // query surface size
    let mut fb_w = 0;
    let mut fb_h = 0;
    let egl_result_w =
        egl::eglQuerySurface(state.display, state.surface, egl::EGL_WIDTH as _, &mut fb_w);
    let egl_result_h = egl::eglQuerySurface(
        state.display,
        state.surface,
        egl::EGL_HEIGHT as _,
        &mut fb_h,
    );
    assert!(egl_result_w == 1);
    assert!(egl_result_h == 1);
    let fb_changed =
        fb_w != shared_state.framebuffer_width || fb_h != shared_state.framebuffer_height;
    shared_state.framebuffer_width = fb_w;
    shared_state.framebuffer_height = fb_h;
    shared_state.dpi_scale =
        shared_state.framebuffer_width as f32 / shared_state.window_width as f32;

    if win_changed || fb_changed || force_update {
        if data.first_frame == false {
            drop(shared_state);
            let event = data.init_event(sapp_event_type_SAPP_EVENTTYPE_RESIZED);
            data.call_event(event);
        }
    }
}

unsafe fn android_frame() {
    UI_THREAD_DATA.with(|data| {
        let mut data = &mut *data.borrow_mut();

        let window = data.android_state.window;
        android_update_dimensions(&mut *data, window, false);

        if data.first_frame {
            if let Some(init_cb) = data.desc.init_cb.as_mut() {
                init_cb();
            }
            if let Some(init_userdata_cb) = data.desc.init_userdata_cb.as_mut() {
                init_userdata_cb(data.desc.user_data);
            }
            data.first_frame = false;
        }

        if let Some(frame_cb) = data.desc.frame_cb.as_mut() {
            frame_cb();
        } else if let Some(frame_userdata_cb) = data.desc.frame_userdata_cb.as_mut() {
            frame_userdata_cb(data.desc.user_data);
        }

        data.frame_count += 1;
        egl::eglSwapBuffers(data.android_state.display, data.android_state.surface);
    });
}

unsafe fn sapp_android_loop(
    sender: mpsc::Sender<()>,
    read_from_main_fd: RawFd,
    write_from_main_fd: RawFd,
) {
    console_info(b"Loop thread started()\0".as_ptr() as _);

    let looper = ndk_sys::ALooper_prepare(0 /* or ALOOPER_PREPARE_ALLOW_NON_CALLBACKS*/);
    ndk_sys::ALooper_addFd(
        looper,
        read_from_main_fd,
        ndk_sys::ALOOPER_POLL_CALLBACK as _,
        ndk_sys::ALOOPER_EVENT_INPUT as _,
        Some(android_main_cb),
        null_mut(),
    );

    UI_THREAD_DATA.with(|data| {
        let mut data = data.borrow_mut();

        data.desc = SAPP_DESC.take().unwrap();
        data.read_from_main_fd = read_from_main_fd;
        data.write_from_main_fd = write_from_main_fd;
        data.sender = Some(sender);
        data.android_state.looper = looper;
        data.first_frame = true;
    });
    notify_main_thread();

    loop {
        // frame
        let should_update =
            UI_THREAD_DATA.with(|data| android_should_update(&data.borrow().android_state));

        if should_update {
            android_frame();
        }

        // process all events (or stop early if app is requested to quit)
        while {
            let should_update =
                UI_THREAD_DATA.with(|data| android_should_update(&data.borrow().android_state));
            let is_thread_stopping = UI_THREAD_DATA.with(|data| data.borrow().is_thread_stopping);

            let block_until_event = !is_thread_stopping && !should_update;

            !is_thread_stopping
                && ndk_sys::ALooper_pollOnce(
                    if block_until_event { -1 } else { 0 },
                    null_mut(),
                    null_mut(),
                    null_mut(),
                ) == ndk_sys::ALOOPER_POLL_CALLBACK
        } {}
    }
}

unsafe fn init_egl() -> bool {
    let display = egl::eglGetDisplay(null_mut() /* EGL_DEFAULT_DISPLAY */);
    if display == /* EGL_NO_DISPLAY */ null_mut() {
        return false;
    }

    if egl::eglInitialize(display, null_mut(), null_mut()) == 0 {
        return false;
    }

    let desc_alpha = UI_THREAD_DATA.with(|data| data.borrow().desc.alpha);

    let alpha_size = if desc_alpha { 8 } else { 0 };
    #[rustfmt::skip]
    let cfg_attributes = vec![
        egl::EGL_SURFACE_TYPE, egl::EGL_WINDOW_BIT,
        egl::EGL_RED_SIZE, 8,
        egl::EGL_GREEN_SIZE, 8,
        egl::EGL_BLUE_SIZE, 8,
        egl::EGL_ALPHA_SIZE, alpha_size,
        egl::EGL_DEPTH_SIZE, 16,
        egl::EGL_STENCIL_SIZE, 0,
        egl::EGL_NONE,
    ];
    let mut available_cfgs: Vec<egl::EGLConfig> = vec![null_mut(); 32];
    let mut cfg_count = 0;

    egl::eglChooseConfig(
        display,
        cfg_attributes.as_ptr() as _,
        available_cfgs.as_ptr() as _,
        32,
        &mut cfg_count as *mut _ as *mut _,
    );
    assert!(cfg_count > 0);
    assert!(cfg_count <= 32);

    // find config with 8-bit rgb buffer if available, ndk sample does not trust egl spec
    let mut config: egl::EGLConfig = null_mut();
    let mut exact_cfg_found = false;
    for c in &mut available_cfgs[0..cfg_count] {
        let mut r: i32 = 0;
        let mut g: i32 = 0;
        let mut b: i32 = 0;
        let mut a: i32 = 0;
        let mut d: i32 = 0;
        if egl::eglGetConfigAttrib(display, *c, egl::EGL_RED_SIZE as _, &mut r) == 1
            && egl::eglGetConfigAttrib(display, *c, egl::EGL_GREEN_SIZE as _, &mut g) == 1
            && egl::eglGetConfigAttrib(display, *c, egl::EGL_BLUE_SIZE as _, &mut b) == 1
            && egl::eglGetConfigAttrib(display, *c, egl::EGL_ALPHA_SIZE as _, &mut a) == 1
            && egl::eglGetConfigAttrib(display, *c, egl::EGL_DEPTH_SIZE as _, &mut d) == 1
            && r == 8
            && g == 8
            && b == 8
            && (alpha_size == 0 || a == alpha_size as _)
            && d == 16
        {
            exact_cfg_found = true;
            config = *c;
            break;
        }
    }
    if !exact_cfg_found {
        config = available_cfgs[0];
    }
    let ctx_attributes = vec![egl::EGL_CONTEXT_CLIENT_VERSION, 3, egl::EGL_NONE];
    let context = egl::eglCreateContext(
        display,
        config,
        /* egl::EGL_NO_CONTEXT */ null_mut(),
        ctx_attributes.as_ptr() as _,
    );
    if context.is_null() {
        return false;
    }

    UI_THREAD_DATA.with(|data| {
        let mut data = data.borrow_mut();
        data.android_state.config = config;
        data.android_state.display = display;
        data.android_state.context = context;
    });
    return true;
}

unsafe fn cleanup_egl_surface(state: &mut AndroidState) {
    if state.display == /* EGL_NO_DISPLAY */ null_mut() {
        return;
    }
    egl::eglMakeCurrent(
        state.display,
        /* EGL_NO_SURFACE */ null_mut(),
        /* egl::EGL_NO_SURFACE */ null_mut(),
        /* egl::EGL_NO_CONTEXT */ null_mut(),
    );
    if state.surface != /* EGL_NO_SURFACE */ null_mut() {
        egl::eglDestroySurface(state.display, state.surface);
        state.surface = /* EGL_NO_SURFACE */ null_mut();
    }
}

unsafe fn init_egl_surface(state: &mut AndroidState, window: *mut ANativeWindow) -> bool {
    assert!(state.display.is_null() == false);
    assert!(state.context.is_null() == false);
    assert!(state.surface.is_null());
    assert!(window.is_null() == false);

    // TODO: set window flags
    // ANativeActivity_setWindowFlags(activity, AWINDOW_FLAG_KEEP_SCREEN_ON, 0);

    // create egl surface and make it current
    let surface = egl::eglCreateWindowSurface(state.display, state.config, window as _, null_mut());

    if surface == /* EGL_NO_SURFACE  */ null_mut() {
        return false;
    }
    if egl::eglMakeCurrent(state.display, surface, surface, state.context) == 0 {
        return false;
    }
    state.surface = surface;
    true
}

unsafe extern "C" fn android_main_cb(fd: RawFd, events: i32, _data: *mut std::ffi::c_void) -> i32 {
    if events as u32 & ndk_sys::ALOOPER_EVENT_INPUT == 0 {
        console_error(b"android_main_cb() encountered unsupported event\0".as_ptr() as _);
        return 1;
    }

    let size = std::mem::size_of::<AndroidMessage>();
    let mut msg = AndroidMessage::Resume;
    if libc::read(fd, &mut msg as *mut _ as *mut _, size) != size as _ {
        console_error(b"android_main_cb() could not read from read_from_main_fd\0".as_ptr() as _);
        return 1;
    }

    match msg {
        AndroidMessage::Create => {
            let result = init_egl();
            assert!(result);
            notify_main_thread();
        }
        AndroidMessage::Resume => {
            UI_THREAD_DATA.with(|data| {
                let mut data = &mut *data.borrow_mut();
                data.android_state.has_resumed = true;

                if data.first_frame == false {
                    let event = data.init_event(sapp_event_type_SAPP_EVENTTYPE_RESUMED);
                    data.call_event(event);
                }
            });
        }
        AndroidMessage::Pause => {
            UI_THREAD_DATA.with(|data| {
                let mut data = &mut *data.borrow_mut();
                data.android_state.has_resumed = false;
                if data.first_frame == false {
                    let event = data.init_event(sapp_event_type_SAPP_EVENTTYPE_SUSPENDED);
                    data.call_event(event);
                }
            });
        }
        AndroidMessage::Focus => {
            UI_THREAD_DATA.with(|data| {
                let mut state = &mut data.borrow_mut().android_state;
                state.has_focus = true;
            });
        }
        AndroidMessage::NoFocus => {
            UI_THREAD_DATA.with(|data| {
                let mut state = &mut data.borrow_mut().android_state;
                state.has_focus = false;
            });
        }
        AndroidMessage::SetInputQueue(input) => {
            UI_THREAD_DATA.with(|data| {
                let mut state = &mut data.borrow_mut().android_state;
                if state.input != input {
                    if state.input.is_null() == false {
                        ndk_sys::AInputQueue_detachLooper(state.input);
                    }
                    if input.is_null() == false {
                        ndk_sys::AInputQueue_attachLooper(
                            input,
                            state.looper,
                            ndk_sys::ALOOPER_POLL_CALLBACK,
                            Some(android_input_cb),
                            /* data */ null_mut(),
                        );
                    }
                }
                state.input = input;
            });

            notify_main_thread();
        }
        AndroidMessage::SetNativeWindow(window) => {
            UI_THREAD_DATA.with(|data| {
                let mut data = &mut *data.borrow_mut();

                if data.android_state.window != window {
                    if data.android_state.window.is_null() == false {
                        cleanup_egl_surface(&mut data.android_state);
                    }
                }

                if window.is_null() == false {
                    console_info(b"Creating egl surface\0".as_ptr() as _);
                    if init_egl_surface(&mut data.android_state, window) {
                        console_info(b"... ok!\0".as_ptr() as _);
                        android_update_dimensions(data, window, true);
                    } else {
                        console_info(b"... failed!\0".as_ptr() as _);
                        //_sapp_android_shutdown();
                    }
                }
                data.android_state.window = window;
            });
            notify_main_thread();
        }
        AndroidMessage::Destroy => {
            UI_THREAD_DATA.with(|data| {
                let mut data = data.borrow_mut();
                data.android_state.cleanup();
                data.is_thread_stopping = true;
            });
            notify_main_thread();
        }
    }

    1
}

unsafe fn android_touch_event(data: &mut UiThreadData, e: *const ndk_sys::AInputEvent) -> bool {
    if ndk_sys::AInputEvent_getType(e) != ndk_sys::AINPUT_EVENT_TYPE_MOTION as _ {
        return false;
    }

    let action_idx = ndk_sys::AMotionEvent_getAction(e);
    let action = action_idx & ndk_sys::AMOTION_EVENT_ACTION_MASK as i32;
    let type_ = match action as u32 {
        ndk_sys::AMOTION_EVENT_ACTION_DOWN => sapp_event_type_SAPP_EVENTTYPE_TOUCHES_BEGAN,
        ndk_sys::AMOTION_EVENT_ACTION_POINTER_DOWN => sapp_event_type_SAPP_EVENTTYPE_TOUCHES_BEGAN,
        ndk_sys::AMOTION_EVENT_ACTION_MOVE => sapp_event_type_SAPP_EVENTTYPE_TOUCHES_MOVED,
        ndk_sys::AMOTION_EVENT_ACTION_UP | ndk_sys::AMOTION_EVENT_ACTION_POINTER_UP => {
            sapp_event_type_SAPP_EVENTTYPE_TOUCHES_ENDED
        }
        ndk_sys::AMOTION_EVENT_ACTION_CANCEL => sapp_event_type_SAPP_EVENTTYPE_TOUCHES_CANCELLED,
        _ => {
            return false;
        }
    };
    let idx = action_idx >> ndk_sys::AMOTION_EVENT_ACTION_POINTER_INDEX_SHIFT;
    let mut event = data.init_event(type_);
    event.num_touches = ndk_sys::AMotionEvent_getPointerCount(e) as _;
    if event.num_touches > SAPP_MAX_TOUCHPOINTS as _ {
        event.num_touches = SAPP_MAX_TOUCHPOINTS as _;
    }
    let shared_state = read_shared_state();
    for i in 0..event.num_touches {
        let mut dst = &mut event.touches[i as usize];
        dst.identifier = ndk_sys::AMotionEvent_getPointerId(e, i as _) as _;
        dst.pos_x = (ndk_sys::AMotionEvent_getX(e, i as _) / shared_state.window_width as f32)
            * shared_state.framebuffer_width as f32;
        dst.pos_y = (ndk_sys::AMotionEvent_getY(e, i as _) / shared_state.window_height as f32)
            * shared_state.framebuffer_height as f32;

        if action == ndk_sys::AMOTION_EVENT_ACTION_POINTER_DOWN as _
            || action == ndk_sys::AMOTION_EVENT_ACTION_POINTER_UP as _
        {
            dst.changed = i == idx;
        } else {
            dst.changed = true;
        }
    }
    data.call_event(event);

    true
}

unsafe extern "C" fn android_input_cb(
    _fd: RawFd,
    events: i32,
    _data: *mut std::ffi::c_void,
) -> i32 {
    if events as u32 & ndk_sys::ALOOPER_EVENT_INPUT == 0 {
        console_error(b"_sapp_android_input_cb() encountered unsupported event\0".as_ptr() as _);
        return 1;
    }
    let mut event: *mut ndk_sys::AInputEvent = null_mut();

    UI_THREAD_DATA.with(|data| {
        let data = &mut *data.borrow_mut();
        let input = data.android_state.input;

        while ndk_sys::AInputQueue_getEvent(input, &mut event) >= 0 {
            if ndk_sys::AInputQueue_preDispatchEvent(input, event) != 0 {
                continue;
            }
            let mut handled = 0;
            if android_touch_event(data, event) {
                handled = 1;
            }
            ndk_sys::AInputQueue_finishEvent(input, event, handled);
        }
    });
    1
}

unsafe fn android_msg(msg: AndroidMessage) {
    let size = std::mem::size_of::<AndroidMessage>();
    let write_from_main_fd = MAIN_THREAD_DATA.with(|data| data.borrow().write_from_main_fd);
    let res = libc::write(write_from_main_fd, &msg as *const _ as *const _, size);
    assert_eq!(res, size as _);
}

unsafe extern "C" fn on_start(_activity: *mut ANativeActivity) {
    console_info(b"NativeActivity onStart()\0".as_ptr() as _);
}

unsafe extern "C" fn on_resume(activity: *mut ANativeActivity) {
    console_info(b"NativeActivity onResume()\0".as_ptr() as _);
    android_msg(AndroidMessage::Resume);
    auto_hide_nav_bar(activity);
}

unsafe extern "C" fn on_save_instance_state(
    _activity: *mut ANativeActivity,
    out_size: *mut ndk_sys::size_t,
) -> *mut std::ffi::c_void {
    console_info(b"NativeActivity onSaveInstanceState()\0".as_ptr() as _);
    *out_size = 0;
    null_mut()
}

unsafe extern "C" fn on_window_focus_changed(_activity: *mut ANativeActivity, has_focus: i32) {
    console_info(b"NativeActivity onFocusChange()\0".as_ptr() as _);
    if has_focus != 0 {
        android_msg(AndroidMessage::Focus);
    } else {
        android_msg(AndroidMessage::NoFocus);
    }
}

unsafe extern "C" fn on_pause(_activity: *mut ANativeActivity) {
    console_info(b"NativeActivity onPause()\0".as_ptr() as _);
    android_msg(AndroidMessage::Pause);
}

unsafe extern "C" fn on_stop(_activity: *mut ANativeActivity) {
    console_info(b"NativeActivity onStop()\0".as_ptr() as _);
}

unsafe extern "C" fn on_destroy(_activity: *mut ANativeActivity) {
    // For some reason even an empty app using nativeactivity.h will crash (WIN DEATH)
    // on my device (Moto X 2nd gen) when the app is removed from the task view
    // (TaskStackView: onTaskViewDismissed).

    // However, if ANativeActivity_finish() is explicitly called from for example
    // _sapp_android_on_stop(), the crash disappears. Is this a bug in NativeActivity?
    android_msg(AndroidMessage::Destroy);
    wait_ui_thread();

    MAIN_THREAD_DATA.with(|data| {
        let data = data.borrow_mut();
        libc::close(data.read_from_main_fd);
        libc::close(data.write_from_main_fd);
        console_info(b"NativeActivity done\0".as_ptr() as _);
    });

    // this is a bit naughty, but causes a clean restart of the app (static globals are reset)
    libc::exit(0);
}

unsafe extern "C" fn on_native_window_created(
    _activity: *mut ANativeActivity,
    window: *mut ANativeWindow,
) {
    console_info(b"NativeActivity onNativeWindowCreated()\0".as_ptr() as _);
    android_msg(AndroidMessage::SetNativeWindow(window));
    wait_ui_thread();
}

unsafe extern "C" fn on_native_window_destroyed(
    _activity: *mut ANativeActivity,
    _window: *mut ANativeWindow,
) {
    console_info(b"NativeActivity onNativeWindowDestroyed()\0".as_ptr() as _);
    android_msg(AndroidMessage::SetNativeWindow(null_mut()));
    wait_ui_thread();
}

unsafe extern "C" fn on_input_queue_created(
    _activity: *mut ANativeActivity,
    queue: *mut AInputQueue,
) {
    console_info(b"NativeActivity onInputQueueCreated()\0".as_ptr() as _);

    android_msg(AndroidMessage::SetInputQueue(queue));
    wait_ui_thread();
}

unsafe extern "C" fn on_input_queue_destroyed(
    _activity: *mut ANativeActivity,
    _queue: *mut AInputQueue,
) {
    console_info(b"NativeActivity onInputQueueDestroyed()\0".as_ptr() as _);
    android_msg(AndroidMessage::SetInputQueue(null_mut()));
    wait_ui_thread();
}

unsafe extern "C" fn on_config_changed(_activity: *mut ANativeActivity) {
    console_info(b"NativeActivity onConfigChanged()\0".as_ptr() as _);
}

unsafe extern "C" fn on_low_memory(_activity: *mut ANativeActivity) {
    console_info(b"NativeActivity onLowMemory()\0".as_ptr() as _);
}

pub unsafe fn console_debug(msg: *const ::std::os::raw::c_char) {
    ndk_sys::__android_log_write(
        ndk_sys::android_LogPriority_ANDROID_LOG_DEBUG as _,
        b"SAPP\0".as_ptr() as _,
        msg,
    );
}

pub unsafe fn console_info(msg: *const ::std::os::raw::c_char) {
    ndk_sys::__android_log_write(
        ndk_sys::android_LogPriority_ANDROID_LOG_INFO as _,
        b"SAPP\0".as_ptr() as _,
        msg,
    );
}

pub unsafe fn console_warn(msg: *const ::std::os::raw::c_char) {
    ndk_sys::__android_log_write(
        ndk_sys::android_LogPriority_ANDROID_LOG_WARN as _,
        b"SAPP\0".as_ptr() as _,
        msg,
    );
}

pub unsafe fn console_error(msg: *const ::std::os::raw::c_char) {
    ndk_sys::__android_log_write(
        ndk_sys::android_LogPriority_ANDROID_LOG_ERROR as _,
        b"SAPP\0".as_ptr() as _,
        msg,
    );
}

unsafe fn auto_hide_nav_bar(activity: *mut ANativeActivity) {
    console_info(b"auto_hide_nav_bar: Start\0".as_ptr() as _);

    let mut env: *mut ndk_sys::JNIEnv = std::ptr::null_mut();
    let java_vm: *mut ndk_sys::JavaVM = (*activity).vm;

    let attach_current_thread = (**java_vm).AttachCurrentThread.unwrap();
    //let detach_current_thread = (**java_vm).DetachCurrentThread.unwrap();
    let res = attach_current_thread(java_vm, &mut env, std::ptr::null_mut());
    assert!(res == 0);

    console_info(b"auto_hide_nav_bar: Current thread attached\0".as_ptr() as _);

    let find_class = (**env).FindClass.unwrap();
    let get_method_id = (**env).GetMethodID.unwrap();
    let call_object_method = (**env).CallObjectMethod.unwrap();
    let get_static_field_id = (**env).GetStaticFieldID.unwrap();
    let get_static_int_field = (**env).GetStaticIntField.unwrap();
    let call_void_method = (**env).CallVoidMethod.unwrap();

    let activity_class = find_class(env, b"android/app/NativeActivity\0".as_ptr() as _);

    console_info(b"auto_hide_nav_bar: Got activity class\0".as_ptr() as _);

    let get_window = get_method_id(
        env,
        activity_class,
        b"getWindow\0".as_ptr() as _,
        b"()Landroid/view/Window;\0".as_ptr() as _,
    );

    let window_class = find_class(env, b"android/view/Window\0".as_ptr() as _);
    let get_decor_view = get_method_id(
        env,
        window_class,
        b"getDecorView\0".as_ptr() as _,
        b"()Landroid/view/View;\0".as_ptr() as _,
    );

    let view_class = find_class(env, b"android/view/View\0".as_ptr() as _);
    let set_system_ui_visibility = get_method_id(
        env,
        view_class,
        b"setSystemUiVisibility\0".as_ptr() as _,
        b"(I)V\0".as_ptr() as _,
    );

    console_info(b"auto_hide_nav_bar: Got set_system_ui_visibility\0".as_ptr() as _);

    let window = call_object_method(env, (*activity).clazz, get_window);
    let decor_view = call_object_method(env, window, get_decor_view);

    let flag_fullscreen_id = get_static_field_id(
        env,
        view_class,
        b"SYSTEM_UI_FLAG_FULLSCREEN\0".as_ptr() as _,
        b"I\0".as_ptr() as _,
    );
    let flag_hide_navigation_id = get_static_field_id(
        env,
        view_class,
        b"SYSTEM_UI_FLAG_HIDE_NAVIGATION\0".as_ptr() as _,
        b"I\0".as_ptr() as _,
    );
    let flag_immersive_sticky_id = get_static_field_id(
        env,
        view_class,
        b"SYSTEM_UI_FLAG_IMMERSIVE_STICKY\0".as_ptr() as _,
        b"I\0".as_ptr() as _,
    );

    console_info(b"auto_hide_nav_bar: Got flags\0".as_ptr() as _);

    let flag_fullscreen = get_static_int_field(env, view_class, flag_fullscreen_id);
    let flag_hide_navigation = get_static_int_field(env, view_class, flag_hide_navigation_id);
    let flag_immersive_sticky = get_static_int_field(env, view_class, flag_immersive_sticky_id);

    let flag = flag_fullscreen | flag_hide_navigation | flag_immersive_sticky;

    call_void_method(env, decor_view, set_system_ui_visibility, flag);

    // detach_current_thread(java_vm);

    console_info(b"auto_hide_nav_bar: Nav bar should be hidden!\0".as_ptr() as _);
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct android_asset {
    pub content: *mut ::std::os::raw::c_char,
    pub content_length: ::std::os::raw::c_int,
}

pub unsafe fn sapp_load_asset(filepath: *const ::std::os::raw::c_char, out: *mut android_asset) {
    let mgr = (*lock_shared_state().activity).assetManager;
    let asset = ndk_sys::AAssetManager_open(mgr, filepath, ndk_sys::AASSET_MODE_BUFFER as _);
    if asset.is_null() {
        return;
    }
    let length = ndk_sys::AAsset_getLength64(asset);
    // TODO: memory leak right here! this buffer would never freed
    let buffer = libc::malloc(length as _);
    if ndk_sys::AAsset_read(asset, buffer, length as _) > 0 {
        ndk_sys::AAsset_close(asset);

        (*out).content_length = length as _;
        (*out).content = buffer as _;
    }
}

pub unsafe fn sapp_is_elapsed_timer_supported() -> bool {
    return false;
}
