pub mod fs;
pub mod webgl;

mod keycodes;

use wasm_bindgen::{closure, JsCast};
use web_sys::*;
pub use webgl::*;

use std::{
    cell::{OnceCell, RefCell},
    path::PathBuf,
    rc::Rc,
    sync::{mpsc::Receiver, Mutex, OnceLock},
    thread_local,
};

use crate::{
    event::EventHandler,
    native::{NativeDisplayData, Request},
};

// get's an element using document.query_selector
fn get_element<T: JsCast>(id: &'static str) -> T {
    document()
        .get_element_by_id(id)
        .unwrap()
        .dyn_into::<T>()
        .unwrap()
}

fn document() -> web_sys::Document {
    web_sys::window().unwrap().document().unwrap()
}

fn get_dpi_scale(high_dpi: bool) -> f64 {
    if high_dpi {
        web_sys::window()
            .map(|w| w.device_pixel_ratio())
            .unwrap_or(1.0)
    } else {
        1.0
    }
}

static mut EVENT_HANDLER: Option<*mut dyn EventHandler> = None;

// SAFETY: Can't have a data race in a single threaded environment, wasm is single threaded
fn get_event_handler() -> &'static mut dyn EventHandler {
    unsafe { &mut *EVENT_HANDLER.unwrap() }
}

pub fn run<F>(conf: &crate::conf::Conf, f: F)
where
    F: 'static + FnOnce() -> Box<dyn EventHandler>,
{
    // setup panic hook
    console_error_panic_hook::set_once();

    // initialize main canvas
    let main_canvas = get_element::<HtmlCanvasElement>(conf.platform.web_canvas_query_selector);
    let high_dpi = conf.high_dpi;
    let dpi = get_dpi_scale(high_dpi) as i32;

    let display_width = main_canvas.client_width() * dpi;
    let display_height = main_canvas.client_height() * dpi;

    main_canvas.set_width(display_width as u32);
    main_canvas.set_height(display_height as u32);

    // setup requests channel
    let (tx, rx) = std::sync::mpsc::channel();

    // setup display
    let mut display = NativeDisplayData::new(
        main_canvas.width() as _,
        main_canvas.height() as _,
        tx,
        Box::new(Clipboard),
    );
    crate::set_display(display);

    // setup event handler
    unsafe {
        let event_handler = Box::leak(f());
        EVENT_HANDLER = Some(event_handler)
    };

    // setup event listeners
    init_mouse_move_events(&main_canvas);

    // run event loop
    event_loop(main_canvas, rx);
}

#[no_mangle]
pub extern "C" fn allocate_vec_u8(len: usize) -> *mut u8 {
    let mut string = vec![0u8; len];
    let ptr = string.as_mut_ptr();
    string.leak();
    ptr
}

struct Clipboard;

impl crate::native::Clipboard for Clipboard {
    fn get(&mut self) -> Option<String> {
        let navigator = window()?.navigator();
        let clipboard = navigator.clipboard()?;
        let promise = clipboard.read_text();
        let future = wasm_bindgen_futures::JsFuture::from(promise);
        let result = pollster::block_on(future).unwrap();
        result.as_string()
    }

    fn set(&mut self, data: &str) {
        let navigator = window().unwrap().navigator();
        if let Some(clipboard) = navigator.clipboard() {
            let promise = clipboard.write_text(data);
            let future = wasm_bindgen_futures::JsFuture::from(promise);
            let _ = pollster::block_on(future).unwrap();
        }
    }
}

fn event_loop(main_canvas: web_sys::HtmlCanvasElement, rx: Receiver<Request>) {
    static mut LAST_CURSOR_CSS: &'static str = "default";
    let event_handler = get_event_handler();

    // process requests
    while let Ok(request) = rx.try_recv() {
        match request {
            Request::SetCursorGrab(grab) => {
                if grab {
                    main_canvas.request_pointer_lock();
                } else {
                    document().exit_pointer_lock();
                }
            }
            Request::ShowMouse(show) => unsafe {
                if !show {
                    main_canvas.style().set_property("cursor", "none").unwrap();
                } else {
                    main_canvas
                        .style()
                        .set_property("cursor", LAST_CURSOR_CSS)
                        .unwrap();
                }
            },
            Request::SetMouseCursor(cursor) => unsafe {
                let css_text = match cursor {
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
                };

                LAST_CURSOR_CSS = css_text;
                main_canvas
                    .style()
                    .set_property("cursor", LAST_CURSOR_CSS)
                    .unwrap();
            },
            Request::SetFullscreen(fullscreen) => {
                if fullscreen {
                    main_canvas.request_fullscreen();
                } else {
                    document().exit_fullscreen();
                }
            }
            Request::SetWindowSize {
                new_width,
                new_height,
            } => {
                let dpi = get_dpi_scale(false) as i32;
                main_canvas.set_width(new_width * dpi as u32);
                main_canvas.set_height(new_height * dpi as u32);

                {
                    let mut d = crate::native_display().lock().unwrap();
                    d.screen_width = new_width as _;
                    d.screen_height = new_width as _;
                }

                // emit resize event
                event_handler.resize_event(new_width as _, new_width as _);
            }
            _ => {}
        }
    }

    // drive event handler implementation
    event_handler.update();
    event_handler.draw();

    // in the words of Dj Khaled, another one!
    let closure = Box::new(move || event_loop(main_canvas, rx));
    let next = closure::Closure::once(closure);
    let fn_ref = next.as_ref().unchecked_ref();
    web_sys::window().unwrap().request_animation_frame(fn_ref);
}

