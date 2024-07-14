//! MacOs implementation is basically a mix between
//! sokol_app's objective C code and Makepad's (<https://github.com/makepad/makepad/blob/live/platform/src/platform/apple>)
//! platform implementation
//!
use {
    crate::{
        conf::{AppleGfxApi, Icon},
        event::{EventHandler, KeyCode, KeyMods, MouseButton},
        native::{
            apple::{apple_util::*, frameworks::*},
            gl, NativeDisplayData, Request,
        },
        native_display, CursorIcon,
    },
    std::{
        cell::RefCell,
        collections::HashMap,
        os::raw::c_void,
        sync::{mpsc, Arc, Mutex},
        thread,
    },
};

struct MainThreadState {
    quit: bool,
    view: *mut Object,
    cur_msg: Message,
}

pub struct MacosDisplay {
    window: ObjcId,
    view: ObjcId,
    fullscreen: bool,
    // [NSCursor hide]/unhide calls should be balanced
    // hide/hide/unhide will keep cursor hidden
    // so need to keep internal cursor state to avoid problems from
    // unbalanced show_mouse() calls
    cursor_shown: bool,
    current_cursor: CursorIcon,
    cursor_grabbed: bool,
    cursors: HashMap<CursorIcon, ObjcId>,
    gfx_api: crate::conf::AppleGfxApi,

    event_handler: Option<Box<dyn EventHandler>>,
    f: Option<Box<dyn 'static + FnOnce() -> Box<dyn EventHandler>>>,
    modifiers: Modifiers,
    state: Arc<Mutex<MainThreadState>>,
    update_requested: bool,
}

impl MacosDisplay {
    fn set_cursor_grab(&mut self, window: *mut Object, grab: bool) {
        if grab == self.cursor_grabbed {
            return;
        }

        self.cursor_grabbed = grab;

        unsafe {
            if grab {
                self.move_mouse_inside_window(window);
                CGAssociateMouseAndMouseCursorPosition(false);
                let () = msg_send![class!(NSCursor), hide];
            } else {
                let () = msg_send![class!(NSCursor), unhide];
                CGAssociateMouseAndMouseCursorPosition(true);
            }
        }
    }
    fn show_mouse(&mut self, show: bool) {
        if show && !self.cursor_shown {
            unsafe {
                let () = msg_send![class!(NSCursor), unhide];
            }
        }
        if !show && self.cursor_shown {
            unsafe {
                let () = msg_send![class!(NSCursor), hide];
            }
        }
        self.cursor_shown = show;
    }
    fn set_mouse_cursor(&mut self, cursor: crate::CursorIcon) {
        if self.current_cursor != cursor {
            self.current_cursor = cursor;
            unsafe {
                let _: () = msg_send![
                    self.window,
                    invalidateCursorRectsForView: self.view
                ];
            }
        }
    }
    fn set_window_size(&mut self, new_width: u32, new_height: u32) {
        let mut frame: NSRect = unsafe { msg_send![self.window, frame] };
        frame.origin.y += frame.size.height;
        frame.origin.y -= new_height as f64;
        frame.size = NSSize {
            width: new_width as f64,
            height: new_height as f64,
        };
        let () = unsafe { msg_send![self.window, setFrame:frame display:true animate:true] };
    }
    fn set_fullscreen(&mut self, fullscreen: bool) {
        if self.fullscreen != fullscreen {
            self.fullscreen = fullscreen;
            unsafe {
                let () = msg_send![self.window, toggleFullScreen: nil];
            }
        }
    }
    fn clipboard_get(&mut self) -> Option<String> {
        unsafe {
            let pasteboard: ObjcId = msg_send![class!(NSPasteboard), generalPasteboard];
            let content: ObjcId = msg_send![pasteboard, stringForType: NSStringPboardType];
            let string = nsstring_to_string(content);
            if string.is_empty() {
                return None;
            }
            Some(string)
        }
    }
    fn clipboard_set(&mut self, data: &str) {
        let str: ObjcId = str_to_nsstring(data);
        unsafe {
            let pasteboard: ObjcId = msg_send![class!(NSPasteboard), generalPasteboard];
            let () = msg_send![pasteboard, clearContents];
            let arr: ObjcId = msg_send![class!(NSArray), arrayWithObject: str];
            let () = msg_send![pasteboard, writeObjects: arr];
        }
    }

    pub fn context(&mut self) -> Option<&mut dyn EventHandler> {
        let event_handler = self.event_handler.as_deref_mut()?;

        Some(event_handler)
    }
}

impl MacosDisplay {
    fn transform_mouse_point(&self, point: &NSPoint) -> (f32, f32) {
        let d = native_display().lock().unwrap();
        let new_x = point.x as f32 * d.dpi_scale;
        let new_y = d.screen_height as f32 - (point.y as f32 * d.dpi_scale) - 1.;

        (new_x, new_y)
    }

    fn move_mouse_inside_window(&self, _window: *mut Object) {
        unsafe {
            let frame: NSRect = msg_send![self.window, frame];
            let origin = self.transform_mouse_point(&frame.origin);
            let point = NSPoint {
                x: (origin.0 as f64) + (frame.size.width / 2.0),
                y: (origin.1 as f64) + (frame.size.height / 2.0),
            };
            CGWarpMouseCursorPosition(point);
        }
    }

