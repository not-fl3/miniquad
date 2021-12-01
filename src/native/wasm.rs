#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]

pub mod fs;
pub mod webgl;

mod keycodes;

pub use webgl::*;

use std::{cell::RefCell, path::PathBuf, thread_local};

use crate::{event::EventHandler, native::NativeDisplay, Context, GraphicsContext};

#[derive(Default)]
struct DroppedFiles {
    paths: Vec<PathBuf>,
    bytes: Vec<Vec<u8>>,
}

struct WasmDisplay {
    clipboard: Option<String>,
    screen_width: f32,
    screen_height: f32,
    dropped_files: DroppedFiles,
}

impl NativeDisplay for WasmDisplay {
    fn screen_size(&self) -> (f32, f32) {
        (self.screen_width as _, self.screen_height as _)
    }
    fn dpi_scale(&self) -> f32 {
        1.
    }
    fn high_dpi(&self) -> bool {
        true
    }
    fn order_quit(&mut self) {
        // there is no escape from wasm
    }
    fn request_quit(&mut self) {
        // there is no escape from wasm
    }
    fn cancel_quit(&mut self) {
        // there is no escape from wasm
    }
    fn set_cursor_grab(&mut self, grab: bool) {
        unsafe { sapp_set_cursor_grab(grab) };
    }
    fn show_mouse(&mut self, shown: bool) {
        unsafe { show_mouse(shown) };
    }
    fn set_mouse_cursor(&mut self, cursor: crate::CursorIcon) {
        unsafe {
            set_mouse_cursor(cursor);
        }
    }
    fn set_window_size(&mut self, _new_width: u32, _new_height: u32) {}
    fn set_fullscreen(&mut self, fullscreen: bool) {
        unsafe {
            sapp_set_fullscreen(fullscreen);
        }
    }
    fn clipboard_get(&mut self) -> Option<String> {
        clipboard_get()
    }
    fn clipboard_set(&mut self, data: &str) {
        clipboard_set(data)
    }
    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn dropped_file_count(&mut self) -> usize {
        self.dropped_files.bytes.len()
    }
    fn dropped_file_bytes(&mut self, index: usize) -> Option<Vec<u8>> {
        self.dropped_files.bytes.get(index).cloned()
    }
    fn dropped_file_path(&mut self, index: usize) -> Option<PathBuf> {
        self.dropped_files.paths.get(index).cloned()
    }
}

struct WasmGlobals {
    event_handler: Box<dyn EventHandler>,
    context: GraphicsContext,
    display: WasmDisplay,
}

thread_local! {
    static GLOBALS: RefCell<Option<WasmGlobals>> = RefCell::new(None);
}

fn with<T, F: FnOnce(&mut WasmGlobals) -> T>(f: F) -> T {
    GLOBALS.with(|globals| {
        let mut globals = globals.borrow_mut();
        let globals = globals.as_mut().unwrap();
        f(globals)
    })
}

