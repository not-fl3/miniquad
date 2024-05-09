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

// SAFETY: Can't have a data race in a single threaded environment, wasm is single threaded
fn get_event_handler(swap: Option<*mut dyn EventHandler>) -> &'static mut dyn EventHandler {
    unsafe {
        static mut EVENT_HANDLER: Option<*mut dyn EventHandler> = None;
        EVENT_HANDLER = swap.or(EVENT_HANDLER);
        &mut *EVENT_HANDLER.unwrap()
    }
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

    // initialize canvas
    main_canvas.set_width((conf.window_width * dpi) as u32);
    main_canvas.set_height((conf.window_width * dpi) as u32);
    main_canvas.focus();

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
    let event_handler = Box::leak(f());
    get_event_handler(Some(event_handler));

    // setup event listeners
    init_mouse_events(&main_canvas);
    init_keyboard_events(&main_canvas);
    init_focus_events(&main_canvas);
    init_resize_events(&main_canvas);
    init_touch_events(&main_canvas);

    // run event loop
    event_loop(main_canvas, "default", rx);
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

fn event_loop(
    main_canvas: web_sys::HtmlCanvasElement,
    last_cursor_css: &'static str,
    rx: Receiver<Request>,
) {
    let event_handler = get_event_handler(None);
    let mut next_cursor_css = last_cursor_css;

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
            Request::ShowMouse(show) => {
                if !show {
                    main_canvas.style().set_property("cursor", "none").unwrap();
                } else {
                    main_canvas
                        .style()
                        .set_property("cursor", next_cursor_css)
                        .unwrap();
                }
            }
            Request::SetMouseCursor(cursor) => {
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

                next_cursor_css = css_text;
                main_canvas
                    .style()
                    .set_property("cursor", next_cursor_css)
                    .unwrap();
            }
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
    let closure = Box::new(move || event_loop(main_canvas, next_cursor_css, rx));
    let next = closure::Closure::once(closure);
    let fn_ref = next.as_ref().unchecked_ref();
    web_sys::window().unwrap().request_animation_frame(fn_ref);
}

fn init_mouse_events(canvas: &HtmlCanvasElement) {
    let mouse_move_closure: closure::Closure<dyn Fn(_)> =
        closure::Closure::new(|ev: MouseEvent| {
            let canvas = ev
                .target()
                .unwrap()
                .dyn_into::<HtmlCanvasElement>()
                .unwrap();
            let rect = canvas.get_bounding_client_rect();
            let event_handler = get_event_handler(None);

            let x = ev.client_x() as f32 - rect.left() as f32;
            let y = ev.client_y() as f32 - rect.top() as f32;

            event_handler.mouse_motion_event(x, y);
            event_handler.raw_mouse_motion(ev.movement_x() as _, ev.movement_y() as _);
        });

    let mouse_down_closure: closure::Closure<dyn Fn(_)> =
        closure::Closure::new(|ev: MouseEvent| {
            let canvas = ev
                .target()
                .unwrap()
                .dyn_into::<HtmlCanvasElement>()
                .unwrap();
            let rect = canvas.get_bounding_client_rect();
            let event_handler = get_event_handler(None);

            let x = ev.client_x() as f32 - rect.left() as f32;
            let y = ev.client_y() as f32 - rect.top() as f32;

            let button = match ev.button() {
                0 => crate::MouseButton::Left,
                1 => crate::MouseButton::Right,
                2 => crate::MouseButton::Middle,
                n => crate::MouseButton::Other(n as _),
            };
        });

    let mouse_up_closure: closure::Closure<dyn Fn(_)> = closure::Closure::new(|ev: MouseEvent| {
        let canvas = ev
            .target()
            .unwrap()
            .dyn_into::<HtmlCanvasElement>()
            .unwrap();
        let rect = canvas.get_bounding_client_rect();
        let event_handler = get_event_handler(None);

        let x = ev.client_x() as f32 - rect.left() as f32;
        let y = ev.client_y() as f32 - rect.top() as f32;

        let button = match ev.button() {
            0 => crate::MouseButton::Left,
            1 => crate::MouseButton::Right,
            2 => crate::MouseButton::Middle,
            n => crate::MouseButton::Other(n as _),
        };
    });

    let mouse_wheel_closure: closure::Closure<dyn Fn(_)> =
        closure::Closure::new(|ev: WheelEvent| {
            ev.prevent_default();

            let x = -ev.delta_x() as f32;
            let y = -ev.delta_y() as f32;

            let event_handler = get_event_handler(None);
            event_handler.mouse_wheel_event(x, y);
        });

    let mouse_down_fn_ref = mouse_down_closure.as_ref().unchecked_ref();
    let mouse_move_ref = mouse_move_closure.as_ref().unchecked_ref();
    let mouse_up_fn_ref = mouse_up_closure.as_ref().unchecked_ref();
    let mouse_wheel_fn_ref = mouse_wheel_closure.as_ref().unchecked_ref();

    canvas.add_event_listener_with_callback("mousemove", mouse_move_ref);
    canvas.add_event_listener_with_callback("mousedown", mouse_down_fn_ref);
    canvas.add_event_listener_with_callback("mouseup", mouse_up_fn_ref);
    canvas.add_event_listener_with_callback("wheel", mouse_wheel_fn_ref);
}

