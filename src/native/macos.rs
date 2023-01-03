//! MacOs implementation is basically a mix between
//! sokol_app's objective C code and Makepad's (https://github.com/makepad/makepad/blob/live/platform/src/platform/apple)
//! platform implementation
//!
use {
    crate::{
        event::{EventHandler, MouseButton},
        native::{
            apple::{apple_util::*, frameworks::*},
            NativeDisplayData,
        },
        Context, CursorIcon, GraphicsContext,
    },
    std::{collections::HashMap, os::raw::c_void},
};

pub struct MacosDisplay {
    window: ObjcId,
    view: ObjcId,
    data: NativeDisplayData,
    fullscreen: bool,
    // [NSCursor hide]/unhide calls should be balanced
    // hide/hide/unhide will keep cursor hidden
    // so need to keep internal cursor state to avoid problems from
    // unbalanced show_mouse() calls
    cursor_shown: bool,
    current_cursor: CursorIcon,
    cursors: HashMap<CursorIcon, ObjcId>,
}
impl crate::native::NativeDisplay for MacosDisplay {
    fn screen_size(&self) -> (f32, f32) {
        (self.data.screen_width as _, self.data.screen_height as _)
    }
    fn dpi_scale(&self) -> f32 {
        self.data.dpi_scale
    }
    fn high_dpi(&self) -> bool {
        self.data.high_dpi
    }
    fn order_quit(&mut self) {
        self.data.quit_ordered = true;
    }
    fn request_quit(&mut self) {
        self.data.quit_requested = true;
    }
    fn cancel_quit(&mut self) {
        self.data.quit_requested = false;
    }

    fn set_cursor_grab(&mut self, _grab: bool) {}
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
    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl MacosDisplay {
    fn transform_mouse_point(&self, point: &NSPoint) -> (f32, f32) {
        let new_x = point.x as f32 * self.data.dpi_scale;
        let new_y = self.data.screen_height as f32 - (point.y as f32 * self.data.dpi_scale) - 1.;

        (new_x, new_y)
    }

    unsafe fn update_dimensions(&mut self) -> Option<(i32, i32)> {
        if self.data.high_dpi {
            let screen: ObjcId = msg_send![self.window, screen];
            let dpi_scale: f64 = msg_send![screen, backingScaleFactor];
            self.data.dpi_scale = dpi_scale as f32;
        } else {
            self.data.dpi_scale = 1.0;
        }

        let bounds: NSRect = msg_send![self.view, bounds];
        let screen_width = (bounds.size.width as f32 * self.data.dpi_scale) as i32;
        let screen_height = (bounds.size.height as f32 * self.data.dpi_scale) as i32;

        let dim_changed =
            screen_width != self.data.screen_width || screen_height != self.data.screen_height;

        self.data.screen_width = screen_width;
        self.data.screen_height = screen_height;

        if dim_changed {
            Some((screen_width, screen_height))
        } else {
            None
        }
    }
}
struct WindowPayload {
    display: MacosDisplay,
    context: Option<GraphicsContext>,
    event_handler: Option<Box<dyn EventHandler>>,
    f: Option<Box<dyn 'static + FnOnce(&mut crate::Context) -> Box<dyn EventHandler>>>,
}
impl WindowPayload {
    pub fn context(&mut self) -> Option<(&mut Context, &mut dyn EventHandler)> {
        let a = self.context.as_mut()?;
        let event_handler = self.event_handler.as_deref_mut()?;

        Some((a.with_display(&mut self.display), event_handler))
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

        // only give user-code a chance to intervene when sapp_quit() wasn't already called
        if !payload.display.data.quit_ordered {
            // if window should be closed and event handling is enabled, give user code
            // a chance to intervene via sapp_cancel_quit()
            payload.display.data.quit_requested = true;
            if let Some((context, event_handler)) = payload.context() {
                event_handler.quit_requested_event(context);
            }

            // user code hasn't intervened, quit the app
            if payload.display.data.quit_requested {
                payload.display.data.quit_ordered = true;
            }
        }
        if payload.display.data.quit_ordered {
            return YES;
        } else {
            return NO;
        }
    }

    extern "C" fn window_did_resize(this: &Object, _: Sel, _: ObjcId) {
        let payload = get_window_payload(this);
        if let Some((w, h)) = unsafe { payload.display.update_dimensions() } {
            if let Some((context, event_handler)) = payload.context() {
                event_handler.resize_event(context, w as _, h as _);
            }
        }
    }

