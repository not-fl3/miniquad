mod egl;
pub mod gl;
mod wayland_client;
mod wayland_egl;

#[macro_use]
mod extensions;

mod shm;

use extensions::{
    viewporter::{wp_viewport, wp_viewport_interface, wp_viewporter, wp_viewporter_interface},
    xdg_decoration::{
        zxdg_decoration_manager_v1, zxdg_decoration_manager_v1_interface,
        zxdg_toplevel_decoration_v1, zxdg_toplevel_decoration_v1_interface,
        ZXDG_TOPLEVEL_DECORATION_V1_MODE_SERVER_SIDE,
    },
    xdg_shell::{self, xdg_surface, xdg_toplevel, xdg_wm_base},
};

use crate::wayland::wayland_client::*;

use egl::{eglGetDisplay, eglInitialize};

struct Decoration {
    surface: *mut wl_surface,
}

struct Decorations {
    buffer: *mut wl_buffer,
    top_decoration: Decoration,
    bottom_decoration: Decoration,
    left_decoration: Decoration,
    right_decoration: Decoration,
}

struct GlobalState {
    compositor: *mut wl_compositor,
    subcompositor: *mut wl_subcompositor,
    xdg_toplevel: *mut xdg_toplevel,
    xdg_wm_base: *mut xdg_wm_base,
    surface: *mut wl_surface,
    decoration_manager: *mut zxdg_decoration_manager_v1,
    viewporter: *mut wp_viewporter,
    shm: *mut wl_shm,
    seat: *mut wl_seat,

    pointer: *mut wl_pointer,
    focused_window: *mut wl_surface,

    decorations: Option<Decorations>,

    closed: bool,
}

static mut GLOBALS: GlobalState = GlobalState {
    compositor: std::ptr::null_mut(),
    subcompositor: std::ptr::null_mut(),
    xdg_toplevel: std::ptr::null_mut(),
    xdg_wm_base: std::ptr::null_mut(),
    surface: std::ptr::null_mut(),
    decoration_manager: std::ptr::null_mut(),
    viewporter: std::ptr::null_mut(),
    shm: std::ptr::null_mut(),
    seat: std::ptr::null_mut(),

    pointer: std::ptr::null_mut(),
    focused_window: std::ptr::null_mut(),

    decorations: None,

    closed: false,
};

#[macro_export]
macro_rules! wl_request_constructor {
    ($instance:expr, $request_name:expr, $interface:expr) => {
        wl_request_constructor!($instance, $request_name, $interface, ())
    };

    ($instance:expr, $request_name:expr, $interface:expr, $($arg:expr),*) => {{
        let id: *mut wl_proxy;

        id = wl_proxy_marshal_constructor(
            $instance as _,
            $request_name,
            $interface as _,
            std::ptr::null_mut::<std::ffi::c_void>(),
            $($arg,)*
        );

        id as *mut _
    }};
}

#[macro_export]
macro_rules! wl_request {
    ($instance:expr, $request_name:expr) => {
        wl_request!($instance, $request_name, ())
    };

    ($instance:expr, $request_name:expr, $($arg:expr),*) => {{
        wl_proxy_marshal(
            $instance as _,
            $request_name,
            $($arg,)*
        )
    }};
}

unsafe fn wl_add_listener<T, T1>(
    wl_proxy: *const T,
    listener: *const T1,
    data: *mut std::ffi::c_void,
) -> i32 {
    wl_proxy_add_listener(wl_proxy as _, listener as _, data as _)
}

unsafe extern "C" fn pointer_handle_enter(
    data: *mut ::std::os::raw::c_void,
    wl_pointer: *mut wl_pointer,
    serial: u32,
    surface: *mut wl_surface,
    surface_x: wl_fixed_t,
    surface_y: wl_fixed_t,
) {
    GLOBALS.focused_window = surface;
}

unsafe extern "C" fn pointer_handle_leave(
    data: *mut ::std::os::raw::c_void,
    wl_pointer: *mut wl_pointer,
    serial: u32,
    surface: *mut wl_surface,
) {
}

unsafe extern "C" fn pointer_handle_motion(
    data: *mut ::std::os::raw::c_void,
    wl_pointer: *mut wl_pointer,
    time: u32,
    surface_x: wl_fixed_t,
    surface_y: wl_fixed_t,
) {
}

