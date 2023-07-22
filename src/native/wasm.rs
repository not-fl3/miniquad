pub mod fs;
pub mod webgl;

mod keycodes;

pub use webgl::*;

use std::{
    cell::RefCell,
    path::PathBuf,
    sync::{mpsc::Receiver, Mutex, OnceLock},
    thread_local,
};

use crate::{
    event::EventHandler,
    native::{NativeDisplayData, Request},
};

// fn dropped_file_count(&mut self) -> usize {
//     self.dropped_files.bytes.len()
// }
// fn dropped_file_bytes(&mut self, index: usize) -> Option<Vec<u8>> {
//     self.dropped_files.bytes.get(index).cloned()
// }
// fn dropped_file_path(&mut self, index: usize) -> Option<PathBuf> {
//     self.dropped_files.paths.get(index).cloned()
// }

thread_local! {
    static EVENT_HANDLER: RefCell<Option<Box<dyn EventHandler>>> = RefCell::new(None);
    static REQUESTS: RefCell<Option<Receiver<Request>>> = RefCell::new(None);
}
fn tl_event_handler<T, F: FnOnce(&mut dyn EventHandler) -> T>(f: F) -> T {
    EVENT_HANDLER.with(|globals| {
        let mut globals = globals.borrow_mut();
        let globals: &mut Box<dyn EventHandler> = globals.as_mut().unwrap();
        f(&mut **globals)
    })
}

static mut CURSOR_ICON: crate::CursorIcon = crate::CursorIcon::Default;
static mut CURSOR_SHOW: bool = true;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sapp_touchpoint {
    pub identifier: usize,
    pub pos_x: f32,
    pub pos_y: f32,
    pub changed: bool,
}

pub fn run<F>(conf: &crate::conf::Conf, f: F)
where
    F: 'static + FnOnce() -> Box<dyn EventHandler>,
{
    {
        use std::ffi::CString;
        use std::panic;

        panic::set_hook(Box::new(|info| {
            let msg = CString::new(format!("{:?}", info)).unwrap_or_else(|_| {
                CString::new(format!("MALFORMED ERROR MESSAGE {:?}", info.location())).unwrap()
            });
            unsafe { console_log(msg.as_ptr()) };
        }));
    }

    // setup initial canvas size
    unsafe {
        setup_canvas_size(conf.high_dpi);
    }

    let (tx, rx) = std::sync::mpsc::channel();
    REQUESTS.with(|r| *r.borrow_mut() = Some(rx));
    let w = unsafe { canvas_width() as _ };
    let h = unsafe { canvas_height() as _ };
    let clipboard = Box::new(Clipboard);
    crate::set_display(NativeDisplayData {
        ..NativeDisplayData::new(w, h, tx, clipboard)
    });
    EVENT_HANDLER.with(|g| {
        *g.borrow_mut() = Some(f());
    });

    // start requestAnimationFrame loop
    unsafe {
        run_animation_loop();
    }
}

pub unsafe fn sapp_width() -> ::std::os::raw::c_int {
    canvas_width()
}

pub unsafe fn sapp_height() -> ::std::os::raw::c_int {
    canvas_height()
}

extern "C" {
    pub fn setup_canvas_size(high_dpi: bool);
    pub fn run_animation_loop();
    pub fn canvas_width() -> i32;
    pub fn canvas_height() -> i32;
    pub fn dpi_scale() -> f32;
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

    pub fn sapp_set_cursor(cursor: *const u8, len: usize);

    pub fn sapp_is_elapsed_timer_supported() -> bool;

    pub fn sapp_set_fullscreen(fullscreen: bool);
    pub fn sapp_is_fullscreen() -> bool;
    pub fn sapp_set_window_size(new_width: u32, new_height: u32);

    pub fn now() -> f64;
}

unsafe fn show_mouse(shown: bool) {
    if shown != CURSOR_SHOW {
        CURSOR_SHOW = shown;
        update_cursor();
    }
}

unsafe fn set_mouse_cursor(icon: crate::CursorIcon) {
    if CURSOR_ICON != icon {
        CURSOR_ICON = icon;
        if CURSOR_SHOW {
            update_cursor();
        }
    }
}

pub unsafe fn update_cursor() {
    let css_name = if !CURSOR_SHOW {
        "none"
    } else {
        match CURSOR_ICON {
            crate::CursorIcon::Default => "default",
            crate::CursorIcon::Help => "help",
            crate::CursorIcon::Pointer => "pointer",
            crate::CursorIcon::Wait => "wait",
            crate::CursorIcon::Crosshair => "crosshair",
            crate::CursorIcon::Text => "text",
            crate::CursorIcon::Move => "move",
            crate::CursorIcon::NotAllowed => "not-allowed",
            crate::CursorIcon::EWResize => "ew-resize",
            crate::CursorIcon::NSResize => "ns-resize",
            crate::CursorIcon::NESWResize => "nesw-resize",
            crate::CursorIcon::NWSEResize => "nwse-resize",
        }
    };
    sapp_set_cursor(css_name.as_ptr(), css_name.len());
}

#[no_mangle]
pub extern "C" fn crate_version() -> u32 {
    let major = env!("CARGO_PKG_VERSION_MAJOR").parse::<u32>().unwrap();
    let minor = env!("CARGO_PKG_VERSION_MINOR").parse::<u32>().unwrap();
    let patch = env!("CARGO_PKG_VERSION_PATCH").parse::<u32>().unwrap();

    (major << 24) + (minor << 16) + patch
}

