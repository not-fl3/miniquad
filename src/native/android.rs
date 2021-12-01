#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    dead_code
)]

pub use crate::native::{
    egl::{self, *},
    gl::{self, *},
    //query_stab::*,
};

use crate::{conf::Conf, event::EventHandler, Context, GraphicsContext};
use ndk_sys::{AInputQueue, ALooper, ANativeActivity, ANativeWindow};

use std::{
    cell::RefCell, os::unix::prelude::RawFd, ptr::null_mut, sync::mpsc, thread, thread_local,
};

use libc::pipe;

#[derive(Default)]
struct MainThreadData {
    read_from_main_fd: RawFd,
    write_from_main_fd: RawFd,
    receiver: Option<mpsc::Receiver<()>>,
}

struct AndroidDisplay {
    config: egl::EGLConfig,
    display: egl::EGLDisplay,
    context: egl::EGLContext,
    surface: egl::EGLSurface,

    looper: *mut ALooper,
    window: *mut ANativeWindow,
    input: *mut AInputQueue,

    has_resumed: bool,
    has_focus: bool,

    window_width: i32,
    window_height: i32,
    framebuffer_width: i32,
    framebuffer_height: i32,
    dpi_scale: f32,
    quit_requested: bool,
    quit_ordered: bool,

    conf: crate::conf::Conf,
}

impl Default for AndroidDisplay {
    fn default() -> AndroidDisplay {
        AndroidDisplay {
            config: null_mut(),
            display: null_mut(),
            context: null_mut(),
            surface: null_mut(),
            looper: null_mut(),
            window: null_mut(),
            input: null_mut(),
            has_resumed: false,
            has_focus: false,

            window_width: 1,
            window_height: 1,
            framebuffer_width: 1,
            framebuffer_height: 1,
            dpi_scale: 1.,
            quit_requested: false,
            quit_ordered: false,

            conf: Conf::default(),
        }
    }
}

impl crate::native::NativeDisplay for AndroidDisplay {
    fn screen_size(&self) -> (f32, f32) {
        (self.framebuffer_width as _, self.framebuffer_height as _)
    }
    fn dpi_scale(&self) -> f32 {
        self.dpi_scale
    }
    fn high_dpi(&self) -> bool {
        self.conf.high_dpi
    }
    fn order_quit(&mut self) {}
    fn request_quit(&mut self) {}
    fn cancel_quit(&mut self) {}