fn init_mouse_move_events(canvas: &HtmlCanvasElement) {
    let closure: closure::Closure<dyn Fn(MouseEvent)> =
        closure::Closure::new(move |ev: MouseEvent| {
            let canvas = ev
                .target()
                .unwrap()
                .dyn_into::<HtmlCanvasElement>()
                .unwrap();
            let rect = canvas.get_bounding_client_rect();
            let event_handler = get_event_handler();

            let x = ev.client_x() as f32 - rect.left() as f32;
            let y = ev.client_y() as f32 - rect.top() as f32;

            event_handler.mouse_motion_event(x, y);
            event_handler.raw_mouse_motion(ev.movement_x() as _, ev.movement_y() as _);
        });

    let fn_ref = closure.as_ref().unchecked_ref();
    canvas.add_event_listener_with_callback("mousemove", fn_ref);
}

fn init_mouse_down_events(canvas: &HtmlCanvasElement) {
    let closure: closure::Closure<dyn Fn(MouseEvent)> =
        closure::Closure::new(move |ev: MouseEvent| {
            let canvas = ev
                .target()
                .unwrap()
                .dyn_into::<HtmlCanvasElement>()
                .unwrap();
            let rect = canvas.get_bounding_client_rect();
            let event_handler = get_event_handler();

            let x = ev.client_x() as f32 - rect.left() as f32;
            let y = ev.client_y() as f32 - rect.top() as f32;

            let button = match ev.button() {
                0 => crate::MouseButton::Left,
                1 => crate::MouseButton::Right,
                2 => crate::MouseButton::Middle,
                n => crate::MouseButton::Other(n as _),
            };
        });

    let fn_ref = closure.as_ref().unchecked_ref();
    canvas.add_event_listener_with_callback("mousedown", fn_ref);
}

#[no_mangle]
pub extern "C" fn mouse_down(x: i32, y: i32, btn: i32) {
    get_event_handler().mouse_button_down_event(
        keycodes::translate_mouse_button(btn),
        x as _,
        y as _,
    );
}

#[no_mangle]
pub extern "C" fn mouse_up(x: i32, y: i32, btn: i32) {
    get_event_handler().mouse_button_up_event(
        keycodes::translate_mouse_button(btn),
        x as _,
        y as _,
    );
}

#[no_mangle]
pub extern "C" fn mouse_wheel(dx: i32, dy: i32) {
    get_event_handler().mouse_wheel_event(dx as _, dy as _);
}

#[no_mangle]
pub extern "C" fn key_down(key: u32, modifiers: u32, repeat: bool) {
    let key = keycodes::translate_keycode(key as _);
    let mods = keycodes::translate_mod(modifiers as _);

    get_event_handler().key_down_event(key, mods, repeat);
}

#[no_mangle]
pub extern "C" fn key_press(key: u32) {
    if let Some(key) = char::from_u32(key) {
        get_event_handler().char_event(key, crate::KeyMods::default(), false);
    }
}

#[no_mangle]
pub extern "C" fn key_up(key: u32, modifiers: u32) {
    let key = keycodes::translate_keycode(key as _);
    let mods = keycodes::translate_mod(modifiers as _);

    get_event_handler().key_up_event(key, mods);
}

#[no_mangle]
pub extern "C" fn touch(phase: u32, id: u32, x: f32, y: f32) {
    let phase = keycodes::translate_touch_phase(phase as _);
    get_event_handler().touch_event(phase, id as _, x as _, y as _);
}

#[no_mangle]
pub extern "C" fn focus(has_focus: bool) {
    if has_focus {
        get_event_handler().window_restored_event();
    } else {
        get_event_handler().window_minimized_event();
    }
}

#[no_mangle]
pub extern "C" fn on_files_dropped_start() {
    let mut d = crate::native_display().lock().unwrap();
    d.dropped_files = Default::default();
}

#[no_mangle]
pub extern "C" fn on_files_dropped_finish() {
    get_event_handler().files_dropped_event()
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