static mut cursor_icon: crate::CursorIcon = crate::CursorIcon::Default;
static mut cursor_shown: bool = true;

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
    F: 'static + FnOnce(&mut crate::Context) -> Box<dyn EventHandler>,
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

    // run user intialisation code
    let mut context = crate::GraphicsContext::new();

    GLOBALS.with(|g| {
        let mut display = WasmDisplay {
            clipboard: None,
            screen_width: unsafe { canvas_width() as _ },
            screen_height: unsafe { canvas_height() as _ },
            dropped_files: Default::default(),
        };
        *g.borrow_mut() = Some(WasmGlobals {
            event_handler: f(&mut Context::new(&mut context, &mut display)),
            context,
            display,
        });
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
    if shown != cursor_shown {
        cursor_shown = shown;
        update_cursor();
    }
}

unsafe fn set_mouse_cursor(icon: crate::CursorIcon) {
    if cursor_icon != icon {
        cursor_icon = icon;
        if cursor_shown {
            update_cursor();
        }
    }
}

pub unsafe fn update_cursor() {
    let css_name = if !cursor_shown {
        "none"
    } else {
        match cursor_icon {
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

#[no_mangle]
pub extern "C" fn on_clipboard_paste(msg: *mut u8, len: usize) {
    let msg = unsafe { String::from_raw_parts(msg, len, len) };

    with(move |globals| globals.display.clipboard = Some(msg));
}

pub fn clipboard_get() -> Option<String> {
    with(|globals| globals.display.clipboard.clone())
}

pub fn clipboard_set(data: &str) {
    let len = data.len();
    let data = std::ffi::CString::new(data).unwrap();
    unsafe { sapp_set_clipboard(data.as_ptr(), len) };
}

#[no_mangle]
pub extern "C" fn frame() {
    with(|globals| {
        globals.event_handler.update(&mut Context::new(
            &mut globals.context,
            &mut globals.display,
        ));
        globals.event_handler.draw(&mut Context::new(
            &mut globals.context,
            &mut globals.display,
        ));
    });
}

#[no_mangle]
pub extern "C" fn mouse_move(x: i32, y: i32) {
    with(|globals| {
        globals.event_handler.mouse_motion_event(
            &mut Context::new(&mut globals.context, &mut globals.display),
            x as _,
            y as _,
        );
    });
}

#[no_mangle]
pub extern "C" fn raw_mouse_move(dx: i32, dy: i32) {
    with(|globals| {
        globals.event_handler.raw_mouse_motion(
            &mut Context::new(&mut globals.context, &mut globals.display),
            dx as _,
            dy as _,
        );
    });
}

#[no_mangle]
pub extern "C" fn mouse_down(x: i32, y: i32, btn: i32) {
    let btn = keycodes::translate_mouse_button(btn);

    with(|globals| {
        globals.event_handler.mouse_button_down_event(
            &mut Context::new(&mut globals.context, &mut globals.display),
            btn,
            x as _,
            y as _,
        );
    });
}

#[no_mangle]
pub extern "C" fn mouse_up(x: i32, y: i32, btn: i32) {
    let btn = keycodes::translate_mouse_button(btn);

    with(|globals| {
        globals.event_handler.mouse_button_up_event(
            &mut Context::new(&mut globals.context, &mut globals.display),
            btn,
            x as _,
            y as _,
        );
    });
}

#[no_mangle]
pub extern "C" fn mouse_wheel(dx: i32, dy: i32) {
    with(|globals| {
        globals.event_handler.mouse_wheel_event(
            &mut Context::new(&mut globals.context, &mut globals.display),
            dx as _,
            dy as _,
        );
    });
}

#[no_mangle]
pub extern "C" fn key_down(key: u32, modifiers: u32, repeat: bool) {
    let key = keycodes::translate_keycode(key as _);
    let mods = keycodes::translate_mod(modifiers as _);

    with(|globals| {
        globals.event_handler.key_down_event(
            &mut Context::new(&mut globals.context, &mut globals.display),
            key,
            mods,
            repeat,
        );
    });
}

#[no_mangle]
pub extern "C" fn key_press(key: u32) {
    if let Some(key) = char::from_u32(key) {
        with(|globals| {
            globals.event_handler.char_event(
                &mut Context::new(&mut globals.context, &mut globals.display),
                key,
                crate::KeyMods::default(),
                false,
            );
        });
    }
}

#[no_mangle]
pub extern "C" fn key_up(key: u32, modifiers: u32) {
    let key = keycodes::translate_keycode(key as _);
    let mods = keycodes::translate_mod(modifiers as _);

    with(|globals| {
        globals.event_handler.key_up_event(
            &mut Context::new(&mut globals.context, &mut globals.display),
            key,
            mods,
        );
    });
}

#[no_mangle]
pub extern "C" fn resize(width: i32, height: i32) {
    with(|globals| {
        globals.display.screen_width = width as _;
        globals.display.screen_height = height as _;

        globals.event_handler.resize_event(
            &mut Context::new(&mut globals.context, &mut globals.display),
            width as _,
            height as _,
        );
    });
}

#[no_mangle]
pub extern "C" fn touch(phase: u32, id: u32, x: f32, y: f32) {
    let phase = keycodes::translate_touch_phase(phase as _);
    with(|globals| {
        globals.event_handler.touch_event(
            &mut Context::new(&mut globals.context, &mut globals.display),
            phase,
            id as _,
            x as _,
            y as _,
        );
    });
}

#[no_mangle]
pub extern "C" fn on_files_dropped_start() {
    with(|globals| {
        globals.display.dropped_files = Default::default();
    });
}

#[no_mangle]
pub extern "C" fn on_files_dropped_finish() {
    with(|globals| {
        globals.event_handler.files_dropped_event(&mut Context::new(
            &mut globals.context,
            &mut globals.display,
        ))
    });
}

#[no_mangle]
pub extern "C" fn on_file_dropped(
    path: *mut u8,
    path_len: usize,
    bytes: *mut u8,
    bytes_len: usize,
) {
    with(|globals| {
        let path = PathBuf::from(unsafe { String::from_raw_parts(path, path_len, path_len) });
        let bytes = unsafe { Vec::from_raw_parts(bytes, bytes_len, bytes_len) };

        globals.display.dropped_files.paths.push(path);
        globals.display.dropped_files.bytes.push(bytes);
    });
}
