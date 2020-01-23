#![allow(warnings)]

#[cfg(not(any(target_os = "linux", target_arch = "wasm32", windows)))]
extern crate sapp_dummy as sapp;
#[cfg(target_os = "linux")]
extern crate sapp_linux as sapp;
#[cfg(target_arch = "wasm32")]
extern crate sapp_wasm as sapp;
#[cfg(windows)]
extern crate sapp_windows as sapp;

pub mod conf;
mod event;
pub mod graphics;

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
        unsafe { sapp::time() as f64 }
    }
}

struct UserData {
    event_handler: Box<dyn EventHandler>,
    context: Context,
}

enum UserDataState {
    Uninitialized(Box<dyn 'static + FnOnce(&mut Context) -> Box<dyn event::EventHandler>>),
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

    let user_data = UserData {
        event_handler: f(&mut context),
        context,
    };
    std::mem::replace(data, UserDataState::Intialized(user_data));
}

extern "C" fn frame(user_data: *mut ::std::os::raw::c_void) {
    let data: &mut UserDataState = unsafe { &mut *(user_data as *mut UserDataState) };

    let data = if let UserDataState::Intialized(ref mut data) = data {
        data
    } else {
        panic!()
    };

    data.event_handler.update(&mut data.context);
    data.event_handler.draw(&mut data.context);
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
            data.event_handler.mouse_motion_event(
                &mut data.context,
                event.mouse_x,
                event.mouse_y,
                0.,
                0.,
            );
        }
        sapp::sapp_event_type_SAPP_EVENTTYPE_MOUSE_DOWN => {
            data.event_handler.mouse_button_down_event(
                &mut data.context,
                MouseButton::Left,
                event.mouse_x,
                event.mouse_y,
            );
        }
        sapp::sapp_event_type_SAPP_EVENTTYPE_MOUSE_UP => {
            data.event_handler.mouse_button_up_event(
                &mut data.context,
                MouseButton::Left,
                event.mouse_x,
                event.mouse_y,
            );
        }
        sapp::sapp_event_type_SAPP_EVENTTYPE_CHAR => {
            if let Some(character) = std::char::from_u32(event.char_code) {
                let mut key_mods = KeyMods::from(event.modifiers);

                data.event_handler.char_event(
                    &mut data.context,
                    character,
                    key_mods,
                    event.key_repeat,
                )
            }
        }
        sapp::sapp_event_type_SAPP_EVENTTYPE_KEY_DOWN => {
            let keycode = KeyCode::from(event.key_code);
            let mut key_mods = KeyMods::from(event.modifiers);

            data.event_handler
                .key_down_event(&mut data.context, keycode, key_mods, false)
        }
        sapp::sapp_event_type_SAPP_EVENTTYPE_KEY_UP => {
            let keycode = KeyCode::from(event.key_code);
            let mut key_mods = KeyMods::from(event.modifiers);

            data.event_handler
                .key_up_event(&mut data.context, keycode, key_mods)
        }
        sapp::sapp_event_type_SAPP_EVENTTYPE_RESIZED => {
            data.context
                .resize(event.window_width as u32, event.window_height as u32);
            data.event_handler.resize_event(
                &mut data.context,
                event.window_width as f32,
                event.window_height as f32,
            );
        }
        _ => {}
    }
}

pub fn start<F>(_conf: conf::Conf, f: F)
where
    F: 'static + FnOnce(&mut Context) -> Box<dyn event::EventHandler>,
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