fn get_keycode(key: &str) -> Option<i32> {
    Some(match key {
        "Space" => 32,
        "Quote" => 222,
        "Comma" => 44,
        "Minus" => 45,
        "Period" => 46,
        "Slash" => 189,
        "Digit0" => 48,
        "Digit1" => 49,
        "Digit2" => 50,
        "Digit3" => 51,
        "Digit4" => 52,
        "Digit5" => 53,
        "Digit6" => 54,
        "Digit7" => 55,
        "Digit8" => 56,
        "Digit9" => 57,
        "Semicolon" => 59,
        "Equal" => 61,
        "KeyA" => 65,
        "KeyB" => 66,
        "KeyC" => 67,
        "KeyD" => 68,
        "KeyE" => 69,
        "KeyF" => 70,
        "KeyG" => 71,
        "KeyH" => 72,
        "KeyI" => 73,
        "KeyJ" => 74,
        "KeyK" => 75,
        "KeyL" => 76,
        "KeyM" => 77,
        "KeyN" => 78,
        "KeyO" => 79,
        "KeyP" => 80,
        "KeyQ" => 81,
        "KeyR" => 82,
        "KeyS" => 83,
        "KeyT" => 84,
        "KeyU" => 85,
        "KeyV" => 86,
        "KeyW" => 87,
        "KeyX" => 88,
        "KeyY" => 89,
        "KeyZ" => 90,
        "BracketLeft" => 91,
        "Backslash" => 92,
        "BracketRight" => 93,
        "Backquote" => 96,
        "Escape" => 256,
        "Enter" => 257,
        "Tab" => 258,
        "Backspace" => 259,
        "Insert" => 260,
        "Delete" => 261,
        "ArrowRight" => 262,
        "ArrowLeft" => 263,
        "ArrowDown" => 264,
        "ArrowUp" => 265,
        "PageUp" => 266,
        "PageDown" => 267,
        "Home" => 268,
        "End" => 269,
        "CapsLock" => 280,
        "ScrollLock" => 281,
        "NumLock" => 282,
        "PrintScreen" => 283,
        "Pause" => 284,
        "F1" => 290,
        "F2" => 291,
        "F3" => 292,
        "F4" => 293,
        "F5" => 294,
        "F6" => 295,
        "F7" => 296,
        "F8" => 297,
        "F9" => 298,
        "F10" => 299,
        "F11" => 300,
        "F12" => 301,
        "F13" => 302,
        "F14" => 303,
        "F15" => 304,
        "F16" => 305,
        "F17" => 306,
        "F18" => 307,
        "F19" => 308,
        "F20" => 309,
        "F21" => 310,
        "F22" => 311,
        "F23" => 312,
        "F24" => 313,
        "Numpad0" => 320,
        "Numpad1" => 321,
        "Numpad2" => 322,
        "Numpad3" => 323,
        "Numpad4" => 324,
        "Numpad5" => 325,
        "Numpad6" => 326,
        "Numpad7" => 327,
        "Numpad8" => 328,
        "Numpad9" => 329,
        "NumpadDecimal" => 330,
        "NumpadDivide" => 331,
        "NumpadMultiply" => 33,
        "NumpadSubtract" => 33,
        "NumpadAdd" => 334,
        "NumpadEnter" => 335,
        "NumpadEqual" => 336,
        "ShiftLeft" => 340,
        "ControlLeft" => 341,
        "AltLeft" => 342,
        "OSLeft" => 343,
        "ShiftRight" => 344,
        "ControlRight" => 345,
        "AltRight" => 346,
        "OSRight" => 347,
        "ContextMenu" => 348,
        _ => return None,
    })
}

