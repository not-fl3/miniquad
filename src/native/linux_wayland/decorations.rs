//! Window decorations(borders, titlebards, close/minimize buttons) are optional
//! Most importantly they are not implemented on Gnome so we need to do some
//! extra work.
//!
//! We use `libdecor` which is also used by `xwayland` so we will get the same
//! decorations as the X11 backend. If it's not available then no decorations
//! will be drawn at all.

#![allow(static_mut_refs)]

use super::*;
use crate::{wl_request, wl_request_constructor};
use core::ffi::{c_char, c_int, c_void};
use extensions::libdecor::*;
use extensions::xdg_shell::*;

/// Window decorations, whether they should be done by the compositor or by us.
/// In the later case we use `libdecor` so it needs to be loaded.
/// If it's not available then no decorations will be drawn at all.
pub(super) enum Decorations {
    Server,
    Client {
        libdecor: LibDecor,
        context: *mut libdecor,
        frame: *mut libdecor_frame,
    },
}

// If we use client decorations, `libdecor` will handle the creation for us.
// So this is used for either server or no decorations.
unsafe fn create_xdg_toplevel(display: &mut WaylandPayload) {
    let xdg_surface: *mut xdg_surface = wl_request_constructor!(
        display.client,
        display.xdg_wm_base,
        extensions::xdg_shell::xdg_wm_base::get_xdg_surface,
        &xdg_surface_interface,
        display.surface
    );
    assert!(!xdg_surface.is_null());
    (display.client.wl_proxy_add_listener)(
        xdg_surface as _,
        &XDG_SURFACE_LISTENER as *const _ as _,
        display as *mut _ as _,
    );

    display.xdg_toplevel = wl_request_constructor!(
        display.client,
        xdg_surface,
        extensions::xdg_shell::xdg_surface::get_toplevel,
        &extensions::xdg_shell::xdg_toplevel_interface
    );
    assert!(!display.xdg_toplevel.is_null());
    (display.client.wl_proxy_add_listener)(
        display.xdg_toplevel as _,
        &XDG_TOPLEVEL_LISTENER as *const _ as _,
        display as *mut _ as _,
    );
}

impl Decorations {
    pub(super) fn new(display: &mut WaylandPayload) -> Option<Self> {
        unsafe {
            if display.decoration_manager.is_null() {
                Decorations::try_client(display)
            } else {
                Some(Decorations::server(display))
            }
        }
    }

    pub(super) unsafe fn set_title(
        &mut self,
        client: &mut LibWaylandClient,
        xdg_toplevel: *mut xdg_toplevel,
        title: &str,
    ) {
        let title = std::ffi::CString::new(title).unwrap();
        match self {
            Decorations::Server => {
                wl_request!(
                    client,
                    xdg_toplevel,
                    extensions::xdg_shell::xdg_toplevel::set_title,
                    title.as_ptr()
                );
            }
            Decorations::Client {
                libdecor, frame, ..
            } => {
                (libdecor.libdecor_frame_set_title)(*frame, title.as_ptr());
            }
        }
    }

    unsafe fn server(display: &mut WaylandPayload) -> Self {
        create_xdg_toplevel(display);

        let server_decoration: *mut extensions::xdg_decoration::zxdg_toplevel_decoration_v1 = wl_request_constructor!(
            display.client,
            display.decoration_manager,
            extensions::xdg_decoration::zxdg_decoration_manager_v1::get_toplevel_decoration,
            &extensions::xdg_decoration::zxdg_toplevel_decoration_v1_interface,
            display.xdg_toplevel
        );
        assert!(!server_decoration.is_null());

        wl_request!(
            display.client,
            server_decoration,
            extensions::xdg_decoration::zxdg_toplevel_decoration_v1::set_mode,
            extensions::xdg_decoration::ZXDG_TOPLEVEL_DECORATION_V1_MODE_SERVER_SIDE
        );
        Decorations::Server
    }

    unsafe fn try_client(display: &mut WaylandPayload) -> Option<Self> {
        if let Ok(libdecor) = LibDecor::try_load() {
            let context = (libdecor.libdecor_new)(display.display, &mut LIBDECOR_INTERFACE as _);
            let frame = (libdecor.libdecor_decorate)(
                context,
                display.surface,
                &mut LIBDECOR_FRAME_INTERFACE as _,
                display as *mut _ as _,
            );
            (libdecor.libdecor_frame_map)(frame);
            display.xdg_toplevel = (libdecor.libdecor_frame_get_xdg_toplevel)(frame);
            assert!(!display.xdg_toplevel.is_null());
            Some(Decorations::Client {
                libdecor,
                context,
                frame,
            })
        } else {
            // If we can't load `libdecor` we just create the `xdg_toplevel` and return `None`
            create_xdg_toplevel(display);
            None
        }
    }

