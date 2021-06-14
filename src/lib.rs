#[cfg(target_os = "android")]
extern crate sapp_android as sapp;

#[cfg(target_os = "android")]
pub use sapp_android;

#[cfg(target_os = "macos")]
extern crate sapp_darwin as sapp;
#[cfg(not(any(
    target_os = "linux",
    target_os = "macos",
    target_os = "ios",
    target_os = "android",
    target_arch = "wasm32",
    windows
)))]
extern crate sapp_dummy as sapp;
#[cfg(target_os = "ios")]
extern crate sapp_ios as sapp;
#[cfg(all(target_os = "linux", feature = "kms"))]
extern crate sapp_kms as sapp;
#[cfg(all(target_os = "linux", not(feature = "kms")))]
extern crate sapp_linux as sapp;

#[cfg(target_arch = "wasm32")]
extern crate sapp_wasm as sapp;
#[cfg(windows)]
extern crate sapp_windows as sapp;

pub mod clipboard;
pub mod conf;
mod event;
pub mod fs;
pub mod graphics;

#[cfg(feature = "log-impl")]
pub mod log;

pub use event::*;

pub use graphics::*;

pub use sapp::gl;

use std::ffi::CString;

#[deprecated(
    since = "0.3",
    note = "libc rand is slow and incosistent across platforms. Please use quad-rnd crate instead."
)]
pub unsafe fn rand() -> i32 {
    sapp::rand()
}
#[deprecated(
    since = "0.3",
    note = "libc rand is slow and incosistent across platforms. Please use quad-rnd crate instead."
)]
pub const RAND_MAX: u32 = sapp::RAND_MAX;

pub mod date {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn now() -> f64 {
        use std::time::SystemTime;

        let time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_else(|e| panic!("{}", e));
        time.as_secs_f64()
    }

    #[cfg(target_arch = "wasm32")]
    pub fn now() -> f64 {
        unsafe { sapp::now() }
    }
}

impl Context {
    /// This function simply quits the application without
    /// giving the user a chance to intervene. Usually this might
    /// be called when the user clicks the 'Ok' button in a 'Really Quit?'
    /// dialog box
    pub fn quit(&self) {
        // its not possible to quit wasm anyway
        #[cfg(not(target_arch = "wasm32"))]
        unsafe {
            sapp::sapp_quit();
        }
    }

    /// Calling request_quit() will trigger "quit_requested_event" event , giving
    /// the user code a chance to intervene and cancel the pending quit process
    /// (for instance to show a 'Really Quit?' dialog box).
    /// If the event handler callback does nothing, the application will be quit as usual.
    /// To prevent this, call the function "cancel_quit()"" from inside the event handler.
    pub fn request_quit(&self) {
        // its not possible to quit wasm anyway
        #[cfg(not(target_arch = "wasm32"))]
        unsafe {
            sapp::sapp_request_quit();
        }
    }

    /// Cancels a pending quit request, either initiated
    /// by the user clicking the window close button, or programmatically
    /// by calling "request_quit()". The only place where calling this
    /// function makes sense is from inside the event handler callback when
    /// the "quit_requested_event" event has been received
    pub fn cancel_quit(&self) {
        // its not possible to quit wasm anyway
        #[cfg(not(target_arch = "wasm32"))]
        unsafe {
            sapp::sapp_cancel_quit();
        }
    }

    /// Capture mouse cursor to the current window
    /// On WASM this will automatically hide cursor
    /// On desktop this will bound cursor to windows border
    /// NOTICE: on desktop cursor will not be automatically released after window lost focus
    ///         so set_cursor_grab(false) on window's focus lost is recommended.
    /// TODO: implement window focus events
    pub fn set_cursor_grab(&self, grab: bool) {
        #[cfg(not(target_os = "ios"))]
        unsafe {
            sapp::sapp_set_cursor_grab(grab);
        }
    }

    /// Show or hide the mouse cursor
    pub fn show_mouse(&self, shown: bool) {
        unsafe {
            sapp::sapp_show_mouse(shown);
        }
    }

