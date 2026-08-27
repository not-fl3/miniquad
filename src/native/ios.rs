//! MacOs implementation is basically a mix between
//! sokol_app's objective C code and Makepad's (<https://github.com/makepad/makepad/blob/live/platform/src/platform/apple>)
//! platform implementation
//!
use {
    crate::{
        conf::{self, AppleGfxApi, Conf},
        event::{EventHandler, KeyCode, KeyMods, TouchPhase},
        fs,
        native::{
            apple::{
                apple_util::{self, *},
                frameworks::{self, *},
            },
            NativeDisplayData,
        },
        native_display,
    },
    std::{
        cell::RefCell,
        os::raw::c_void,
        sync::{mpsc, Arc, Mutex},
    },
};

// iOS 27 / TN3187: windows must be created via `initWithWindowScene:`
// or they stay invisible. Window creation defers to the
// `UISceneWillConnectNotification` observer below; both that and
// `did_finish_launching_with_options` run on the main thread, so
// `thread_local` storage is sound.
thread_local! {
    static PENDING_SCENE_VIEW_AND_CTRL: RefCell<Option<(ObjcId, ObjcId)>> =
        RefCell::new(None);
}

struct MainThreadState {
    quit: bool,
    paused: bool,
    update_requested: bool,
    view: *mut Object,
    keymods: KeyMods,
}

struct IosDisplay {
    view: ObjcId,
    view_dlg: ObjcId,
    view_ctrl: ObjcId,
    display_link: ObjcId,
    _textfield_dlg: ObjcId,
    textfield: ObjcId,
    current_element_id: u64,
    gfx_api: conf::AppleGfxApi,

    event_handler: Option<Box<dyn EventHandler>>,
    _gles2: bool,
    f: Option<Box<dyn 'static + FnOnce() -> Box<dyn EventHandler>>>,
    state: Arc<Mutex<MainThreadState>>,
    /// UIKit-side events; drained at the start of every `drawInMTKView:`.
    messages_rx: mpsc::Receiver<Message>,
    /// User-code requests (`schedule_update` etc.); same drain
    /// cadence as `messages_rx`.
    requests_rx: mpsc::Receiver<crate::native::Request>,
    blocking_event_loop: bool,
}

impl IosDisplay {
    fn show_keyboard(&mut self, show: bool) {
        unsafe {
            if show {
                msg_send_![self.textfield, becomeFirstResponder];
            } else {
                msg_send_![self.textfield, resignFirstResponder];
            }
        }
    }

    fn init_event_handler(&mut self) {
        let f = self.f.take().unwrap();

        if self.gfx_api == AppleGfxApi::OpenGl {
            crate::native::gl::load_gl_funcs(|proc| {
                let name = std::ffi::CString::new(proc).unwrap();

                unsafe { get_proc_address(name.as_ptr() as _) }
            });
        }

        self.event_handler = Some(f());
    }
}

fn get_window_payload(this: &Object) -> &mut IosDisplay {
    unsafe {
        let ptr: *mut c_void = *this.get_ivar("display_ptr");
        &mut *(ptr as *mut IosDisplay)
    }
}