fn init_keyboard_events(canvas: &HtmlCanvasElement) {
    let key_up_closure: closure::Closure<dyn Fn(_)> = closure::Closure::new(|ev: KeyboardEvent| {
        let event_handler = get_event_handler(None);

        if let Some(key) = get_keycode(&ev.code()) {
            let keycode = keycodes::translate_keycode(key);
            let repeat = ev.repeat();
            let modifiers = crate::KeyMods {
                shift: ev.shift_key(),
                ctrl: ev.ctrl_key(),
                alt: ev.alt_key(),
                logo: ev.meta_key(),
            };

            event_handler.key_down_event(keycode, modifiers, repeat);
        };
    });

    let key_down_closure: closure::Closure<dyn Fn(_)> =
        closure::Closure::new(|ev: KeyboardEvent| {
            let event_handler = get_event_handler(None);
            let repeat = ev.repeat();

            if let Some(key) = get_keycode(&ev.code()) {
                let keycode = keycodes::translate_keycode(key);

                let modifiers = crate::KeyMods {
                    shift: ev.shift_key(),
                    ctrl: ev.ctrl_key(),
                    alt: ev.alt_key(),
                    logo: ev.meta_key(),
                };

                // prevent page interactions
                // space, arrow keys, F1-F10, Tab, Backspace, /, PageUp, PageDown, Home, End
                match key {
                    32 | 39 | 47 => {
                        // for "space", "quote", and "slash" preventDefault will prevent
                        // key_press event, so send it here instead
                        ev.prevent_default();
                        if let Some(c) = char::from_u32(ev.char_code()) {
                            event_handler.char_event(c, modifiers, repeat);
                        }
                    }
                    n if (262..=265).contains(&n)
                        | (290..=299).contains(&n)
                        | (258..=259).contains(&n)
                        | (266..=269).contains(&n) =>
                    {
                        ev.prevent_default()
                    }
                    _ => {}
                }

                event_handler.key_down_event(keycode, modifiers, repeat);
            };
        });

    let keypress_closure: closure::Closure<dyn Fn(_)> =
        closure::Closure::new(|ev: KeyboardEvent| {
            let event_handler = get_event_handler(None);
            let repeat = ev.repeat();
            let key = ev.key();

            let modifiers = crate::KeyMods {
                shift: ev.shift_key(),
                ctrl: ev.ctrl_key(),
                alt: ev.alt_key(),
                logo: ev.meta_key(),
            };

            if let Some(c) = key.chars().next() {
                event_handler.char_event(c, modifiers, repeat);
            }
        });

    let key_down_closure_ref = key_down_closure.as_ref().unchecked_ref();
    let keypress_fn_ref = keypress_closure.as_ref().unchecked_ref();
    let key_up_fn_ref = key_up_closure.as_ref().unchecked_ref();

    canvas.add_event_listener_with_callback("keypress", keypress_fn_ref);
    canvas.add_event_listener_with_callback("keydown", key_up_fn_ref);
    canvas.add_event_listener_with_callback("keydown", key_down_closure_ref);
}

fn init_focus_events(canvas: &HtmlCanvasElement) {
    let focus_closure: closure::Closure<dyn Fn()> = closure::Closure::new(|| {
        let event_handler = get_event_handler(None);
        event_handler.window_restored_event()
    });

    let blur_closure: closure::Closure<dyn Fn()> = closure::Closure::new(|| {
        let event_handler = get_event_handler(None);
        event_handler.window_minimized_event()
    });

    let visibility_change_closure: closure::Closure<dyn Fn()> = closure::Closure::new(|| {
        let event_handler = get_event_handler(None);

        if let Ok(hidden) = document().has_focus() {
            if hidden {
                event_handler.window_minimized_event()
            } else {
                event_handler.window_restored_event()
            }
        };
    });

    let focus_fn_ref = focus_closure.as_ref().unchecked_ref();
    let blur_fn_ref = blur_closure.as_ref().unchecked_ref();
    let visibility_change_fn_ref = visibility_change_closure.as_ref().unchecked_ref();

    canvas.add_event_listener_with_callback("focus", focus_fn_ref);
    canvas.add_event_listener_with_callback("blur", blur_fn_ref);
    web_sys::window()
        .unwrap()
        .add_event_listener_with_callback("visibilitychange", visibility_change_fn_ref);
}

fn init_resize_events(canvas: &HtmlCanvasElement) {
    let handler: closure::Closure<dyn Fn(_)> = closure::Closure::new(|ev: ResizeObserverEntry| {
        let event_handler = get_event_handler(None);

        let rect = ev.content_rect();
        let width = rect.width() as _;
        let height = rect.height() as _;

        event_handler.resize_event(width, height)
    });

    let fn_ref = handler.as_ref().unchecked_ref();
    let resize_observer = ResizeObserver::new(fn_ref).unwrap();

    resize_observer.observe(canvas);
}

