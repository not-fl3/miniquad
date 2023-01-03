//! MacOs implementation is basically a mix between
//! sokol_app's objective C code and Makepad's (https://github.com/makepad/makepad/blob/live/platform/src/platform/apple)
//! platform implementation
//!
use {
    crate::{
        conf::Conf,
        event::{EventHandler, MouseButton},
        fs,
        native::{
            apple::{
                apple_util::{self, *},
                frameworks::{self, *},
            },
            NativeDisplayData,
        },
        Context, GraphicsContext,
    },
    std::os::raw::c_void,
};

struct IosDisplay {
    data: NativeDisplayData,
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
    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

struct WindowPayload {
    display: IosDisplay,
    context: Option<GraphicsContext>,
    event_handler: Option<Box<dyn EventHandler>>,
    gles2: bool,
    f: Option<Box<dyn 'static + FnOnce(&mut crate::Context) -> Box<dyn EventHandler>>>,
}
impl WindowPayload {
    pub fn context(&mut self) -> Option<(&mut Context, &mut dyn EventHandler)> {
        let a = self.context.as_mut()?;
        let event_handler = self.event_handler.as_deref_mut()?;

        Some((a.with_display(&mut self.display), event_handler))
    }
}

fn get_window_payload(this: &Object) -> &mut WindowPayload {
    unsafe {
        let ptr: *mut c_void = *this.get_ivar("display_ptr");
        &mut *(ptr as *mut WindowPayload)
    }
}

pub fn define_glk_view() -> *const Class {
    let superclass = class!(GLKView);
    let mut decl = ClassDecl::new("QuadView", superclass).unwrap();

    extern "C" fn touches_began(this: &Object, _: Sel, _: ObjcId, event: ObjcId) {
        unsafe {
            let payload = get_window_payload(this);

            let enumerator: ObjcId = msg_send![event, allTouches];
            let enumerator: ObjcId = msg_send![enumerator, objectEnumerator];

            let mut ios_touch: ObjcId;

            while {
                ios_touch = msg_send![enumerator, nextObject];
                ios_touch != nil
            } {
                let mut ios_pos: NSPoint = msg_send![ios_touch, locationInView: this];

                if payload.display.data.high_dpi {
                    ios_pos.x *= 2.;
                    ios_pos.y *= 2.;
                }
                if let Some((context, event_handler)) = payload.context() {
                    event_handler.mouse_button_down_event(
                        context,
                        MouseButton::Left,
                        ios_pos.x as _,
                        ios_pos.y as _,
                    );
                }
            }
        }
    }

    extern "C" fn touches_moved(_: &Object, _: Sel, _: ObjcId, _: ObjcId) {}

    extern "C" fn touches_ended(_: &Object, _: Sel, _: ObjcId, _: ObjcId) {}

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

pub fn define_glk_view_dlg() -> *const Class {
    let superclass = class!(NSObject);
    let mut decl = ClassDecl::new("QuadViewDlg", superclass).unwrap();

    extern "C" fn draw_in_rect(this: &Object, _: Sel, _: ObjcId, _: ObjcId) {
        let payload = get_window_payload(this);
        if payload.event_handler.is_none() {
            let f = payload.f.take().unwrap();
            let mut context = GraphicsContext::new();
            context.features = Features::from_gles2(payload.gles2);
            payload.context = Some(context);
            payload.event_handler = Some(f(payload
                .context
                .as_mut()
                .unwrap()
                .with_display(&mut payload.display)));
        }

        let main_screen: ObjcId = unsafe { msg_send![class!(UIScreen), mainScreen] };
        let screen_rect: NSRect = unsafe { msg_send![main_screen, bounds] };
        let (screen_width, screen_height) = if payload.display.data.high_dpi {
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

        if payload.display.data.screen_width != screen_width
            || payload.display.data.screen_height != screen_height
        {
            payload.display.data.screen_width = screen_width;
            payload.display.data.screen_height = screen_height;
            if let Some((context, event_handler)) = payload.context() {
                event_handler.resize_event(context, screen_width as _, screen_height as _);
            }
        }

        if let Some((context, event_handler)) = payload.context() {
            event_handler.update(context);
            event_handler.draw(context);
        }
    }

    unsafe {
        decl.add_method(
            sel!(glkView: drawInRect:),
            draw_in_rect as extern "C" fn(&Object, Sel, ObjcId, ObjcId),
        );
    }
    decl.add_ivar::<*mut c_void>("display_ptr");
    return decl.register();
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

            let eagl_context_obj: ObjcId = msg_send![class!(EAGLContext), alloc];
            let mut eagl_context_obj: ObjcId = msg_send![eagl_context_obj, initWithAPI: 3];
            let mut gles2 = false;
            if eagl_context_obj.is_null() {
                eagl_context_obj = msg_send![eagl_context_obj, initWithAPI: 2];
                gles2 = true;
            }

            let payload = Box::new(WindowPayload {
                display: IosDisplay {
                    data: NativeDisplayData {
                        screen_width,
                        screen_height,
                        high_dpi: conf.high_dpi,
                        ..Default::default()
                    },
                },
                f: Some(Box::new(f)),
                event_handler: None,
                context: None,
                gles2,
            });
            let payload_ptr = Box::into_raw(payload) as *mut std::ffi::c_void;

            let glk_view_dlg_obj: ObjcId = msg_send![define_glk_view_dlg(), alloc];
            let glk_view_dlg_obj: ObjcId = msg_send![glk_view_dlg_obj, init];

            (*glk_view_dlg_obj).set_ivar("display_ptr", payload_ptr);

            let glk_view_obj: ObjcId = msg_send![define_glk_view(), alloc];
            let glk_view_obj: ObjcId = msg_send![glk_view_obj, initWithFrame: screen_rect];

            (*glk_view_obj).set_ivar("display_ptr", payload_ptr);

            let _: () = msg_send![
                glk_view_obj,
                setDrawableColorFormat: frameworks::GLKViewDrawableColorFormatRGBA8888
            ];
            let _: () = msg_send![
                glk_view_obj,
                setDrawableDepthFormat: frameworks::GLKViewDrawableDepthFormat::Format24 as i32
            ];
            let _: () = msg_send![
                glk_view_obj,
                setDrawableStencilFormat: frameworks::GLKViewDrawableStencilFormat::FormatNone
                    as i32
            ];
            let _: () = msg_send![glk_view_obj, setContext: eagl_context_obj];
            let _: () = msg_send![glk_view_obj, setDelegate: glk_view_dlg_obj];
            let _: () = msg_send![glk_view_obj, setEnableSetNeedsDisplay: NO];
            let _: () = msg_send![glk_view_obj, setUserInteractionEnabled: YES];
            let _: () = msg_send![glk_view_obj, setMultipleTouchEnabled: YES];
            if conf.high_dpi {
                let _: () = msg_send![glk_view_obj, setContentScaleFactor: 2.0];
            } else {
                let _: () = msg_send![glk_view_obj, setContentScaleFactor: 1.0];
            }
            let _: () = msg_send![window_obj, addSubview: glk_view_obj];

            let view_ctrl_obj: ObjcId = msg_send![class!(GLKViewController), alloc];
            let view_ctrl_obj: ObjcId = msg_send![view_ctrl_obj, init];

            let _: () = msg_send![view_ctrl_obj, setView: glk_view_obj];
            let _: () = msg_send![view_ctrl_obj, setPreferredFramesPerSecond:60];
            let _: () = msg_send![window_obj, setRootViewController: view_ctrl_obj];

            let _: () = msg_send![window_obj, makeKeyAndVisible];
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
static mut RUN_ARGS: Option<(
    Box<dyn FnOnce(&mut crate::Context) -> Box<dyn EventHandler>>,
    Conf,
)> = None;

pub unsafe fn run<F>(conf: Conf, f: F)
where
    F: 'static + FnOnce(&mut crate::Context) -> Box<dyn EventHandler>,
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