    unsafe fn update_dimensions(&mut self) -> Option<(i32, i32)> {
        let mut d = native_display().lock().unwrap();
        if d.high_dpi {
            let dpi_scale: f64 = msg_send![self.window, backingScaleFactor];
            d.dpi_scale = dpi_scale as f32;
        } else {
            let bounds: NSRect = msg_send![self.view, bounds];
            let backing_size: NSSize = msg_send![self.view, convertSizeToBacking: NSSize {width: bounds.size.width, height: bounds.size.height}];

            d.dpi_scale = (backing_size.width / bounds.size.width) as f32;
        }

        let bounds: NSRect = msg_send![self.view, bounds];
        let screen_width = (bounds.size.width as f32 * d.dpi_scale) as i32;
        let screen_height = (bounds.size.height as f32 * d.dpi_scale) as i32;

        let dim_changed = screen_width != d.screen_width || screen_height != d.screen_height;

        d.screen_width = screen_width;
        d.screen_height = screen_height;

        if dim_changed {
            Some((screen_width, screen_height))
        } else {
            None
        }
    }

    fn process_request(&mut self, request: Request) {
        use Request::*;
        match request {
            ScheduleUpdate => {
                self.update_requested = true;
            }
            SetCursorGrab(grab) => self.set_cursor_grab(self.window, grab),
            ShowMouse(show) => self.show_mouse(show),
            SetMouseCursor(icon) => self.set_mouse_cursor(icon),
            SetWindowSize {
                new_width,
                new_height,
            } => self.set_window_size(new_width as _, new_height as _),
            SetFullscreen(fullscreen) => self.set_fullscreen(fullscreen),
            SetWindowPosition { .. } => {
                eprintln!("Not implemented for macos");
            }
            _ => {}
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Resize {
        width: i32,
        height: i32,
    },
    RawMouseMove {
        dx: f32,
        dy: f32,
    },
    MouseMove {
        x: f32,
        y: f32,
    },
    MouseButtonDown {
        button: MouseButton,
        x: f32,
        y: f32,
    },
    MouseButtonUp {
        button: MouseButton,
        x: f32,
        y: f32,
    },
    MouseWheel {
        dx: f32,
        dy: f32,
    },
    Character {
        character: char,
        keymods: KeyMods,
        repeat: bool,
    },
    KeyDown {
        keycode: KeyCode,
        keymods: KeyMods,
        repeat: bool,
    },
    KeyUp {
        keycode: KeyCode,
        keymods: KeyMods,
    },
    Destroy,
}
unsafe impl Send for Message {}

thread_local! {
    static MESSAGES_TX: RefCell<Option<mpsc::Sender<Message>>> = RefCell::new(None);
}

fn send_message(message: Message) {
    MESSAGES_TX.with(|tx| {
        let mut tx = tx.borrow_mut();
        tx.as_mut().unwrap().send(message).unwrap();
    })
}

#[derive(Default)]
struct Modifiers {
    left_shift: bool,
    right_shift: bool,
    left_control: bool,
    right_control: bool,
    left_alt: bool,
    right_alt: bool,
    left_command: bool,
    right_command: bool,
}

impl Modifiers {
    const NS_RIGHT_SHIFT_KEY_MASK: u64 = 0x020004;
    const NS_LEFT_SHIFT_KEY_MASK: u64 = 0x020002;
    const NS_RIGHT_COMMAND_KEY_MASK: u64 = 0x100010;
    const NS_LEFT_COMMAND_KEY_MASK: u64 = 0x100008;
    const NS_RIGHT_ALTERNATE_KEY_MASK: u64 = 0x080040;
    const NS_LEFT_ALTERNATE_KEY_MASK: u64 = 0x080020;
    const NS_RIGHT_CONTROL_KEY_MASK: u64 = 0x042000;
    const NS_LEFT_CONTROL_KEY_MASK: u64 = 0x040001;

    pub fn new(flags: u64) -> Self {
        Self {
            left_shift: flags & Self::NS_LEFT_SHIFT_KEY_MASK == Self::NS_LEFT_SHIFT_KEY_MASK,
            right_shift: flags & Self::NS_RIGHT_SHIFT_KEY_MASK == Self::NS_RIGHT_SHIFT_KEY_MASK,
            left_alt: flags & Self::NS_LEFT_ALTERNATE_KEY_MASK == Self::NS_LEFT_ALTERNATE_KEY_MASK,
            right_alt: flags & Self::NS_RIGHT_ALTERNATE_KEY_MASK
                == Self::NS_RIGHT_ALTERNATE_KEY_MASK,
            left_control: flags & Self::NS_LEFT_CONTROL_KEY_MASK == Self::NS_LEFT_CONTROL_KEY_MASK,
            right_control: flags & Self::NS_RIGHT_CONTROL_KEY_MASK
                == Self::NS_RIGHT_CONTROL_KEY_MASK,
            left_command: flags & Self::NS_LEFT_COMMAND_KEY_MASK == Self::NS_LEFT_COMMAND_KEY_MASK,
            right_command: flags & Self::NS_RIGHT_COMMAND_KEY_MASK
                == Self::NS_RIGHT_COMMAND_KEY_MASK,
        }
    }
}
pub fn define_app_delegate() -> *const Class {
    let superclass = class!(NSObject);
    let mut decl = ClassDecl::new("NSAppDelegate", superclass).unwrap();
    unsafe {
        decl.add_method(
            sel!(applicationShouldTerminateAfterLastWindowClosed:),
            yes1 as extern "C" fn(&Object, Sel, ObjcId) -> BOOL,
        );
    }

    return decl.register();
}

pub fn define_cocoa_window_delegate() -> *const Class {
    extern "C" fn window_should_close(this: &Object, _: Sel, _: ObjcId) -> BOOL {
        let payload = get_window_payload(this);

        unsafe {
            let capture_manager = msg_send_![class![MTLCaptureManager], sharedCaptureManager];
            msg_send_![capture_manager, stopCapture];
        }

        // only give user-code a chance to intervene when sapp_quit() wasn't already called
        if !native_display().lock().unwrap().quit_ordered {
            // if window should be closed and event handling is enabled, give user code
            // a chance to intervene via sapp_cancel_quit()
            native_display().lock().unwrap().quit_requested = true;
            if let Some(event_handler) = payload.context() {
                event_handler.quit_requested_event();
            }

            // user code hasn't intervened, quit the app
            if native_display().lock().unwrap().quit_requested {
                native_display().lock().unwrap().quit_ordered = true;
            }
        }
        if native_display().lock().unwrap().quit_ordered {
            return YES;
        } else {
            return NO;
        }
    }

    extern "C" fn window_did_resize(this: &Object, _: Sel, _: ObjcId) {
        let payload = get_window_payload(this);
        if let Some((w, h)) = unsafe { payload.update_dimensions() } {
            send_message(Message::Resize {
                width: w,
                height: h,
            });
        }
    }

    extern "C" fn window_did_change_screen(this: &Object, _: Sel, _: ObjcId) {
        let payload = get_window_payload(this);
        if let Some((w, h)) = unsafe { payload.update_dimensions() } {
            send_message(Message::Resize {
                width: w,
                height: h,
            });
        }
    }
    extern "C" fn window_did_enter_fullscreen(this: &Object, _: Sel, _: ObjcId) {
        let payload = get_window_payload(this);
        payload.fullscreen = true;
    }
    extern "C" fn window_did_exit_fullscreen(this: &Object, _: Sel, _: ObjcId) {
        let payload = get_window_payload(this);
        payload.fullscreen = false;
    }
    let superclass = class!(NSObject);
    let mut decl = ClassDecl::new("RenderWindowDelegate", superclass).unwrap();

    // Add callback methods
    unsafe {
        decl.add_method(
            sel!(windowShouldClose:),
            window_should_close as extern "C" fn(&Object, Sel, ObjcId) -> BOOL,
        );

        decl.add_method(
            sel!(windowDidResize:),
            window_did_resize as extern "C" fn(&Object, Sel, ObjcId),
        );
        decl.add_method(
            sel!(windowDidChangeScreen:),
            window_did_change_screen as extern "C" fn(&Object, Sel, ObjcId),
        );
        decl.add_method(
            sel!(windowDidEnterFullScreen:),
            window_did_enter_fullscreen as extern "C" fn(&Object, Sel, ObjcId),
        );
        decl.add_method(
            sel!(windowDidExitFullScreen:),
            window_did_exit_fullscreen as extern "C" fn(&Object, Sel, ObjcId),
        );
    }
    // Store internal state as user data
    decl.add_ivar::<*mut c_void>("display_ptr");

    return decl.register();
}

unsafe fn get_proc_address(name: *const u8) -> Option<unsafe extern "C" fn()> {
    mod libc {
        use std::ffi::{c_char, c_int, c_void};

        pub const RTLD_LAZY: c_int = 1;
        extern "C" {
            pub fn dlopen(filename: *const c_char, flag: c_int) -> *mut c_void;
            pub fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
        }
    }
    static mut OPENGL: *mut std::ffi::c_void = std::ptr::null_mut();

    if OPENGL.is_null() {
        OPENGL = libc::dlopen(
            b"/System/Library/Frameworks/OpenGL.framework/Versions/Current/OpenGL\0".as_ptr() as _,
            libc::RTLD_LAZY,
        );
    }

    assert!(!OPENGL.is_null());

    let symbol = libc::dlsym(OPENGL, name as _);
    if symbol.is_null() {
        return None;
    }
    Some(unsafe { std::mem::transmute_copy(&symbol) })
}

// methods for both metal or OPENGL view
unsafe fn view_base_decl(decl: &mut ClassDecl) {
    extern "C" fn mouse_moved(this: &Object, _sel: Sel, event: ObjcId) {
        let payload = get_window_payload(this);

        unsafe {
            if payload.cursor_grabbed {
                let dx: f64 = msg_send!(event, deltaX);
                let dy: f64 = msg_send!(event, deltaY);
                send_message(Message::RawMouseMove {
                    dx: dx as f32,
                    dy: dy as f32,
                });
            } else {
                let point: NSPoint = msg_send!(event, locationInWindow);
                let point = payload.transform_mouse_point(&point);
                send_message(Message::MouseMove {
                    x: point.0,
                    y: point.1,
                });
            }
        }
    }

    fn fire_mouse_event(this: &Object, event: ObjcId, down: bool, btn: MouseButton) {
        let payload = get_window_payload(this);

        unsafe {
            let point: NSPoint = msg_send!(event, locationInWindow);
            let point = payload.transform_mouse_point(&point);
            if down {
                send_message(Message::MouseButtonDown {
                    button: btn,
                    x: point.0,
                    y: point.1,
                });
            } else {
                send_message(Message::MouseButtonUp {
                    button: btn,
                    x: point.0,
                    y: point.1,
                });
            }
        }
    }
    extern "C" fn mouse_down(this: &Object, _sel: Sel, event: ObjcId) {
        fire_mouse_event(this, event, true, MouseButton::Left);
    }
    extern "C" fn mouse_up(this: &Object, _sel: Sel, event: ObjcId) {
        fire_mouse_event(this, event, false, MouseButton::Left);
    }
    extern "C" fn right_mouse_down(this: &Object, _sel: Sel, event: ObjcId) {
        fire_mouse_event(this, event, true, MouseButton::Right);
    }
    extern "C" fn right_mouse_up(this: &Object, _sel: Sel, event: ObjcId) {
        fire_mouse_event(this, event, false, MouseButton::Right);
    }
    extern "C" fn other_mouse_down(this: &Object, _sel: Sel, event: ObjcId) {
        fire_mouse_event(this, event, true, MouseButton::Middle);
    }
    extern "C" fn other_mouse_up(this: &Object, _sel: Sel, event: ObjcId) {
        fire_mouse_event(this, event, false, MouseButton::Middle);
    }
    extern "C" fn scroll_wheel(_: &Object, _sel: Sel, event: ObjcId) {
        unsafe {
            let mut dx: f64 = msg_send![event, scrollingDeltaX];
            let mut dy: f64 = msg_send![event, scrollingDeltaY];

            if !msg_send![event, hasPreciseScrollingDeltas] {
                dx *= 10.0;
                dy *= 10.0;
            }
            send_message(Message::MouseWheel {
                dx: dx as f32,
                dy: dy as f32,
            });
        }
    }
    extern "C" fn reset_cursor_rects(this: &Object, _sel: Sel) {
        let payload = get_window_payload(this);

        unsafe {
            let cursor_id = {
                let current_cursor = payload.current_cursor;
                let cursor_id = *payload
                    .cursors
                    .entry(current_cursor)
                    .or_insert_with(|| load_mouse_cursor(current_cursor.clone()));
                assert!(!cursor_id.is_null());
                cursor_id
            };

            let bounds: NSRect = msg_send![this, bounds];
            let _: () = msg_send![
                this,
                addCursorRect: bounds
                cursor: cursor_id
            ];
        }
    }

    extern "C" fn key_down(_: &Object, _sel: Sel, event: ObjcId) {
        let mods = get_event_key_modifier(event);
        let repeat: bool = unsafe { msg_send!(event, isARepeat) };
        if let Some(key) = get_event_keycode(event) {
            send_message(Message::KeyDown {
                keycode: key,
                keymods: mods,
                repeat,
            });
        }

        if let Some(character) = get_event_char(event) {
            send_message(Message::Character {
                character,
                keymods: mods,
                repeat,
            });
        }
    }

    extern "C" fn key_up(_: &Object, _sel: Sel, event: ObjcId) {
        let mods = get_event_key_modifier(event);
        if let Some(key) = get_event_keycode(event) {
            send_message(Message::KeyUp {
                keycode: key,
                keymods: mods,
            });
        }
    }

    extern "C" fn flags_changed(this: &Object, _sel: Sel, event: ObjcId) {
        fn produce_event(
            keycode: crate::KeyCode,
            mods: crate::KeyMods,
            old_pressed: bool,
            new_pressed: bool,
        ) {
            if new_pressed ^ old_pressed {
                if new_pressed {
                    send_message(Message::KeyDown {
                        keycode,
                        keymods: mods,
                        repeat: false,
                    });
                } else {
                    send_message(Message::KeyUp {
                        keycode,
                        keymods: mods,
                    });
                }
            }
        }

        let payload = get_window_payload(this);
        let mods = get_event_key_modifier(event);
        let flags: u64 = unsafe { msg_send![event, modifierFlags] };
        let new_modifiers = Modifiers::new(flags);

        produce_event(
            crate::KeyCode::LeftShift,
            mods,
            payload.modifiers.left_shift,
            new_modifiers.left_shift,
        );
        produce_event(
            crate::KeyCode::RightShift,
            mods,
            payload.modifiers.right_shift,
            new_modifiers.right_shift,
        );
        produce_event(
            crate::KeyCode::LeftControl,
            mods,
            payload.modifiers.left_control,
            new_modifiers.left_control,
        );
        produce_event(
            crate::KeyCode::RightControl,
            mods,
            payload.modifiers.right_control,
            new_modifiers.right_control,
        );
        produce_event(
            crate::KeyCode::LeftSuper,
            mods,
            payload.modifiers.left_command,
            new_modifiers.left_command,
        );
        produce_event(
            crate::KeyCode::RightSuper,
            mods,
            payload.modifiers.right_command,
            new_modifiers.right_command,
        );
        produce_event(
            crate::KeyCode::LeftAlt,
            mods,
            payload.modifiers.left_alt,
            new_modifiers.left_alt,
        );
        produce_event(
            crate::KeyCode::RightAlt,
            mods,
            payload.modifiers.right_alt,
            new_modifiers.right_alt,
        );

        payload.modifiers = new_modifiers;
    }

    extern "C" fn process_message(this: &Object, _: Sel, _: ObjcId) {
        use Message::*;
        let payload = get_window_payload(this);
        let msg = {
            let state = payload.state.lock().unwrap();
            state.cur_msg
        };
        match msg {
            Destroy => {
                let mut state = payload.state.lock().unwrap();
                state.quit = true;
            }
            MouseMove { x, y } => {
                if let Some(ref mut event_handler) = payload.event_handler {
                    event_handler.mouse_motion_event(x, y);
                }
            }
            RawMouseMove { dx, dy } => {
                if let Some(ref mut event_handler) = payload.event_handler {
                    event_handler.raw_mouse_motion(dx, dy);
                }
            }
            MouseButtonDown { button, x, y } => {
                if let Some(ref mut event_handler) = payload.event_handler {
                    event_handler.mouse_button_down_event(button, x, y);
                }
            }
            MouseButtonUp { button, x, y } => {
                if let Some(ref mut event_handler) = payload.event_handler {
                    event_handler.mouse_button_up_event(button, x, y);
                }
            }
            MouseWheel { dx, dy } => {
                if let Some(ref mut event_handler) = payload.event_handler {
                    event_handler.mouse_wheel_event(dx as f32, dy as f32);
                }
            }
            Character {
                character,
                keymods,
                repeat,
            } => {
                if let Some(ref mut event_handler) = payload.event_handler {
                    event_handler.char_event(character, keymods, repeat);
                }
            }
            KeyDown {
                keycode,
                keymods,
                repeat,
            } => {
                if let Some(ref mut event_handler) = payload.event_handler {
                    event_handler.key_down_event(keycode, keymods, repeat);
                }
            }
            KeyUp { keycode, keymods } => {
                if let Some(ref mut event_handler) = payload.event_handler {
                    event_handler.key_up_event(keycode, keymods);
                }
            }
            Resize { width, height } => {
                if let Some(ref mut event_handler) = payload.event_handler {
                    event_handler.resize_event(width as _, height as _);
                }
            }
        }
    }

    // apparently, its impossible to use performSelectorOnMainThread
    // with primitive type argument, so the only way to pass
    // YES to setNeedsDisplay - send a no argument message
    // https://stackoverflow.com/questions/6120614/passing-primitives-through-performselectoronmainthread
    extern "C" fn set_needs_display_hack(this: &Object, _: Sel) {
        unsafe {
            msg_send_![this, setNeedsDisplay: YES];
        }
    }

    decl.add_method(
        sel!(canBecomeKey),
        yes as extern "C" fn(&Object, Sel) -> BOOL,
    );
    decl.add_method(
        sel!(acceptsFirstResponder),
        yes as extern "C" fn(&Object, Sel) -> BOOL,
    );
    decl.add_method(sel!(isOpaque), yes as extern "C" fn(&Object, Sel) -> BOOL);
    decl.add_method(
        sel!(resetCursorRects),
        reset_cursor_rects as extern "C" fn(&Object, Sel),
    );
    decl.add_method(
        sel!(mouseMoved:),
        mouse_moved as extern "C" fn(&Object, Sel, ObjcId),
    );
    decl.add_method(
        sel!(mouseDragged:),
        mouse_moved as extern "C" fn(&Object, Sel, ObjcId),
    );
    decl.add_method(
        sel!(rightMouseDragged:),
        mouse_moved as extern "C" fn(&Object, Sel, ObjcId),
    );
    decl.add_method(
        sel!(otherMouseDragged:),
        mouse_moved as extern "C" fn(&Object, Sel, ObjcId),
    );
    decl.add_method(
        sel!(mouseDown:),
        mouse_down as extern "C" fn(&Object, Sel, ObjcId),
    );
    decl.add_method(
        sel!(mouseUp:),
        mouse_up as extern "C" fn(&Object, Sel, ObjcId),
    );
    decl.add_method(
        sel!(rightMouseDown:),
        right_mouse_down as extern "C" fn(&Object, Sel, ObjcId),
    );
    decl.add_method(
        sel!(rightMouseUp:),
        right_mouse_up as extern "C" fn(&Object, Sel, ObjcId),
    );
    decl.add_method(
        sel!(otherMouseDown:),
        other_mouse_down as extern "C" fn(&Object, Sel, ObjcId),
    );
    decl.add_method(
        sel!(otherMouseUp:),
        other_mouse_up as extern "C" fn(&Object, Sel, ObjcId),
    );
    decl.add_method(
        sel!(scrollWheel:),
        scroll_wheel as extern "C" fn(&Object, Sel, ObjcId),
    );
    decl.add_method(
        sel!(keyDown:),
        key_down as extern "C" fn(&Object, Sel, ObjcId),
    );
    decl.add_method(
        sel!(flagsChanged:),
        flags_changed as extern "C" fn(&Object, Sel, ObjcId),
    );
    decl.add_method(sel!(keyUp:), key_up as extern "C" fn(&Object, Sel, ObjcId));
    decl.add_method(
        sel!(processMessage:),
        process_message as extern "C" fn(&Object, Sel, ObjcId),
    );
    decl.add_method(
        sel!(setNeedsDisplayHack),
        set_needs_display_hack as extern "C" fn(&Object, Sel),
    );
}

pub fn define_opengl_view_class() -> *const Class {
    //extern "C" fn dealloc(this: &Object, _sel: Sel) {}

    extern "C" fn reshape(this: &Object, _sel: Sel) {
        let payload = get_window_payload(this);

        unsafe {
            let superclass = superclass(this);
            let () = msg_send![super(this, superclass), reshape];

            if let Some((w, h)) = payload.update_dimensions() {
                // left this because post_processing.rs turns black after the first resize
                // and somehow this makes it draw until manual window resize
                // (bug existed even on original macos implementation)

                // send_message(Message::Resize { width: w, height: h });
                if let Some(ref mut event_handler) = payload.event_handler {
                    event_handler.resize_event(w as _, h as _);
                }
            }
        }
    }

    extern "C" fn draw_rect(this: &Object, _sel: Sel, _rect: NSRect) {
        let payload = get_window_payload(this);
        let mut updated = false;

        if let Some(event_handler) = payload.context() {
            event_handler.update();
            event_handler.draw();
            updated = true;
        }
        if updated {
            payload.update_requested = false;
        }

        unsafe {
            let ctx: ObjcId = msg_send![this, openGLContext];
            assert!(!ctx.is_null());
            let () = msg_send![ctx, flushBuffer];

            let d = native_display().lock().unwrap();
            if d.quit_requested || d.quit_ordered {
                drop(d);
                let () = msg_send![payload.window, performClose: nil];
            }
        }
    }

    extern "C" fn prepare_open_gl(this: &Object, _sel: Sel) {
        let payload = get_window_payload(this);
        unsafe {
            let superclass = superclass(this);
            let () = msg_send![super(this, superclass), prepareOpenGL];
            let mut swap_interval = 1;
            let ctx: ObjcId = msg_send![this, openGLContext];
            let () = msg_send![ctx,
                               setValues:&mut swap_interval
                               forParameter:NSOpenGLContextParameterSwapInterval];
            let () = msg_send![ctx, makeCurrentContext];
        }

        gl::load_gl_funcs(|proc| {
            let name = std::ffi::CString::new(proc).unwrap();

            unsafe { get_proc_address(name.as_ptr() as _) }
        });

        let f = payload.f.take().unwrap();
        payload.event_handler = Some(f());
    }

    let superclass = class!(NSOpenGLView);
    let mut decl: ClassDecl = ClassDecl::new("RenderViewClass", superclass).unwrap();
    unsafe {
        //decl.add_method(sel!(dealloc), dealloc as extern "C" fn(&Object, Sel));
        decl.add_method(
            sel!(prepareOpenGL),
            prepare_open_gl as extern "C" fn(&Object, Sel),
        );
        decl.add_method(sel!(reshape), reshape as extern "C" fn(&Object, Sel));
        decl.add_method(
            sel!(drawRect:),
            draw_rect as extern "C" fn(&Object, Sel, NSRect),
        );

        view_base_decl(&mut decl);
    }

    decl.add_ivar::<*mut c_void>("display_ptr");

    return decl.register();
}

pub fn define_metal_view_class() -> *const Class {
    let superclass = class!(MTKView);
    let mut decl = ClassDecl::new("RenderViewClass", superclass).unwrap();
    decl.add_ivar::<*mut c_void>("display_ptr");

    extern "C" fn draw_rect(this: &Object, _sel: Sel, _rect: NSRect) {
        let payload = get_window_payload(this);

        if payload.event_handler.is_none() {
            let f = payload.f.take().unwrap();
            payload.event_handler = Some(f());
        }

        let mut updated = false;

        if let Some(event_handler) = payload.context() {
            event_handler.update();
            event_handler.draw();
            updated = true;
        }
        if updated {
            payload.update_requested = false;
        }

        unsafe {
            let d = native_display().lock().unwrap();
            if d.quit_requested || d.quit_ordered {
                drop(d);
                let () = msg_send![payload.window, performClose: nil];
            }
        }
    }

    unsafe {
        //decl.add_method(sel!(dealloc), dealloc as extern "C" fn(&Object, Sel));
        decl.add_method(
            sel!(drawRect:),
            draw_rect as extern "C" fn(&Object, Sel, NSRect),
        );

        view_base_decl(&mut decl);
    }

    return decl.register();
}

fn get_window_payload(this: &Object) -> &mut MacosDisplay {
    unsafe {
        let ptr: *mut c_void = *this.get_ivar("display_ptr");
        &mut *(ptr as *mut MacosDisplay)
    }
}

unsafe fn create_metal_view(_: NSRect, sample_count: i32, _: bool) -> ObjcId {
    let mtl_device_obj = MTLCreateSystemDefaultDevice();
    let view_class = define_metal_view_class();
    let view: ObjcId = msg_send![view_class, alloc];
    let view: ObjcId = msg_send![view, init];

    let () = msg_send![view, setDevice: mtl_device_obj];
    let () = msg_send![view, setColorPixelFormat: MTLPixelFormat::BGRA8Unorm];
    let () = msg_send![
        view,
        setDepthStencilPixelFormat: MTLPixelFormat::Depth32Float_Stencil8
    ];
    let () = msg_send![view, setSampleCount: sample_count];
    let () = msg_send![view, setEnableSetNeedsDisplay: true];
    let () = msg_send![view, setPaused: true];

    view
}

unsafe fn create_opengl_view(window_frame: NSRect, sample_count: i32, high_dpi: bool) -> ObjcId {
    use NSOpenGLPixelFormatAttribute::*;

    let mut attrs: Vec<u32> = vec![];

    attrs.push(NSOpenGLPFAAccelerated as _);
    attrs.push(NSOpenGLPFADoubleBuffer as _);
    attrs.push(NSOpenGLPFAOpenGLProfile as _);
    attrs.push(NSOpenGLPFAOpenGLProfiles::NSOpenGLProfileVersion3_2Core as _);
    attrs.push(NSOpenGLPFAColorSize as _);
    attrs.push(24);
    attrs.push(NSOpenGLPFAAlphaSize as _);
    attrs.push(8);
    attrs.push(NSOpenGLPFADepthSize as _);
    attrs.push(24);
    attrs.push(NSOpenGLPFAStencilSize as _);
    attrs.push(8);
    if sample_count > 1 {
        attrs.push(NSOpenGLPFAMultisample as _);
        attrs.push(NSOpenGLPFASampleBuffers as _);
        attrs.push(1 as _);
        attrs.push(NSOpenGLPFASamples as _);
        attrs.push(sample_count as _);
    } else {
        attrs.push(NSOpenGLPFASampleBuffers as _);
        attrs.push(0);
    }
    attrs.push(0);

    let glpixelformat_obj: ObjcId = msg_send![class!(NSOpenGLPixelFormat), alloc];
    let glpixelformat_obj: ObjcId =
        msg_send![glpixelformat_obj, initWithAttributes: attrs.as_ptr()];
    assert!(!glpixelformat_obj.is_null());

    let view_class = define_opengl_view_class();
    let view: ObjcId = msg_send![view_class, alloc];
    let view: ObjcId = msg_send![
        view,
        initWithFrame: window_frame
        pixelFormat: glpixelformat_obj
    ];

    if high_dpi {
        let () = msg_send![view, setWantsBestResolutionOpenGLSurface: YES];
    } else {
        let () = msg_send![view, setWantsBestResolutionOpenGLSurface: NO];
    }

    view
}

struct MacosClipboard;
impl crate::native::Clipboard for MacosClipboard {
    fn get(&mut self) -> Option<String> {
        None
    }
    fn set(&mut self, _data: &str) {}
}

unsafe extern "C" fn release_data(info: *mut c_void, _: *const c_void, _: usize) {
    drop(Box::from_raw(info));
}

unsafe fn set_icon(ns_app: ObjcId, icon: &Icon) {
    let width = 64 as usize;
    let height = 64 as usize;
    let colors = &icon.big[..];
    let rgb = CGColorSpaceCreateDeviceRGB();
    let bits_per_component: usize = 8; // number of bits in UInt8
    let bits_per_pixel = 4 * bits_per_component; // ARGB uses 4 components
    let bytes_per_row = width * 4; // bitsPerRow / 8

    let data = colors.as_ptr();
    let size = colors.len();
    let boxed = Box::new(colors);
    let info = Box::into_raw(boxed);
    let provider = CGDataProviderCreateWithData(info as *mut c_void, data, size, release_data);
    let image = CGImageCreate(
        width,
        height,
        bits_per_component,
        bits_per_pixel,
        bytes_per_row,
        rgb,
        kCGBitmapByteOrderDefault | kCGImageAlphaLast,
        provider,
        std::ptr::null(),
        false,
        kCGRenderingIntentDefault,
    );

    let size = NSSize {
        width: width as f64,
        height: height as f64,
    };
    let ns_image: ObjcId = msg_send![class!(NSImage), alloc];
    let () = msg_send![ns_image, initWithCGImage: image size: size];

    let () = msg_send![ns_app, setApplicationIconImage: ns_image];
    CGDataProviderRelease(provider);
    CGColorSpaceRelease(rgb);
    CGImageRelease(image);
}

/// Initialize the system menu bar for this application
/// - ns_app: This NSApplication
unsafe fn initialize_menu_bar(ns_app: ObjcId) {
    // Adapted from Winit `menu::initialize`

    // System menu bar
    let menu_bar = msg_send_![class!(NSMenu), new];
    // Entry for the app menu dropdown in the menu bar
    let app_menu_item = msg_send_![class!(NSMenuItem), new];
    let app_menu = msg_send_![class!(NSMenu), new];

    // Hook up the menu components to the application
    msg_send_![app_menu_item, setSubmenu: app_menu];
    msg_send_![menu_bar, addItem: app_menu_item];
    msg_send_![ns_app, setMainMenu: menu_bar];

    // Add quit menu entry with shortcut command-q
    // It uses NSRunningApplication.localizedName, which will try to use the localized name,
    //  and will go through a chain of fallbacks based on what name strings are set in
    //  the Application bundle files, ending with the executable name.
    let running_application = msg_send_![class!(NSRunningApplication), currentApplication];
    let application_name = msg_send_![running_application, localizedName];
    let quit_item_title = str_to_nsstring(&format!("Quit {}", nsstring_to_string(application_name)));
    let quit_item = msg_send_![class!(NSMenuItem), alloc];
    let quit_item = msg_send_![
        quit_item,
        initWithTitle: quit_item_title
        action: sel!(terminate:)
        keyEquivalent: str_to_nsstring("q")
    ];
    msg_send_![app_menu, addItem: quit_item];
}

pub unsafe fn run<F>(conf: crate::conf::Conf, f: F)
where
    F: 'static + FnOnce() -> Box<dyn EventHandler>,
{
    let (tx, requests_rx) = std::sync::mpsc::channel();
    let clipboard = Box::new(MacosClipboard);
    crate::set_display(NativeDisplayData {
        high_dpi: conf.high_dpi,
        gfx_api: conf.platform.apple_gfx_api,
        blocking_event_loop: conf.platform.blocking_event_loop,
        ..NativeDisplayData::new(conf.window_width, conf.window_height, tx, clipboard)
    });

    let (tx, rx) = std::sync::mpsc::channel();

    MESSAGES_TX.with(move |messages_tx| *messages_tx.borrow_mut() = Some(tx));

    let state_original = Arc::new(Mutex::new(MainThreadState {
        quit: false,
        view: std::ptr::null_mut(),
        cur_msg: Message::MouseMove { x: 0., y: 0. },
    }));

    let mut display = MacosDisplay {
        view: std::ptr::null_mut(),
        window: std::ptr::null_mut(),
        fullscreen: false,
        cursor_shown: true,
        current_cursor: CursorIcon::Default,
        cursor_grabbed: false,
        cursors: HashMap::new(),
        gfx_api: conf.platform.apple_gfx_api,
        f: Some(Box::new(f)),
        event_handler: None,
        modifiers: Modifiers::default(),
        update_requested: true,
        state: state_original.clone(),
    };

    let app_delegate_class = define_app_delegate();
    let app_delegate_instance: ObjcId = msg_send![app_delegate_class, new];

    let ns_app: ObjcId = msg_send![class!(NSApplication), sharedApplication];
    let () = msg_send![ns_app, setDelegate: app_delegate_instance];
    let () = msg_send![
        ns_app,
        setActivationPolicy: NSApplicationActivationPolicy::NSApplicationActivationPolicyRegular
            as i64
    ];
    let () = msg_send![ns_app, activateIgnoringOtherApps: YES];

    if let Some(icon) = &conf.icon {
        set_icon(ns_app, icon);
    }

    initialize_menu_bar(ns_app);

    let window_masks = NSWindowStyleMask::NSTitledWindowMask as u64
        | NSWindowStyleMask::NSClosableWindowMask as u64
        | NSWindowStyleMask::NSMiniaturizableWindowMask as u64
        | NSWindowStyleMask::NSResizableWindowMask as u64;
    //| NSWindowStyleMask::NSFullSizeContentViewWindowMask as u64;

    let window_frame = NSRect {
        origin: NSPoint { x: 0., y: 0. },
        size: NSSize {
            width: conf.window_width as f64,
            height: conf.window_height as f64,
        },
    };

    let window: ObjcId = msg_send![class!(NSWindow), alloc];
    let window: ObjcId = msg_send![
        window,
        initWithContentRect: window_frame
        styleMask: window_masks as u64
        backing: NSBackingStoreType::NSBackingStoreBuffered as u64
        defer: NO
    ];
    assert!(!window.is_null());

    let window_delegate_class = define_cocoa_window_delegate();
    let window_delegate: ObjcId = msg_send![window_delegate_class, new];
    let () = msg_send![window, setDelegate: window_delegate];

    (*window_delegate).set_ivar("display_ptr", &mut display as *mut _ as *mut c_void);

    let title = str_to_nsstring(&conf.window_title);
    //let () = msg_send![window, setReleasedWhenClosed: NO];
    let () = msg_send![window, setTitle: title];
    let () = msg_send![window, center];
    let () = msg_send![window, setAcceptsMouseMovedEvents: YES];

    let view = match conf.platform.apple_gfx_api {
        AppleGfxApi::OpenGl => create_opengl_view(window_frame, conf.sample_count, conf.high_dpi),
        AppleGfxApi::Metal => create_metal_view(window_frame, conf.sample_count, conf.high_dpi),
    };
    {
        let mut d = native_display().lock().unwrap();
        d.view = view;
    }
    {
        let mut s = state_original.lock().unwrap();
        s.view = view;
    }
    (*view).set_ivar("display_ptr", &mut display as *mut _ as *mut c_void);

    display.window = window;
    display.view = view;

    let () = msg_send![window, setContentView: view];

    let _ = display.update_dimensions();

    assert!(!view.is_null());

    let () = msg_send![window, makeFirstResponder: view];

    if conf.fullscreen {
        let () = msg_send![window, toggleFullScreen: nil];
    }

    let () = msg_send![window, makeKeyAndOrderFront: nil];

    struct SendHack<F>(F);
    unsafe impl<F> Send for SendHack<F> {}

    let state = SendHack(state_original.clone());
    thread::spawn(move || {
        let s = state.0;

        loop {
            while let Ok(request) = requests_rx.try_recv() {
                let s = s.lock().unwrap();
                let view = s.view;
                let payload = get_window_payload(view.as_ref().unwrap());
                payload.process_request(request);
            }

            let block_on_wait = {
                let s = s.lock().unwrap();
                let view = s.view;

                let payload = get_window_payload(view.as_ref().unwrap());
                conf.platform.blocking_event_loop && !payload.update_requested
            };

            if block_on_wait {
                let res = rx.recv();

                if let Ok(msg) = res {
                    let view;
                    {
                        let mut s = s.lock().unwrap();
                        view = s.view;
                        s.cur_msg = msg;
                    }
                    msg_send_![&*view, performSelectorOnMainThread:sel!(processMessage:) withObject:nil waitUntilDone:YES];
                }
            } else {
                // process all the messages from the main thread
                while let Ok(msg) = rx.try_recv() {
                    let view;
                    {
                        let mut s = s.lock().unwrap();
                        view = s.view;
                        s.cur_msg = msg;
                    }
                    msg_send_![&*view, performSelectorOnMainThread:sel!(processMessage:) withObject:nil waitUntilDone:YES];
                }
            }

            let update_requested;
            let view;
            {
                let s = s.lock().unwrap();
                view = s.view;

                let payload = get_window_payload(view.as_ref().unwrap());
                update_requested = payload.update_requested;
            }

            if !conf.platform.blocking_event_loop || update_requested {
                unsafe {
                    msg_send_![view, performSelectorOnMainThread:sel!(setNeedsDisplayHack) withObject:nil waitUntilDone:NO];
                }
            }
            thread::yield_now();
        }
    });

    let () = msg_send![ns_app, run];

    // run should never return
    // but just in case
    unreachable!();
}
