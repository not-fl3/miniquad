//! MacOs implementation is basically a mix between
//! sokol_app's objective C code and Makepad's (https://github.com/makepad/makepad/blob/live/platform/src/platform/apple)
//! platform implementation
//!
use {
    crate::{
        conf::{self, AppleGfxApi, Conf},
        event::{EventHandler, TouchPhase},
        fs,
        native::{
            apple::{
                apple_util::{self, *},
                frameworks::{self, *},
            },
            NativeDisplayData,
        },
    },
    std::os::raw::c_void,
};

struct IosDisplay {
    data: NativeDisplayData,
    view: ObjcId,
    gfx_api: conf::AppleGfxApi,
}

impl crate::native::NativeDisplay for IosDisplay {
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
    fn show_mouse(&mut self, _show: bool) {}
    fn set_mouse_cursor(&mut self, _cursor: crate::CursorIcon) {}
    fn set_window_size(&mut self, _new_width: u32, _new_height: u32) {}
    fn set_fullscreen(&mut self, _fullscreen: bool) {}
    fn clipboard_get(&mut self) -> Option<String> {
        None
    }
    fn clipboard_set(&mut self, _data: &str) {}
    #[cfg(target_vendor = "apple")]
    fn apple_gfx_api(&self) -> crate::conf::AppleGfxApi {
        self.gfx_api
    }
    #[cfg(target_vendor = "apple")]
    fn apple_view(&mut self) -> Option<crate::native::apple::frameworks::ObjcId> {
        Some(self.view)
    }

    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
mod tl_display {
    use super::*;
    use crate::NATIVE_DISPLAY;

    use std::cell::RefCell;

    thread_local! {
        static DISPLAY: RefCell<Option<IosDisplay>> = RefCell::new(None);
    }

    fn with_native_display(f: &mut dyn FnMut(&mut dyn crate::NativeDisplay)) {
        DISPLAY.with(|d| {
            let mut d = d.borrow_mut();
            let mut d = d.as_mut().unwrap();
            f(&mut *d);
        })
    }

    pub(super) fn with<T>(mut f: impl FnMut(&mut IosDisplay) -> T) -> T {
        DISPLAY.with(|d| {
            let mut d = d.borrow_mut();
            let mut d = d.as_mut().unwrap();
            f(&mut *d)
        })
    }

    pub(super) fn set_display(display: IosDisplay) {
        DISPLAY.with(|d| *d.borrow_mut() = Some(display));
        NATIVE_DISPLAY.with(|d| *d.borrow_mut() = Some(with_native_display));
    }
}

struct WindowPayload {
    event_handler: Option<Box<dyn EventHandler>>,
    gles2: bool,
    f: Option<Box<dyn 'static + FnOnce() -> Box<dyn EventHandler>>>,
}

fn get_window_payload(this: &Object) -> &mut WindowPayload {
    unsafe {
        let ptr: *mut c_void = *this.get_ivar("display_ptr");
        &mut *(ptr as *mut WindowPayload)
    }
}

pub fn define_glk_or_mtk_view(superclass: &Class) -> *const Class {
    let mut decl = ClassDecl::new("QuadView", superclass).unwrap();

    fn on_touch(this: &Object, event: ObjcId, mut callback: impl FnMut(u64, f32, f32)) {
        unsafe {
            let enumerator: ObjcId = msg_send![event, allTouches];
            let size: u64 = msg_send![enumerator, count];
            let enumerator: ObjcId = msg_send![enumerator, objectEnumerator];

            for touch_id in 0..size {
                let ios_touch: ObjcId = msg_send![enumerator, nextObject];
                let mut ios_pos: NSPoint = msg_send![ios_touch, locationInView: this];

                tl_display::with(|d| {
                    if d.data.high_dpi {
                        ios_pos.x *= 2.;
                        ios_pos.y *= 2.;
                    }
                });

                callback(touch_id, ios_pos.x as _, ios_pos.y as _);
            }
        }
    }
    extern "C" fn touches_began(this: &Object, _: Sel, _: ObjcId, event: ObjcId) {
        unsafe {
            let payload = get_window_payload(this);

            if let Some(ref mut event_handler) = payload.event_handler {
                on_touch(this, event, |id, x, y| {
                    event_handler.touch_event(TouchPhase::Started, id, x as _, y as _);
                });
            }
        }
    }

    extern "C" fn touches_moved(this: &Object, _: Sel, _: ObjcId, event: ObjcId) {
        unsafe {
            let payload = get_window_payload(this);

            if let Some(ref mut event_handler) = payload.event_handler {
                on_touch(this, event, |id, x, y| {
                    event_handler.touch_event(TouchPhase::Moved, id, x as _, y as _);
                });
            }
        }
    }

    extern "C" fn touches_ended(this: &Object, _: Sel, _: ObjcId, event: ObjcId) {
        unsafe {
            let payload = get_window_payload(this);

            if let Some(ref mut event_handler) = payload.event_handler {
                on_touch(this, event, |id, x, y| {
                    event_handler.touch_event(TouchPhase::Ended, id, x as _, y as _);
                });
            }
        }
    }

    extern "C" fn touches_canceled(_: &Object, _: Sel, _: ObjcId, _: ObjcId) {}

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
    static mut opengl: *mut std::ffi::c_void = std::ptr::null_mut();

    if opengl.is_null() {
        opengl = libc::dlopen(
            b"/System/Library/Frameworks/OpenGLES.framework/OpenGLES\0".as_ptr() as _,
            libc::RTLD_LAZY,
        );
    }

    assert!(!opengl.is_null());

    let symbol = libc::dlsym(opengl, name as _);
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
            let f = payload.f.take().unwrap();

            if tl_display::with(|d| d.gfx_api == AppleGfxApi::OpenGl) {
                crate::native::gl::load_gl_funcs(|proc| {
                    let name = std::ffi::CString::new(proc).unwrap();

                    unsafe { get_proc_address(name.as_ptr() as _) }
                });
            }

            payload.event_handler = Some(f());
        }