unsafe extern "C" fn pointer_handle_button(
    data: *mut ::std::os::raw::c_void,
    wl_pointer: *mut wl_pointer,
    serial: u32,
    time: u32,
    button: u32,
    state: u32,
) {
    // if we have client side decorations
    if let Some(ref decorations) = GLOBALS.decorations {
        // for whatever reason actual constant is in linux/input-event-codes, not wayland headers
        if button == 0x110 {
            if GLOBALS.focused_window == decorations.top_decoration.surface {
                wl_request!(
                    GLOBALS.xdg_toplevel,
                    xdg_toplevel::r#move,
                    GLOBALS.seat,
                    serial
                );
            }
        }
    }
}

unsafe extern "C" fn pointer_handle_axis(
    data: *mut ::std::os::raw::c_void,
    wl_pointer: *mut wl_pointer,
    time: u32,
    axis: u32,
    value: wl_fixed_t,
) {
}

unsafe extern "C" fn pointer_handle_frame(
    data: *mut ::std::os::raw::c_void,
    wl_pointer: *mut wl_pointer,
) {
}

unsafe extern "C" fn pointer_handle_axis_source(
    data: *mut ::std::os::raw::c_void,
    wl_pointer: *mut wl_pointer,
    axis_source: u32,
) {
}

unsafe extern "C" fn pointer_handle_axis_stop(
    data: *mut ::std::os::raw::c_void,
    wl_pointer: *mut wl_pointer,
    time: u32,
    axis: u32,
) {
}

unsafe extern "C" fn pointer_handle_axis_discrete(
    data: *mut ::std::os::raw::c_void,
    wl_pointer: *mut wl_pointer,
    axis: u32,
    discrete: i32,
) {
}

static mut pointer_listener: wl_pointer_listener = wl_pointer_listener {
    enter: Some(pointer_handle_enter),
    leave: Some(pointer_handle_leave),
    motion: Some(pointer_handle_motion),
    button: Some(pointer_handle_button),
    axis: Some(pointer_handle_axis),
    frame: Some(pointer_handle_frame),
    axis_source: Some(pointer_handle_axis_source),
    axis_stop: Some(pointer_handle_axis_stop),
    axis_discrete: Some(pointer_handle_axis_discrete),
};

unsafe extern "C" fn seat_handle_capabilities(
    data: *mut std::ffi::c_void,
    seat: *mut wl_seat,
    caps: wl_seat_capability,
) {
    if (caps & wl_seat_capability_WL_SEAT_CAPABILITY_POINTER) != 0 && GLOBALS.pointer.is_null() {
        GLOBALS.pointer = wl_request_constructor!(seat, WL_SEAT_GET_POINTER, &wl_pointer_interface);
        wl_add_listener(GLOBALS.pointer, &pointer_listener, std::ptr::null_mut());
    }
}

extern "C" fn seat_handle_name(data: *mut std::ffi::c_void, seat: *mut wl_seat, name: *const i8) {}

static mut seat_listener: wl_seat_listener = wl_seat_listener {
    capabilities: Some(seat_handle_capabilities),
    name: Some(seat_handle_name),
};

unsafe extern "C" fn registry_add_object(
    data: *mut std::ffi::c_void,
    registry: *mut wl_registry,
    name: u32,
    interface: *const i8,
    version: u32,
) {
    println!(
        "{:?}",
        std::ffi::CStr::from_ptr(interface).to_str().unwrap()
    );
    if strcmp(interface, b"wl_compositor\x00" as *const u8 as *const _) == 0 {
        GLOBALS.compositor = wl_registry_bind(registry, name, &wl_compositor_interface, 1) as _;
    } else if strcmp(interface, b"wl_subcompositor\x00" as *const u8 as *const _) == 0 {
        GLOBALS.subcompositor =
            wl_registry_bind(registry, name, &wl_subcompositor_interface, 1) as _;
    } else if strcmp(interface, b"xdg_wm_base\x00" as *const u8 as *const _) == 0 {
        GLOBALS.xdg_wm_base =
            wl_registry_bind(registry, name, &xdg_shell::xdg_wm_base_interface, 1) as _;
        wl_add_listener(
            GLOBALS.xdg_wm_base,
            &xdg_wm_base_listener,
            std::ptr::null_mut(),
        );
    } else if strcmp(
        interface,
        b"zxdg_decoration_manager_v1\00" as *const u8 as *const _,
    ) == 0
    {
        GLOBALS.decoration_manager =
            wl_registry_bind(registry, name, &zxdg_decoration_manager_v1_interface, 1) as _;
    } else if strcmp(interface, b"wp_viewporter\x00" as *const u8 as *const _) == 0 {
        GLOBALS.viewporter = wl_registry_bind(registry, name, &wp_viewporter_interface, 1) as _;
    } else if strcmp(interface, b"wl_shm\x00" as *const u8 as *const _) == 0 {
        GLOBALS.shm = wl_registry_bind(registry, name, &wl_shm_interface, 1) as _;
    } else if strcmp(interface, b"wl_seat\x00" as *const u8 as *const _) == 0 {
        let seat_version = 4.min(version);
        GLOBALS.seat = wl_registry_bind(registry, name, &wl_seat_interface, seat_version) as _;

        wl_add_listener(GLOBALS.seat, &seat_listener, std::ptr::null_mut());
    }
}