    extern "C" fn window_did_change_screen(this: &Object, _: Sel, _: ObjcId) {
        let payload = get_window_payload(this);
        if let Some((w, h)) = unsafe { payload.display.update_dimensions() } {
            if let Some((context, event_handler)) = payload.context() {
                event_handler.resize_event(context, w as _, h as _);
            }
        }
    }
    extern "C" fn window_did_enter_fullscreen(this: &Object, _: Sel, _: ObjcId) {
        let payload = get_window_payload(this);
        payload.display.fullscreen = true;
    }
    extern "C" fn window_did_exit_fullscreen(this: &Object, _: Sel, _: ObjcId) {
        let payload = get_window_payload(this);
        payload.display.fullscreen = false;
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

pub fn define_cocoa_view_class() -> *const Class {
    //extern "C" fn dealloc(this: &Object, _sel: Sel) {}

    extern "C" fn reshape(this: &Object, _sel: Sel) {
        let payload = get_window_payload(this);

        unsafe {
            let superclass = superclass(this);
            let () = msg_send![super(this, superclass), reshape];

            if let Some((w, h)) = payload.display.update_dimensions() {
                if let Some((context, event_handler)) = payload.context() {
                    event_handler.resize_event(context, w as _, h as _);
                }
            }
        }
    }

    extern "C" fn reset_cursor_rects(this: &Object, _sel: Sel) {
        let payload = get_window_payload(this);
        unsafe {
            let current_cursor = payload.display.current_cursor;
            let cursor_id = *payload
                .display
                .cursors
                .entry(current_cursor)
                .or_insert_with(|| load_mouse_cursor(current_cursor.clone()));
            assert!(!cursor_id.is_null());
            let bounds: NSRect = msg_send![this, bounds];
            let _: () = msg_send![
                this,
                addCursorRect: bounds
                cursor: cursor_id
            ];
        }
    }

    extern "C" fn draw_rect(this: &Object, _sel: Sel, _rect: NSRect) {
        let payload = get_window_payload(this);
        if let Some((context, event_handler)) = payload.context() {
            event_handler.update(context);
            event_handler.draw(context);
        }

        unsafe {
            let ctx: ObjcId = msg_send![this, openGLContext];
            assert!(!ctx.is_null());
            let () = msg_send![ctx, flushBuffer];

            if payload.display.data.quit_requested || payload.display.data.quit_ordered {
                let () = msg_send![payload.display.window, performClose: nil];
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

        payload.context = Some(GraphicsContext::new(false));

        let f = payload.f.take().unwrap();
        payload.event_handler = Some(f(payload
            .context
            .as_mut()
            .unwrap()
            .with_display(&mut payload.display)));
    }

    extern "C" fn timer_fired(this: &Object, _sel: Sel, _: ObjcId) {
        unsafe {
            let () = msg_send!(this, setNeedsDisplay: YES);
        }
    }
    extern "C" fn mouse_moved(this: &Object, _sel: Sel, event: ObjcId) {
        let payload = get_window_payload(this);

        unsafe {
            let point: NSPoint = msg_send!(event, locationInWindow);
            let point = payload.display.transform_mouse_point(&point);
            if let Some((context, event_handler)) = payload.context() {
                event_handler.mouse_motion_event(context, point.0, point.1);
            }
        }
    }

    fn fire_mouse_event(this: &Object, event: ObjcId, down: bool, btn: MouseButton) {
        let payload = get_window_payload(this);

        unsafe {
            let point: NSPoint = msg_send!(event, locationInWindow);
            let point = payload.display.transform_mouse_point(&point);
            if let Some((context, event_handler)) = payload.context() {
                if down {
                    event_handler.mouse_button_down_event(context, btn, point.0, point.1);
                } else {
                    event_handler.mouse_button_up_event(context, btn, point.0, point.1);
                }
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
    extern "C" fn scroll_wheel(this: &Object, _sel: Sel, event: ObjcId) {
        let payload = get_window_payload(this);
        unsafe {
            let mut dx: f64 = msg_send![event, scrollingDeltaX];
            let mut dy: f64 = msg_send![event, scrollingDeltaY];

            if !msg_send![event, hasPreciseScrollingDeltas] {
                dx *= 10.0;
                dy *= 10.0;
            }
            if let Some((context, event_handler)) = payload.context() {
                event_handler.mouse_wheel_event(context, dx as f32, dy as f32);
            }
        }
    }
    extern "C" fn key_down(this: &Object, _sel: Sel, event: ObjcId) {
        let payload = get_window_payload(this);
        let mods = get_event_key_modifier(event);
        let repeat: bool = unsafe { msg_send!(event, isARepeat) };
        if let Some(key) = get_event_keycode(event) {
            if let Some((context, event_handler)) = payload.context() {
                event_handler.key_down_event(context, key, mods, repeat);
            }
        }

        if let Some(character) = get_event_char(event) {
            if let Some((context, event_handler)) = payload.context() {
                event_handler.char_event(context, character, mods, repeat);
            }
        }
    }
    extern "C" fn key_up(this: &Object, _sel: Sel, event: ObjcId) {
        let payload = get_window_payload(this);
        let mods = get_event_key_modifier(event);
        if let Some(key) = get_event_keycode(event) {
            if let Some((context, event_handler)) = payload.context() {
                event_handler.key_up_event(context, key, mods);
            }
        }
    }
    let superclass = class!(NSOpenGLView);
    let mut decl = ClassDecl::new("RenderViewClass", superclass).unwrap();
    unsafe {
        //decl.add_method(sel!(dealloc), dealloc as extern "C" fn(&Object, Sel));
        decl.add_method(
            sel!(timerFired:),
            timer_fired as extern "C" fn(&Object, Sel, ObjcId),
        );

        decl.add_method(sel!(reshape), reshape as extern "C" fn(&Object, Sel));
        decl.add_method(
            sel!(prepareOpenGL),
            prepare_open_gl as extern "C" fn(&Object, Sel),
        );
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
            sel!(drawRect:),
            draw_rect as extern "C" fn(&Object, Sel, NSRect),
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
        decl.add_method(sel!(keyUp:), key_up as extern "C" fn(&Object, Sel, ObjcId));
    }

    decl.add_ivar::<*mut c_void>("display_ptr");

    return decl.register();
}

fn get_window_payload(this: &Object) -> &mut WindowPayload {
    unsafe {
        let ptr: *mut c_void = *this.get_ivar("display_ptr");
        &mut *(ptr as *mut WindowPayload)
    }
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

    let view_class = define_cocoa_view_class();
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

pub unsafe fn run<F>(conf: crate::conf::Conf, f: F)
where
    F: 'static + FnOnce(&mut crate::Context) -> Box<dyn EventHandler>,
{
    let mut payload = WindowPayload {
        display: MacosDisplay {
            view: std::ptr::null_mut(),
            window: std::ptr::null_mut(),
            data: NativeDisplayData {
                high_dpi: conf.high_dpi,
                ..Default::default()
            },
            fullscreen: false,
            cursor_shown: true,
            current_cursor: CursorIcon::Default,
            cursors: HashMap::new(),
        },
        f: Some(Box::new(f)),
        event_handler: None,
        context: None,
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

    (*window_delegate).set_ivar("display_ptr", &mut payload as *mut _ as *mut c_void);

    let title = str_to_nsstring(&conf.window_title);
    //let () = msg_send![window, setReleasedWhenClosed: NO];
    let () = msg_send![window, setTitle: title];
    let () = msg_send![window, center];
    let () = msg_send![window, setAcceptsMouseMovedEvents: YES];

    let view = create_opengl_view(window_frame, conf.sample_count, conf.high_dpi);
    (*view).set_ivar("display_ptr", &mut payload as *mut _ as *mut c_void);

    payload.display.window = window;
    payload.display.view = view;

    let nstimer: ObjcId = msg_send![
        class!(NSTimer),
        timerWithTimeInterval: 0.001
        target: view
        selector: sel!(timerFired:)
        userInfo: nil
        repeats: true
    ];
    let nsrunloop: ObjcId = msg_send![class!(NSRunLoop), currentRunLoop];
    let () = msg_send![nsrunloop, addTimer: nstimer forMode: NSDefaultRunLoopMode];
    assert!(!view.is_null());

    let () = msg_send![window, setContentView: view];
    let () = msg_send![window, makeFirstResponder: view];

    if conf.fullscreen {
        let () = msg_send![window, toggleFullScreen: nil];
    }

    let () = msg_send![window, makeKeyAndOrderFront: nil];

    let _ = payload.display.update_dimensions();

    let ns_app: ObjcId = msg_send![class!(NSApplication), sharedApplication];
    let () = msg_send![ns_app, run];

    // run should never return
    // but just in case
    unreachable!();
}