        let main_screen: ObjcId = unsafe { msg_send![class!(UIScreen), mainScreen] };
        let screen_rect: NSRect = unsafe { msg_send![main_screen, bounds] };
        let high_dpi = tl_display::with(|d| d.data.high_dpi);

        let (screen_width, screen_height) = if high_dpi {
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

        if tl_display::with(|d| d.data.screen_width != screen_width)
            || tl_display::with(|d| d.data.screen_height != screen_height)
        {
            tl_display::with(|d| {
                d.data.screen_width = screen_width;
                d.data.screen_height = screen_height;
            });
            if let Some(ref mut event_handler) = payload.event_handler {
                event_handler.resize_event(screen_width as _, screen_height as _);
            }
        }

        if let Some(ref mut event_handler) = payload.event_handler {
            event_handler.update();
            event_handler.draw();
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
    return decl.register();
}

// metal or opengl view and the objects required to collect all the window events
struct View {
    view: ObjcId,
    view_dlg: ObjcId,
    view_ctrl: ObjcId,
    // this view failed to create gles3 context, but succeeded with gles2
    gles2: bool,
}

unsafe fn create_opengl_view(screen_rect: NSRect, sample_count: i32, high_dpi: bool) -> View {
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
    msg_send_![glk_view_obj, setEnableSetNeedsDisplay: NO];
    msg_send_![glk_view_obj, setUserInteractionEnabled: YES];
    msg_send_![glk_view_obj, setMultipleTouchEnabled: YES];
    if high_dpi {
        msg_send_![glk_view_obj, setContentScaleFactor: 2.0];
    } else {
        msg_send_![glk_view_obj, setContentScaleFactor: 1.0];
    }

    let view_ctrl_obj: ObjcId = msg_send![class!(GLKViewController), alloc];
    let view_ctrl_obj: ObjcId = msg_send![view_ctrl_obj, init];

    msg_send_![view_ctrl_obj, setView: glk_view_obj];
    msg_send_![view_ctrl_obj, setPreferredFramesPerSecond:60];

    View {
        view: glk_view_obj,
        view_dlg: glk_view_dlg_obj,
        view_ctrl: view_ctrl_obj,
        gles2,
    }
}

unsafe fn create_metal_view(screen_rect: NSRect, sample_count: i32, high_dpi: bool) -> View {
    let mtk_view_obj: ObjcId = msg_send![define_glk_or_mtk_view(class!(MTKView)), alloc];
    let mtk_view_obj: ObjcId = msg_send![mtk_view_obj, initWithFrame: screen_rect];

    let mtk_view_dlg_obj: ObjcId = msg_send![define_glk_or_mtk_view_dlg(class!(NSObject)), alloc];
    let mtk_view_dlg_obj: ObjcId = msg_send![mtk_view_dlg_obj, init];

    let view_ctrl_obj: ObjcId = msg_send![class!(UIViewController), alloc];
    let view_ctrl_obj: ObjcId = msg_send![view_ctrl_obj, init];

    msg_send_![view_ctrl_obj, setView: mtk_view_obj];

    msg_send_![mtk_view_obj, setPreferredFramesPerSecond:60];
    msg_send_![mtk_view_obj, setDelegate: mtk_view_dlg_obj];
    let device = MTLCreateSystemDefaultDevice();
    msg_send_![mtk_view_obj, setDevice: device];
    msg_send_![mtk_view_obj, setUserInteractionEnabled: YES];

    View {
        view: mtk_view_obj,
        view_dlg: mtk_view_dlg_obj,
        view_ctrl: view_ctrl_obj,

        gles2: false,
    }
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

            let (screen_width, screen_height) = if conf.high_dpi {
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

            tl_display::set_display(IosDisplay {
                data: NativeDisplayData {
                    screen_width,
                    screen_height,
                    high_dpi: conf.high_dpi,
                    ..Default::default()
                },
                view: view.view,
                gfx_api: conf.platform.apple_gfx_api,
            });
            let payload = Box::new(WindowPayload {
                f: Some(Box::new(f)),
                event_handler: None,
                gles2: view.gles2,
            });
            let payload_ptr = Box::into_raw(payload) as *mut std::ffi::c_void;

            (*view.view).set_ivar("display_ptr", payload_ptr);
            (*view.view_dlg).set_ivar("display_ptr", payload_ptr);

            msg_send_![window_obj, addSubview: view.view];

            msg_send_![window_obj, setRootViewController: view.view_ctrl];

            msg_send_![window_obj, makeKeyAndVisible];
        }
        YES
    }

    unsafe {
        decl.add_method(
            sel!(application: didFinishLaunchingWithOptions:),
            did_finish_launching_with_options
                as extern "C" fn(&Object, Sel, ObjcId, ObjcId) -> BOOL,
        );
    }

    return decl.register();
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