unsafe extern "C" fn registry_remove_object(
    data: *mut std::ffi::c_void,
    registry: *mut wl_registry,
    name: u32,
) {
}

unsafe extern "C" fn xdg_surface_handle_configure(
    data: *mut std::ffi::c_void,
    xdg_surface: *mut xdg_surface,
    serial: u32,
) {
    wl_request!(xdg_surface, xdg_surface::ack_configure, serial);
    wl_request!(GLOBALS.surface, WL_SURFACE_COMMIT);
}

unsafe extern "C" fn xdg_toplevel_handle_close(
    data: *mut std::ffi::c_void,
    xdg_toplevel: *mut xdg_toplevel,
) {
    GLOBALS.closed = true;
}

#[no_mangle]
extern "C" {
    pub fn strcmp(_: *const i8, _: *const i8) -> i32;
}

unsafe fn wl_registry_bind(
    wl_registry: *const wl_registry,
    name: u32,
    interface: *const wl_interface,
    version: u32,
) -> *mut std::ffi::c_void {
    let id: *mut wl_proxy;

    id = wl_proxy_marshal_constructor_versioned(
        wl_registry as _,
        WL_REGISTRY_BIND,
        interface as _,
        version,
        name,
        (*interface).name,
        version,
        std::ptr::null_mut::<std::ffi::c_void>(),
    );

    id as *mut _
}

unsafe fn create_decoration(
    parent: *mut wl_surface,
    buffer: *mut wl_buffer,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
) -> Decoration {
    let surface = wl_request_constructor!(
        GLOBALS.compositor,
        WL_COMPOSITOR_CREATE_SURFACE,
        &wl_surface_interface,
    );

    let subsurface = wl_request_constructor!(
        GLOBALS.subcompositor,
        WL_SUBCOMPOSITOR_GET_SUBSURFACE,
        &wl_subsurface_interface,
        surface,
        parent
    );

    wl_request!(subsurface, WL_SUBSURFACE_SET_POSITION, x, y);

    let viewport = wl_request_constructor!(
        GLOBALS.viewporter,
        wp_viewporter::get_viewport,
        &wp_viewport_interface,
        surface
    );

    wl_request!(viewport, wp_viewport::set_destination, w, h);
    wl_request!(surface, WL_SURFACE_ATTACH, buffer, 0, 0);
    wl_request!(surface, WL_SURFACE_COMMIT);

    Decoration { surface }
}

unsafe extern "C" fn handle_wm_base_ping(
    _: *mut std::ffi::c_void,
    xdg_wm_base: *mut xdg_wm_base,
    serial: u32,
) {
    wl_proxy_marshal(xdg_wm_base as _, xdg_wm_base::pong, serial);
}

static mut xdg_wm_base_listener: xdg_shell::xdg_wm_base_listener =
    xdg_shell::xdg_wm_base_listener {
        ping: Some(handle_wm_base_ping),
    };

