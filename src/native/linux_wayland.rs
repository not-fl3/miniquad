#![allow(dead_code)]

mod libwayland_client;
mod libwayland_egl;
mod libxkbcommon;

mod decorations;
mod extensions;
mod keycodes;
mod shm;

use libwayland_client::*;
use libwayland_egl::*;
use libxkbcommon::*;

use crate::{
    event::{EventHandler, KeyCode, KeyMods, MouseButton},
    native::{egl, NativeDisplayData},
};

use std::collections::HashSet;

fn wl_fixed_to_double(f: i32) -> f32 {
    (f as f32) / 256.0
}

/// A thing to pass around within *void pointer of wayland's event handler
struct WaylandPayload {
    client: LibWaylandClient,
    // this is libwayland-egl.so, a library with ~4 functions
    // not the libEGL.so(which will be loaded, but not here)
    egl: LibWaylandEgl,
    xkb: LibXkbCommon,
    compositor: *mut wl_compositor,
    subcompositor: *mut wl_subcompositor,
    xdg_toplevel: *mut extensions::xdg_shell::xdg_toplevel,
    xdg_wm_base: *mut extensions::xdg_shell::xdg_wm_base,
    surface: *mut wl_surface,
    decoration_manager: *mut extensions::xdg_decoration::zxdg_decoration_manager_v1,
    viewporter: *mut extensions::viewporter::wp_viewporter,
    shm: *mut wl_shm,
    seat: *mut wl_seat,
    xkb_context: *mut xkb_context,
    keymap: *mut xkb_keymap,
    xkb_state: *mut xkb_state,

    egl_window: *mut wl_egl_window,
    pointer: *mut wl_pointer,
    keyboard: *mut wl_keyboard,
    focused_window: *mut wl_surface,
    //xkb_state: xkb::XkbState,
    decorations: Option<decorations::Decorations>,

    event_handler: Option<Box<dyn EventHandler>>,
    closed: bool,
}

