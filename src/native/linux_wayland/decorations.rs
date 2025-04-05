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
    None,
    Server,
    LibDecor {
        libdecor: LibDecor,
        context: *mut libdecor,
        frame: *mut libdecor_frame,
    },
    Fallback(fallback::Decorations),
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
    pub(super) fn new(
        display: &mut WaylandPayload,
        fallback: crate::conf::WaylandDecorations,
    ) -> Self {
        use crate::conf::WaylandDecorations::*;
        unsafe {
            if !display.decoration_manager.is_null() {
                Decorations::server(display)
            } else {
                match fallback {
                    ServerOnly => Decorations::none(display),
                    ServerWithLibDecorFallback => Decorations::try_libdecor(display),
                    ServerWithMiniquadFallback => Decorations::fallback(display),
                }
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
            Decorations::None | Decorations::Server | Decorations::Fallback(..) => {
                wl_request!(
                    client,
                    xdg_toplevel,
                    extensions::xdg_shell::xdg_toplevel::set_title,
                    title.as_ptr()
                );
            }
            Decorations::LibDecor {
                libdecor, frame, ..
            } => {
                (libdecor.libdecor_frame_set_title)(*frame, title.as_ptr());
            }
        }
    }

    unsafe fn none(display: &mut WaylandPayload) -> Self {
        create_xdg_toplevel(display);
        Decorations::None
    }

    unsafe fn fallback(display: &mut WaylandPayload) -> Self {
        create_xdg_toplevel(display);
        let d = crate::native_display().lock().unwrap();
        let dpi_scale = d.dpi_scale as i32;
        let decorations = fallback::Decorations::new(
            display,
            d.screen_width / dpi_scale,
            d.screen_height / dpi_scale,
        );
        Decorations::Fallback(decorations)
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

    unsafe fn try_libdecor(display: &mut WaylandPayload) -> Self {
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
            Decorations::LibDecor {
                libdecor,
                context,
                frame,
            }
        } else {
            Decorations::none(display)
        }
    }

    fn libdecor(&mut self) -> Option<&mut LibDecor> {
        if let Decorations::LibDecor { libdecor, .. } = self {
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
        // Currently non-integer scales are not supported
        let dpi_scale = d.dpi_scale as i32;
        let screen_width = width * dpi_scale;
        let screen_height = height * dpi_scale;
        // screen_width / screen_height are the actual numbers of pixels
        d.screen_width = screen_width;
        d.screen_height = screen_height;
        drop(d);

        let mut window_width = screen_width;
        let mut window_height = screen_height;
        if let Decorations::Fallback(fallback) = &payload.decorations {
            window_width -= fallback::Decorations::WIDTH * 2 * dpi_scale;
            window_height -=
                (fallback::Decorations::BAR_HEIGHT + fallback::Decorations::WIDTH) * dpi_scale;
            fallback.resize(&mut payload.client, width, height);
        }
        (payload.egl.wl_egl_window_resize)(payload.egl_window, window_width, window_height, 0, 0);
        // We need to ensure that the buffer has been correctly resized before setting the
        // dpi_scale, since Wayland would rather crash than letting you have a width that's an
        // odd number on a display with 2x dpi...
        (payload.client.wl_display_dispatch_pending)(payload.display);
        wl_request!(
            payload.client,
            payload.surface,
            WL_SURFACE_SET_BUFFER_SCALE,
            dpi_scale
        );
        // The compositor can send multiple resizing configure during a single frame, and we
        // probably don't want to fire the resize event for every one of them
        // So if we still have a Resize event in the queue, instead of pushing a new one, we batch
        // them by modifying the dimension
        if let Some(WaylandEvent::Resize(width, height)) = payload.events.last_mut() {
            *width = screen_width as _;
            *height = screen_height as _;
        } else {
            payload
                .events
                .push(WaylandEvent::Resize(screen_width as _, screen_height as _));
        }
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
    let libdecor = display.decorations.libdecor().unwrap();

    let mut width: c_int = 0;
    let mut height: c_int = 0;

    if (libdecor.libdecor_configuration_get_content_size)(
        configuration,
        frame,
        &mut width,
        &mut height,
    ) == 0
    {
        let d = crate::native_display().lock().unwrap();
        let dpi_scale = d.dpi_scale as i32;
        width = d.screen_width / dpi_scale;
        height = d.screen_height / dpi_scale;
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

/// This module is drawing some sort of a window border, just for GNOME
/// looks horrible, doesn't fit OS theme at all, but better than nothing
mod fallback {
    use crate::{
        native::linux_wayland::{
            extensions::viewporter::{wp_viewport, wp_viewport_interface, wp_viewporter},
            libwayland_client::*,
            shm, WaylandPayload,
        },
        wl_request, wl_request_constructor,
    };

    pub struct Decoration {
        pub surface: *mut wl_surface,
        pub subsurface: *mut wl_subsurface,
        pub viewport: *mut wp_viewport,
    }

    pub(crate) struct Decorations {
        buffer: *mut wl_buffer,
        pub top_decoration: Decoration,
        pub bottom_decoration: Decoration,
        pub left_decoration: Decoration,
        pub right_decoration: Decoration,
    }

    #[allow(clippy::too_many_arguments)]
    unsafe fn create_decoration(
        display: &mut WaylandPayload,
        compositor: *mut wl_compositor,
        subcompositor: *mut wl_subcompositor,
        parent: *mut wl_surface,
        buffer: *mut wl_buffer,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
    ) -> Decoration {
        let surface = wl_request_constructor!(
            display.client,
            compositor,
            WL_COMPOSITOR_CREATE_SURFACE,
            display.client.wl_surface_interface,
        );

        let subsurface = wl_request_constructor!(
            display.client,
            subcompositor,
            WL_SUBCOMPOSITOR_GET_SUBSURFACE,
            display.client.wl_subsurface_interface,
            surface,
            parent
        );

        wl_request!(display.client, subsurface, WL_SUBSURFACE_SET_POSITION, x, y);

        let viewport = wl_request_constructor!(
            display.client,
            display.viewporter,
            wp_viewporter::get_viewport,
            &wp_viewport_interface,
            surface
        );

        wl_request!(display.client, viewport, wp_viewport::set_destination, w, h);
        wl_request!(display.client, surface, WL_SURFACE_ATTACH, buffer, 0, 0);
        wl_request!(display.client, surface, WL_SURFACE_COMMIT);

        Decoration {
            surface,
            subsurface,
            viewport,
        }
    }

    impl Decorations {
        pub const WIDTH: i32 = 2;
        pub const BAR_HEIGHT: i32 = 15;

        pub(super) unsafe fn new(
            display: &mut WaylandPayload,
            width: i32,
            height: i32,
        ) -> Decorations {
            let buffer = shm::create_shm_buffer(
                &mut display.client,
                display.shm,
                1,
                1,
                &[200, 200, 200, 255],
            );

            Decorations {
                buffer,
                top_decoration: create_decoration(
                    display,
                    display.compositor,
                    display.subcompositor,
                    display.surface,
                    buffer,
                    -Self::WIDTH,
                    -Self::BAR_HEIGHT,
                    width + Self::WIDTH * Self::WIDTH,
                    Self::BAR_HEIGHT,
                ),
                left_decoration: create_decoration(
                    display,
                    display.compositor,
                    display.subcompositor,
                    display.surface,
                    buffer,
                    -Self::WIDTH,
                    -Self::BAR_HEIGHT,
                    Self::WIDTH,
                    height + Self::BAR_HEIGHT,
                ),
                right_decoration: create_decoration(
                    display,
                    display.compositor,
                    display.subcompositor,
                    display.surface,
                    buffer,
                    width,
                    -Self::BAR_HEIGHT,
                    Self::WIDTH,
                    height + Self::BAR_HEIGHT,
                ),
                bottom_decoration: create_decoration(
                    display,
                    display.compositor,
                    display.subcompositor,
                    display.surface,
                    buffer,
                    -Self::WIDTH,
                    height,
                    width + Self::WIDTH,
                    Self::WIDTH,
                ),
            }
        }

        pub unsafe fn resize(&self, client: &mut LibWaylandClient, width: i32, height: i32) {
            wl_request!(
                client,
                self.top_decoration.viewport,
                wp_viewport::set_destination,
                width,
                Self::BAR_HEIGHT
            );
            wl_request!(client, self.top_decoration.surface, WL_SURFACE_COMMIT);

            wl_request!(
                client,
                self.left_decoration.viewport,
                wp_viewport::set_destination,
                Self::WIDTH,
                height
            );
            wl_request!(client, self.left_decoration.surface, WL_SURFACE_COMMIT);

            wl_request!(
                client,
                self.right_decoration.subsurface,
                WL_SUBSURFACE_SET_POSITION,
                width - Self::WIDTH * 2,
                -Self::BAR_HEIGHT
            );
            wl_request!(
                client,
                self.right_decoration.viewport,
                wp_viewport::set_destination,
                Self::WIDTH,
                height
            );
            wl_request!(client, self.right_decoration.surface, WL_SURFACE_COMMIT);

            wl_request!(
                client,
                self.bottom_decoration.subsurface,
                WL_SUBSURFACE_SET_POSITION,
                0,
                height - Self::BAR_HEIGHT - Self::WIDTH
            );
            wl_request!(
                client,
                self.bottom_decoration.viewport,
                wp_viewport::set_destination,
                width - Self::WIDTH * 2,
                Self::WIDTH
            );
            wl_request!(client, self.bottom_decoration.surface, WL_SURFACE_COMMIT);
        }
    }
}
