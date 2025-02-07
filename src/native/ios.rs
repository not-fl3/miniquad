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
        thread::{self},
    },
};

struct MainThreadState {
    quit: bool,
    paused: bool,
    update_requested: bool,
    view: *mut Object,
    keymods: KeyMods,
    cur_msg: Message,
}

struct IosDisplay {
    view: ObjcId,
    view_ctrl: ObjcId,
    _textfield_dlg: ObjcId,
    textfield: ObjcId,
    gfx_api: conf::AppleGfxApi,

    event_handler: Option<Box<dyn EventHandler>>,
    _gles2: bool,
    f: Option<Box<dyn 'static + FnOnce() -> Box<dyn EventHandler>>>,
    state: Arc<Mutex<MainThreadState>>,
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

#[derive(Debug, Clone, Copy)]
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
}
unsafe impl Send for Message {}

thread_local! {
    static MESSAGES_TX: RefCell<Option<mpsc::Sender<Message>>> = const { RefCell::new(None) };
}

impl MainThreadState {
    fn process_request(&mut self, request: crate::native::Request) {
        use crate::native::Request::*;

        if let ScheduleUpdate = request {
            self.update_requested = true;
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

            for touch_id in 0..size {
                let ios_touch: ObjcId = msg_send![enumerator, nextObject];
                let mut ios_pos: NSPoint = msg_send![ios_touch, locationInView: this];

                if native_display().lock().unwrap().high_dpi {
                    ios_pos.x *= 2.;
                    ios_pos.y *= 2.;
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

    extern "C" fn process_message(this: &Object, _: Sel, _: ObjcId) {
        let payload = get_window_payload(this);
        if payload.event_handler.is_none() {
            payload.init_event_handler();
        }
        let msg = {
            let state = payload.state.lock().unwrap();
            state.cur_msg
        };
        match msg {
            Message::Pause => {
                let mut state = payload.state.lock().unwrap();
                state.paused = true;
            }
            Message::Resume => {
                let mut state = payload.state.lock().unwrap();
                state.paused = false;
            }
            Message::Destroy => {
                let mut state = payload.state.lock().unwrap();
                state.quit = true;
            }
            Message::Touch {
                phase,
                touch_id,
                x,
                y,
            } => {
                if let Some(ref mut event_handler) = payload.event_handler {
                    event_handler.touch_event(phase, touch_id, x, y);
                }
            }
            Message::Character { character } => {
                if let Some(character) = char::from_u32(character) {
                    if let Some(ref mut event_handler) = payload.event_handler {
                        event_handler.char_event(character, Default::default(), false);
                    }
                }
            }
            Message::KeyDown { keycode } => {
                let mut state = payload.state.lock().unwrap();
                match keycode {
                    KeyCode::LeftShift | KeyCode::RightShift => state.keymods.shift = true,
                    KeyCode::LeftControl | KeyCode::RightControl => state.keymods.ctrl = true,
                    KeyCode::LeftAlt | KeyCode::RightAlt => state.keymods.alt = true,
                    KeyCode::LeftSuper | KeyCode::RightSuper => state.keymods.logo = true,
                    _ => {}
                }
                if let Some(ref mut event_handler) = payload.event_handler {
                    event_handler.key_down_event(keycode, state.keymods, false);
                }
            }
            Message::KeyUp { keycode } => {
                let mut state = payload.state.lock().unwrap();
                match keycode {
                    KeyCode::LeftShift | KeyCode::RightShift => state.keymods.shift = false,
                    KeyCode::LeftControl | KeyCode::RightControl => state.keymods.ctrl = false,
                    KeyCode::LeftAlt | KeyCode::RightAlt => state.keymods.alt = false,
                    KeyCode::LeftSuper | KeyCode::RightSuper => state.keymods.logo = false,
                    _ => {}
                }
                if let Some(ref mut event_handler) = payload.event_handler {
                    event_handler.key_up_event(keycode, state.keymods);
                }
            }
            Message::Resize { width, height } => {
                if let Some(ref mut event_handler) = payload.event_handler {
                    event_handler.resize_event(width as _, height as _);
                }
            }
        }
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
        decl.add_method(
            sel!(processMessage:),
            process_message as extern "C" fn(&Object, Sel, ObjcId),
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
        let payload = get_window_payload(this);
        if payload.event_handler.is_none() {
            payload.init_event_handler();
        }

        let main_screen: ObjcId = unsafe { msg_send![class!(UIScreen), mainScreen] };
        let screen_rect: NSRect = unsafe { msg_send![main_screen, bounds] };
        let high_dpi = native_display().lock().unwrap().high_dpi;

        let (screen_width, screen_height) = if high_dpi {
            (
                screen_rect.size.width as i32 * 2,
                screen_rect.size.height as i32 * 2,
            )
        } else {
            let content_scale_factor: f64 = unsafe { msg_send![payload.view, contentScaleFactor] };
            (
                (screen_rect.size.width * content_scale_factor) as i32,
                (screen_rect.size.height * content_scale_factor) as i32,
            )
        };

        if native_display().lock().unwrap().screen_width != screen_width
            || native_display().lock().unwrap().screen_height != screen_height
        {
            {
                let mut d = native_display().lock().unwrap();
                d.screen_width = screen_width;
                d.screen_height = screen_height;
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

    unsafe {
        decl.add_method(
            sel!(glkView: drawInRect:),
            draw_in_rect as extern "C" fn(&Object, Sel, ObjcId, ObjcId),
        );

        decl.add_method(
            sel!(drawInMTKView:),
            draw_in_rect2 as extern "C" fn(&Object, Sel, ObjcId),
        );
    }

    decl.add_ivar::<*mut c_void>("display_ptr");
    decl.register()
}

// metal or opengl view and the objects required to collect all the window events
struct View {
    view: ObjcId,
    view_dlg: ObjcId,
    view_ctrl: ObjcId,
    // this view failed to create gles3 context, but succeeded with gles2
    _gles2: bool,
}

unsafe fn create_opengl_view(screen_rect: NSRect, _sample_count: i32, high_dpi: bool) -> View {
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
        msg_send_![glk_view_obj, setContentScaleFactor: 2.0];
    } else {
        msg_send_![glk_view_obj, setContentScaleFactor: 1.0];
    }

    let view_ctrl_obj: ObjcId = msg_send![class!(UIViewController), alloc];
    let view_ctrl_obj: ObjcId = msg_send![view_ctrl_obj, init];

    msg_send_![view_ctrl_obj, setView: glk_view_obj];

    View {
        view: glk_view_obj,
        view_dlg: glk_view_dlg_obj,
        view_ctrl: view_ctrl_obj,
        _gles2: gles2,
    }
}

unsafe fn create_metal_view(screen_rect: NSRect, _sample_count: i32, _high_dpi: bool) -> View {
    let mtk_view_obj: ObjcId = msg_send![define_glk_or_mtk_view(class!(MTKView)), alloc];
    let mtk_view_obj: ObjcId = msg_send![mtk_view_obj, initWithFrame: screen_rect];

    let mtk_view_dlg_obj: ObjcId = msg_send![define_glk_or_mtk_view_dlg(class!(NSObject)), alloc];
    let mtk_view_dlg_obj: ObjcId = msg_send![mtk_view_dlg_obj, init];

    let view_ctrl_obj: ObjcId = msg_send![class!(UIViewController), alloc];
    let view_ctrl_obj: ObjcId = msg_send![view_ctrl_obj, init];

    msg_send_![view_ctrl_obj, setView: mtk_view_obj];

    msg_send_![mtk_view_obj, setEnableSetNeedsDisplay: YES];
    msg_send_![mtk_view_obj, setPaused: YES];
    msg_send_![mtk_view_obj, setPreferredFramesPerSecond:60];
    msg_send_![mtk_view_obj, setDelegate: mtk_view_dlg_obj];
    let device = MTLCreateSystemDefaultDevice();
    msg_send_![mtk_view_obj, setDevice: device];
    msg_send_![mtk_view_obj, setUserInteractionEnabled: YES];

    View {
        view: mtk_view_obj,
        view_dlg: mtk_view_dlg_obj,
        view_ctrl: view_ctrl_obj,

        _gles2: false,
    }
}

struct IosClipboard;
impl crate::native::Clipboard for IosClipboard {
    fn get(&mut self) -> Option<String> {
        None
    }
    fn set(&mut self, _data: &str) {}
}

pub fn define_app_delegate() -> *const Class {
    let superclass = class!(NSObject);
    let mut decl = ClassDecl::new("NSAppDelegate", superclass).unwrap();

    extern "C" fn did_finish_launching_with_options(
        _: &Object,
        _: Sel,
        _: ObjcId,
        _: ObjcId,
    ) -> BOOL {
        unsafe {
            let (f, conf) = RUN_ARGS.take().unwrap();

            let main_screen: ObjcId = msg_send![class!(UIScreen), mainScreen];
            let screen_rect: NSRect = msg_send![main_screen, bounds];

            let (_, _) = if conf.high_dpi {
                (
                    screen_rect.size.width as i32 * 2,
                    screen_rect.size.height as i32 * 2,
                )
            } else {
                (
                    screen_rect.size.width as i32,
                    screen_rect.size.height as i32,
                )
            };

            let window_obj: ObjcId = msg_send![class!(UIWindow), alloc];
            let window_obj: ObjcId = msg_send![window_obj, initWithFrame: screen_rect];

            let view = match conf.platform.apple_gfx_api {
                AppleGfxApi::OpenGl => {
                    create_opengl_view(screen_rect, conf.sample_count, conf.high_dpi)
                }
                AppleGfxApi::Metal => {
                    create_metal_view(screen_rect, conf.sample_count, conf.high_dpi)
                }
            };

            let (textfield_dlg, textfield) = {
                let textfield_dlg = msg_send_![msg_send_![define_textfield_dlg(), alloc], init];
                let textfield = msg_send_![
                    msg_send_![class!(UITextField), alloc],
                    initWithFrame:NSRect::new(10.0, 10.0, 100.0, 50.0)];
                msg_send_![textfield, setAutocapitalizationType:0]; // UITextAutocapitalizationTypeNone
                msg_send_![textfield, setAutocorrectionType:1]; // UITextAutocorrectionTypeNo
                msg_send_![textfield, setSpellCheckingType:1]; // UITextSpellCheckingTypeNo
                msg_send_![textfield, setHidden: YES];
                msg_send_![textfield, setDelegate: textfield_dlg];
                // to make backspace work - with empty text there is no event on text removal
                msg_send_![textfield, setText: apple_util::str_to_nsstring("x")];
                msg_send_![view.view, addSubview: textfield];

                let notification_center = msg_send_![class!(NSNotificationCenter), defaultCenter];
                msg_send_![notification_center, addObserver:textfield_dlg
                           selector:sel!(keyboardWasShown:)
                           name:UIKeyboardDidShowNotification object:nil];
                msg_send_![notification_center, addObserver:textfield_dlg
                           selector:sel!(keyboardWillBeHidden:)
                           name:UIKeyboardWillHideNotification object:nil];
                msg_send_![notification_center, addObserver:textfield_dlg
                           selector:sel!(keyboardDidChangeFrame:)
                           name:UIKeyboardDidChangeFrameNotification object:nil];
                (textfield_dlg, textfield)
            };

            let (tx, rx) = std::sync::mpsc::channel();

            MESSAGES_TX.with(move |messages_tx| *messages_tx.borrow_mut() = Some(tx));

            let clipboard = Box::new(IosClipboard);
            let (tx, requests_rx) = std::sync::mpsc::channel();
            crate::set_display(NativeDisplayData {
                high_dpi: conf.high_dpi,
                gfx_api: conf.platform.apple_gfx_api,
                blocking_event_loop: conf.platform.blocking_event_loop,
                view: view.view,
                ..NativeDisplayData::new(conf.window_width, conf.window_height, tx, clipboard)
            });

            let state_original = Arc::new(Mutex::new(MainThreadState {
                quit: false,
                paused: true,
                update_requested: true,
                view: view.view,
                keymods: KeyMods {
                    shift: false,
                    ctrl: false,
                    alt: false,
                    logo: false,
                },
                cur_msg: Message::Resume,
            }));

            let payload = Box::new(IosDisplay {
                view: view.view,
                view_ctrl: view.view_ctrl,
                textfield,
                _textfield_dlg: textfield_dlg,
                gfx_api: conf.platform.apple_gfx_api,

                f: Some(Box::new(f)),
                event_handler: None,
                _gles2: view._gles2,
                state: state_original.clone(),
            });
            let payload_ptr = Box::into_raw(payload) as *mut std::ffi::c_void;

            (*view.view).set_ivar("display_ptr", payload_ptr);
            (*view.view_dlg).set_ivar("display_ptr", payload_ptr);
            (*textfield_dlg).set_ivar("display_ptr", payload_ptr);

            msg_send_![window_obj, addSubview: view.view];

            msg_send_![window_obj, setRootViewController: view.view_ctrl];

            msg_send_![window_obj, makeKeyAndVisible];

            struct SendHack<F>(F);
            unsafe impl<F> Send for SendHack<F> {}

            let state = SendHack(state_original.clone());
            thread::spawn(move || {
                let s = state.0;

                loop {
                    while let Ok(request) = requests_rx.try_recv() {
                        s.lock().unwrap().process_request(request);
                    }

                    let block_on_wait = {
                        let s = s.lock().unwrap();
                        (conf.platform.blocking_event_loop && !s.update_requested) || s.paused
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
                        update_requested = s.update_requested;
                        view = s.view;
                    }

                    if !conf.platform.blocking_event_loop || update_requested {
                        match conf.platform.apple_gfx_api {
                            AppleGfxApi::OpenGl => {
                                // Why it differs from Metal? I don't realy know. Looks like a bug.
                                // Somehow it needs `setNeedsDisplay` to redraw after touch.
                                // With plain `display` it draws only after another touch.
                                // But when it's not blocking_event_loop it makes fps really drop with `setNeedsDisplay`.
                                // I hope it will work the same on the real device.
                                if conf.platform.blocking_event_loop {
                                    msg_send_![&*view, performSelectorOnMainThread:sel!(setNeedsDisplay) withObject:nil waitUntilDone:NO];
                                } else {
                                    msg_send_![&*view, performSelectorOnMainThread:sel!(display) withObject:nil waitUntilDone:YES];
                                }
                            }
                            AppleGfxApi::Metal => {
                                msg_send_![&*view, performSelectorOnMainThread:sel!(setNeedsDisplay) withObject:nil waitUntilDone:NO];
                            }
                        }
                    }

                    thread::yield_now();
                }
            });
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

fn define_textfield_dlg() -> *const Class {
    let superclass = class!(NSObject);
    let mut decl = ClassDecl::new("NSTexfieldDlg", superclass).unwrap();

    // those 3 callbacks are for resizing the canvas when keyboard is opened
    // which is not currenlty supported by miniquad
    extern "C" fn keyboard_was_shown(_: &Object, _: Sel, _notif: ObjcId) {}
    extern "C" fn keyboard_will_be_hidden(_: &Object, _: Sel, _notif: ObjcId) {}
    extern "C" fn keyboard_did_change_frame(_: &Object, _: Sel, _notif: ObjcId) {}

    extern "C" fn should_change_characters_in_range(
        _: &Object,
        _: Sel,
        _textfield: ObjcId,
        _range: NSRange,
        string: ObjcId,
    ) -> BOOL {
        unsafe {
            let len: u64 = msg_send![string, length];
            if len > 0 {
                for i in 0..len {
                    let c: u16 = msg_send![string, characterAtIndex: i];

                    match c {
                        c if c >= 32 && !(0xD800..=0xDFFF).contains(&c) => {
                            send_message(Message::Character {
                                character: c as u32,
                            })
                        }
                        10 => {
                            send_message(Message::KeyDown {
                                keycode: crate::event::KeyCode::Enter,
                            });
                            send_message(Message::KeyUp {
                                keycode: crate::event::KeyCode::Enter,
                            });
                        }
                        32 => {
                            send_message(Message::Character {
                                character: ' ' as u32,
                            });
                            send_message(Message::KeyDown {
                                keycode: crate::event::KeyCode::Space,
                            });
                            send_message(Message::KeyUp {
                                keycode: crate::event::KeyCode::Space,
                            });
                        }
                        _ => {}
                    }
                }
            } else {
                send_message(Message::KeyDown {
                    keycode: crate::event::KeyCode::Backspace,
                });
                send_message(Message::KeyUp {
                    keycode: crate::event::KeyCode::Backspace,
                });
            }
        }
        NO
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
            sel!(textField: shouldChangeCharactersInRange: replacementString:),
            should_change_characters_in_range
                as extern "C" fn(&Object, Sel, ObjcId, NSRange, ObjcId) -> BOOL,
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