#[macro_export]
macro_rules! wl_request_constructor {
    ($libwayland:expr, $instance:expr, $request_name:expr, $interface:expr) => {
        wl_request_constructor!($libwayland, $instance, $request_name, $interface, ())
    };

    ($libwayland:expr, $instance:expr, $request_name:expr, $interface:expr, $($arg:expr),*) => {{
        let id: *mut wl_proxy;

        id = ($libwayland.wl_proxy_marshal_constructor)(
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
    ($libwayland:expr, $instance:expr, $request_name:expr) => {
        wl_request!($libwayland, $instance, $request_name, ())
    };

    ($libwayland:expr, $instance:expr, $request_name:expr, $($arg:expr),*) => {{
        ($libwayland.wl_proxy_marshal)(
            $instance as _,
            $request_name,
            $($arg,)*
        )
    }};
}

static mut SEAT_LISTENER: wl_seat_listener = wl_seat_listener {
    capabilities: Some(seat_handle_capabilities),
    name: Some(seat_handle_name),
};

unsafe extern "C" fn seat_handle_capabilities(
    data: *mut std::ffi::c_void,
    seat: *mut wl_seat,
    caps: wl_seat_capability,
) {
    let display: &mut WaylandPayload = &mut *(data as *mut _);

    if caps & wl_seat_capability_WL_SEAT_CAPABILITY_POINTER != 0 {
        // struct wl_pointer *pointer = wl_seat_get_pointer (seat);
        let id: *mut wl_proxy = wl_request_constructor!(
            display.client,
            seat,
            WL_SEAT_GET_POINTER,
            display.client.wl_pointer_interface
        );
        assert!(!id.is_null());
        // wl_pointer_add_listener (pointer, &pointer_listener, NULL);
        (display.client.wl_proxy_add_listener)(id, &POINTER_LISTENER as *const _ as _, data);
    }

    if caps & wl_seat_capability_WL_SEAT_CAPABILITY_KEYBOARD != 0 {
        // struct wl_keyboard *keyboard = wl_seat_get_keyboard(seat);
        let id: *mut wl_proxy = wl_request_constructor!(
            display.client,
            seat,
            WL_SEAT_GET_KEYBOARD,
            display.client.wl_keyboard_interface
        );
        assert!(!id.is_null());
        // wl_keyboard_add_listener(keyboard, &keyboard_listener, NULL);
        (display.client.wl_proxy_add_listener)(id, &KEYBOARD_LISTENER as *const _ as _, data);
    }
}

enum WaylandEvent {
    KeyboardKey(KeyCode, bool),
    PointerMotion(f32, f32),
    PointerButton(MouseButton, bool),
    PointerAxis(f32, f32),
}

static mut EVENTS: Vec<WaylandEvent> = Vec::new();

static mut KEYBOARD_LISTENER: wl_keyboard_listener = wl_keyboard_listener {
    keymap: Some(keyboard_handle_keymap),
    enter: Some(keyboard_handle_enter),
    leave: Some(keyboard_handle_leave),
    key: Some(keyboard_handle_key),
    modifiers: Some(keyboard_handle_modifiers),
    repeat_info: Some(keyboard_handle_repeat_info),
};

unsafe extern "C" fn keyboard_handle_keymap(
    data: *mut ::std::os::raw::c_void,
    _wl_keyboard: *mut wl_keyboard,
    _format: u32,
    fd: i32,
    size: u32,
) {
    let display: &mut WaylandPayload = &mut *(data as *mut _);
    let map_shm = libc::mmap(
        std::ptr::null_mut::<std::ffi::c_void>(),
        size as usize,
        libc::PROT_READ,
        libc::MAP_PRIVATE,
        fd,
        0,
    );
    assert!(map_shm != libc::MAP_FAILED);
    (display.xkb.xkb_keymap_unref)(display.keymap);
    display.keymap = (display.xkb.xkb_keymap_new_from_string)(
        display.xkb_context,
        map_shm as *mut libc::FILE,
        1,
        0,
    );
    libc::munmap(map_shm, size as usize);
    libc::close(fd);
    (display.xkb.xkb_state_unref)(display.xkb_state);
    display.xkb_state = (display.xkb.xkb_state_new)(display.keymap);
}
unsafe extern "C" fn keyboard_handle_enter(
    _data: *mut ::std::os::raw::c_void,
    _wl_keyboard: *mut wl_keyboard,
    _serial: u32,
    _surface: *mut wl_surface,
    _keys: *mut wl_array,
) {
}
unsafe extern "C" fn keyboard_handle_leave(
    _data: *mut ::std::os::raw::c_void,
    _wl_keyboard: *mut wl_keyboard,
    _serial: u32,
    _surface: *mut wl_surface,
) {
}
unsafe extern "C" fn keyboard_handle_key(
    data: *mut ::std::os::raw::c_void,
    _wl_keyboard: *mut wl_keyboard,
    _serial: u32,
    _time: u32,
    key: u32,
    state: u32,
) {
    let display: &mut WaylandPayload = &mut *(data as *mut _);
    // https://wayland-book.com/seat/keyboard.html
    // To translate this to an XKB scancode, you must add 8 to the evdev scancode.
    let keysym = (display.xkb.xkb_state_key_get_one_sym)(display.xkb_state, key + 8);
    let keycode = keycodes::translate(keysym);
    EVENTS.push(WaylandEvent::KeyboardKey(keycode, state == 1));
}
unsafe extern "C" fn keyboard_handle_modifiers(
    data: *mut ::std::os::raw::c_void,
    _wl_keyboard: *mut wl_keyboard,
    _serial: u32,
    mods_depressed: u32,
    mods_latched: u32,
    mods_locked: u32,
    group: u32,
) {
    let display: &mut WaylandPayload = &mut *(data as *mut _);
    (display.xkb.xkb_state_update_mask)(
        display.xkb_state,
        mods_depressed,
        mods_latched,
        mods_locked,
        0,
        0,
        group,
    );
}
unsafe extern "C" fn keyboard_handle_repeat_info(
    _data: *mut ::std::os::raw::c_void,
    _wl_keyboard: *mut wl_keyboard,
    _rate: i32,
    _delay: i32,
) {
}

static mut POINTER_LISTENER: wl_pointer_listener = wl_pointer_listener {
    enter: Some(pointer_handle_enter),
    leave: Some(pointer_handle_leave),
    motion: Some(pointer_handle_motion),
    button: Some(pointer_handle_button),
    axis: Some(pointer_handle_axis),
    frame: Some(pointer_handle_frame),
    axis_source: Some(pointer_handle_axis_source),
    axis_stop: Some(pointer_handle_axis_stop),
    axis_discrete: Some(pointer_handle_axis_discrete),
    axis_value120: Some(pointer_handle_axis_value120),
    axis_relative_direction: Some(pointer_handle_axis_relative_direction),
};

unsafe extern "C" fn pointer_handle_enter(
    _data: *mut ::std::os::raw::c_void,
    _wl_pointer: *mut wl_pointer,
    _serial: u32,
    _surface: *mut wl_surface,
    _surface_x: i32,
    _surface_y: i32,
) {
}
unsafe extern "C" fn pointer_handle_leave(
    _data: *mut ::std::os::raw::c_void,
    _wl_pointer: *mut wl_pointer,
    _serial: u32,
    _surface: *mut wl_surface,
) {
}
unsafe extern "C" fn pointer_handle_motion(
    _data: *mut ::std::os::raw::c_void,
    _wl_pointer: *mut wl_pointer,
    _time: u32,
    surface_x: i32,
    surface_y: i32,
) {
    // From wl_fixed_to_double(), it simply divides by 256
    let (x, y) = (wl_fixed_to_double(surface_x), wl_fixed_to_double(surface_y));
    EVENTS.push(WaylandEvent::PointerMotion(x, y));
}
unsafe extern "C" fn pointer_handle_button(
    _data: *mut ::std::os::raw::c_void,
    _wl_pointer: *mut wl_pointer,
    _serial: u32,
    _time: u32,
    button: u32,
    state: u32,
) {
    // The code is defined in the kernel's linux/input-event-codes.h header file, e.g. BTN_LEFT
    let button = match button {
        272 => MouseButton::Left,
        273 => MouseButton::Right,
        274 => MouseButton::Middle,
        _ => MouseButton::Unknown,
    };
    EVENTS.push(WaylandEvent::PointerButton(button, state == 1));
}
unsafe extern "C" fn pointer_handle_axis(
    _data: *mut ::std::os::raw::c_void,
    _wl_pointer: *mut wl_pointer,
    _time: u32,
    axis: u32,
    value: i32,
) {
    let mut value = wl_fixed_to_double(value);
    // Normalize the value to {-1, 0, 1}
    value /= value.abs();

    // https://wayland-book.com/seat/pointer.html
    if axis == 0 {
        // Vertical scroll
        // Wayland defines the direction differently to miniquad so lets flip it
        value = -value;
        EVENTS.push(WaylandEvent::PointerAxis(0.0, value));
    } else if axis == 1 {
        // Horizontal scroll
        EVENTS.push(WaylandEvent::PointerAxis(value, 0.0));
    }
}
unsafe extern "C" fn pointer_handle_frame(
    _data: *mut ::std::os::raw::c_void,
    _wl_pointer: *mut wl_pointer,
) {
}
unsafe extern "C" fn pointer_handle_axis_source(
    _data: *mut ::std::os::raw::c_void,
    _wl_pointer: *mut wl_pointer,
    _axis_source: u32,
) {
}
unsafe extern "C" fn pointer_handle_axis_stop(
    _data: *mut ::std::os::raw::c_void,
    _wl_pointer: *mut wl_pointer,
    _time: u32,
    _axis: u32,
) {
}
unsafe extern "C" fn pointer_handle_axis_discrete(
    _data: *mut ::std::os::raw::c_void,
    _wl_pointer: *mut wl_pointer,
    _axis: u32,
    _discrete: i32,
) {
}
unsafe extern "C" fn pointer_handle_axis_value120(
    _data: *mut ::std::os::raw::c_void,
    _wl_pointer: *mut wl_pointer,
    _axis: u32,
    _value120: i32,
) {
}
unsafe extern "C" fn pointer_handle_axis_relative_direction(
    _data: *mut ::std::os::raw::c_void,
    _wl_pointer: *mut wl_pointer,
    _axis: u32,
    _direction: u32,
) {
}

extern "C" fn seat_handle_name(
    _data: *mut std::ffi::c_void,
    _seat: *mut wl_seat,
    _name: *const ::std::os::raw::c_char,
) {
}

unsafe extern "C" fn registry_add_object(
    data: *mut std::ffi::c_void,
    registry: *mut wl_registry,
    name: u32,
    interface: *const ::std::os::raw::c_char,
    version: u32,
) {
    let display: &mut WaylandPayload = &mut *(data as *mut _);

    let interface = std::ffi::CStr::from_ptr(interface).to_str().unwrap();
    match interface {
        "wl_compositor" => {
            display.compositor = display.client.wl_registry_bind(
                registry,
                name,
                display.client.wl_compositor_interface,
                1,
            ) as _;
        }
        "wl_subcompositor" => {
            display.subcompositor = display.client.wl_registry_bind(
                registry,
                name,
                display.client.wl_subcompositor_interface,
                1,
            ) as _;
        }
        "xdg_wm_base" => {
            display.xdg_wm_base = display.client.wl_registry_bind(
                registry,
                name,
                &extensions::xdg_shell::xdg_wm_base_interface,
                1,
            ) as _;
        }
        "zxdg_decoration_manager" => {
            display.decoration_manager = display.client.wl_registry_bind(
                registry,
                name,
                &extensions::xdg_decoration::zxdg_decoration_manager_v1_interface,
                1,
            ) as _;
        }
        "wp_viewporter" => {
            display.viewporter = display.client.wl_registry_bind(
                registry,
                name,
                &extensions::viewporter::wp_viewporter_interface,
                1,
            ) as _;
        }
        "wl_shm" => {
            display.shm =
                display
                    .client
                    .wl_registry_bind(registry, name, display.client.wl_shm_interface, 1)
                    as _;
        }
        "wl_seat" => {
            let seat_version = 4.min(version);
            display.seat = display.client.wl_registry_bind(
                registry,
                name,
                display.client.wl_seat_interface,
                seat_version,
            ) as _;
            (display.client.wl_proxy_add_listener)(
                display.seat as _,
                &SEAT_LISTENER as *const _ as _,
                data,
            );
        }

        _ => {}
    }
}

unsafe extern "C" fn registry_remove_object(
    _data: *mut std::ffi::c_void,
    _registry: *mut wl_registry,
    _name: u32,
) {
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

unsafe extern "C" fn xdg_toplevel_handle_close(
    data: *mut std::ffi::c_void,
    _xdg_toplevel: *mut extensions::xdg_shell::xdg_toplevel,
) {
    assert!(!data.is_null());
    let payload: &mut WaylandPayload = &mut *(data as *mut _);
    payload.closed = true;
}

unsafe extern "C" fn xdg_toplevel_handle_configure(
    data: *mut std::ffi::c_void,
    _toplevel: *mut extensions::xdg_shell::xdg_toplevel,
    width: i32,
    height: i32,
    _states: *mut wl_array,
) -> () {
    assert!(!data.is_null());
    let payload: &mut WaylandPayload = &mut *(data as *mut _);
    let mut d = crate::native_display().lock().unwrap();

    if width != 0 && height != 0 {
        let (egl_w, egl_h) = if payload.decorations.is_some() {
            // Otherwise window will resize iteself on sway
            // I have no idea why
            (
                width - decorations::Decorations::WIDTH * 2,
                height - decorations::Decorations::BAR_HEIGHT - decorations::Decorations::WIDTH,
            )
        } else {
            (width, height)
        };
        (payload.egl.wl_egl_window_resize)(payload.egl_window, egl_w, egl_h, 0, 0);

        d.screen_width = width;
        d.screen_height = height;

        if let Some(ref decorations) = payload.decorations {
            drop(d);
            decorations.resize(&mut payload.client, width, height);
        }

        if let Some(ref mut event_handler) = payload.event_handler {
            event_handler.resize_event(width as _, height as _);
        }
    }
}

struct WaylandClipboard;
impl crate::native::Clipboard for WaylandClipboard {
    fn get(&mut self) -> Option<String> {
        None
    }
    fn set(&mut self, _data: &str) {}
}

pub fn run<F>(conf: &crate::conf::Conf, f: &mut Option<F>) -> Option<()>
where
    F: 'static + FnOnce() -> Box<dyn EventHandler>,
{
    unsafe {
        let client = LibWaylandClient::try_load()?;
        let egl = LibWaylandEgl::try_load()?;
        let xkb = LibXkbCommon::try_load()?;

        let wdisplay = (client.wl_display_connect)(std::ptr::null_mut());
        if wdisplay.is_null() {
            eprintln!("Failed to connect to Wayland display.");
            return None;
        }

        let registry: *mut wl_proxy = wl_request_constructor!(
            client,
            wdisplay,
            WL_DISPLAY_GET_REGISTRY,
            client.wl_registry_interface
        );
        assert!(!registry.is_null());

        let registry_listener = wl_registry_listener {
            global: Some(registry_add_object),
            global_remove: Some(registry_remove_object),
        };

        let xkb_context = (xkb.xkb_context_new)(0);

        let mut display = WaylandPayload {
            client: client.clone(),
            egl,
            xkb,
            compositor: std::ptr::null_mut(),
            subcompositor: std::ptr::null_mut(),
            xdg_toplevel: std::ptr::null_mut(),
            xdg_wm_base: std::ptr::null_mut(),
            surface: std::ptr::null_mut(),
            decoration_manager: std::ptr::null_mut(),
            viewporter: std::ptr::null_mut(),
            shm: std::ptr::null_mut(),
            seat: std::ptr::null_mut(),
            xkb_context,
            keymap: std::ptr::null_mut(),
            xkb_state: std::ptr::null_mut(),
            egl_window: std::ptr::null_mut(),
            pointer: std::ptr::null_mut(),
            keyboard: std::ptr::null_mut(),
            focused_window: std::ptr::null_mut(),
            decorations: None,
            event_handler: None,
            closed: false,
        };

        let (tx, rx) = std::sync::mpsc::channel();
        let clipboard = Box::new(WaylandClipboard);
        crate::set_display(NativeDisplayData {
            ..NativeDisplayData::new(conf.window_width, conf.window_height, tx, clipboard)
        });

        (display.client.wl_proxy_add_listener)(
            registry,
            &registry_listener as *const _ as _,
            &mut display as *mut _ as _,
        );
        (display.client.wl_display_roundtrip)(wdisplay);

        assert!(display.compositor.is_null() == false);
        assert!(display.xdg_wm_base.is_null() == false);
        assert!(display.subcompositor.is_null() == false);
        assert!(display.seat.is_null() == false);
        //assert!(display.keymap.is_null() == false);
        //assert!(display.xkb_state.is_null() == false);

        if display.decoration_manager.is_null() && conf.platform.wayland_use_fallback_decorations {
            eprintln!("Decoration manager not found, will draw fallback decorations");
        }

        let mut libegl = egl::LibEgl::try_load()?;
        let (context, config, egl_display) = egl::create_egl_context(
            &mut libegl,
            wdisplay as *mut _,
            conf.platform.framebuffer_alpha,
        )
        .unwrap();

        display.surface = wl_request_constructor!(
            display.client,
            display.compositor,
            WL_COMPOSITOR_CREATE_SURFACE,
            display.client.wl_surface_interface
        );
        assert!(display.surface.is_null() == false);

        let xdg_surface: *mut extensions::xdg_shell::xdg_surface = wl_request_constructor!(
            display.client,
            display.xdg_wm_base,
            extensions::xdg_shell::xdg_wm_base::get_xdg_surface,
            &extensions::xdg_shell::xdg_surface_interface,
            display.surface
        );
        assert!(xdg_surface.is_null() == false);

        let xdg_surface_listener = extensions::xdg_shell::xdg_surface_listener {
            configure: Some(xdg_surface_handle_configure),
        };

        (display.client.wl_proxy_add_listener)(
            xdg_surface as _,
            &xdg_surface_listener as *const _ as _,
            &mut display as *mut _ as _,
        );

        display.xdg_toplevel = wl_request_constructor!(
            display.client,
            xdg_surface,
            extensions::xdg_shell::xdg_surface::get_toplevel,
            &extensions::xdg_shell::xdg_toplevel_interface
        );
        assert!(display.xdg_toplevel.is_null() == false);

        let xdg_toplevel_listener = extensions::xdg_shell::xdg_toplevel_listener {
            configure: Some(xdg_toplevel_handle_configure),
            close: Some(xdg_toplevel_handle_close),
        };

        (display.client.wl_proxy_add_listener)(
            display.xdg_toplevel as _,
            &xdg_toplevel_listener as *const _ as _,
            &mut display as *mut _ as _,
        );

        wl_request!(display.client, display.surface, WL_SURFACE_COMMIT);
        (display.client.wl_display_roundtrip)(wdisplay);

        display.egl_window = (display.egl.wl_egl_window_create)(
            display.surface as _,
            conf.window_width as _,
            conf.window_height as _,
        );

        let egl_surface = (libegl.eglCreateWindowSurface.unwrap())(
            egl_display,
            config,
            display.egl_window as _,
            std::ptr::null_mut(),
        );

        if egl_surface == /* EGL_NO_SURFACE  */ std::ptr::null_mut() {
            panic!("surface creation failed");
        }
        if (libegl.eglMakeCurrent.unwrap())(egl_display, egl_surface, egl_surface, context) == 0 {
            panic!("eglMakeCurrent failed");
        }

        crate::native::gl::load_gl_funcs(|proc| {
            let name = std::ffi::CString::new(proc).unwrap();
            libegl.eglGetProcAddress.expect("non-null function pointer")(name.as_ptr() as _)
        });

        if display.decoration_manager.is_null() == false {
            let server_decoration: *mut extensions::xdg_decoration::zxdg_toplevel_decoration_v1 = wl_request_constructor!(
                display.client,
                display.decoration_manager,
                extensions::xdg_decoration::zxdg_decoration_manager_v1::get_toplevel_decoration,
                &extensions::xdg_decoration::zxdg_toplevel_decoration_v1_interface,
                display.xdg_toplevel
            );
            assert!(server_decoration.is_null() == false);

            wl_request!(
                display.client,
                server_decoration,
                extensions::xdg_decoration::zxdg_toplevel_decoration_v1::set_mode,
                extensions::xdg_decoration::ZXDG_TOPLEVEL_DECORATION_V1_MODE_SERVER_SIDE
            );
        } else if conf.platform.wayland_use_fallback_decorations {
            display.decorations = Some(decorations::Decorations::new(
                &mut display,
                conf.window_width,
                conf.window_height,
            ));
        }

        let event_handler = (f.take().unwrap())();
        display.event_handler = Some(event_handler);

        let mut keymods = KeyMods {
            shift: false,
            ctrl: false,
            alt: false,
            logo: false,
        };
        let mut repeated_keys: HashSet<KeyCode> = HashSet::new();
        let (mut last_mouse_x, mut last_mouse_y) = (0.0, 0.0);

        while display.closed == false {
            (client.wl_display_dispatch_pending)(wdisplay);

            if let Some(ref mut event_handler) = display.event_handler {
                for keycode in &repeated_keys {
                    event_handler.key_down_event(keycode.clone(), keymods, true);
                }

                for event in EVENTS.drain(..) {
                    match event {
                        WaylandEvent::KeyboardKey(keycode, state) => {
                            match keycode {
                                KeyCode::LeftShift | KeyCode::RightShift => keymods.shift = state,
                                KeyCode::LeftControl | KeyCode::RightControl => {
                                    keymods.ctrl = state
                                }
                                KeyCode::LeftAlt | KeyCode::RightAlt => keymods.alt = state,
                                KeyCode::LeftSuper | KeyCode::RightSuper => keymods.logo = state,
                                _ => {}
                            }

                            if state {
                                event_handler.key_down_event(keycode, keymods, false);
                                repeated_keys.insert(keycode);
                            } else {
                                event_handler.key_up_event(keycode, keymods);
                                repeated_keys.remove(&keycode);
                            }
                        }
                        WaylandEvent::PointerMotion(x, y) => {
                            event_handler.mouse_motion_event(x, y);
                            (last_mouse_x, last_mouse_y) = (x, y);
                        }
                        WaylandEvent::PointerButton(button, state) => {
                            if state {
                                event_handler.mouse_button_down_event(
                                    button,
                                    last_mouse_x,
                                    last_mouse_y,
                                );
                            } else {
                                event_handler.mouse_button_up_event(
                                    button,
                                    last_mouse_x,
                                    last_mouse_y,
                                );
                            }
                        }
                        WaylandEvent::PointerAxis(x, y) => event_handler.mouse_wheel_event(x, y),
                    }
                }

                event_handler.update();
                event_handler.draw();
            }

            (libegl.eglSwapBuffers.unwrap())(egl_display, egl_surface);
        }
    }

    Some(())
}
