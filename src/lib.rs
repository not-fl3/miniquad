#![allow(warnings)]

#[cfg(target_os = "macos")]
extern crate sapp_darwin as sapp;
#[cfg(not(any(
    target_os = "linux",
    target_os = "macos",
    target_arch = "wasm32",
    windows
)))]
extern crate sapp_dummy as sapp;
#[cfg(target_os = "linux")]
extern crate sapp_linux as sapp;
#[cfg(target_arch = "wasm32")]
extern crate sapp_wasm as sapp;
#[cfg(windows)]
extern crate sapp_windows as sapp;

pub mod conf;
mod event;
pub mod fs;
pub mod graphics;

#[cfg(feature = "log-impl")]
pub mod log;

pub use event::*;

pub use graphics::*;

use std::ffi::CString;

pub use sapp::{rand, RAND_MAX};

pub mod date {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn now() -> f64 {
        use std::time::SystemTime;

        let time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_else(|e| panic!(e));
        time.as_secs_f64()
    }

    #[cfg(target_arch = "wasm32")]
    pub fn now() -> f64 {
        unsafe { sapp::time() }
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

macro_rules! magic_call {
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
    let mut context = graphics::Context::new();

    let user_data = f(context);
    std::mem::replace(data, UserDataState::Intialized(user_data));
}

extern "C" fn frame(user_data: *mut ::std::os::raw::c_void) {
    let data: &mut UserDataState = unsafe { &mut *(user_data as *mut UserDataState) };

    let data = if let UserDataState::Intialized(ref mut data) = data {
        data
    } else {
        panic!()
    };

    magic_call!(data, update);
    magic_call!(data, draw);
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
            magic_call!(
                data,
                mouse_motion_event,
                event.mouse_x,
                event.mouse_y,
                0.,
                0.
            );
        }
        sapp::sapp_event_type_SAPP_EVENTTYPE_MOUSE_SCROLL => {
            magic_call!(data, mouse_wheel_event, event.scroll_x, event.scroll_y);
        }
        sapp::sapp_event_type_SAPP_EVENTTYPE_MOUSE_DOWN => {
            magic_call!(
                data,
                mouse_button_down_event,
                MouseButton::from(event.mouse_button),
                event.mouse_x,
                event.mouse_y
            );
        }
        sapp::sapp_event_type_SAPP_EVENTTYPE_MOUSE_UP => {
            let btn = MouseButton::from(event.mouse_button);
            magic_call!(
                data,
                mouse_button_up_event,
                MouseButton::from(event.mouse_button),
                event.mouse_x,
                event.mouse_y
            );
        }
        sapp::sapp_event_type_SAPP_EVENTTYPE_CHAR => {
            if let Some(character) = std::char::from_u32(event.char_code) {
                let mut key_mods = KeyMods::from(event.modifiers);

                magic_call!(data, char_event, character, key_mods, event.key_repeat)
            }
        }
        sapp::sapp_event_type_SAPP_EVENTTYPE_KEY_DOWN => {
            let keycode = KeyCode::from(event.key_code);
            let mut key_mods = KeyMods::from(event.modifiers);

            magic_call!(data, key_down_event, keycode, key_mods, false)
        }
        sapp::sapp_event_type_SAPP_EVENTTYPE_KEY_UP => {
            let keycode = KeyCode::from(event.key_code);
            let mut key_mods = KeyMods::from(event.modifiers);

            magic_call!(data, key_up_event, keycode, key_mods);
        }
        sapp::sapp_event_type_SAPP_EVENTTYPE_RESIZED => {
            magic_call!(
                data,
                resize_event,
                event.window_width as f32,
                event.window_height as f32
            );
        }
        sapp::sapp_event_type_SAPP_EVENTTYPE_TOUCHES_BEGAN
        | sapp::sapp_event_type_SAPP_EVENTTYPE_TOUCHES_ENDED
        | sapp::sapp_event_type_SAPP_EVENTTYPE_TOUCHES_CANCELLED
        | sapp::sapp_event_type_SAPP_EVENTTYPE_TOUCHES_MOVED => {
            for i in 0..(event.num_touches as usize) {
                if event.touches[i].changed {
                    magic_call!(
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
            magic_call!(data, quit_requested_event);
        }
        _ => {}
    }
}

pub fn start<F>(_conf: conf::Conf, f: F)
where
    F: 'static + FnOnce(Context) -> UserData,
{
    let mut desc: sapp::sapp_desc = unsafe { std::mem::zeroed() };

    let title = CString::new("").unwrap_or_else(|e| panic!(e));

    let mut user_data = Box::new(UserDataState::Uninitialized(Box::new(f)));

    desc.width = 800;
    desc.height = 600;
    desc.window_title = title.as_ptr();
    desc.user_data = &mut *user_data as *mut _ as *mut _;
    desc.init_userdata_cb = Some(init);
    desc.frame_userdata_cb = Some(frame);
    desc.event_userdata_cb = Some(event);

    std::mem::forget(user_data);

    unsafe { sapp::sapp_run(&desc as *const _) };
}