pub fn init_window() {
    unsafe {
        let display = wl_display_connect(std::ptr::null_mut());
        if display.is_null() {
            panic!("Failed to connect to Wayland display.");
        }
        let registry =
            wl_request_constructor!(display, WL_DISPLAY_GET_REGISTRY, &wl_registry_interface);

        let mut registry_listener = wl_registry_listener {
            global: Some(registry_add_object),
            global_remove: Some(registry_remove_object),
        };
        wl_add_listener(registry, &registry_listener, std::ptr::null_mut());
        wl_display_roundtrip(display);

        if GLOBALS.compositor.is_null() {
            panic!("No compositor!");
        }
        if GLOBALS.xdg_wm_base.is_null() {
            panic!("No xdg_wm_base!");
        }
        if GLOBALS.subcompositor.is_null() {
            panic!("No subcompositor!");
        }
        if GLOBALS.seat.is_null() {
            panic!("No seat!");
        }

        if GLOBALS.decoration_manager.is_null() {
            println!("Decoration manager not found, window decarations disabled");
        }

        let egl_display = eglGetDisplay(display as _);
        eglInitialize(egl_display, std::ptr::null_mut(), std::ptr::null_mut());

        egl::eglBindAPI(egl::EGL_OPENGL_API);
        let attributes = [
            egl::EGL_RED_SIZE,
            8,
            egl::EGL_GREEN_SIZE,
            8,
            egl::EGL_BLUE_SIZE,
            8,
            egl::EGL_DEPTH_SIZE,
            8,
            egl::EGL_NONE,
        ];
        let mut config: egl::EGLConfig = std::mem::zeroed();
        let mut num_config = 0;

        egl::eglChooseConfig(
            egl_display,
            attributes.as_ptr() as _,
            &mut config,
            1,
            &mut num_config,
        );
        let egl_context = egl::eglCreateContext(
            egl_display,
            config,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        );

        GLOBALS.surface = wl_request_constructor!(
            GLOBALS.compositor,
            WL_COMPOSITOR_CREATE_SURFACE,
            &wl_surface_interface
        );
        assert!(GLOBALS.surface.is_null() == false);

        let xdg_surface: *mut xdg_surface = wl_request_constructor!(
            GLOBALS.xdg_wm_base,
            xdg_wm_base::get_xdg_surface,
            &xdg_shell::xdg_surface_interface,
            GLOBALS.surface
        );
        assert!(xdg_surface.is_null() == false);

        GLOBALS.xdg_toplevel = wl_request_constructor!(
            xdg_surface,
            xdg_surface::get_toplevel,
            &xdg_shell::xdg_toplevel_interface
        );
        assert!(GLOBALS.xdg_toplevel.is_null() == false);

        let mut xdg_surface_listener = xdg_shell::xdg_surface_listener {
            configure: Some(xdg_surface_handle_configure),
        };

        wl_add_listener(
            xdg_surface as _,
            &xdg_surface_listener,
            std::ptr::null_mut(),
        );

        extern "C" fn noop(
            _: *mut std::ffi::c_void,
            _: *mut crate::wayland::xdg_toplevel,
            _: i32,
            _: i32,
            _: *mut crate::wayland::wl_array,
        ) -> () {
        }

        let mut xdg_toplevel_listener = xdg_shell::xdg_toplevel_listener {
            configure: Some(noop),
            close: Some(xdg_toplevel_handle_close),
        };

        wl_add_listener(
            GLOBALS.xdg_toplevel as _,
            &xdg_toplevel_listener,
            std::ptr::null_mut(),
        );

        wl_request!(GLOBALS.surface, WL_SURFACE_COMMIT);
        wl_display_roundtrip(display);

        let egl_window = wayland_egl::wl_egl_window_create(GLOBALS.surface as _, 512, 512);
        let egl_surface =
            egl::eglCreateWindowSurface(egl_display, config, egl_window as _, std::ptr::null_mut());
        egl::eglMakeCurrent(egl_display, egl_surface, egl_surface, egl_context);

        if GLOBALS.decoration_manager.is_null() == false {
            let server_decoration: *mut zxdg_toplevel_decoration_v1 = wl_request_constructor!(
                GLOBALS.decoration_manager,
                zxdg_decoration_manager_v1::get_toplevel_decoration,
                &zxdg_toplevel_decoration_v1_interface,
                GLOBALS.xdg_toplevel
            );
            assert!(server_decoration.is_null() == false);

            wl_request!(
                server_decoration,
                zxdg_toplevel_decoration_v1::set_mode,
                ZXDG_TOPLEVEL_DECORATION_V1_MODE_SERVER_SIDE
            );
        } else {
            let buffer = shm::create_shm_buffer(GLOBALS.shm, 1, 1, &[200, 200, 200, 255]);

            let decorations = Decorations {
                buffer,
                top_decoration: create_decoration(GLOBALS.surface, buffer, -2, -15, 512 + 4, 15),
                left_decoration: create_decoration(GLOBALS.surface, buffer, -2, -2, 2, 512 + 2),
                bottom_decoration: create_decoration(GLOBALS.surface, buffer, 512, -2, 2, 512 + 2),
                right_decoration: create_decoration(GLOBALS.surface, buffer, -2, 512, 512 + 4, 2),
            };

            GLOBALS.decorations = Some(decorations);
        }

        while GLOBALS.closed == false {
            wl_display_dispatch_pending(display);

            crate::_sapp_frame();

            egl::eglSwapBuffers(egl_display, egl_surface);
        }
    }
}