/// Apply a `Message` to the payload's event handler + state. Called
/// inline at the start of each `drawInMTKView:` for every pending
/// message.
fn dispatch_message(payload: &mut IosDisplay, msg: Message) {
    match msg {
        Message::Pause => {
            payload.state.lock().unwrap().paused = true;
            if let Some(ref mut event_handler) = payload.event_handler {
                event_handler.window_minimized_event();
            }
        }
        Message::Resume => {
            let mut state = payload.state.lock().unwrap();
            state.paused = false;
            state.update_requested = true;
            if let Some(ref mut event_handler) = payload.event_handler {
                event_handler.window_restored_event();
            }
        }
        Message::Destroy => {
            payload.state.lock().unwrap().quit = true;
        }
        Message::Touch {
            phase,
            touch_id,
            x,
            y,
        } => {
            payload.state.lock().unwrap().update_requested = true;
            if let Some(ref mut event_handler) = payload.event_handler {
                event_handler.touch_event(phase, touch_id, x, y);
            }
        }
        Message::Character { character } => {
            payload.state.lock().unwrap().update_requested = true;
            if let Some(character) = char::from_u32(character) {
                if let Some(ref mut event_handler) = payload.event_handler {
                    event_handler.char_event(character, Default::default(), false);
                }
            }
        }
        Message::KeyDown { keycode } => {
            let keymods = {
                let mut state = payload.state.lock().unwrap();
                state.update_requested = true;
                match keycode {
                    KeyCode::LeftShift | KeyCode::RightShift => state.keymods.shift = true,
                    KeyCode::LeftControl | KeyCode::RightControl => state.keymods.ctrl = true,
                    KeyCode::LeftAlt | KeyCode::RightAlt => state.keymods.alt = true,
                    KeyCode::LeftSuper | KeyCode::RightSuper => state.keymods.logo = true,
                    _ => {}
                }
                state.keymods
            };
            if let Some(ref mut event_handler) = payload.event_handler {
                event_handler.key_down_event(keycode, keymods, false);
            }
        }
        Message::KeyUp { keycode } => {
            let keymods = {
                let mut state = payload.state.lock().unwrap();
                state.update_requested = true;
                match keycode {
                    KeyCode::LeftShift | KeyCode::RightShift => state.keymods.shift = false,
                    KeyCode::LeftControl | KeyCode::RightControl => state.keymods.ctrl = false,
                    KeyCode::LeftAlt | KeyCode::RightAlt => state.keymods.alt = false,
                    KeyCode::LeftSuper | KeyCode::RightSuper => state.keymods.logo = false,
                    _ => {}
                }
                state.keymods
            };
            if let Some(ref mut event_handler) = payload.event_handler {
                event_handler.key_up_event(keycode, keymods);
            }
        }
        Message::Resize { width, height } => {
            payload.state.lock().unwrap().update_requested = true;
            if let Some(ref mut event_handler) = payload.event_handler {
                event_handler.resize_event(width as _, height as _);
            }
        }
        Message::ImePreedit(text) => {
            if let Some(ref mut event_handler) = payload.event_handler {
                event_handler.on_ime_preedit(&text);
            }
        }
        Message::ImeCommit(text) => {
            if let Some(ref mut event_handler) = payload.event_handler {
                event_handler.on_ime_commit(text.as_deref());
            }
        }
        Message::ImeStateChanged {
            text,
            selection_start,
            selection_end,
            composing_start,
            composing_end,
            element_id,
        } => {
            if let Some(ref mut event_handler) = payload.event_handler {
                event_handler.on_ime_state_changed(
                    &text,
                    selection_start,
                    selection_end,
                    composing_start,
                    composing_end,
                    element_id,
                );
            }
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    Resize {
        width: i32,
        height: i32,
    },
    Touch {
        phase: TouchPhase,
        touch_id: u64,
        x: f32,
        y: f32,
    },
    Character {
        character: u32,
    },
    KeyDown {
        keycode: KeyCode,
    },
    KeyUp {
        keycode: KeyCode,
    },
    Pause,
    Resume,
    Destroy,
    ImePreedit(String),
    ImeCommit(Option<String>),
    ImeStateChanged {
        text: String,
        selection_start: usize,
        selection_end: usize,
        composing_start: Option<usize>,
        composing_end: Option<usize>,
        element_id: u64,
    },
}
unsafe impl Send for Message {}

thread_local! {
    static MESSAGES_TX: RefCell<Option<mpsc::Sender<Message>>> = const { RefCell::new(None) };
}

impl IosDisplay {
    fn process_request(&mut self, request: crate::native::Request) {
        use crate::native::Request::*;

        match request {
            ScheduleUpdate => {
                self.state.lock().unwrap().update_requested = true;
            }
            ShowKeyboard(show) => {
                self.show_keyboard(show);
            }
            UpdateTextInputState {
                text,
                selection_start,
                selection_end,
                is_password,
                is_multiline: _,
                element_id,
                max_length: _,
            } => unsafe {
                self.current_element_id = element_id;

                if element_id == 0 {
                    self.show_keyboard(false);
                    return;
                }
                self.show_keyboard(true);

                let current_secure: BOOL = msg_send![self.textfield, isSecureTextEntry];
                let want_secure = if is_password { YES } else { NO };
                if current_secure != want_secure {
                    msg_send_![self.textfield, setSecureTextEntry: want_secure];
                }

                let ns_text = apple_util::str_to_nsstring(&text);
                let current_ns_text: ObjcId = msg_send![self.textfield, text];
                let is_same_text = if !current_ns_text.is_null() {
                    let is_equal: BOOL = msg_send![current_ns_text, isEqualToString: ns_text];
                    is_equal != NO
                } else {
                    false
                };

                if !is_same_text {
                    msg_send_![self.textfield, setText: ns_text];
                }
                
                let beginning: ObjcId = msg_send![self.textfield, beginningOfDocument];
                if !beginning.is_null() {
                    let start_pos: ObjcId = msg_send![self.textfield, positionFromPosition: beginning offset: selection_start as i64];
                    let end_pos: ObjcId = msg_send![self.textfield, positionFromPosition: beginning offset: selection_end as i64];
                    if !start_pos.is_null() && !end_pos.is_null() {
                        let range: ObjcId = msg_send![self.textfield, textRangeFromPosition: start_pos toPosition: end_pos];
                        if !range.is_null() {
                            let current_range: ObjcId = msg_send![self.textfield, selectedTextRange];
                            let is_same_range: BOOL = if !current_range.is_null() {
                                msg_send![current_range, isEqual: range]
                            } else {
                                NO
                            };
                            if is_same_range == NO {
                                msg_send_![self.textfield, setSelectedTextRange: range];
                            }
                        }
                    }
                }
            },
            SetImePosition { .. } => {}
            SetImeEnabled(..) => {}
            _ => {}
        }
    }
}

fn send_message(message: Message) {
    MESSAGES_TX.with(|tx| {
        let mut tx = tx.borrow_mut();
        tx.as_mut().unwrap().send(message).unwrap();
    })
}

pub fn define_glk_or_mtk_view(superclass: &Class) -> *const Class {
    let mut decl = ClassDecl::new("QuadView", superclass).unwrap();

    fn on_touch(this: &Object, event: ObjcId, phase: TouchPhase) {
        unsafe {
            let enumerator: ObjcId = msg_send![event, allTouches];
            let size: u64 = msg_send![enumerator, count];
            let enumerator: ObjcId = msg_send![enumerator, objectEnumerator];

            for _ in 0..size {
                let ios_touch: ObjcId = msg_send![enumerator, nextObject];
                // Use the UITouch pointer as a stable ID instead of loop index
                let touch_id = ios_touch as u64;
                let mut ios_pos: NSPoint = msg_send![ios_touch, locationInView: this];

                if native_display().lock().unwrap().high_dpi {
                    let main_screen: ObjcId = msg_send![class!(UIScreen), mainScreen];
                    let scale: f64 = msg_send![main_screen, scale];

                    ios_pos.x *= scale;
                    ios_pos.y *= scale;
                } else {
                    let content_scale_factor: f64 = msg_send![this, contentScaleFactor];
                    ios_pos.x *= content_scale_factor;
                    ios_pos.y *= content_scale_factor;
                }

                send_message(Message::Touch {
                    phase,
                    touch_id,
                    x: ios_pos.x as f32,
                    y: ios_pos.y as f32,
                });
            }
        }
    }
    extern "C" fn touches_began(this: &Object, _: Sel, _: ObjcId, event: ObjcId) {
        on_touch(this, event, TouchPhase::Started);
    }

    extern "C" fn touches_moved(this: &Object, _: Sel, _: ObjcId, event: ObjcId) {
        on_touch(this, event, TouchPhase::Moved);
    }

    extern "C" fn touches_ended(this: &Object, _: Sel, _: ObjcId, event: ObjcId) {
        on_touch(this, event, TouchPhase::Ended);
    }

    extern "C" fn touches_canceled(this: &Object, _: Sel, _: ObjcId, event: ObjcId) {
        on_touch(this, event, TouchPhase::Cancelled);
    }

    unsafe {
        decl.add_method(sel!(isOpaque), yes as extern "C" fn(&Object, Sel) -> BOOL);
        decl.add_method(
            sel!(touchesBegan: withEvent:),
            touches_began as extern "C" fn(&Object, Sel, ObjcId, ObjcId),
        );
        decl.add_method(
            sel!(touchesMoved: withEvent:),
            touches_moved as extern "C" fn(&Object, Sel, ObjcId, ObjcId),
        );
        decl.add_method(
            sel!(touchesEnded: withEvent:),
            touches_ended as extern "C" fn(&Object, Sel, ObjcId, ObjcId),
        );
        decl.add_method(
            sel!(touchesCanceled: withEvent:),
            touches_canceled as extern "C" fn(&Object, Sel, ObjcId, ObjcId),
        );
    }

    decl.add_ivar::<*mut c_void>("display_ptr");
    decl.register()
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
            b"/System/Library/Frameworks/OpenGLES.framework/OpenGLES\0".as_ptr() as _,
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

pub fn define_glk_or_mtk_view_dlg(superclass: &Class) -> *const Class {
    let mut decl = ClassDecl::new("QuadViewDlg", superclass).unwrap();

    extern "C" fn draw_in_rect(this: &Object, _: Sel, _: ObjcId, _: ObjcId) {
        static DRAW_ENTER_COUNT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        let draw_count = DRAW_ENTER_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        if draw_count == 0 || draw_count == 60 || draw_count % 300 == 0 {
            log(&format!("[miniquad] draw_in_rect entered (call count: {})", draw_count));
        }

        let payload = get_window_payload(this);
        if payload.event_handler.is_none() {
            log("[miniquad] draw_in_rect: initializing event handler");
            payload.init_event_handler();
        }

        // Drain requests + UIKit-side messages and dispatch inline
        // before drawing this frame.
        while let Ok(request) = payload.requests_rx.try_recv() {
            payload.process_request(request);
        }
        while let Ok(msg) = payload.messages_rx.try_recv() {
            dispatch_message(payload, msg);
        }

        // Skip the draw if paused. `CADisplayLink` keeps ticking cheaply.
        if payload.state.lock().unwrap().paused {
            return;
        }

        // Measure the view, not the device screen — iOS-on-Mac
        // windowed mode has view < UIScreen.
        let view_bounds: NSRect = unsafe { msg_send![payload.view, bounds] };
        let content_scale_factor: f64 =
            unsafe { msg_send![payload.view, contentScaleFactor] };
        let screen_width = (view_bounds.size.width * content_scale_factor) as i32;
        let screen_height = (view_bounds.size.height * content_scale_factor) as i32;
        let dpi_scale = content_scale_factor as f32;

        let needs_update = {
            let d = native_display().lock().unwrap();
            d.screen_width != screen_width
                || d.screen_height != screen_height
                || d.dpi_scale != dpi_scale
        };
        if needs_update {
            {
                let mut d = native_display().lock().unwrap();
                d.screen_width = screen_width;
                d.screen_height = screen_height;
                d.dpi_scale = dpi_scale;
            }
            send_message(Message::Resize {
                width: screen_width,
                height: screen_height,
            });
        }

        if let Some(ref mut event_handler) = payload.event_handler {
            event_handler.update();
            event_handler.draw();
            let mut s = payload.state.lock().unwrap();
            s.update_requested = false;
        }
    }
    // wrapper to make sel! macros happy
    extern "C" fn draw_in_rect2(this: &Object, s: Sel, o: ObjcId) {
        draw_in_rect(this, s, o, nil);
    }

    // `MTKViewDelegate` requires this alongside `drawInMTKView:`;
    // missing it crashes `_resizeDrawable` with
    // `NSInvalidArgumentException`. Sync `native_display` here so
    // the next frame's `setScissorRect` (and anything else reading
    // `screen_size`) matches the new drawable — `draw_in_rect`'s
    // `UIScreen.bounds` poll is the fallback but lags drawableSize
    // during rotation animations.
    extern "C" fn drawable_size_will_change(_: &Object, _: Sel, _: ObjcId, size: NSSize) {
        let width = size.width as i32;
        let height = size.height as i32;
        let changed = {
            let mut display = native_display().lock().unwrap();
            let changed =
                display.screen_width != width || display.screen_height != height;
            if changed {
                display.screen_width = width;
                display.screen_height = height;
            }
            changed
        };
        if changed {
            send_message(Message::Resize { width, height });
        }
    }

    extern "C" fn on_display_link(this: &Object, _: Sel, _: ObjcId) {
        let ptr: *mut c_void = unsafe { *this.get_ivar("display_ptr") };
        if ptr.is_null() {
            return;
        }
        let payload = unsafe { &mut *(ptr as *mut IosDisplay) };

        while let Ok(msg) = payload.messages_rx.try_recv() {
            dispatch_message(payload, msg);
        }
        while let Ok(request) = payload.requests_rx.try_recv() {
            payload.process_request(request);
        }

        let paused = payload.state.lock().unwrap().paused;
        if paused {
            return;
        }

        if payload.blocking_event_loop && !payload.state.lock().unwrap().update_requested {
            return;
        }

        unsafe {
            match payload.gfx_api {
                AppleGfxApi::Metal => {
                    let _: () = msg_send![payload.view, draw];
                }
                AppleGfxApi::OpenGl => {
                    let _: () = msg_send![payload.view, display];
                }
            }
        }
    }

    unsafe {
        decl.add_method(
            sel!(glkView: drawInRect:),
            draw_in_rect as extern "C" fn(&Object, Sel, ObjcId, ObjcId),
        );

        decl.add_method(
            sel!(drawInMTKView:),
            draw_in_rect2 as extern "C" fn(&Object, Sel, ObjcId),
        );

        decl.add_method(
            sel!(mtkView: drawableSizeWillChange:),
            drawable_size_will_change as extern "C" fn(&Object, Sel, ObjcId, NSSize),
        );

        decl.add_method(
            sel!(onDisplayLink:),
            on_display_link as extern "C" fn(&Object, Sel, ObjcId),
        );
    }

    decl.add_ivar::<*mut c_void>("display_ptr");
    decl.register()
}

static KEYBOARD_HEIGHT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

fn set_keyboard_height(h: f64) {
    KEYBOARD_HEIGHT.store(h.to_bits(), std::sync::atomic::Ordering::Relaxed);
}

fn get_keyboard_height() -> f64 {
    f64::from_bits(KEYBOARD_HEIGHT.load(std::sync::atomic::Ordering::Relaxed))
}

fn get_keyboard_height_from_notif(notif: ObjcId) -> f64 {
    unsafe {
        let user_info: ObjcId = msg_send![notif, userInfo];
        if user_info.is_null() {
            return 0.0;
        }
        let frame_key = apple_util::str_to_nsstring("UIKeyboardFrameEndUserInfoKey");
        let val: ObjcId = msg_send![user_info, objectForKey: frame_key];
        if val.is_null() {
            return 0.0;
        }
        let rect: NSRect = msg_send![val, CGRectValue];
        let main_screen: ObjcId = msg_send![class!(UIScreen), mainScreen];
        let screen_rect: NSRect = msg_send![main_screen, bounds];
        if rect.origin.y < screen_rect.size.height {
            rect.size.height
        } else {
            0.0
        }
    }
}

pub fn define_view_controller() -> *const Class {
    let superclass = class!(UIViewController);
    let mut decl = ClassDecl::new("QuadViewController", superclass).unwrap();

    extern "C" fn view_did_layout_subviews(this: &Object, _: Sel) {
        unsafe {
            let view: ObjcId = msg_send![this, view];
            let subviews: ObjcId = msg_send![view, subviews];
            let count: u64 = msg_send![subviews, count];
            if count > 0 {
                let inner_view: ObjcId = msg_send![subviews, objectAtIndex: 0];
                let bounds: NSRect = msg_send![view, bounds];
                let insets: UIEdgeInsets = msg_send![view, safeAreaInsets];
                let top_inset = insets.top;
                let kb_height = get_keyboard_height();
                let safe_frame = NSRect {
                    origin: NSPoint {
                        x: bounds.origin.x,
                        y: bounds.origin.y + top_inset,
                    },
                    size: NSSize {
                        width: bounds.size.width,
                        height: (bounds.size.height - top_inset - kb_height).max(0.0),
                    },
                };
                let _: () = msg_send![inner_view, setFrame: safe_frame];
            }
        }
    }

    extern "C" fn preferred_status_bar_style(_: &Object, _: Sel) -> i64 {
        1 // UIStatusBarStyleLightContent
    }

    unsafe {
        decl.add_method(
            sel!(viewDidLayoutSubviews),
            view_did_layout_subviews as extern "C" fn(&Object, Sel),
        );
        decl.add_method(
            sel!(preferredStatusBarStyle),
            preferred_status_bar_style as extern "C" fn(&Object, Sel) -> i64,
        );
    }
    decl.register()
}

// metal or opengl view and the objects required to collect all the window events
struct View {
    view: ObjcId,
    view_dlg: ObjcId,
    view_ctrl: ObjcId,
    display_link: ObjcId,
    // this view failed to create gles3 context, but succeeded with gles2
    _gles2: bool,
}

unsafe fn create_opengl_view(screen_rect: NSRect, _sample_count: i32, high_dpi: bool) -> View {
    let container_view: ObjcId = msg_send![class!(UIView), alloc];
    let container_view: ObjcId = msg_send![container_view, initWithFrame: screen_rect];
    let black_color: ObjcId = msg_send![class!(UIColor), blackColor];
    let _: () = msg_send![container_view, setBackgroundColor: black_color];

    let glk_view_obj: ObjcId = msg_send![define_glk_or_mtk_view(class!(GLKView)), alloc];
    let glk_view_obj: ObjcId = msg_send![glk_view_obj, initWithFrame: screen_rect];

    let glk_view_dlg_obj: ObjcId = msg_send![define_glk_or_mtk_view_dlg(class!(NSObject)), alloc];
    let glk_view_dlg_obj: ObjcId = msg_send![glk_view_dlg_obj, init];

    let eagl_context_obj: ObjcId = msg_send![class!(EAGLContext), alloc];
    let mut eagl_context_obj: ObjcId = msg_send![eagl_context_obj, initWithAPI: 3];
    let mut gles2 = false;
    if eagl_context_obj.is_null() {
        eagl_context_obj = msg_send![eagl_context_obj, initWithAPI: 2];
        gles2 = true;
    }

    msg_send_![
        glk_view_obj,
        setDrawableColorFormat: frameworks::GLKViewDrawableColorFormatRGBA8888
    ];
    msg_send_![
        glk_view_obj,
        setDrawableDepthFormat: frameworks::GLKViewDrawableDepthFormat::Format24 as i32
    ];
    msg_send_![
        glk_view_obj,
        setDrawableStencilFormat: frameworks::GLKViewDrawableStencilFormat::FormatNone as i32
    ];
    msg_send_![glk_view_obj, setContext: eagl_context_obj];

    msg_send_![glk_view_obj, setDelegate: glk_view_dlg_obj];
    msg_send_![glk_view_obj, setEnableSetNeedsDisplay: YES];
    msg_send_![glk_view_obj, setUserInteractionEnabled: YES];
    msg_send_![glk_view_obj, setMultipleTouchEnabled: YES];
    if high_dpi {
        let main_screen: ObjcId = msg_send![class!(UIScreen), mainScreen];
        let scale: f64 = msg_send![main_screen, scale];

        msg_send_![glk_view_obj, setContentScaleFactor: scale];
    } else {
        msg_send_![glk_view_obj, setContentScaleFactor: 1.0];
    }

    let view_ctrl_obj: ObjcId = msg_send![define_view_controller(), alloc];
    let view_ctrl_obj: ObjcId = msg_send![view_ctrl_obj, init];

    let _: () = msg_send![container_view, addSubview: glk_view_obj];
    let _: () = msg_send![view_ctrl_obj, setView: container_view];

    let display_link: ObjcId = msg_send![
        class!(CADisplayLink),
        displayLinkWithTarget: glk_view_dlg_obj
        selector: sel!(onDisplayLink:)
    ];
    let main_run_loop: ObjcId = msg_send![class!(NSRunLoop), mainRunLoop];
    let common_modes = if frameworks::NSRunLoopCommonModes.is_null() {
        apple_util::str_to_nsstring("kCFRunLoopCommonModes")
    } else {
        frameworks::NSRunLoopCommonModes
    };
    msg_send_![display_link, addToRunLoop: main_run_loop forMode: common_modes];

    View {
        view: glk_view_obj,
        view_dlg: glk_view_dlg_obj,
        view_ctrl: view_ctrl_obj,
        display_link,
        _gles2: gles2,
    }
}

unsafe fn create_metal_view(screen_rect: NSRect, sample_count: i32, _high_dpi: bool) -> View {
    let container_view: ObjcId = msg_send![class!(UIView), alloc];
    let container_view: ObjcId = msg_send![container_view, initWithFrame: screen_rect];
    let black_color: ObjcId = msg_send![class!(UIColor), blackColor];
    let _: () = msg_send![container_view, setBackgroundColor: black_color];

    let mtk_view_obj: ObjcId = msg_send![define_glk_or_mtk_view(class!(MTKView)), alloc];
    let mtk_view_obj: ObjcId = msg_send![mtk_view_obj, initWithFrame: screen_rect];

    let mtk_view_dlg_obj: ObjcId = msg_send![define_glk_or_mtk_view_dlg(class!(NSObject)), alloc];
    let mtk_view_dlg_obj: ObjcId = msg_send![mtk_view_dlg_obj, init];

    let view_ctrl_obj: ObjcId = msg_send![define_view_controller(), alloc];
    let view_ctrl_obj: ObjcId = msg_send![view_ctrl_obj, init];

    let _: () = msg_send![container_view, addSubview: mtk_view_obj];
    let _: () = msg_send![view_ctrl_obj, setView: container_view];

    msg_send_![mtk_view_obj, setEnableSetNeedsDisplay: NO];
    msg_send_![mtk_view_obj, setPaused: YES];
    msg_send_![mtk_view_obj, setPreferredFramesPerSecond: 60];
    msg_send_![mtk_view_obj, setDelegate: mtk_view_dlg_obj];
    let device = MTLCreateSystemDefaultDevice();
    msg_send_![mtk_view_obj, setDevice: device];
    msg_send_![mtk_view_obj, setUserInteractionEnabled: YES];
    msg_send_![mtk_view_obj, setMultipleTouchEnabled: YES];

    if _high_dpi {
        let main_screen: ObjcId = msg_send![class!(UIScreen), mainScreen];
        let scale: f64 = msg_send![main_screen, scale];
        msg_send_![mtk_view_obj, setContentScaleFactor: scale];
    } else {
        msg_send_![mtk_view_obj, setContentScaleFactor: 1.0];
    }

    if sample_count > 1 {
        msg_send_![mtk_view_obj, setSampleCount: sample_count as u64];
    }

    let display_link: ObjcId = msg_send![
        class!(CADisplayLink),
        displayLinkWithTarget: mtk_view_dlg_obj
        selector: sel!(onDisplayLink:)
    ];
    let main_run_loop: ObjcId = msg_send![class!(NSRunLoop), mainRunLoop];
    let common_modes = if frameworks::NSRunLoopCommonModes.is_null() {
        apple_util::str_to_nsstring("kCFRunLoopCommonModes")
    } else {
        frameworks::NSRunLoopCommonModes
    };
    msg_send_![display_link, addToRunLoop: main_run_loop forMode: common_modes];

    View {
        view: mtk_view_obj,
        view_dlg: mtk_view_dlg_obj,
        view_ctrl: view_ctrl_obj,
        display_link,
        _gles2: false,
    }
}

struct IosClipboard;
impl crate::native::Clipboard for IosClipboard {
    fn get(&mut self) -> Option<String> {
        unsafe {
            let pasteboard: ObjcId = msg_send![class!(UIPasteboard), generalPasteboard];
            if pasteboard.is_null() {
                return None;
            }
            let ns_string: ObjcId = msg_send![pasteboard, string];
            if ns_string.is_null() {
                None
            } else {
                Some(apple_util::nsstring_to_string(ns_string))
            }
        }
    }
    fn set(&mut self, data: &str) {
        unsafe {
            let pasteboard: ObjcId = msg_send![class!(UIPasteboard), generalPasteboard];
            if !pasteboard.is_null() {
                let ns_string = apple_util::str_to_nsstring(data);
                let _: () = msg_send![pasteboard, setString: ns_string];
            }
        }
    }
}

pub fn define_app_delegate() -> *const Class {
    let superclass = class!(NSObject);
    let mut decl = ClassDecl::new("NSAppDelegate", superclass).unwrap();
    decl.add_ivar::<ObjcId>("window");

    extern "C" fn get_window(this: &Object, _: Sel) -> ObjcId {
        unsafe { *this.get_ivar("window") }
    }

    extern "C" fn set_window(this: &mut Object, _: Sel, window: ObjcId) {
        unsafe {
            this.set_ivar("window", window);
        }
    }

    extern "C" fn did_finish_launching_with_options(
        delegate_self: &Object,
        _: Sel,
        _: ObjcId,
        _: ObjcId,
    ) -> BOOL {
        unsafe {
            // Routed through a raw pointer to satisfy the Rust 2024
            // `static_mut_refs` lint. Split across two statements so
            // clippy's `deref_addrof` doesn't fold it back.
            let run_args_ptr = &raw mut RUN_ARGS;
            let (f, conf) = (*run_args_ptr).take().unwrap();

            let main_screen: ObjcId = msg_send![class!(UIScreen), mainScreen];
            let screen_rect: NSRect = msg_send![main_screen, bounds];

            let scale: f64 = if conf.high_dpi {
                msg_send![main_screen, scale]
            } else {
                1.0
            };
            let screen_width = (screen_rect.size.width * scale) as i32;
            let screen_height = (screen_rect.size.height * scale) as i32;

            // Window creation defers to `scene_will_connect`; view +
            // view_ctrl built now so the render thread has them ready.
            let view = match conf.platform.apple_gfx_api {
                AppleGfxApi::OpenGl => {
                    create_opengl_view(screen_rect, conf.sample_count, conf.high_dpi)
                }
                AppleGfxApi::Metal => {
                    create_metal_view(screen_rect, conf.sample_count, conf.high_dpi)
                }
            };

            let (textfield_dlg, textfield) = {
                let textview_dlg = msg_send_![msg_send_![define_textview_dlg(), alloc], init];
                let custom_class = define_hidden_textview();
                let textview = msg_send_![
                    msg_send_![custom_class, alloc],
                    initWithFrame: screen_rect];
                msg_send_![textview, setAutocapitalizationType: 0]; // UITextAutocapitalizationTypeNone
                msg_send_![textview, setAutocorrectionType: 2]; // UITextAutocorrectionTypeYes / Default
                msg_send_![textview, setSpellCheckingType: 2]; // UITextSpellCheckingTypeYes / Default
                
                let font: ObjcId = msg_send![class!(UIFont), systemFontOfSize: 16.0f64];
                if !font.is_null() {
                    let _: () = msg_send![textview, setFont: font];
                }

                let clear_color: ObjcId = msg_send![class!(UIColor), clearColor];
                msg_send_![textview, setTintColor: clear_color];
                msg_send_![textview, setTextColor: clear_color];
                msg_send_![textview, setBackgroundColor: clear_color];
                msg_send_![textview, setDelegate: textview_dlg];
                msg_send_![textview, setText: apple_util::str_to_nsstring("")];
                msg_send_![view.view, addSubview: textview];

                let long_press_recognizer = msg_send_![
                    msg_send_![class!(UILongPressGestureRecognizer), alloc],
                    initWithTarget: textview_dlg
                    action: sel!(handleLongPress:)
                ];
                msg_send_![long_press_recognizer, setMinimumPressDuration: 0.45f64];
                msg_send_![long_press_recognizer, setCancelsTouchesInView: NO];
                msg_send_![view.view, addGestureRecognizer: long_press_recognizer];

                let notification_center = msg_send_![class!(NSNotificationCenter), defaultCenter];
                let will_show = apple_util::str_to_nsstring("UIKeyboardWillShowNotification");
                let will_hide = apple_util::str_to_nsstring("UIKeyboardWillHideNotification");
                let will_change = apple_util::str_to_nsstring("UIKeyboardWillChangeFrameNotification");
                let did_show = apple_util::str_to_nsstring("UIKeyboardDidShowNotification");
                let did_change = apple_util::str_to_nsstring("UIKeyboardDidChangeFrameNotification");

                msg_send_![notification_center, addObserver:textview_dlg
                           selector:sel!(keyboardWasShown:)
                           name:will_show object:nil];
                msg_send_![notification_center, addObserver:textview_dlg
                           selector:sel!(keyboardWillBeHidden:)
                           name:will_hide object:nil];
                msg_send_![notification_center, addObserver:textview_dlg
                           selector:sel!(keyboardDidChangeFrame:)
                           name:will_change object:nil];
                msg_send_![notification_center, addObserver:textview_dlg
                           selector:sel!(keyboardWasShown:)
                           name:did_show object:nil];
                msg_send_![notification_center, addObserver:textview_dlg
                           selector:sel!(keyboardDidChangeFrame:)
                           name:did_change object:nil];
                let text_did_change_name = apple_util::str_to_nsstring("UITextViewTextDidChangeNotification");
                msg_send_![notification_center, addObserver:textview_dlg
                           selector:sel!(textViewDidChangeNotification:)
                           name:text_did_change_name object:textview];
                let selection_did_change_name = apple_util::str_to_nsstring("UITextViewTextDidChangeSelectionNotification");
                msg_send_![notification_center, addObserver:textview_dlg
                           selector:sel!(textViewDidChangeSelectionNotification:)
                           name:selection_did_change_name object:textview];
                (textview_dlg, textview)
            };

            let (tx, rx) = std::sync::mpsc::channel();

            MESSAGES_TX.with(move |messages_tx| *messages_tx.borrow_mut() = Some(tx));

            let clipboard = Box::new(IosClipboard);
            let (tx, requests_rx) = std::sync::mpsc::channel();
            crate::set_display(NativeDisplayData {
                high_dpi: conf.high_dpi,
                dpi_scale: scale as f32,
                gfx_api: conf.platform.apple_gfx_api,
                blocking_event_loop: conf.platform.blocking_event_loop,
                view: view.view,
                ..NativeDisplayData::new(screen_width, screen_height, tx, clipboard)
            });

            let state_original = Arc::new(Mutex::new(MainThreadState {
                quit: false,
                paused: false,
                update_requested: true,
                view: view.view,
                keymods: KeyMods {
                    shift: false,
                    ctrl: false,
                    alt: false,
                    logo: false,
                },
            }));

            let payload = Box::new(IosDisplay {
                view: view.view,
                view_dlg: view.view_dlg,
                view_ctrl: view.view_ctrl,
                display_link: view.display_link,
                textfield,
                _textfield_dlg: textfield_dlg,
                current_element_id: 0,
                gfx_api: conf.platform.apple_gfx_api,

                f: Some(Box::new(f)),
                event_handler: None,
                _gles2: view._gles2,
                state: state_original.clone(),
                messages_rx: rx,
                requests_rx,
                blocking_event_loop: conf.platform.blocking_event_loop,
            });
            let payload_ptr = Box::into_raw(payload) as *mut std::ffi::c_void;

            (*view.view).set_ivar("display_ptr", payload_ptr);
            (*view.view_dlg).set_ivar("display_ptr", payload_ptr);
            (*textfield_dlg).set_ivar("display_ptr", payload_ptr);

            let window: ObjcId = msg_send![class!(UIWindow), alloc];
            let window: ObjcId = msg_send![window, initWithFrame: screen_rect];
            msg_send_![window, setRootViewController: view.view_ctrl];
            msg_send_![window, makeKeyAndVisible];
            let delegate_mut = delegate_self as *const Object as *mut Object;
            (*delegate_mut).set_ivar("window", window);
            log("[miniquad] didFinishLaunchingWithOptions: created window & makeKeyAndVisible");
        }
        YES
    }

    extern "C" fn application_did_become_active(_: &Object, _: Sel, _: ObjcId) {
        send_message(Message::Resume);
    }

    extern "C" fn application_will_resign_active(_: &Object, _: Sel, _: ObjcId) {
        send_message(Message::Pause);
    }

    unsafe {
        decl.add_method(
            sel!(window),
            get_window as extern "C" fn(&Object, Sel) -> ObjcId,
        );
        decl.add_method(
            sel!(setWindow:),
            set_window as extern "C" fn(&mut Object, Sel, ObjcId),
        );
        decl.add_method(
            sel!(application: didFinishLaunchingWithOptions:),
            did_finish_launching_with_options
                as extern "C" fn(&Object, Sel, ObjcId, ObjcId) -> BOOL,
        );
        decl.add_method(
            sel!(applicationDidBecomeActive:),
            application_did_become_active as extern "C" fn(&Object, Sel, ObjcId),
        );
        decl.add_method(
            sel!(applicationWillResignActive:),
            application_will_resign_active as extern "C" fn(&Object, Sel, ObjcId),
        );
    }
    decl.register()
}

fn define_hidden_textview() -> *const Class {
    let superclass = class!(UITextView);
    let mut decl = ClassDecl::new("QuadHiddenTextView", superclass).unwrap();

    // Allow touches to pass through directly to the underlying QuadView
    extern "C" fn hit_test(_: &Object, _: Sel, _: NSPoint, _: ObjcId) -> ObjcId {
        nil
    }

    // Support standard context menu actions
    extern "C" fn can_perform_action(_: &Object, _: Sel, action: Sel, _: ObjcId) -> BOOL {
        if action == sel!(copy:)
            || action == sel!(paste:)
            || action == sel!(cut:)
            || action == sel!(selectAll:)
            || action == sel!(select:)
        {
            YES
        } else {
            NO
        }
    }

    unsafe {
        decl.add_method(
            sel!(hitTest:withEvent:),
            hit_test as extern "C" fn(&Object, Sel, NSPoint, ObjcId) -> ObjcId,
        );
        decl.add_method(
            sel!(canPerformAction:withSender:),
            can_perform_action as extern "C" fn(&Object, Sel, Sel, ObjcId) -> BOOL,
        );
    }
    decl.register()
}

fn sync_textview_state(this: &Object, textview: ObjcId) {
    unsafe {
        if textview.is_null() {
            return;
        }
        let ns_text: ObjcId = msg_send![textview, text];
        let text = if ns_text.is_null() {
            String::new()
        } else {
            apple_util::nsstring_to_string(ns_text)
        };

        let marked_range: ObjcId = msg_send![textview, markedTextRange];
        let (composing_start, composing_end, preedit_str) = if !marked_range.is_null() {
            let beginning: ObjcId = msg_send![textview, beginningOfDocument];
            let start_pos: ObjcId = msg_send![marked_range, start];
            let end_pos: ObjcId = msg_send![marked_range, end];
            let start_offset: i64 = msg_send![textview, offsetFromPosition: beginning toPosition: start_pos];
            let end_offset: i64 = msg_send![textview, offsetFromPosition: beginning toPosition: end_pos];

            let ns_preedit: ObjcId = msg_send![textview, textInRange: marked_range];
            let preedit = if !ns_preedit.is_null() {
                apple_util::nsstring_to_string(ns_preedit)
            } else {
                String::new()
            };

            let cs = if start_offset >= 0 { start_offset as usize } else { 0 };
            let ce = if end_offset >= 0 { end_offset as usize } else { cs };
            (Some(cs), Some(ce), preedit)
        } else {
            (None, None, String::new())
        };

        let selected_range: ObjcId = msg_send![textview, selectedTextRange];
        let (sel_start, sel_end) = if !selected_range.is_null() {
            let beginning: ObjcId = msg_send![textview, beginningOfDocument];
            let start_pos: ObjcId = msg_send![selected_range, start];
            let end_pos: ObjcId = msg_send![selected_range, end];
            let start_offset: i64 = msg_send![textview, offsetFromPosition: beginning toPosition: start_pos];
            let end_offset: i64 = msg_send![textview, offsetFromPosition: beginning toPosition: end_pos];
            let s_start = if start_offset >= 0 { start_offset as usize } else { 0 };
            let s_end = if end_offset >= 0 { end_offset as usize } else { s_start };
            (s_start, s_end)
        } else {
            let u16_len = text.encode_utf16().count();
            (u16_len, u16_len)
        };

        let element_id = {
            let ptr: *mut c_void = *this.get_ivar("display_ptr");
            if !ptr.is_null() {
                let payload = &mut *(ptr as *mut IosDisplay);
                payload.current_element_id
            } else {
                0
            }
        };

        send_message(Message::ImePreedit(preedit_str));

        send_message(Message::ImeStateChanged {
            text,
            selection_start: sel_start,
            selection_end: sel_end,
            composing_start,
            composing_end,
            element_id,
        });
    }
}

fn define_textview_dlg() -> *const Class {
    let superclass = class!(NSObject);
    let mut decl = ClassDecl::new("QuadTextViewDlg", superclass).unwrap();

    extern "C" fn update_keyboard_layout(this: &Object, notif: ObjcId, is_hide: bool) {
        let height = if is_hide {
            0.0
        } else {
            get_keyboard_height_from_notif(notif)
        };
        set_keyboard_height(height);
        let ptr: *mut c_void = unsafe { *this.get_ivar("display_ptr") };
        if !ptr.is_null() {
            let payload = unsafe { &mut *(ptr as *mut IosDisplay) };
            unsafe {
                let view: ObjcId = msg_send![payload.view_ctrl, view];
                let _: () = msg_send![view, setNeedsLayout];
                let _: () = msg_send![view, layoutIfNeeded];
            }
        }
    }

    extern "C" fn keyboard_was_shown(this: &Object, _: Sel, notif: ObjcId) {
        update_keyboard_layout(this, notif, false);
    }
    extern "C" fn keyboard_will_be_hidden(this: &Object, _: Sel, notif: ObjcId) {
        update_keyboard_layout(this, notif, true);
    }
    extern "C" fn keyboard_did_change_frame(this: &Object, _: Sel, notif: ObjcId) {
        update_keyboard_layout(this, notif, false);
    }

    extern "C" fn text_view_did_change_notif(this: &Object, _: Sel, notif: ObjcId) {
        unsafe {
            let textview: ObjcId = msg_send![notif, object];
            sync_textview_state(this, textview);
        }
    }

    extern "C" fn text_view_did_change_selection_notif(this: &Object, _: Sel, notif: ObjcId) {
        unsafe {
            let textview: ObjcId = msg_send![notif, object];
            sync_textview_state(this, textview);
        }
    }

    extern "C" fn text_view_delegate_did_change(this: &Object, _: Sel, textview: ObjcId) {
        sync_textview_state(this, textview);
    }

    extern "C" fn text_view_delegate_did_change_selection(this: &Object, _: Sel, textview: ObjcId) {
        sync_textview_state(this, textview);
    }

    extern "C" fn should_change_text_in_range(
        _: &Object,
        _: Sel,
        _textview: ObjcId,
        _range: NSRange,
        _text: ObjcId,
    ) -> BOOL {
        YES
    }

    extern "C" fn handle_long_press(this: &Object, _: Sel, recognizer: ObjcId) {
        unsafe {
            let state: i64 = msg_send![recognizer, state];
            // UIGestureRecognizerStateBegan == 1
            if state == 1 {
                let ptr: *mut c_void = *this.get_ivar("display_ptr");
                if !ptr.is_null() {
                    let payload = &mut *(ptr as *mut IosDisplay);
                    if payload.current_element_id != 0 {
                        let point: NSPoint = msg_send![recognizer, locationInView: payload.view];
                        let menu: ObjcId = msg_send![class!(UIMenuController), sharedMenuController];
                        let target_rect = NSRect::new(point.x, point.y, 1.0, 1.0);
                        let _: () = msg_send![menu, setTargetRect: target_rect inView: payload.view];
                        let _: () = msg_send![menu, setMenuVisible: YES animated: YES];
                    }
                }
            }
        }
    }

    unsafe {
        decl.add_method(
            sel!(keyboardWasShown:),
            keyboard_was_shown as extern "C" fn(&Object, Sel, ObjcId),
        );
        decl.add_method(
            sel!(keyboardWillBeHidden:),
            keyboard_will_be_hidden as extern "C" fn(&Object, Sel, ObjcId),
        );
        decl.add_method(
            sel!(keyboardDidChangeFrame:),
            keyboard_did_change_frame as extern "C" fn(&Object, Sel, ObjcId),
        );
        decl.add_method(
            sel!(textViewDidChangeNotification:),
            text_view_did_change_notif as extern "C" fn(&Object, Sel, ObjcId),
        );
        decl.add_method(
            sel!(textViewDidChangeSelectionNotification:),
            text_view_did_change_selection_notif as extern "C" fn(&Object, Sel, ObjcId),
        );
        decl.add_method(
            sel!(textViewDidChange:),
            text_view_delegate_did_change as extern "C" fn(&Object, Sel, ObjcId),
        );
        decl.add_method(
            sel!(textViewDidChangeSelection:),
            text_view_delegate_did_change_selection as extern "C" fn(&Object, Sel, ObjcId),
        );
        decl.add_method(
            sel!(textView: shouldChangeTextInRange: replacementText:),
            should_change_text_in_range
                as extern "C" fn(&Object, Sel, ObjcId, NSRange, ObjcId) -> BOOL,
        );
        decl.add_method(
            sel!(handleLongPress:),
            handle_long_press as extern "C" fn(&Object, Sel, ObjcId),
        );
    }
    decl.add_ivar::<*mut c_void>("display_ptr");
    decl.register()
}

pub fn log(message: &str) {
    let nsstring = apple_util::str_to_nsstring(message);
    let _: () = unsafe { frameworks::NSLog(nsstring) };
}

pub fn load_file<F: Fn(crate::fs::Response) + 'static>(path: &str, on_loaded: F) {
    let path = std::path::Path::new(&path);
    let path_without_extension = path.with_extension("");
    let path_without_extension = path_without_extension.to_str().unwrap();
    let extension = path.extension().unwrap_or_default().to_str().unwrap();

    unsafe {
        let nsstring = apple_util::str_to_nsstring(&format!(
            "loading: {} {}",
            path_without_extension, extension
        ));
        let _: () = frameworks::NSLog(nsstring);

        let main_bundle: ObjcId = msg_send![class!(NSBundle), mainBundle];
        let resource = apple_util::str_to_nsstring(path_without_extension);
        let type_ = apple_util::str_to_nsstring(extension);
        let file_path: ObjcId = msg_send![main_bundle, pathForResource:resource ofType:type_];
        if file_path.is_null() {
            on_loaded(Err(fs::Error::IOSAssetNoSuchFile));
            return;
        }
        let file_data: ObjcId = msg_send![class!(NSData), dataWithContentsOfFile: file_path];
        if file_data.is_null() {
            on_loaded(Err(fs::Error::IOSAssetNoData));
            return;
        }
        let bytes: *mut u8 = msg_send![file_data, bytes];
        if bytes.is_null() {
            on_loaded(Err(fs::Error::IOSAssetNoData));
            return;
        }
        let length: usize = msg_send![file_data, length];
        let slice = std::slice::from_raw_parts(bytes, length);
        on_loaded(Ok(slice.to_vec()))
    }
}

// this is the way to pass argument to UiApplicationMain
// this static will be used exactly once, to .take() the "run" arguments
#[allow(clippy::type_complexity)]
static mut RUN_ARGS: Option<(Box<dyn FnOnce() -> Box<dyn EventHandler>>, Conf)> = None;

pub unsafe fn run<F>(conf: Conf, f: F)
where
    F: 'static + FnOnce() -> Box<dyn EventHandler>,
{
    RUN_ARGS = Some((Box::new(f), conf));

    std::panic::set_hook(Box::new(|info| {
        let nsstring = apple_util::str_to_nsstring(&format!("{:?}", info));
        let _: () = frameworks::NSLog(nsstring);
    }));

    let argc = 1;
    let mut argv = b"Miniquad\0" as *const u8 as *mut i8;

    let class: ObjcId = msg_send!(define_app_delegate(), class);
    let class_string = frameworks::NSStringFromClass(class as _);

    UIApplicationMain(argc, &mut argv, nil, class_string);
}
