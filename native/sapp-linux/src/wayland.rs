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

static mut COMPOSITOR: *mut wl_compositor = std::ptr::null_mut();
static mut SUBCOMPOSITOR: *mut wl_subcompositor = std::ptr::null_mut();
static mut XDG_TOPLEVEL: *mut xdg_toplevel = std::ptr::null_mut();
static mut XDG_WM_BASE: *mut xdg_wm_base = std::ptr::null_mut();
static mut SURFACE: *mut wl_surface = std::ptr::null_mut();
static mut ZXDG_DECORATION_MANAGER: *mut zxdg_decoration_manager_v1 = std::ptr::null_mut();
static mut VIEWPORTER: *mut wp_viewporter = std::ptr::null_mut();
static mut SHM: *mut wl_shm = std::ptr::null_mut();
static mut SEAT: *mut wl_seat = std::ptr::null_mut();

static mut POINTER: *mut wl_pointer = std::ptr::null_mut();
static mut FOCUSED_WINDOW: *mut wl_surface = std::ptr::null_mut();

static mut TOP_DECORATION: *mut wl_surface = std::ptr::null_mut();

static mut CLOSED: bool = false;

unsafe extern "C" fn pointer_handle_enter(
    data: *mut ::std::os::raw::c_void,
    wl_pointer: *mut wl_pointer,
    serial: u32,
    surface: *mut wl_surface,
    surface_x: wl_fixed_t,
    surface_y: wl_fixed_t,
) {
    FOCUSED_WINDOW = surface;
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
    if FOCUSED_WINDOW.is_null() {
        return;
    }

    // for whatever reason actual constant is in linux/input-event-codes, not wayland headers
    if button == 0x110 {
        if FOCUSED_WINDOW == TOP_DECORATION {
            xdg_toplevel_move(XDG_TOPLEVEL, SEAT, serial);
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
    if (caps & wl_seat_capability_WL_SEAT_CAPABILITY_POINTER) != 0 && POINTER.is_null() {
        POINTER = wl_seat_get_pointer(seat);
        wl_pointer_add_listener(POINTER, &pointer_listener, std::ptr::null_mut());
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
    // println!(
    //     "{:?}",
    //     std::ffi::CStr::from_ptr(interface).to_str().unwrap()
    // );
    if strcmp(interface, b"wl_compositor\x00" as *const u8 as *const _) == 0 {
        COMPOSITOR = wl_registry_bind(registry, name, &wl_compositor_interface, 1) as _;
    } else if strcmp(interface, b"wl_subcompositor\x00" as *const u8 as *const _) == 0 {
        SUBCOMPOSITOR = wl_registry_bind(registry, name, &wl_subcompositor_interface, 1) as _;
    } else if strcmp(interface, b"xdg_wm_base\x00" as *const u8 as *const _) == 0 {
        XDG_WM_BASE = wl_registry_bind(registry, name, &xdg_shell::xdg_wm_base_interface, 1) as _;
        xdg_wm_base_add_listener(XDG_WM_BASE, &xdg_wm_base_listener, std::ptr::null_mut());
    } else if strcmp(
        interface,
        b"zxdg_decoration_manager_v1\00" as *const u8 as *const _,
    ) == 0
    {
        ZXDG_DECORATION_MANAGER =
            wl_registry_bind(registry, name, &zxdg_decoration_manager_v1_interface, 1) as _;
    } else if strcmp(interface, b"wp_viewporter\x00" as *const u8 as *const _) == 0 {
        VIEWPORTER = wl_registry_bind(registry, name, &wp_viewporter_interface, 1) as _;
    } else if strcmp(interface, b"wl_shm\x00" as *const u8 as *const _) == 0 {
        SHM = wl_registry_bind(registry, name, &wl_shm_interface, 1) as _;
    } else if strcmp(interface, b"wl_seat\x00" as *const u8 as *const _) == 0 {
        let seat_version = 4.min(version);
        SEAT = wl_registry_bind(registry, name, &wl_seat_interface, seat_version) as _;

        wl_seat_add_listener(SEAT, &seat_listener, std::ptr::null_mut());
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
    xdg_surface_ack_configure(xdg_surface, serial);
    wl_surface_commit(SURFACE);
}

unsafe extern "C" fn xdg_toplevel_handle_close(
    data: *mut std::ffi::c_void,
    xdg_toplevel: *mut xdg_toplevel,
) {
    CLOSED = true;
}

#[no_mangle]
extern "C" {
    pub fn strcmp(_: *const i8, _: *const i8) -> i32;
}

unsafe fn wl_display_get_registry(display: *mut wl_display) -> *mut wl_registry {
    let registry: *mut wl_proxy;

    registry = wl_proxy_marshal_constructor(
        display as *mut _,
        WL_DISPLAY_GET_REGISTRY,
        &wl_registry_interface,
        std::ptr::null_mut::<std::ffi::c_void>(),
    );
    registry as *mut _ as *mut wl_registry
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

unsafe fn wl_surface_attach(wl_surface: *mut wl_surface, buffer: *mut wl_buffer, x: i32, y: i32) {
    wl_proxy_marshal(wl_surface as _, WL_SURFACE_ATTACH, buffer, x, y);
}

unsafe fn wl_surface_commit(wl_surface: *const wl_surface) {
    wl_proxy_marshal(wl_surface as _, WL_SURFACE_COMMIT)
}

unsafe fn wl_registry_add_listener(
    wl_registry: *const wl_registry,
    listener: *const wl_registry_listener,
    data: *mut std::ffi::c_void,
) -> i32 {
    wl_proxy_add_listener(wl_registry as _, listener as _, data as _)
}

unsafe fn wl_pointer_add_listener(
    wl_pointer: *const wl_pointer,
    listener: *const wl_pointer_listener,
    data: *mut std::ffi::c_void,
) -> i32 {
    wl_proxy_add_listener(wl_pointer as _, listener as _, data as _)
}

unsafe fn wl_seat_add_listener(
    wl_seat: *const wl_seat,
    listener: *const wl_seat_listener,
    data: *mut std::ffi::c_void,
) -> i32 {
    wl_proxy_add_listener(wl_seat as _, listener as _, data as _)
}

unsafe fn xdg_wm_base_add_listener(
    xdg_wm_base: *const xdg_wm_base,
    listener: *const xdg_shell::xdg_wm_base_listener,
    data: *mut std::ffi::c_void,
) -> i32 {
    wl_proxy_add_listener(xdg_wm_base as _, listener as _, data as _)
}

unsafe fn wl_seat_get_pointer(wl_seat: *const wl_seat) -> *mut wl_pointer {
    let id: *mut wl_proxy;

    id = wl_proxy_marshal_constructor(
        wl_seat as _,
        WL_SEAT_GET_POINTER,
        &wl_pointer_interface as _,
        std::ptr::null_mut::<std::ffi::c_void>(),
    );

    id as *mut _
}

unsafe fn wl_compositor_create_surface(wl_compositor: *mut wl_compositor) -> *mut wl_surface {
    let id: *mut wl_proxy;

    id = wl_proxy_marshal_constructor(
        wl_compositor as _,
        WL_COMPOSITOR_CREATE_SURFACE,
        &wl_surface_interface as _,
        std::ptr::null_mut::<std::ffi::c_void>(),
    );

    id as *mut _
}

unsafe fn wl_subcompositor_get_subsurface(
    wl_subcompositor: *mut wl_subcompositor,
    surface: *mut wl_surface,
    parent: *mut wl_surface,
) -> *mut wl_subsurface {
    let id: *mut wl_proxy;

    id = wl_proxy_marshal_constructor(
        wl_subcompositor as _,
        WL_SUBCOMPOSITOR_GET_SUBSURFACE,
        &wl_subsurface_interface as _,
        std::ptr::null_mut::<std::ffi::c_void>(),
        surface,
        parent,
    );

    id as *mut _
}

unsafe fn wl_subsurface_set_position(wl_subsurface: *mut wl_subsurface, x: i32, y: i32) {
    wl_proxy_marshal(wl_subsurface as _, WL_SUBSURFACE_SET_POSITION, x, y);
}

unsafe fn xdg_wm_base_get_xdg_surface(
    xdg_wm_base: *mut xdg_wm_base,
    surface: *mut wl_surface,
) -> *mut xdg_surface {
    let id: *mut wl_proxy;

    id = wl_proxy_marshal_constructor(
        xdg_wm_base as _,
        xdg_wm_base::get_xdg_surface,
        &xdg_shell::xdg_surface_interface as _,
        std::ptr::null_mut::<std::ffi::c_void>(),
        surface,
    );

    id as *mut _
}

unsafe fn xdg_surface_get_toplevel(xdg_surface: *mut xdg_surface) -> *mut xdg_toplevel {
    let id: *mut wl_proxy;

    id = wl_proxy_marshal_constructor(
        xdg_surface as _,
        xdg_surface::get_toplevel,
        &xdg_shell::xdg_toplevel_interface as _,
        std::ptr::null_mut::<std::ffi::c_void>(),
    );

    id as *mut _
}

unsafe fn xdg_surface_ack_configure(xdg_surface: *mut xdg_surface, serial: u32) {
    wl_proxy_marshal(xdg_surface as _, xdg_surface::ack_configure, serial);
}

unsafe fn xdg_toplevel_move(xdg_toplevel: *mut xdg_toplevel, seat: *mut wl_seat, serial: u32) {
    wl_proxy_marshal(xdg_toplevel as _, xdg_toplevel::r#move as _, seat, serial);
}

unsafe fn zxdg_decoration_manager_v1_get_toplevel_decoration(
    zxdg_decoration_manager_v1: *mut zxdg_decoration_manager_v1,
    xdg_toplevel: *mut xdg_toplevel,
) -> *mut zxdg_toplevel_decoration_v1 {
    let id;

    id = wl_proxy_marshal_constructor(
        zxdg_decoration_manager_v1 as _,
        zxdg_decoration_manager_v1::get_toplevel_decoration,
        &zxdg_toplevel_decoration_v1_interface as _,
        std::ptr::null_mut::<std::ffi::c_void>(),
        XDG_TOPLEVEL,
    );

    id as *mut _
}

unsafe fn zxdg_toplevel_decoration_v1_set_mode(
    zxdg_toplevel_decoration_v1: *mut zxdg_toplevel_decoration_v1,
    mode: u32,
) {
    wl_proxy_marshal(
        zxdg_toplevel_decoration_v1 as _,
        zxdg_toplevel_decoration_v1::set_mode,
        mode,
    );
}

unsafe fn wp_viewporter_get_viewport(
    wp_viewporter: *mut wp_viewporter,
    surface: *mut wl_surface,
) -> *mut wp_viewport {
    let id;

    id = wl_proxy_marshal_constructor(
        wp_viewporter as _,
        wp_viewporter::get_viewport,
        &wp_viewport_interface as _,
        std::ptr::null_mut::<std::ffi::c_void>(),
        surface,
    );

    id as _
}

unsafe fn wp_viewport_set_destination(wp_viewport: *mut wp_viewport, width: i32, height: i32) {
    wl_proxy_marshal(
        wp_viewport as _,
        wp_viewport::set_destination,
        width,
        height,
    );
}

unsafe fn create_decoration(
    parent: *mut wl_surface,
    buffer: *mut wl_buffer,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
) -> *mut wl_surface {
    let surface = wl_compositor_create_surface(COMPOSITOR);
    let subsurface = wl_subcompositor_get_subsurface(SUBCOMPOSITOR, surface, parent);
    wl_subsurface_set_position(subsurface, x, y);
    let viewport = wp_viewporter_get_viewport(VIEWPORTER, surface);
    wp_viewport_set_destination(viewport, w, h);
    wl_surface_attach(surface, buffer, 0, 0);
    wl_surface_commit(surface);

    surface
}

unsafe extern "C" fn handle_wm_base_ping(
    _: *mut std::ffi::c_void,
    xdg_wm_base: *mut xdg_wm_base,
    serial: u32,
) {
    wl_proxy_marshal(xdg_wm_base as _, xdg_wm_base::pong, serial);
}

static mut xdg_wm_base_listener: xdg_shell::xdg_wm_base_listener = xdg_shell::xdg_wm_base_listener {
    ping: Some(handle_wm_base_ping),
};

pub fn init_window() {
    unsafe {
        let display = wl_display_connect(std::ptr::null_mut());
        if display.is_null() {
            panic!("Failed to connect to Wayland display.");
        }
        let registry = wl_display_get_registry(display);

        let mut registry_listener = wl_registry_listener {
            global: Some(registry_add_object),
            global_remove: Some(registry_remove_object),
        };
        wl_registry_add_listener(registry, &registry_listener, std::ptr::null_mut());
        wl_display_roundtrip(display);

        if COMPOSITOR.is_null() {
            panic!("No compositor!");
        }
        if XDG_WM_BASE.is_null() {
            panic!("No xdg_wm_base!");
        }
        if SUBCOMPOSITOR.is_null() {
            panic!("No subcompositor!");
        }
        if SEAT.is_null() {
            panic!("No seat!");
        }

        if ZXDG_DECORATION_MANAGER.is_null() {
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

        SURFACE = wl_compositor_create_surface(COMPOSITOR);
        assert!(SURFACE.is_null() == false);
        let xdg_surface = xdg_wm_base_get_xdg_surface(XDG_WM_BASE, SURFACE);
        assert!(xdg_surface.is_null() == false);
        XDG_TOPLEVEL = xdg_surface_get_toplevel(xdg_surface);
        assert!(XDG_TOPLEVEL.is_null() == false);

        let mut xdg_surface_listener = xdg_shell::xdg_surface_listener {
            configure: Some(xdg_surface_handle_configure),
        };

        wl_proxy_add_listener(
            xdg_surface as _,
            std::mem::transmute(&mut xdg_surface_listener),
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

        wl_proxy_add_listener(
            XDG_TOPLEVEL as _,
            std::mem::transmute(&mut xdg_toplevel_listener),
            std::ptr::null_mut(),
        );

        wl_surface_commit(SURFACE);
        wl_display_roundtrip(display);

        let egl_window = wayland_egl::wl_egl_window_create(SURFACE as _, 512, 512);
        let egl_surface =
            egl::eglCreateWindowSurface(egl_display, config, egl_window as _, std::ptr::null_mut());
        egl::eglMakeCurrent(egl_display, egl_surface, egl_surface, egl_context);

        if ZXDG_DECORATION_MANAGER.is_null() == false {
            let server_decoration = zxdg_decoration_manager_v1_get_toplevel_decoration(
                ZXDG_DECORATION_MANAGER,
                XDG_TOPLEVEL,
            );
            assert!(server_decoration.is_null() == false);

            zxdg_toplevel_decoration_v1_set_mode(
                server_decoration,
                ZXDG_TOPLEVEL_DECORATION_V1_MODE_SERVER_SIDE,
            );
        } else {
            let buffer = shm::create_shm_buffer(1, 1, &[200, 200, 200, 255]);

            // idk doest it takes ownership or not yet
            std::mem::forget(buffer);

            TOP_DECORATION = create_decoration(SURFACE, buffer, -2, -15, 512 + 4, 15);
            create_decoration(SURFACE, buffer, -2, -2, 2, 512 + 2);
            create_decoration(SURFACE, buffer, 512, -2, 2, 512 + 2);
            create_decoration(SURFACE, buffer, -2, 512, 512 + 4, 2);
        }

        while CLOSED == false {
            wl_display_dispatch_pending(display);

            crate::_sapp_frame();

            egl::eglSwapBuffers(egl_display, egl_surface);
        }
    }
}