    /// Set the mouse cursor icon.
    pub fn set_mouse_cursor(&self, _cursor_icon: CursorIcon) {
        #[cfg(any(
            target_arch = "wasm32",
            all(target_os = "linux", not(feature = "kms")),
            windows,
        ))]
        unsafe {
            sapp::sapp_set_mouse_cursor(match _cursor_icon {
                CursorIcon::Default => sapp::SAPP_CURSOR_DEFAULT,
                CursorIcon::Help => sapp::SAPP_CURSOR_HELP,
                CursorIcon::Pointer => sapp::SAPP_CURSOR_POINTER,
                CursorIcon::Wait => sapp::SAPP_CURSOR_WAIT,
                CursorIcon::Crosshair => sapp::SAPP_CURSOR_CROSSHAIR,
                CursorIcon::Text => sapp::SAPP_CURSOR_TEXT,
                CursorIcon::Move => sapp::SAPP_CURSOR_MOVE,
                CursorIcon::NotAllowed => sapp::SAPP_CURSOR_NOTALLOWED,
                CursorIcon::EWResize => sapp::SAPP_CURSOR_EWRESIZE,
                CursorIcon::NSResize => sapp::SAPP_CURSOR_NSRESIZE,
                CursorIcon::NESWResize => sapp::SAPP_CURSOR_NESWRESIZE,
                CursorIcon::NWSEResize => sapp::SAPP_CURSOR_NWSERESIZE,
            });
        }
    }

    /// Set the application's window size.
    #[allow(unused_variables)]
    pub fn set_window_size(&self, new_width: u32, new_height: u32) {
        #[cfg(not(any(
            target_os = "linux",
            target_os = "macos",
            target_os = "ios",
            target_os = "android",
        )))]
        unsafe {
            if sapp::sapp_is_fullscreen() {
                #[cfg(feature = "log-impl")]
                warn!("Unable to set windowsize while fullscreen: https://github.com/not-fl3/miniquad/issues/179");
                return;
            }

            sapp::sapp_set_window_size(new_width, new_height);
        }
    }

    #[allow(unused_variables)]
    pub fn set_fullscreen(&self, fullscreen: bool) {
        #[cfg(not(any(
            target_os = "linux",
            target_os = "macos",
            target_os = "ios",
            target_os = "android",
        )))]
        unsafe {
            sapp::sapp_set_fullscreen(fullscreen);
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
pub enum CursorIcon {
    Default,
    Help,
    Pointer,
    Wait,
    Crosshair,
    Text,
    Move,
    NotAllowed,
    EWResize,
    NSResize,
    NESWResize,
    NWSEResize,
}

pub enum UserData {
    Owning((Box<dyn EventHandler>, Context)),
    Free(Box<dyn EventHandlerFree>),
}

impl UserData {
    pub fn owning(event_handler: impl EventHandler + 'static, ctx: Context) -> UserData {
        UserData::Owning((Box::new(event_handler), ctx))
    }

    pub fn free(event_handler: impl EventHandlerFree + 'static) -> UserData {
        UserData::Free(Box::new(event_handler))
    }
}

/// call appropriate event handler function - with or without Context reference
macro_rules! event_call {
    ( $event_handler:expr, $fn:ident $(, $args:expr)*) => {{
        match $event_handler {
            UserData::Owning((ref mut event_handler, ref mut context)) => {
                event_handler.$fn(context, $($args,)*);
            }
            UserData::Free(ref mut event_handler) => {
                event_handler.$fn($($args,)*);
            }
        }
    }};
}

enum UserDataState {
    Uninitialized(Box<dyn 'static + FnOnce(Context) -> UserData>),
    Intialized(UserData),
    Empty,
}

extern "C" fn init(user_data: *mut ::std::os::raw::c_void) {
    let data: &mut UserDataState = unsafe { &mut *(user_data as *mut UserDataState) };
    let empty = UserDataState::Empty;

    let f = std::mem::replace(data, empty);
    let f = if let UserDataState::Uninitialized(f) = f {
        f
    } else {
        panic!();
    };
    let context = graphics::Context::new();

    let user_data = f(context);
    *data = UserDataState::Intialized(user_data);
}

extern "C" fn frame(user_data: *mut ::std::os::raw::c_void) {
    let data: &mut UserDataState = unsafe { &mut *(user_data as *mut UserDataState) };

    let data = if let UserDataState::Intialized(ref mut data) = data {
        data
    } else {
        panic!()
    };

    event_call!(data, update);
    event_call!(data, draw);
}

extern "C" fn event(event: *const sapp::sapp_event, user_data: *mut ::std::os::raw::c_void) {
    let data: &mut UserDataState = unsafe { &mut *(user_data as *mut UserDataState) };
    let event = unsafe { &*event };

    let data = if let UserDataState::Intialized(ref mut data) = data {
        data
    } else {
        panic!()
    };

    match event.type_ {
        sapp::sapp_event_type_SAPP_EVENTTYPE_MOUSE_MOVE => {
            event_call!(data, mouse_motion_event, event.mouse_x, event.mouse_y);
        }
        sapp::sapp_event_type_SAPP_EVENTTYPE_MOUSE_SCROLL => {
            event_call!(data, mouse_wheel_event, event.scroll_x, event.scroll_y);
        }
        sapp::sapp_event_type_SAPP_EVENTTYPE_MOUSE_DOWN => {
            event_call!(
                data,
                mouse_button_down_event,
                MouseButton::from(event.mouse_button),
                event.mouse_x,
                event.mouse_y
            );
        }
        sapp::sapp_event_type_SAPP_EVENTTYPE_MOUSE_UP => {
            event_call!(
                data,
                mouse_button_up_event,
                MouseButton::from(event.mouse_button),
                event.mouse_x,
                event.mouse_y
            );
        }
        sapp::sapp_event_type_SAPP_EVENTTYPE_CHAR => {
            if let Some(character) = std::char::from_u32(event.char_code) {
                let key_mods = KeyMods::from(event.modifiers);

                event_call!(data, char_event, character, key_mods, event.key_repeat)
            }
        }
        sapp::sapp_event_type_SAPP_EVENTTYPE_KEY_DOWN => {
            let keycode = KeyCode::from(event.key_code);
            let key_mods = KeyMods::from(event.modifiers);

            event_call!(data, key_down_event, keycode, key_mods, event.key_repeat)
        }
        sapp::sapp_event_type_SAPP_EVENTTYPE_KEY_UP => {
            let keycode = KeyCode::from(event.key_code);
            let key_mods = KeyMods::from(event.modifiers);

            event_call!(data, key_up_event, keycode, key_mods);
        }
        sapp::sapp_event_type_SAPP_EVENTTYPE_RESIZED => {
            event_call!(
                data,
                resize_event,
                event.framebuffer_width as f32,
                event.framebuffer_height as f32
            );
        }
        sapp::sapp_event_type_SAPP_EVENTTYPE_TOUCHES_BEGAN
        | sapp::sapp_event_type_SAPP_EVENTTYPE_TOUCHES_ENDED
        | sapp::sapp_event_type_SAPP_EVENTTYPE_TOUCHES_CANCELLED
        | sapp::sapp_event_type_SAPP_EVENTTYPE_TOUCHES_MOVED => {
            for i in 0..(event.num_touches as usize) {
                if event.touches[i].changed {
                    event_call!(
                        data,
                        touch_event,
                        event.type_.into(),
                        event.touches[i].identifier as u64,
                        event.touches[i].pos_x,
                        event.touches[i].pos_y
                    );
                }
            }
        }
        sapp::sapp_event_type_SAPP_EVENTTYPE_QUIT_REQUESTED => {
            event_call!(data, quit_requested_event);
        }
        #[cfg(not(target_os = "ios"))]
        sapp::sapp_event_type_SAPP_EVENTTYPE_RAW_DEVICE => {
            event_call!(data, raw_mouse_motion, event.mouse_dx, event.mouse_dy);
        }
        sapp::sapp_event_type_SAPP_EVENTTYPE_RESUMED => {
            event_call!(data, window_restored_event);
        }
        sapp::sapp_event_type_SAPP_EVENTTYPE_SUSPENDED => {
            event_call!(data, window_minimized_event);
        }
        _ => {}
    }
}

/// Start miniquad.
/// Initialization callback will be called when miniquad's Context is ready.
/// User can take ownership on Context and store it in user Code. Or return it back
/// to miniquad and give miniquad ownership on Context.
///
/// Variant wth EventHandler:
/// ```no_run
/// # use miniquad::*;
/// struct Stage;
///
/// impl EventHandler for Stage {
///     fn update(&mut self, _ctx: &mut Context) {}
///     fn draw(&mut self, _ctx: &mut Context) {}
/// }
/// fn main() {
///     miniquad::start(conf::Conf::default(), |ctx| UserData::owning(Stage, ctx));
/// }
/// ```
///
/// Variant wth EventHandlerFree:
/// ```no_run
/// # use miniquad::*;
/// struct Stage {
///     ctx: Context,
/// }
/// impl EventHandlerFree for Stage {
///     fn update(&mut self) {}
///     fn draw(&mut self) {}
/// }
/// fn main() {
///     miniquad::start(conf::Conf::default(), |ctx| UserData::free(Stage { ctx }));
/// }
/// ```
pub fn start<F>(conf: conf::Conf, f: F)
where
    F: 'static + FnOnce(Context) -> UserData,
{
    let mut desc: sapp::sapp_desc = unsafe { std::mem::zeroed() };

    let title = CString::new(conf.window_title.as_bytes()).unwrap_or_else(|e| panic!("{}", e));

    let mut user_data = Box::new(UserDataState::Uninitialized(Box::new(f)));

    desc.sample_count = conf.sample_count;
    desc.width = conf.window_width;
    desc.height = conf.window_height;
    desc.fullscreen = conf.fullscreen as _;
    desc.high_dpi = conf.high_dpi as _;
    desc.window_title = title.as_ptr();

    #[cfg(not(any(target_os = "macos", target_os = "ios", target_os = "android",)))]
    {
        desc.window_resizable = conf.window_resizable as _;
    }

    desc.user_data = &mut *user_data as *mut _ as *mut _;
    desc.init_userdata_cb = Some(init);
    desc.frame_userdata_cb = Some(frame);
    desc.event_userdata_cb = Some(event);

    Box::leak(user_data);

    unsafe { sapp::sapp_run(&desc as *const _) };
}