#[no_mangle]
pub extern "C" fn allocate_vec_u8(len: usize) -> *mut u8 {
    let mut string = vec![0u8; len];
    let ptr = string.as_mut_ptr();
    string.leak();
    ptr
}

static CLIPBOARD: OnceLock<Mutex<Option<String>>> = OnceLock::new();
struct Clipboard;
impl crate::native::Clipboard for Clipboard {
    fn get(&mut self) -> Option<String> {
        CLIPBOARD
            .get_or_init(|| Mutex::new(None))
            .lock()
            .unwrap()
            .clone()
    }

    fn set(&mut self, data: &str) {
        let len = data.len();
        let data = std::ffi::CString::new(data).unwrap();
        unsafe { sapp_set_clipboard(data.as_ptr(), len) };
    }
}

#[no_mangle]
pub extern "C" fn on_clipboard_paste(msg: *mut u8, len: usize) {
    let msg = unsafe { String::from_raw_parts(msg, len, len) };

    *CLIPBOARD.get_or_init(|| Mutex::new(None)).lock().unwrap() = Some(msg);
}

#[no_mangle]
pub extern "C" fn frame() {
    REQUESTS.with(|r| {
        while let Ok(request) = r.borrow_mut().as_mut().unwrap().try_recv() {
            use Request::*;
            match request {
                Request::SetCursorGrab(grab) => unsafe { sapp_set_cursor_grab(grab) },
                Request::ShowMouse(show) => unsafe { show_mouse(show) },
                Request::SetMouseCursor(cursor) => unsafe {
                    set_mouse_cursor(cursor);
                },
                Request::SetFullscreen(fullscreen) => unsafe {
                    sapp_set_fullscreen(fullscreen);
                },
                _ => {}
            }
        }
    });
    tl_event_handler(|event_handler| {
        event_handler.update();
        event_handler.draw();
    });
}

#[no_mangle]
pub extern "C" fn mouse_move(x: i32, y: i32) {
    tl_event_handler(|event_handler| {
        event_handler.mouse_motion_event(x as _, y as _);
    });
}

#[no_mangle]
pub extern "C" fn raw_mouse_move(dx: i32, dy: i32) {
    tl_event_handler(|event_handler| {
        event_handler.raw_mouse_motion(dx as _, dy as _);
    });
}

#[no_mangle]
pub extern "C" fn mouse_down(x: i32, y: i32, btn: i32) {
    let btn = keycodes::translate_mouse_button(btn);

    tl_event_handler(|event_handler| {
        event_handler.mouse_button_down_event(btn, x as _, y as _);
    });
}

#[no_mangle]
pub extern "C" fn mouse_up(x: i32, y: i32, btn: i32) {
    let btn = keycodes::translate_mouse_button(btn);

    tl_event_handler(|event_handler| {
        event_handler.mouse_button_up_event(btn, x as _, y as _);
    });
}

#[no_mangle]
pub extern "C" fn mouse_wheel(dx: i32, dy: i32) {
    tl_event_handler(|event_handler| {
        event_handler.mouse_wheel_event(dx as _, dy as _);
    });
}

#[no_mangle]
pub extern "C" fn key_down(key: u32, modifiers: u32, repeat: bool) {
    let key = keycodes::translate_keycode(key as _);
    let mods = keycodes::translate_mod(modifiers as _);

    tl_event_handler(|event_handler| {
        event_handler.key_down_event(key, mods, repeat);
    });
}

#[no_mangle]
pub extern "C" fn key_press(key: u32) {
    if let Some(key) = char::from_u32(key) {
        tl_event_handler(|event_handler| {
            event_handler.char_event(key, crate::KeyMods::default(), false);
        });
    }
}

#[no_mangle]
pub extern "C" fn key_up(key: u32, modifiers: u32) {
    let key = keycodes::translate_keycode(key as _);
    let mods = keycodes::translate_mod(modifiers as _);

    tl_event_handler(|event_handler| {
        event_handler.key_up_event(key, mods);
    });
}

#[no_mangle]
pub extern "C" fn resize(width: i32, height: i32) {
    {
        let mut d = crate::native_display().lock().unwrap();
        d.screen_width = width as _;
        d.screen_height = height as _;
    }
    tl_event_handler(|event_handler| {
        event_handler.resize_event(width as _, height as _);
    });
}

#[no_mangle]
pub extern "C" fn touch(phase: u32, id: u32, x: f32, y: f32) {
    let phase = keycodes::translate_touch_phase(phase as _);
    tl_event_handler(|event_handler| {
        event_handler.touch_event(phase, id as _, x as _, y as _);
    });
}

#[no_mangle]
pub extern "C" fn focus(has_focus: bool) {
    tl_event_handler(|event_handler| {
        if has_focus {
            event_handler.window_restored_event();
        } else {
            event_handler.window_minimized_event();
        }
    });
}

#[no_mangle]
pub extern "C" fn on_files_dropped_start() {
    let mut d = crate::native_display().lock().unwrap();
    d.dropped_files = Default::default();
}

#[no_mangle]
pub extern "C" fn on_files_dropped_finish() {
    tl_event_handler(|event_handler| event_handler.files_dropped_event());
}

#[no_mangle]
pub extern "C" fn on_file_dropped(
    path: *mut u8,
    path_len: usize,
    bytes: *mut u8,
    bytes_len: usize,
) {
    let mut d = crate::native_display().lock().unwrap();
    let path = PathBuf::from(unsafe { String::from_raw_parts(path, path_len, path_len) });
    let bytes = unsafe { Vec::from_raw_parts(bytes, bytes_len, bytes_len) };

    d.dropped_files.paths.push(path);
    d.dropped_files.bytes.push(bytes);
}