    fn libdecor(&mut self) -> Option<&mut LibDecor> {
        if let Decorations::Client { libdecor, .. } = self {
            Some(libdecor)
        } else {
            None
        }
    }
}

unsafe extern "C" fn xdg_surface_handle_configure(
    data: *mut std::ffi::c_void,
    xdg_surface: *mut extensions::xdg_shell::xdg_surface,
    serial: u32,
) {
    assert!(!data.is_null());
    let payload: &mut WaylandPayload = &mut *(data as *mut _);

    wl_request!(
        payload.client,
        xdg_surface,
        extensions::xdg_shell::xdg_surface::ack_configure,
        serial
    );
    wl_request!(payload.client, payload.surface, WL_SURFACE_COMMIT)
}

unsafe extern "C" fn handle_configure(data: *mut std::ffi::c_void, width: i32, height: i32) {
    assert!(!data.is_null());
    let payload: &mut WaylandPayload = &mut *(data as *mut _);

    if width != 0 && height != 0 {
        let mut d = crate::native_display().lock().unwrap();
        let screen_width = ((width as f32) * d.dpi_scale) as i32;
        let screen_height = ((height as f32) * d.dpi_scale) as i32;
        // screen_width / screen_height are the actual numbers of pixels
        d.screen_width = screen_width;
        d.screen_height = screen_height;
        drop(d);

        (payload.egl.wl_egl_window_resize)(payload.egl_window, screen_width, screen_height, 0, 0);
        payload
            .events
            .push(WaylandEvent::Resize(screen_width as _, screen_height as _));
    }
}

unsafe extern "C" fn xdg_toplevel_handle_configure(
    data: *mut std::ffi::c_void,
    _toplevel: *mut extensions::xdg_shell::xdg_toplevel,
    width: i32,
    height: i32,
    _states: *mut wl_array,
) {
    handle_configure(data, width, height);
}

unsafe extern "C" fn libdecor_frame_handle_configure(
    frame: *mut libdecor_frame,
    configuration: *mut libdecor_configuration,
    data: *mut c_void,
) {
    let display: &mut WaylandPayload = &mut *(data as *mut _);
    let libdecor = display.decorations.as_mut().unwrap().libdecor().unwrap();

    let mut width: c_int = 0;
    let mut height: c_int = 0;

    if (libdecor.libdecor_configuration_get_content_size)(
        configuration,
        frame,
        &mut width,
        &mut height,
    ) == 0
    {
        // libdecor failed to retrieve the new dimension, so we use the old value
        let d = crate::native_display().lock().unwrap();
        width = d.screen_width;
        height = d.screen_height;
        drop(d);
    }
    let state = (libdecor.libdecor_state_new)(width, height);
    (libdecor.libdecor_frame_commit)(frame, state, configuration);
    (libdecor.libdecor_state_free)(state);

    handle_configure(data, width, height);
}

unsafe extern "C" fn xdg_toplevel_handle_close(
    _data: *mut std::ffi::c_void,
    _xdg_toplevel: *mut extensions::xdg_shell::xdg_toplevel,
) {
    crate::native_display().try_lock().unwrap().quit_requested = true;
}

unsafe extern "C" fn libdecor_frame_handle_close(_frame: *mut libdecor_frame, _data: *mut c_void) {
    crate::native_display().try_lock().unwrap().quit_requested = true;
}

unsafe extern "C" fn libdecor_frame_handle_commit(_frame: *mut libdecor_frame, _data: *mut c_void) {
}
unsafe extern "C" fn libdecor_handle_error(
    _context: *mut libdecor,
    _error: *mut libdecor_error,
    message: *const c_char,
) {
    let message = core::ffi::CStr::from_ptr(message).to_str().unwrap();
    eprintln!("{}", message);
}
static mut LIBDECOR_INTERFACE: libdecor_interface = libdecor_interface {
    error: libdecor_handle_error,
};
static mut LIBDECOR_FRAME_INTERFACE: libdecor_frame_interface = libdecor_frame_interface {
    configure: libdecor_frame_handle_configure,
    close: libdecor_frame_handle_close,
    commit: libdecor_frame_handle_commit,
};
static mut XDG_TOPLEVEL_LISTENER: xdg_toplevel_listener = xdg_toplevel_listener {
    configure: xdg_toplevel_handle_configure,
    close: xdg_toplevel_handle_close,
};
static mut XDG_SURFACE_LISTENER: xdg_surface_listener = xdg_surface_listener {
    configure: xdg_surface_handle_configure,
};