fn init_touch_events(canvas: &HtmlCanvasElement) {
    let touch_start_closure: closure::Closure<dyn Fn(_)> =
        closure::Closure::new(|ev: TouchEvent| {
            ev.prevent_default();

            let event_handler = get_event_handler(None);
            let new_touches = ev.changed_touches();

            (0..new_touches.length())
                .flat_map(|idx| new_touches.item(idx))
                .for_each(|touch| {
                    let id = touch.identifier();
                    let x = touch.client_x() as f32;
                    let y = touch.client_y() as f32;

                    event_handler.touch_event(crate::TouchPhase::Started, id as _, x, y);
                });
        });

    let touch_move_closure: closure::Closure<dyn Fn(_)> =
        closure::Closure::new(|ev: TouchEvent| {
            ev.prevent_default();

            let event_handler = get_event_handler(None);
            let new_touches = ev.changed_touches();

            (0..new_touches.length())
                .flat_map(|idx| new_touches.item(idx))
                .for_each(|touch| {
                    let id = touch.identifier();
                    let x = touch.client_x() as f32;
                    let y = touch.client_y() as f32;

                    event_handler.touch_event(crate::TouchPhase::Moved, id as _, x, y);
                });
        });

    let touch_end_closure: closure::Closure<dyn Fn(_)> = closure::Closure::new(|ev: TouchEvent| {
        ev.prevent_default();

        let event_handler = get_event_handler(None);
        let new_touches = ev.changed_touches();

        (0..new_touches.length())
            .flat_map(|idx| new_touches.item(idx))
            .for_each(|touch| {
                let id = touch.identifier();
                let x = touch.client_x() as f32;
                let y = touch.client_y() as f32;

                event_handler.touch_event(crate::TouchPhase::Ended, id as _, x, y);
            });
    });

    let touch_cancel_closure: closure::Closure<dyn Fn(_)> =
        closure::Closure::new(|ev: TouchEvent| {
            ev.prevent_default();
            let event_handler = get_event_handler(None);

            let new_touches = ev.changed_touches();

            (0..new_touches.length())
                .flat_map(|idx| new_touches.item(idx))
                .for_each(|touch| {
                    let id = touch.identifier();
                    let x = touch.client_x() as f32;
                    let y = touch.client_y() as f32;

                    event_handler.touch_event(crate::TouchPhase::Cancelled, id as _, x, y);
                });
        });

    let touch_start_fn_ref = touch_start_closure.as_ref().unchecked_ref();
    let touch_move_fn_ref = touch_move_closure.as_ref().unchecked_ref();
    let touch_end_fn_ref = touch_end_closure.as_ref().unchecked_ref();
    let touch_cancel_fn_ref = touch_cancel_closure.as_ref().unchecked_ref();

    canvas.add_event_listener_with_callback("touchstart", touch_start_fn_ref);
    canvas.add_event_listener_with_callback("touchmove", touch_move_fn_ref);
    canvas.add_event_listener_with_callback("touchend", touch_end_fn_ref);
    canvas.add_event_listener_with_callback("touchcancel", touch_cancel_fn_ref);
}

fn init_file_drop_events(canvas: &HtmlCanvasElement) {
    let drag_over_closure: closure::Closure<dyn Fn(_)> =
        closure::Closure::new(|ev: DragEvent| ev.prevent_default());

    let drop_closure: closure::Closure<dyn Fn(_)> = closure::Closure::new(|ev: DragEvent| {
        let event_handler = get_event_handler(None);
        ev.prevent_default();

        if let Some(dt) = ev.data_transfer() {
            event_handler.files_dropped_event();

            if let Some(files) = dt.files() {
                let count = files.length();

                let mut paths = Vec::with_capacity(count as _);
                let mut bytes = Vec::<Vec<u8>>::with_capacity(count as _);

                for i in 0..count {
                    if let Some(file) = files.item(i) {
                        let name = file.name();
                        let array_buffer = file.array_buffer();

                        let data = wasm_bindgen_futures::JsFuture::from(array_buffer);
                        let data = pollster::block_on(data).unwrap();

                        // collect
                        paths.push(PathBuf::from(name));
                        bytes.push(js_sys::Uint8Array::new(&data).to_vec());
                    }
                }

                // update
                let mut d = crate::native_display().lock().unwrap();
                d.dropped_files.paths = paths;
                d.dropped_files.bytes = bytes;

                // notify
                event_handler.files_dropped_event();
            }
        }
    });

    let drop_fn_ref = drop_closure.as_ref().unchecked_ref();
    let drag_over_fn_ref = drag_over_closure.as_ref().unchecked_ref();

    canvas.add_event_listener_with_callback("dragover", drag_over_fn_ref);
    canvas.add_event_listener_with_callback("drop", drop_fn_ref);
}