    fn set_cursor_grab(&mut self, _grab: bool) {}
    fn show_mouse(&mut self, _shown: bool) {}
    fn set_mouse_cursor(&mut self, _cursor_icon: crate::CursorIcon) {}
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

#[derive(Default)]
struct UiThreadData {
    egl: Option<egl::LibEgl>,
    read_from_main_fd: RawFd,
    write_from_main_fd: RawFd,
    display: AndroidDisplay,
    event_handler: Option<Box<dyn EventHandler>>,
    context: Option<GraphicsContext>,
    first_frame: bool,
    frame_count: u64,
    is_thread_stopping: bool,
    sender: Option<mpsc::Sender<()>>,
}

impl UiThreadData {
    unsafe fn cleanup_android_state(&mut self) {
        let egl = self.egl.as_mut().unwrap();

        if !self.display.display.is_null() {
            (egl.eglMakeCurrent.unwrap())(self.display.display, null_mut(), null_mut(), null_mut());
            if !self.display.surface.is_null() {
                console_info(b"Destroying egl surface\0".as_ptr() as _);
                (egl.eglDestroySurface.unwrap())(self.display.display, self.display.surface);
                self.display.surface = null_mut();
            }
            if !self.display.context.is_null() {
                console_info(b"Aestroying egl context\0".as_ptr() as _);
                (egl.eglDestroyContext.unwrap())(self.display.display, self.display.context);
                self.display.context = null_mut();
            }
            console_info(b"Terminating egl display\0".as_ptr() as _);
            (egl.eglTerminate.unwrap())(self.display.display);
            self.display.display = null_mut();
        }
    }
}

fn wait_ui_thread() {
    MAIN_THREAD_DATA.with(|data| {
        let mut data = data.borrow_mut();
        let rx = data.receiver.as_mut().expect("not main thread");

        rx.recv().unwrap()
    })
}

fn notify_main_thread() {
    UI_THREAD_DATA.with(|data| {
        let mut data = data.borrow_mut();
        let tx = data.sender.as_mut().expect("not ui thread");

        tx.send(()).unwrap()
    })
}

#[derive(Debug)]
#[repr(C)]
enum AndroidMessage {
    Create,
    Resume,
    Pause,
    Destroy,
    Focus,
    NoFocus,
    SetInputQueue(*mut AInputQueue),
    SetNativeWindow(*mut ANativeWindow),
}

thread_local! {
    static MAIN_THREAD_DATA: RefCell<MainThreadData> = RefCell::new(Default::default());
    static UI_THREAD_DATA: RefCell<UiThreadData> = RefCell::new(Default::default());

}

// used only once, to pass sapp_desc from main_thread to ui_thread
// will be None most of the time
static mut SAPP_DESC: Option<
    Box<dyn 'static + FnOnce(&mut crate::Context) -> Box<dyn EventHandler>>,
> = None;

pub fn run<F>(conf: crate::conf::Conf, f: F)
where
    F: 'static + FnOnce(&mut crate::Context) -> Box<dyn EventHandler>,
{
    unsafe {
        SAPP_DESC = Some(Box::new(f));
    }

    UI_THREAD_DATA.with(move |data| data.borrow_mut().display.conf = conf);

    {
        use std::ffi::CString;
        use std::panic;

        panic::set_hook(Box::new(|info| {
            let msg = CString::new(format!("{:?}", info)).unwrap_or_else(|_| {
                CString::new(format!("MALFORMED ERROR MESSAGE {:?}", info.location())).unwrap()
            });
            unsafe {
                console_error(msg.as_ptr());
            }
        }));
    }
}

extern "C" {
    fn sokol_main();
}

#[no_mangle]
unsafe extern "C" fn sapp_ANativeActivity_onCreate(
    activity: *mut ::std::os::raw::c_void,
    _saved_state: *mut ::std::os::raw::c_void,
    _saved_state_size: ::std::os::raw::c_int,
) {
    console_info(b"ANativeActivity_onCreate\0".as_ptr() as _);

    sokol_main();

    let activity: *mut ANativeActivity = activity as _;

    let mut pipe_fd: [RawFd; 2] = Default::default();
    if pipe(pipe_fd.as_mut_ptr()) != 0 {
        console_error(b"Could not create thread pipe\0".as_ptr() as _);
        return;
    }
    let read_from_main_fd = pipe_fd[0];
    let write_from_main_fd = pipe_fd[1];

    let (tx, rx) = mpsc::channel();

    MAIN_THREAD_DATA.with(|data| {
        let mut data = data.borrow_mut();
        data.read_from_main_fd = read_from_main_fd;
        data.write_from_main_fd = write_from_main_fd;
        data.receiver = Some(rx);
    });

    thread::spawn(move || sapp_android_loop(tx, read_from_main_fd, write_from_main_fd));

    wait_ui_thread();

    android_msg(AndroidMessage::Create);

    wait_ui_thread();

    let mut callbacks = (*activity).callbacks.as_mut().unwrap();
    (*callbacks).onStart = Some(on_start);
    (*callbacks).onResume = Some(on_resume);
    (*callbacks).onSaveInstanceState = Some(on_save_instance_state);
    (*callbacks).onWindowFocusChanged = Some(on_window_focus_changed);
    (*callbacks).onPause = Some(on_pause);
    (*callbacks).onStop = Some(on_stop);
    (*callbacks).onDestroy = Some(on_destroy);
    (*callbacks).onNativeWindowCreated = Some(on_native_window_created);
    (*callbacks).onNativeWindowDestroyed = Some(on_native_window_destroyed);
    (*callbacks).onInputQueueCreated = Some(on_input_queue_created);
    (*callbacks).onInputQueueDestroyed = Some(on_input_queue_destroyed);
    (*callbacks).onConfigurationChanged = Some(on_config_changed);
    (*callbacks).onLowMemory = Some(on_low_memory);

    console_info(b"NativeActivity successfully created\0".as_ptr() as _);

    auto_hide_nav_bar(activity);
}

fn android_should_update(state: &AndroidDisplay) -> bool {
    let is_in_front = state.has_resumed && state.has_focus;
    let has_surface = state.surface.is_null() == false;

    is_in_front && has_surface
}

unsafe fn android_update_dimensions(
    data: &mut UiThreadData,
    window: *mut ANativeWindow,
    force_update: bool,
) {
    let state = &mut data.display;
    assert!(state.display.is_null() == false);
    assert!(state.context.is_null() == false);
    assert!(state.surface.is_null() == false);
    assert!(window.is_null() == false);

    let egl = data.egl.as_mut().unwrap();

    let win_w = ndk_sys::ANativeWindow_getWidth(window);
    let win_h = ndk_sys::ANativeWindow_getHeight(window);

    let win_changed = state.window_width != win_w || state.window_height != win_h;
    state.window_width = win_w;
    state.window_height = win_h;

    if win_changed || force_update {
        if !state.conf.high_dpi {
            let buf_w = win_w / 2;
            let buf_h = win_h / 2;
            let mut format = 0;
            let egl_result = (egl.eglGetConfigAttrib.unwrap())(
                state.display,
                state.config,
                egl::EGL_NATIVE_VISUAL_ID as _,
                &mut format,
            );
            assert!(egl_result == 1);
            // NOTE: calling ANativeWindow_setBuffersGeometry() with the same dimensions
            // as the ANativeWindow size results in weird display artefacts, that's
            // why it's only called when the buffer geometry is different from
            // the window size
            let result = ndk_sys::ANativeWindow_setBuffersGeometry(window, buf_w, buf_h, format);
            assert!(result == 0);
        }
    }

    // query surface size
    let mut fb_w = 0;
    let mut fb_h = 0;
    let egl_result_w = (egl.eglQuerySurface.unwrap())(
        state.display,
        state.surface,
        egl::EGL_WIDTH as _,
        &mut fb_w,
    );
    let egl_result_h = (egl.eglQuerySurface.unwrap())(
        state.display,
        state.surface,
        egl::EGL_HEIGHT as _,
        &mut fb_h,
    );
    assert!(egl_result_w == 1);
    assert!(egl_result_h == 1);
    let fb_changed = fb_w != state.framebuffer_width || fb_h != state.framebuffer_height;
    state.framebuffer_width = fb_w;
    state.framebuffer_height = fb_h;
    state.dpi_scale = state.framebuffer_width as f32 / state.window_width as f32;

    if win_changed || fb_changed || force_update {
        if data.first_frame == false {
            data.event_handler.as_mut().unwrap().resize_event(
                &mut Context::new(data.context.as_mut().unwrap(), state),
                fb_w as _,
                fb_h as _,
            );
        }
    }
}

unsafe fn android_frame() {
    UI_THREAD_DATA.with(|data| {
        let mut data = &mut *data.borrow_mut();

        let window = data.display.window;
        android_update_dimensions(&mut *data, window, false);

        if data.first_frame {
            let mut context = crate::GraphicsContext::new();
            data.event_handler = Some((SAPP_DESC.take().unwrap())(&mut Context::new(
                &mut context,
                &mut data.display,
            )));
            data.context = Some(context);
            data.first_frame = false;
        }

        data.event_handler
            .as_mut()
            .unwrap()
            .update(&mut Context::new(
                data.context.as_mut().unwrap(),
                &mut data.display,
            ));
        data.event_handler.as_mut().unwrap().draw(&mut Context::new(
            data.context.as_mut().unwrap(),
            &mut data.display,
        ));

        data.frame_count += 1;
        let egl = data.egl.as_mut().unwrap();
        (egl.eglSwapBuffers.unwrap())(data.display.display, data.display.surface);
    });
}

unsafe fn sapp_android_loop(
    sender: mpsc::Sender<()>,
    read_from_main_fd: RawFd,
    write_from_main_fd: RawFd,
) {
    console_info(b"Loop thread started()\0".as_ptr() as _);

    let looper = ndk_sys::ALooper_prepare(0 /* or ALOOPER_PREPARE_ALLOW_NON_CALLBACKS*/);
    ndk_sys::ALooper_addFd(
        looper,
        read_from_main_fd,
        ndk_sys::ALOOPER_POLL_CALLBACK as _,
        ndk_sys::ALOOPER_EVENT_INPUT as _,
        Some(android_main_cb),
        null_mut(),
    );

    UI_THREAD_DATA.with(|data| {
        let mut data = data.borrow_mut();

        data.read_from_main_fd = read_from_main_fd;
        data.write_from_main_fd = write_from_main_fd;
        data.sender = Some(sender);
        data.display.looper = looper;
        data.first_frame = true;
    });
    notify_main_thread();

    loop {
        // frame
        let should_update =
            UI_THREAD_DATA.with(|data| android_should_update(&data.borrow().display));

        if should_update {
            android_frame();
        }

        // process all events (or stop early if app is requested to quit)
        while {
            let should_update =
                UI_THREAD_DATA.with(|data| android_should_update(&data.borrow().display));
            let is_thread_stopping = UI_THREAD_DATA.with(|data| data.borrow().is_thread_stopping);

            let block_until_event = !is_thread_stopping && !should_update;

            !is_thread_stopping
                && ndk_sys::ALooper_pollOnce(
                    if block_until_event { -1 } else { 0 },
                    null_mut(),
                    null_mut(),
                    null_mut(),
                ) == ndk_sys::ALOOPER_POLL_CALLBACK
        } {}
    }
}

unsafe fn init_egl() -> bool {
    let mut egl = LibEgl::try_load().expect("Cant load LibEGL");

    let desc_alpha =
        UI_THREAD_DATA.with(|data| data.borrow().display.conf.platform.framebuffer_alpha);

    let (context, config, display) = egl::create_egl_context(
        &mut egl,
        null_mut(), /* EGL_DEFAULT_DISPLAY */
        desc_alpha,
    )
    .expect("Cant create EGL context");

    crate::native::gl::load_gl_funcs(|proc| {
        let name = std::ffi::CString::new(proc).unwrap();
        egl.eglGetProcAddress.expect("non-null function pointer")(name.as_ptr() as _)
    });

    UI_THREAD_DATA.with(|data| {
        let mut data = data.borrow_mut();
        data.egl = Some(egl);
        data.display.config = config;
        data.display.display = display;
        data.display.context = context;
    });
    return true;
}

unsafe fn cleanup_egl_surface(state: &mut AndroidDisplay, egl: &mut LibEgl) {
    if state.display == /* EGL_NO_DISPLAY */ null_mut() {
        return;
    }
    (egl.eglMakeCurrent.unwrap())(
        state.display,
        /* EGL_NO_SURFACE */ null_mut(),
        /* egl::EGL_NO_SURFACE */ null_mut(),
        /* egl::EGL_NO_CONTEXT */ null_mut(),
    );
    if state.surface != /* EGL_NO_SURFACE */ null_mut() {
        (egl.eglDestroySurface.unwrap())(state.display, state.surface);
        state.surface = /* EGL_NO_SURFACE */ null_mut();
    }
}

unsafe fn init_egl_surface(
    state: &mut AndroidDisplay,
    egl: &mut LibEgl,
    window: *mut ANativeWindow,
) -> bool {
    assert!(state.display.is_null() == false);
    assert!(state.context.is_null() == false);
    assert!(state.surface.is_null());
    assert!(window.is_null() == false);

    // TODO: set window flags
    // ANativeActivity_setWindowFlags(activity, AWINDOW_FLAG_KEEP_SCREEN_ON, 0);

    // create egl surface and make it current
    let surface =
        (egl.eglCreateWindowSurface.unwrap())(state.display, state.config, window as _, null_mut());

    if surface == /* EGL_NO_SURFACE  */ null_mut() {
        return false;
    }

    if (egl.eglMakeCurrent.unwrap())(state.display, surface, surface, state.context) == 0 {
        return false;
    }
    state.surface = surface;
    true
}

unsafe extern "C" fn android_main_cb(fd: RawFd, events: i32, _data: *mut std::ffi::c_void) -> i32 {
    if events as u32 & ndk_sys::ALOOPER_EVENT_INPUT == 0 {
        console_error(b"android_main_cb() encountered unsupported event\0".as_ptr() as _);
        return 1;
    }

    let size = std::mem::size_of::<AndroidMessage>();
    let mut msg = AndroidMessage::Resume;
    if libc::read(fd, &mut msg as *mut _ as *mut _, size) != size as _ {
        console_error(b"android_main_cb() could not read from read_from_main_fd\0".as_ptr() as _);
        return 1;
    }

    match msg {
        AndroidMessage::Create => {
            let result = init_egl();
            assert!(result);
            notify_main_thread();
        }
        AndroidMessage::Resume => {
            UI_THREAD_DATA.with(|data| {
                let mut data = &mut *data.borrow_mut();
                data.display.has_resumed = true;

                if data.first_frame == false {
                    data.event_handler
                        .as_mut()
                        .unwrap()
                        .window_restored_event(&mut Context::new(
                            data.context.as_mut().unwrap(),
                            &mut data.display,
                        ));
                }
            });
        }
        AndroidMessage::Pause => {
            UI_THREAD_DATA.with(|data| {
                let mut data = &mut *data.borrow_mut();
                data.display.has_resumed = false;
                if data.first_frame == false {
                    data.event_handler
                        .as_mut()
                        .unwrap()
                        .window_minimized_event(&mut Context::new(
                            data.context.as_mut().unwrap(),
                            &mut data.display,
                        ));
                }
            });
        }
        AndroidMessage::Focus => {
            UI_THREAD_DATA.with(|data| {
                let mut state = &mut data.borrow_mut().display;
                state.has_focus = true;
            });
        }
        AndroidMessage::NoFocus => {
            UI_THREAD_DATA.with(|data| {
                let mut state = &mut data.borrow_mut().display;
                state.has_focus = false;
            });
        }
        AndroidMessage::SetInputQueue(input) => {
            UI_THREAD_DATA.with(|data| {
                let mut state = &mut data.borrow_mut().display;
                if state.input != input {
                    if state.input.is_null() == false {
                        ndk_sys::AInputQueue_detachLooper(state.input);
                    }
                    if input.is_null() == false {
                        ndk_sys::AInputQueue_attachLooper(
                            input,
                            state.looper,
                            ndk_sys::ALOOPER_POLL_CALLBACK,
                            Some(android_input_cb),
                            /* data */ null_mut(),
                        );
                    }
                }
                state.input = input;
            });

            notify_main_thread();
        }
        AndroidMessage::SetNativeWindow(window) => {
            UI_THREAD_DATA.with(|data| {
                let mut data = &mut *data.borrow_mut();

                if data.display.window != window {
                    if data.display.window.is_null() == false {
                        cleanup_egl_surface(&mut data.display, data.egl.as_mut().unwrap());
                    }
                }

                if window.is_null() == false {
                    console_info(b"Creating egl surface\0".as_ptr() as _);
                    if init_egl_surface(&mut data.display, data.egl.as_mut().unwrap(), window) {
                        console_info(b"... ok!\0".as_ptr() as _);
                        android_update_dimensions(data, window, true);
                    } else {
                        console_info(b"... failed!\0".as_ptr() as _);
                        //_sapp_android_shutdown();
                    }
                }
                data.display.window = window;
            });
            notify_main_thread();
        }
        AndroidMessage::Destroy => {
            UI_THREAD_DATA.with(|data| {
                let mut data = data.borrow_mut();
                data.cleanup_android_state();
                data.is_thread_stopping = true;
            });
            notify_main_thread();
        }
    }

    1
}

unsafe fn android_touch_event(data: &mut UiThreadData, e: *const ndk_sys::AInputEvent) -> bool {
    if ndk_sys::AInputEvent_getType(e) != ndk_sys::AINPUT_EVENT_TYPE_MOTION as _ {
        return false;
    }

    let action_idx = ndk_sys::AMotionEvent_getAction(e);
    let action = action_idx & ndk_sys::AMOTION_EVENT_ACTION_MASK as i32;
    let phase = match action as u32 {
        ndk_sys::AMOTION_EVENT_ACTION_DOWN => crate::event::TouchPhase::Started,
        ndk_sys::AMOTION_EVENT_ACTION_POINTER_DOWN => crate::event::TouchPhase::Started,
        ndk_sys::AMOTION_EVENT_ACTION_MOVE => crate::event::TouchPhase::Moved,
        ndk_sys::AMOTION_EVENT_ACTION_UP | ndk_sys::AMOTION_EVENT_ACTION_POINTER_UP => {
            crate::event::TouchPhase::Ended
        }
        ndk_sys::AMOTION_EVENT_ACTION_CANCEL => crate::event::TouchPhase::Cancelled,
        _ => {
            return false;
        }
    };
    let idx = action_idx >> ndk_sys::AMOTION_EVENT_ACTION_POINTER_INDEX_SHIFT;
    let num_touches = ndk_sys::AMotionEvent_getPointerCount(e) as _;

    for i in 0..num_touches {
        let identifier = ndk_sys::AMotionEvent_getPointerId(e, i as _) as _;
        let pos_x = (ndk_sys::AMotionEvent_getX(e, i as _) / data.display.window_width as f32)
            * data.display.framebuffer_width as f32;
        let pos_y = (ndk_sys::AMotionEvent_getY(e, i as _) / data.display.window_height as f32)
            * data.display.framebuffer_height as f32;

        let changed = if action == ndk_sys::AMOTION_EVENT_ACTION_POINTER_DOWN as _
            || action == ndk_sys::AMOTION_EVENT_ACTION_POINTER_UP as _
        {
            i == idx
        } else {
            true
        };

        if changed {
            data.event_handler.as_mut().unwrap().touch_event(
                &mut Context::new(data.context.as_mut().unwrap(), &mut data.display),
                phase,
                identifier,
                pos_x,
                pos_y,
            );
        }
    }

    true
}

unsafe extern "C" fn android_input_cb(
    _fd: RawFd,
    events: i32,
    _data: *mut std::ffi::c_void,
) -> i32 {
    if events as u32 & ndk_sys::ALOOPER_EVENT_INPUT == 0 {
        console_error(b"_sapp_android_input_cb() encountered unsupported event\0".as_ptr() as _);
        return 1;
    }
    let mut event: *mut ndk_sys::AInputEvent = null_mut();

    UI_THREAD_DATA.with(|data| {
        let data = &mut *data.borrow_mut();
        let input = data.display.input;

        while ndk_sys::AInputQueue_getEvent(input, &mut event) >= 0 {
            if ndk_sys::AInputQueue_preDispatchEvent(input, event) != 0 {
                continue;
            }
            let mut handled = 0;
            if android_touch_event(data, event) {
                handled = 1;
            }
            ndk_sys::AInputQueue_finishEvent(input, event, handled);
        }
    });
    1
}

unsafe fn android_msg(msg: AndroidMessage) {
    let size = std::mem::size_of::<AndroidMessage>();
    let write_from_main_fd = MAIN_THREAD_DATA.with(|data| data.borrow().write_from_main_fd);
    let res = libc::write(write_from_main_fd, &msg as *const _ as *const _, size);
    assert_eq!(res, size as _);
}

unsafe extern "C" fn on_start(activity: *mut ANativeActivity) {
    ACTIVITY = activity;
    console_info(b"NativeActivity onStart()\0".as_ptr() as _);
}

unsafe extern "C" fn on_resume(activity: *mut ANativeActivity) {
    console_info(b"NativeActivity onResume()\0".as_ptr() as _);
    android_msg(AndroidMessage::Resume);
    auto_hide_nav_bar(activity);
}

unsafe extern "C" fn on_save_instance_state(
    _activity: *mut ANativeActivity,
    out_size: *mut ndk_sys::size_t,
) -> *mut std::ffi::c_void {
    console_info(b"NativeActivity onSaveInstanceState()\0".as_ptr() as _);
    *out_size = 0;
    null_mut()
}

unsafe extern "C" fn on_window_focus_changed(_activity: *mut ANativeActivity, has_focus: i32) {
    console_info(b"NativeActivity onFocusChange()\0".as_ptr() as _);
    if has_focus != 0 {
        android_msg(AndroidMessage::Focus);
    } else {
        android_msg(AndroidMessage::NoFocus);
    }
}

unsafe extern "C" fn on_pause(_activity: *mut ANativeActivity) {
    console_info(b"NativeActivity onPause()\0".as_ptr() as _);
    android_msg(AndroidMessage::Pause);
}

unsafe extern "C" fn on_stop(_activity: *mut ANativeActivity) {
    console_info(b"NativeActivity onStop()\0".as_ptr() as _);
}

unsafe extern "C" fn on_destroy(_activity: *mut ANativeActivity) {
    // For some reason even an empty app using nativeactivity.h will crash (WIN DEATH)
    // on my device (Moto X 2nd gen) when the app is removed from the task view
    // (TaskStackView: onTaskViewDismissed).

    // However, if ANativeActivity_finish() is explicitly called from for example
    // _sapp_android_on_stop(), the crash disappears. Is this a bug in NativeActivity?
    android_msg(AndroidMessage::Destroy);
    wait_ui_thread();

    MAIN_THREAD_DATA.with(|data| {
        let data = data.borrow_mut();
        libc::close(data.read_from_main_fd);
        libc::close(data.write_from_main_fd);
        console_info(b"NativeActivity done\0".as_ptr() as _);
    });

    // this is a bit naughty, but causes a clean restart of the app (static globals are reset)
    libc::exit(0);
}

unsafe extern "C" fn on_native_window_created(
    _activity: *mut ANativeActivity,
    window: *mut ANativeWindow,
) {
    console_info(b"NativeActivity onNativeWindowCreated()\0".as_ptr() as _);
    android_msg(AndroidMessage::SetNativeWindow(window));
    wait_ui_thread();
}

unsafe extern "C" fn on_native_window_destroyed(
    _activity: *mut ANativeActivity,
    _window: *mut ANativeWindow,
) {
    console_info(b"NativeActivity onNativeWindowDestroyed()\0".as_ptr() as _);
    android_msg(AndroidMessage::SetNativeWindow(null_mut()));
    wait_ui_thread();
}

unsafe extern "C" fn on_input_queue_created(
    _activity: *mut ANativeActivity,
    queue: *mut AInputQueue,
) {
    console_info(b"NativeActivity onInputQueueCreated()\0".as_ptr() as _);

    android_msg(AndroidMessage::SetInputQueue(queue));
    wait_ui_thread();
}

unsafe extern "C" fn on_input_queue_destroyed(
    _activity: *mut ANativeActivity,
    _queue: *mut AInputQueue,
) {
    console_info(b"NativeActivity onInputQueueDestroyed()\0".as_ptr() as _);
    android_msg(AndroidMessage::SetInputQueue(null_mut()));
    wait_ui_thread();
}

unsafe extern "C" fn on_config_changed(_activity: *mut ANativeActivity) {
    console_info(b"NativeActivity onConfigChanged()\0".as_ptr() as _);
}

unsafe extern "C" fn on_low_memory(_activity: *mut ANativeActivity) {
    console_info(b"NativeActivity onLowMemory()\0".as_ptr() as _);
}

pub unsafe fn console_debug(msg: *const ::std::os::raw::c_char) {
    ndk_sys::__android_log_write(
        ndk_sys::android_LogPriority_ANDROID_LOG_DEBUG as _,
        b"SAPP\0".as_ptr() as _,
        msg,
    );
}

pub unsafe fn console_info(msg: *const ::std::os::raw::c_char) {
    ndk_sys::__android_log_write(
        ndk_sys::android_LogPriority_ANDROID_LOG_INFO as _,
        b"SAPP\0".as_ptr() as _,
        msg,
    );
}

pub unsafe fn console_warn(msg: *const ::std::os::raw::c_char) {
    ndk_sys::__android_log_write(
        ndk_sys::android_LogPriority_ANDROID_LOG_WARN as _,
        b"SAPP\0".as_ptr() as _,
        msg,
    );
}

pub unsafe fn console_error(msg: *const ::std::os::raw::c_char) {
    ndk_sys::__android_log_write(
        ndk_sys::android_LogPriority_ANDROID_LOG_ERROR as _,
        b"SAPP\0".as_ptr() as _,
        msg,
    );
}

pub unsafe fn with_jni_env<F: FnMut(*mut ndk_sys::JNIEnv)>(mut f: F) {
    let mut env: *mut ndk_sys::JNIEnv = std::ptr::null_mut();
    let java_vm: *mut ndk_sys::JavaVM = (*ACTIVITY).vm;

    let attach_current_thread = (**java_vm).AttachCurrentThread.unwrap();
    let res = attach_current_thread(java_vm, &mut env, std::ptr::null_mut());
    assert!(res == 0);

    f(env);
}

unsafe fn auto_hide_nav_bar(activity: *mut ANativeActivity) {
    console_info(b"auto_hide_nav_bar: Start\0".as_ptr() as _);

    let mut env: *mut ndk_sys::JNIEnv = std::ptr::null_mut();
    let java_vm: *mut ndk_sys::JavaVM = (*activity).vm;

    let attach_current_thread = (**java_vm).AttachCurrentThread.unwrap();
    //let detach_current_thread = (**java_vm).DetachCurrentThread.unwrap();
    let res = attach_current_thread(java_vm, &mut env, std::ptr::null_mut());
    assert!(res == 0);

    console_info(b"auto_hide_nav_bar: Current thread attached\0".as_ptr() as _);

    let find_class = (**env).FindClass.unwrap();
    let get_method_id = (**env).GetMethodID.unwrap();
    let call_object_method = (**env).CallObjectMethod.unwrap();
    let get_static_field_id = (**env).GetStaticFieldID.unwrap();
    let get_static_int_field = (**env).GetStaticIntField.unwrap();
    let call_void_method = (**env).CallVoidMethod.unwrap();

    let activity_class = find_class(env, b"android/app/NativeActivity\0".as_ptr() as _);

    console_info(b"auto_hide_nav_bar: Got activity class\0".as_ptr() as _);

    let get_window = get_method_id(
        env,
        activity_class,
        b"getWindow\0".as_ptr() as _,
        b"()Landroid/view/Window;\0".as_ptr() as _,
    );

    let window_class = find_class(env, b"android/view/Window\0".as_ptr() as _);
    let get_decor_view = get_method_id(
        env,
        window_class,
        b"getDecorView\0".as_ptr() as _,
        b"()Landroid/view/View;\0".as_ptr() as _,
    );

    let view_class = find_class(env, b"android/view/View\0".as_ptr() as _);
    let set_system_ui_visibility = get_method_id(
        env,
        view_class,
        b"setSystemUiVisibility\0".as_ptr() as _,
        b"(I)V\0".as_ptr() as _,
    );

    console_info(b"auto_hide_nav_bar: Got set_system_ui_visibility\0".as_ptr() as _);

    let window = call_object_method(env, (*activity).clazz, get_window);
    let decor_view = call_object_method(env, window, get_decor_view);

    let flag_fullscreen_id = get_static_field_id(
        env,
        view_class,
        b"SYSTEM_UI_FLAG_FULLSCREEN\0".as_ptr() as _,
        b"I\0".as_ptr() as _,
    );
    let flag_hide_navigation_id = get_static_field_id(
        env,
        view_class,
        b"SYSTEM_UI_FLAG_HIDE_NAVIGATION\0".as_ptr() as _,
        b"I\0".as_ptr() as _,
    );
    let flag_immersive_sticky_id = get_static_field_id(
        env,
        view_class,
        b"SYSTEM_UI_FLAG_IMMERSIVE_STICKY\0".as_ptr() as _,
        b"I\0".as_ptr() as _,
    );

    console_info(b"auto_hide_nav_bar: Got flags\0".as_ptr() as _);

    let flag_fullscreen = get_static_int_field(env, view_class, flag_fullscreen_id);
    let flag_hide_navigation = get_static_int_field(env, view_class, flag_hide_navigation_id);
    let flag_immersive_sticky = get_static_int_field(env, view_class, flag_immersive_sticky_id);

    let flag = flag_fullscreen | flag_hide_navigation | flag_immersive_sticky;

    call_void_method(env, decor_view, set_system_ui_visibility, flag);

    // detach_current_thread(java_vm);

    console_info(b"auto_hide_nav_bar: Nav bar should be hidden!\0".as_ptr() as _);
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct android_asset {
    pub content: *mut ::std::os::raw::c_char,
    pub content_length: ::std::os::raw::c_int,
}

static mut ACTIVITY: *mut ANativeActivity = std::ptr::null_mut();

pub unsafe fn sapp_load_asset(filepath: *const ::std::os::raw::c_char, out: *mut android_asset) {
    let mgr = (*ACTIVITY).assetManager;
    let asset = ndk_sys::AAssetManager_open(mgr, filepath, ndk_sys::AASSET_MODE_BUFFER as _);
    if asset.is_null() {
        return;
    }
    let length = ndk_sys::AAsset_getLength64(asset);
    // TODO: memory leak right here! this buffer would never freed
    let buffer = libc::malloc(length as _);
    if ndk_sys::AAsset_read(asset, buffer, length as _) > 0 {
        ndk_sys::AAsset_close(asset);

        (*out).content_length = length as _;
        (*out).content = buffer as _;
    }
}

pub unsafe fn sapp_is_elapsed_timer_supported() -> bool {
    return false;
}
