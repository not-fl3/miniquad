#![allow(dead_code, static_mut_refs)]

mod libwayland_client;
mod libwayland_egl;
mod libxkbcommon;

mod clipboard;
mod decorations;
mod drag_n_drop;
mod extensions;
mod keycodes;
mod shm;

use crate::{wl_request, wl_request_constructor};
use libwayland_client::*;
use libwayland_egl::*;
use libxkbcommon::*;

use crate::{
    event::{EventHandler, KeyCode, KeyMods, MouseButton},
    native::{egl, NativeDisplayData, Request},
};

use core::time::Duration;
use std::collections::HashMap;

fn wl_fixed_to_double(f: i32) -> f32 {
    (f as f32) / 256.0
}

/// A thing to pass around within *void pointer of wayland's event handler
struct WaylandPayload {
    client: LibWaylandClient,
    display: *mut wl_display,
    registry: *mut wl_registry,
    // this is libwayland-egl.so, a library with ~4 functions
    // not the libEGL.so(which will be loaded, but not here)
    egl: LibWaylandEgl,
    xkb: LibXkbCommon,
    compositor: *mut wl_compositor,
    subcompositor: *mut wl_subcompositor,
    xdg_toplevel: *mut extensions::xdg_shell::xdg_toplevel,
    xdg_wm_base: *mut extensions::xdg_shell::xdg_wm_base,
    surface: *mut wl_surface,
    viewporter: *mut extensions::viewporter::wp_viewporter,
    shm: *mut wl_shm,
    seat: *mut wl_seat,
    data_device_manager: *mut wl_data_device_manager,
    data_device: *mut wl_data_device,
    xkb_context: *mut xkb_context,
    xkb_state: *mut xkb_state,
    keymap: XkbKeymap,

    egl_window: *mut wl_egl_window,
    pointer_context: PointerContext,
    keyboard: *mut wl_keyboard,
    touch: *mut wl_touch,
    touch_positions: HashMap<core::ffi::c_int, (f32, f32)>,
    focused_window: *mut wl_surface,
    decoration_manager: *mut extensions::xdg_decoration::zxdg_decoration_manager_v1,
    decorations: decorations::Decorations,

    events: Vec<WaylandEvent>,
    keyboard_context: KeyboardContext,
    drag_n_drop: drag_n_drop::WaylandDnD,
    update_requested: bool,
}

impl WaylandPayload {
    /// Poll new events, `blocking` specifies whether it should block until a new event is
    /// available
    // needs to combine both the Wayland events and the key repeat events
    // the implementation is translated from glfw
    unsafe fn poll_new_event(&mut self, blocking: bool) {
        let mut fds = [
            libc::pollfd {
                fd: (self.client.wl_display_get_fd)(self.display),
                events: libc::POLLIN,
                revents: 0,
            },
            libc::pollfd {
                fd: self.keyboard_context.timerfd,
                events: libc::POLLIN,
                revents: 0,
            },
        ];
        (self.client.wl_display_flush)(self.display);
        while (self.client.wl_display_prepare_read)(self.display) != 0 {
            (self.client.wl_display_dispatch_pending)(self.display);
        }
        if libc::poll(fds.as_mut_ptr(), 2, if blocking { i32::MAX } else { 0 }) > 0 {
            // if the Wayland display has events available
            if fds[0].revents & libc::POLLIN == 1 {
                (self.client.wl_display_read_events)(self.display);
                (self.client.wl_display_dispatch_pending)(self.display);
            } else {
                (self.client.wl_display_cancel_read)(self.display);
            }
            // if key repeat takes place
            if fds[1].revents & libc::POLLIN == 1 {
                let mut count: [libc::size_t; 1] = [0];
                let n_bits = core::mem::size_of::<libc::size_t>();
                assert_eq!(
                    libc::read(
                        self.keyboard_context.timerfd,
                        count.as_mut_ptr() as _,
                        n_bits
                    ),
                    n_bits as _
                );
                for _ in 0..count[0] {
                    self.keyboard_context.generate_key_repeat_events(
                        &mut self.xkb,
                        self.keymap.xkb_keymap,
                        self.xkb_state,
                        &mut self.events,
                    );
                }
            }
        } else {
            (self.client.wl_display_cancel_read)(self.display);
        }
        let errno = (self.client.wl_display_get_error)(self.display);
        // A non-zero errno means the compositor decided that we need to die.
        // Nothing more we can do at this point :(
        // If we want the detailed error message, we need to run with `WAYLAND_DEBUG=client`, since
        // the message string is not accessible to us.
        match errno {
            0 => (),
            EPROTO => {
                let mut interface: *const wl_interface = std::ptr::null();
                let mut id = 0;
                let code = (self.client.wl_display_get_protocol_error)(
                    self.display,
                    &mut interface,
                    &mut id,
                );
                let name = core::ffi::CStr::from_ptr((*interface).name)
                    .to_str()
                    .unwrap();
                panic!(
                    "Wayland protocol error at {}#{} with code {}",
                    name, id, code
                )
            }
            _ => {
                panic!("Wayland display error with code {}", errno)
            }
        }
    }
    unsafe fn init_data_device(&mut self) {
        self.data_device = wl_request_constructor!(
            self.client,
            self.data_device_manager,
            WL_DATA_DEVICE_MANAGER_GET_DATA_DEVICE,
            self.client.wl_data_device_interface,
            self.seat
        );
        assert!(!self.data_device.is_null());
        DATA_DEVICE_LISTENER.data_offer = data_device_handle_data_offer;
        DATA_DEVICE_LISTENER.enter = drag_n_drop::data_device_handle_enter;
        DATA_DEVICE_LISTENER.leave = drag_n_drop::data_device_handle_leave;
        DATA_DEVICE_LISTENER.drop = drag_n_drop::data_device_handle_drop;
        DATA_DEVICE_LISTENER.selection = clipboard::data_device_handle_selection;
        (self.client.wl_proxy_add_listener)(
            self.data_device as _,
            &DATA_DEVICE_LISTENER as *const _ as _,
            self as *mut _ as _,
        );
    }
    unsafe fn init_pointer_context(&mut self) {
        if !self.pointer_context.cursor_shape_manager.is_null() {
            self.pointer_context.cursor_shape_device = wl_request_constructor!(
                self.client,
                self.pointer_context.cursor_shape_manager,
                extensions::cursor::CURSOR_SHAPE_MANAGER_GET_POINTER,
                &extensions::cursor::wp_cursor_shape_device_v1_interface,
                self.pointer_context.pointer
            );
            assert!(!self.pointer_context.cursor_shape_device.is_null());
        } else {
            eprintln!("Wayland compositor does not support cursor shape");
        }
    }
    unsafe fn set_fullscreen(&mut self, full: bool) {
        if full {
            wl_request!(
                self.client,
                self.xdg_toplevel,
                extensions::xdg_shell::xdg_toplevel::set_fullscreen,
                std::ptr::null_mut::<wl_output>()
            );
        } else {
            wl_request!(
                self.client,
                self.xdg_toplevel,
                extensions::xdg_shell::xdg_toplevel::unset_fullscreen
            );
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum RepeatInfo {
    Repeat { delay: Duration, gap: Duration },
    NoRepeat,
}

impl Default for RepeatInfo {
    // default value copied from winit
    fn default() -> Self {
        Self::Repeat {
            delay: Duration::from_millis(200),
            gap: Duration::from_millis(40),
        }
    }
}

// key repeat in Wayland needs to be handled by the client
// `KeyboardContext` is mostly for tracking the currently repeated key
// Note that apparently `timerfd` is not unix compliant and only available on linux
struct KeyboardContext {
    enter_serial: Option<core::ffi::c_uint>,
    repeat_info: RepeatInfo,
    /// This is the actual key being sent by Wayland, not `keysym` or Miniquad `Keycode`
    repeated_key: Option<core::ffi::c_uint>,
    timerfd: core::ffi::c_int,
    keymods: KeyMods,
}

fn new_itimerspec() -> libc::itimerspec {
    libc::itimerspec {
        it_interval: libc::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        it_value: libc::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
    }
}

impl KeyboardContext {
    fn new() -> Self {
        Self {
            enter_serial: None,
            repeat_info: Default::default(),
            repeated_key: None,
            timerfd: unsafe { libc::timerfd_create(libc::CLOCK_MONOTONIC, libc::TFD_CLOEXEC) },
            keymods: Default::default(),
        }
    }
    fn track_key_down(&mut self, key: core::ffi::c_uint) {
        let mut timer = new_itimerspec();
        match self.repeat_info {
            RepeatInfo::Repeat { delay, gap } => {
                self.repeated_key = Some(key);
                timer.it_interval.tv_sec = gap.as_secs() as _;
                timer.it_interval.tv_nsec = gap.subsec_nanos() as _;
                timer.it_value.tv_sec = delay.as_secs() as _;
                timer.it_value.tv_nsec = delay.subsec_nanos() as _;
            }
            RepeatInfo::NoRepeat => self.repeated_key = None,
        }
        unsafe {
            libc::timerfd_settime(self.timerfd, 0, &timer, core::ptr::null_mut());
        }
    }
    fn track_key_up(&mut self, key: core::ffi::c_uint) {
        if self.repeated_key == Some(key) {
            self.repeated_key = None;
            unsafe {
                libc::timerfd_settime(self.timerfd, 0, &new_itimerspec(), core::ptr::null_mut());
            }
        }
    }
    unsafe fn generate_key_repeat_events(
        &self,
        libxkb: &mut LibXkbCommon,
        xkb_keymap: *mut xkb_keymap,
        xkb_state: *mut xkb_state,
        events: &mut Vec<WaylandEvent>,
    ) {
        if let Some(key) = self.repeated_key {
            self.generate_key_events(libxkb, xkb_keymap, xkb_state, key, true, events)
        }
    }
    unsafe fn generate_key_events(
        &self,
        libxkb: &mut LibXkbCommon,
        xkb_keymap: *mut xkb_keymap,
        xkb_state: *mut xkb_state,
        key: core::ffi::c_uint,
        repeat: bool,
        events: &mut Vec<WaylandEvent>,
    ) {
        // The keycodes in Miniquad are obtained without modifiers
        let keysym = libxkb.keymap_key_get_sym_without_mod(xkb_keymap, key + 8);
        let keycode = keycodes::translate_keysym(keysym);
        events.push(WaylandEvent::KeyDown(keycode, self.keymods, repeat));

        // To obtain the underlying character, we do need to provide the modifiers
        let keysym = (libxkb.xkb_state_key_get_one_sym)(xkb_state, key + 8);
        let chr = (libxkb.xkb_keysym_to_utf32)(keysym);
        if chr > 0 {
            if let Some(chr) = char::from_u32(chr) {
                events.push(WaylandEvent::Char(chr, self.keymods, repeat));
            }
        }
    }
}

struct PointerContext {
    pointer: *mut wl_pointer,
    enter_serial: Option<core::ffi::c_uint>,
    position: (f32, f32),
    /// Wayland does not remember what cursor icon a window has; if the cursor leaves and comes
    /// back, it will not be reset to what icon it had unless we keep track of it.
    cursor_icon: Option<crate::CursorIcon>,
    /// Wayland requires that only the window with focus can set the cursor. So if we don't have
    /// the focus yet, we queue the cursor icon and apply it once we regain focus.
    queued_cursor_icon: Option<Option<crate::CursorIcon>>,
    cursor_shape_manager: *mut extensions::cursor::wp_cursor_shape_manager_v1,
    cursor_shape_device: *mut extensions::cursor::wp_cursor_shape_device_v1,
    pointer_constraints: *mut extensions::cursor::zwp_pointer_constraints_v1,
    locked_pointer: *mut extensions::cursor::zwp_locked_pointer_v1,
    relative_pointer_manager: *mut extensions::cursor::zwp_relative_pointer_manager_v1,
    relative_pointer: *mut extensions::cursor::zwp_relative_pointer_v1,
}
impl PointerContext {
    fn new() -> Self {
        Self {
            pointer: std::ptr::null_mut(),
            enter_serial: None,
            position: (0., 0.),
            cursor_icon: Some(crate::CursorIcon::Default),
            queued_cursor_icon: None,
            cursor_shape_manager: std::ptr::null_mut(),
            cursor_shape_device: std::ptr::null_mut(),
            pointer_constraints: std::ptr::null_mut(),
            locked_pointer: std::ptr::null_mut(),
            relative_pointer_manager: std::ptr::null_mut(),
            relative_pointer: std::ptr::null_mut(),
        }
    }
    unsafe fn set_cursor_with_serial(
        &mut self,
        client: &mut LibWaylandClient,
        icon: Option<crate::CursorIcon>,
        serial: core::ffi::c_uint,
    ) {
        self.cursor_icon = icon;
        if let Some(icon) = icon {
            if !self.cursor_shape_device.is_null() {
                wl_request!(
                    client,
                    self.cursor_shape_device,
                    extensions::cursor::CURSOR_SHAPE_DEVICE_SET_SHAPE,
                    serial,
                    extensions::cursor::translate_cursor(icon)
                );
            }
        } else {
            wl_request!(
                client,
                self.pointer,
                WL_POINTER_SET_CURSOR,
                serial,
                std::ptr::null_mut::<wl_surface>(),
                0,
                0
            );
        }
    }
    fn handle_enter(&mut self, client: &mut LibWaylandClient, serial: core::ffi::c_uint) {
        self.enter_serial = Some(serial);
        let change = self.queued_cursor_icon.take().unwrap_or(self.cursor_icon);
        unsafe {
            self.set_cursor_with_serial(client, change, serial);
        }
    }
    /// Change the cursor to the given icon (or hide it if `None` is passed)
    /// If the window currently does not have focus, the change will be queued and applied once the
    /// window regains focus
    fn set_cursor(&mut self, client: &mut LibWaylandClient, icon: Option<crate::CursorIcon>) {
        if let Some(serial) = self.enter_serial {
            unsafe {
                self.set_cursor_with_serial(client, icon, serial);
            }
        } else {
            self.queued_cursor_icon = Some(icon);
        }
    }
    unsafe fn set_grab(&mut self, data: *mut std::ffi::c_void, grab: bool) {
        let display: &mut WaylandPayload = &mut *(data as *mut _);
        if grab {
            if self.locked_pointer.is_null() {
                if !self.pointer_constraints.is_null() {
                    self.locked_pointer = wl_request_constructor!(
                        display.client,
                        self.pointer_constraints,
                        extensions::cursor::POINTER_CONSTRAINTS_LOCK_POINTER,
                        &extensions::cursor::zwp_locked_pointer_v1_interface,
                        display.surface,
                        self.pointer,
                        std::ptr::null_mut::<wl_region>(),
                        extensions::cursor::zwp_pointer_constraints_v1_lifetime_PERSISTENT
                    );
                    assert!(!self.locked_pointer.is_null());
                } else {
                    eprintln!("Wayland compositor does not support locked pointer");
                }
            }

            if self.relative_pointer.is_null() {
                if !self.relative_pointer_manager.is_null() {
                    self.relative_pointer = wl_request_constructor!(
                        display.client,
                        self.relative_pointer_manager,
                        extensions::cursor::RELATIVE_POINTER_MANAGER_GET_RELATIVE_POINTER,
                        &extensions::cursor::zwp_relative_pointer_v1_interface,
                        self.pointer
                    );
                    assert!(!self.relative_pointer.is_null());
                    (RELATIVE_POINTER_LISTENER.relative_motion) =
                        relative_pointer_handle_relative_motion;
                    (display.client.wl_proxy_add_listener)(
                        self.relative_pointer as _,
                        &RELATIVE_POINTER_LISTENER as *const _ as _,
                        data,
                    );
                } else {
                    eprintln!("Wayland compositor does not support relative pointer");
                }
            }
        } else {
            if !self.locked_pointer.is_null() {
                wl_request!(display.client, self.locked_pointer, 0);
                (display.client.wl_proxy_destroy)(self.locked_pointer as _);
                self.locked_pointer = std::ptr::null_mut();
            }
            if !self.relative_pointer.is_null() {
                wl_request!(display.client, self.relative_pointer, 0);
                (display.client.wl_proxy_destroy)(self.relative_pointer as _);
                self.relative_pointer = std::ptr::null_mut();
            }
        }
    }
}

static mut SEAT_LISTENER: wl_seat_listener = wl_seat_listener::dummy();
static mut KEYBOARD_LISTENER: wl_keyboard_listener = wl_keyboard_listener::dummy();
static mut POINTER_LISTENER: wl_pointer_listener = wl_pointer_listener::dummy();
static mut TOUCH_LISTENER: wl_touch_listener = wl_touch_listener::dummy();
static mut OUTPUT_LISTENER: wl_output_listener = wl_output_listener::dummy();
static mut DATA_DEVICE_LISTENER: wl_data_device_listener = wl_data_device_listener::dummy();
static mut DATA_OFFER_LISTENER: wl_data_offer_listener = wl_data_offer_listener::dummy();
static mut XDG_WM_BASE_LISTENER: extensions::xdg_shell::xdg_wm_base_listener =
    extensions::xdg_shell::xdg_wm_base_listener::dummy();
static mut RELATIVE_POINTER_LISTENER: extensions::cursor::zwp_relative_pointer_v1_listener =
    extensions::cursor::zwp_relative_pointer_v1_listener::dummy();

unsafe extern "C" fn seat_handle_capabilities(
    data: *mut std::ffi::c_void,
    seat: *mut wl_seat,
    caps: wl_seat_capability,
) {
    let display: &mut WaylandPayload = &mut *(data as *mut _);

    if caps & wl_seat_capability_WL_SEAT_CAPABILITY_POINTER != 0 {
        display.pointer_context.pointer = wl_request_constructor!(
            display.client,
            seat,
            WL_SEAT_GET_POINTER,
            display.client.wl_pointer_interface
        );
        assert!(!display.pointer_context.pointer.is_null());
        POINTER_LISTENER.enter = pointer_handle_enter;
        POINTER_LISTENER.axis = pointer_handle_axis;
        POINTER_LISTENER.motion = pointer_handle_motion;
        POINTER_LISTENER.button = pointer_handle_button;
        POINTER_LISTENER.leave = pointer_handle_leave;
        (display.client.wl_proxy_add_listener)(
            display.pointer_context.pointer as _,
            &POINTER_LISTENER as *const _ as _,
            data,
        );
    }

    if caps & wl_seat_capability_WL_SEAT_CAPABILITY_KEYBOARD != 0 {
        display.keyboard = wl_request_constructor!(
            display.client,
            seat,
            WL_SEAT_GET_KEYBOARD,
            display.client.wl_keyboard_interface
        );
        assert!(!display.keyboard.is_null());
        KEYBOARD_LISTENER.enter = keyboard_handle_enter;
        KEYBOARD_LISTENER.keymap = keyboard_handle_keymap;
        KEYBOARD_LISTENER.repeat_info = keyboard_handle_repeat_info;
        KEYBOARD_LISTENER.key = keyboard_handle_key;
        KEYBOARD_LISTENER.modifiers = keyboard_handle_modifiers;
        KEYBOARD_LISTENER.leave = keyboard_handle_leave;
        (display.client.wl_proxy_add_listener)(
            display.keyboard as _,
            &KEYBOARD_LISTENER as *const _ as _,
            data,
        );
    }

    if caps & wl_seat_capability_WL_SEAT_CAPABILITY_TOUCH != 0 {
        display.touch = wl_request_constructor!(
            display.client,
            seat,
            WL_SEAT_GET_TOUCH,
            display.client.wl_touch_interface
        );
        assert!(!display.touch.is_null());
        TOUCH_LISTENER.down = touch_handle_down;
        TOUCH_LISTENER.up = touch_handle_up;
        TOUCH_LISTENER.motion = touch_handle_motion;
        TOUCH_LISTENER.cancel = touch_handle_cancel;
        (display.client.wl_proxy_add_listener)(
            display.touch as _,
            &TOUCH_LISTENER as *const _ as _,
            data,
        );
    }
}

enum WaylandEvent {
    KeyDown(KeyCode, KeyMods, bool),
    KeyUp(KeyCode, KeyMods),
    Char(char, KeyMods, bool),
    PointerMotion(f32, f32),
    RawMotion(f32, f32),
    PointerButton(MouseButton, bool),
    PointerAxis(f32, f32),
    Touch(crate::TouchPhase, u64, f32, f32),
    FilesDropped(String),
    Resize(f32, f32),
    WindowMinimized,
    WindowRestored,
}

unsafe extern "C" fn keyboard_handle_keymap(
    data: *mut ::core::ffi::c_void,
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
    (display.xkb.xkb_keymap_unref)(display.keymap.xkb_keymap);
    display.keymap.xkb_keymap = (display.xkb.xkb_keymap_new_from_string)(
        display.xkb_context,
        map_shm as *mut libc::FILE,
        1,
        0,
    );
    libc::munmap(map_shm, size as usize);
    libc::close(fd);
    display.keymap.cache_mod_indices(&mut display.xkb);
    (display.xkb.xkb_state_unref)(display.xkb_state);
    display.xkb_state = (display.xkb.xkb_state_new)(display.keymap.xkb_keymap);
}
unsafe extern "C" fn keyboard_handle_enter(
    data: *mut ::core::ffi::c_void,
    _wl_keyboard: *mut wl_keyboard,
    serial: ::core::ffi::c_uint,
    _surface: *mut wl_surface,
    _keys: *mut wl_array,
) {
    let display: &mut WaylandPayload = &mut *(data as *mut _);
    // Needed for setting the clipboard
    display.keyboard_context.enter_serial = Some(serial);
    display.events.push(WaylandEvent::WindowRestored);
}
unsafe extern "C" fn keyboard_handle_leave(
    data: *mut ::core::ffi::c_void,
    _wl_keyboard: *mut wl_keyboard,
    _serial: u32,
    _surface: *mut wl_surface,
) {
    // Clear modifiers
    let display: &mut WaylandPayload = &mut *(data as *mut _);
    (display.xkb.xkb_state_update_mask)(display.xkb_state, 0, 0, 0, 0, 0, 0);
    display.keyboard_context.keymods = KeyMods::default();
    display.keyboard_context.repeated_key = None;
    display.keyboard_context.enter_serial = None;
    display.events.push(WaylandEvent::WindowMinimized);
}
unsafe extern "C" fn keyboard_handle_key(
    data: *mut ::core::ffi::c_void,
    _wl_keyboard: *mut wl_keyboard,
    _serial: u32,
    _time: u32,
    key: u32,
    state: wl_keyboard_key_state,
) {
    let display: &mut WaylandPayload = &mut *(data as *mut _);
    let libxkb = &mut display.xkb;
    let xkb_keymap = display.keymap.xkb_keymap;
    let xkb_state = display.xkb_state;
    // https://wayland-book.com/seat/keyboard.html
    // To translate this to an XKB scancode, you must add 8 to the evdev scancode.
    let keysym = libxkb.keymap_key_get_sym_without_mod(xkb_keymap, key + 8);
    let keycode = keycodes::translate_keysym(keysym);
    let keymods = display.keymap.get_keymods(libxkb, xkb_state);
    display.keyboard_context.keymods = keymods;
    match state {
        0 => {
            display.keyboard_context.track_key_up(key);
            display.events.push(WaylandEvent::KeyUp(keycode, keymods));
        }
        1 | 2 => {
            let repeat = state == 2;
            let should_repeat = (libxkb.xkb_keymap_key_repeats)(xkb_keymap, key + 8) == 1;
            if !repeat && should_repeat {
                display.keyboard_context.track_key_down(key);
            }
            display.keyboard_context.generate_key_events(
                libxkb,
                xkb_keymap,
                xkb_state,
                key,
                repeat,
                &mut display.events,
            );
        }
        _ => {
            eprintln!("Unknown wl_keyboard::key_state");
        }
    };
}
unsafe extern "C" fn keyboard_handle_modifiers(
    data: *mut ::core::ffi::c_void,
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
    data: *mut ::core::ffi::c_void,
    _wl_keyboard: *mut wl_keyboard,
    rate: i32,
    delay: i32,
) {
    let display: &mut WaylandPayload = &mut *(data as *mut _);
    display.keyboard_context.repeat_info = if rate == 0 {
        RepeatInfo::NoRepeat
    } else {
        RepeatInfo::Repeat {
            delay: Duration::from_millis(delay as u64),
            gap: Duration::from_micros(1_000_000 / rate as u64),
        }
    };
}

unsafe extern "C" fn pointer_handle_enter(
    data: *mut ::core::ffi::c_void,
    _wl_pointer: *mut wl_pointer,
    serial: u32,
    surface: *mut wl_surface,
    _surface_x: i32,
    _surface_y: i32,
) {
    let display: &mut WaylandPayload = &mut *(data as *mut _);
    display.focused_window = surface;
    if surface == display.surface {
        display
            .pointer_context
            .handle_enter(&mut display.client, serial);
    }
}

unsafe extern "C" fn pointer_handle_leave(
    data: *mut ::core::ffi::c_void,
    _wl_pointer: *mut wl_pointer,
    _serial: u32,
    _surface: *mut wl_surface,
) {
    let display: &mut WaylandPayload = &mut *(data as *mut _);
    display.pointer_context.enter_serial = None;
}

unsafe extern "C" fn pointer_handle_motion(
    data: *mut ::core::ffi::c_void,
    _wl_pointer: *mut wl_pointer,
    _time: u32,
    surface_x: i32,
    surface_y: i32,
) {
    let display: &mut WaylandPayload = &mut *(data as *mut _);
    if display.focused_window == display.surface {
        // From wl_fixed_to_double(), it simply divides by 256
        let d = crate::native_display().lock().unwrap();
        let x = wl_fixed_to_double(surface_x) * d.dpi_scale;
        let y = wl_fixed_to_double(surface_y) * d.dpi_scale;
        display.pointer_context.position = (x, y);
        display.events.push(WaylandEvent::PointerMotion(x, y));
    }
}
unsafe extern "C" fn pointer_handle_button(
    data: *mut ::core::ffi::c_void,
    _wl_pointer: *mut wl_pointer,
    _serial: u32,
    _time: u32,
    button: u32,
    state: u32,
) {
    let display: &mut WaylandPayload = &mut *(data as *mut _);
    if display.focused_window == display.surface {
        // The code is defined in the kernel's linux/input-event-codes.h header file, e.g. BTN_LEFT
        let button = match button {
            272 => MouseButton::Left,
            273 => MouseButton::Right,
            274 => MouseButton::Middle,
            _ => MouseButton::Unknown,
        };
        display
            .events
            .push(WaylandEvent::PointerButton(button, state == 1));
    }
}
unsafe extern "C" fn pointer_handle_axis(
    data: *mut ::core::ffi::c_void,
    _wl_pointer: *mut wl_pointer,
    _time: u32,
    axis: u32,
    value: i32,
) {
    let display: &mut WaylandPayload = &mut *(data as *mut _);
    let mut value = wl_fixed_to_double(value);
    // https://wayland-book.com/seat/pointer.html
    if axis == 0 {
        // Vertical scroll
        // Wayland defines the direction differently to miniquad so lets flip it
        value = -value;
        display.events.push(WaylandEvent::PointerAxis(0.0, value));
    } else if axis == 1 {
        // Horizontal scroll
        display.events.push(WaylandEvent::PointerAxis(value, 0.0));
    }
}

unsafe extern "C" fn relative_pointer_handle_relative_motion(
    data: *mut ::core::ffi::c_void,
    _relative_pointer: *mut extensions::cursor::zwp_relative_pointer_v1,
    _utime_hi: core::ffi::c_uint,
    _utime_lo: core::ffi::c_uint,
    dx: wl_fixed_t,
    dy: wl_fixed_t,
    _dx_unaccel: wl_fixed_t,
    _dy_unaccel: wl_fixed_t,
) {
    let display: &mut WaylandPayload = &mut *(data as *mut _);
    if display.focused_window == display.surface {
        // From wl_fixed_to_double(), it simply divides by 256
        let dx = wl_fixed_to_double(dx);
        let dy = wl_fixed_to_double(dy);
        display.events.push(WaylandEvent::RawMotion(dx, dy));
    }
}

unsafe extern "C" fn output_handle_scale(
    _data: *mut std::ffi::c_void,
    _output: *mut wl_output,
    factor: core::ffi::c_int,
) {
    let mut d = crate::native_display().try_lock().unwrap();
    if d.high_dpi {
        let dpi_scale = d.dpi_scale as i32;
        d.screen_width = d.screen_width / dpi_scale * factor;
        d.screen_height = d.screen_height / dpi_scale * factor;
        d.dpi_scale = factor as _;
    }
}

unsafe extern "C" fn touch_handle_down(
    data: *mut std::ffi::c_void,
    _touch: *mut wl_touch,
    _serial: core::ffi::c_uint,
    _time: core::ffi::c_uint,
    surface: *mut wl_surface,
    id: core::ffi::c_int,
    x: wl_fixed_t,
    y: wl_fixed_t,
) {
    let display: &mut WaylandPayload = &mut *(data as *mut _);
    display.focused_window = surface;
    if display.focused_window == display.surface {
        let d = crate::native_display().lock().unwrap();
        let x = wl_fixed_to_double(x) * d.dpi_scale;
        let y = wl_fixed_to_double(y) * d.dpi_scale;
        display.touch_positions.insert(id, (x, y));
        display.events.push(WaylandEvent::Touch(
            crate::TouchPhase::Started,
            id as _,
            x,
            y,
        ));
    }
}

unsafe extern "C" fn touch_handle_motion(
    data: *mut std::ffi::c_void,
    _touch: *mut wl_touch,
    _time: core::ffi::c_uint,
    id: core::ffi::c_int,
    x: wl_fixed_t,
    y: wl_fixed_t,
) {
    let display: &mut WaylandPayload = &mut *(data as *mut _);
    if display.focused_window == display.surface {
        let d = crate::native_display().lock().unwrap();
        let x = wl_fixed_to_double(x) * d.dpi_scale;
        let y = wl_fixed_to_double(y) * d.dpi_scale;
        display.touch_positions.insert(id, (x, y));
        display
            .events
            .push(WaylandEvent::Touch(crate::TouchPhase::Moved, id as _, x, y));
    }
}

unsafe extern "C" fn touch_handle_up(
    data: *mut std::ffi::c_void,
    _touch: *mut wl_touch,
    _serial: core::ffi::c_uint,
    _time: core::ffi::c_uint,
    id: core::ffi::c_int,
) {
    let display: &mut WaylandPayload = &mut *(data as *mut _);
    if display.focused_window == display.surface {
        if let Some((x, y)) = display.touch_positions.remove(&id) {
            display
                .events
                .push(WaylandEvent::Touch(crate::TouchPhase::Ended, id as _, x, y));
        }
    }
}

unsafe extern "C" fn touch_handle_cancel(data: *mut std::ffi::c_void, _touch: *mut wl_touch) {
    let display: &mut WaylandPayload = &mut *(data as *mut _);
    for (id, (x, y)) in display.touch_positions.drain() {
        display.events.push(WaylandEvent::Touch(
            crate::TouchPhase::Cancelled,
            id as _,
            x,
            y,
        ));
    }
}

unsafe extern "C" fn registry_add_object(
    data: *mut std::ffi::c_void,
    registry: *mut wl_registry,
    name: u32,
    interface: *const ::core::ffi::c_char,
    version: u32,
) {
    let display: &mut WaylandPayload = &mut *(data as *mut _);

    let interface = std::ffi::CStr::from_ptr(interface).to_str().unwrap();
    match interface {
        "wl_output" => {
            let wl_output: *mut wl_output = display.client.wl_registry_bind(
                registry,
                name,
                display.client.wl_output_interface,
                3.min(version),
            ) as _;
            assert!(!wl_output.is_null());
            OUTPUT_LISTENER.scale = output_handle_scale;
            (display.client.wl_proxy_add_listener)(
                wl_output as _,
                &OUTPUT_LISTENER as *const _ as _,
                display as *mut _ as _,
            );
        }
        "wl_compositor" => {
            display.compositor = display.client.wl_registry_bind(
                registry,
                name,
                display.client.wl_compositor_interface,
                3.min(version),
            ) as _;
            assert!(!display.compositor.is_null());
            display.surface = wl_request_constructor!(
                display.client,
                display.compositor,
                WL_COMPOSITOR_CREATE_SURFACE,
                display.client.wl_surface_interface
            );
            assert!(!display.surface.is_null());
        }
        "wl_subcompositor" => {
            display.subcompositor = display.client.wl_registry_bind(
                registry,
                name,
                display.client.wl_subcompositor_interface,
                1,
            ) as _;
            assert!(!display.subcompositor.is_null());
        }
        "xdg_wm_base" => {
            display.xdg_wm_base = display.client.wl_registry_bind(
                registry,
                name,
                &extensions::xdg_shell::xdg_wm_base_interface,
                1,
            ) as _;
            assert!(!display.xdg_wm_base.is_null());
            XDG_WM_BASE_LISTENER.ping = xdg_wm_base_handle_ping;
            (display.client.wl_proxy_add_listener)(
                display.xdg_wm_base as _,
                &XDG_WM_BASE_LISTENER as *const _ as _,
                display as *mut _ as _,
            );
        }
        "zxdg_decoration_manager" | "zxdg_decoration_manager_v1" => {
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
        "wp_cursor_shape_manager_v1" => {
            display.pointer_context.cursor_shape_manager = display.client.wl_registry_bind(
                registry,
                name,
                &extensions::cursor::wp_cursor_shape_manager_v1_interface as _,
                1,
            ) as _;
        }
        "zwp_pointer_constraints_v1" => {
            display.pointer_context.pointer_constraints = display.client.wl_registry_bind(
                registry,
                name,
                &extensions::cursor::zwp_pointer_constraints_v1_interface as _,
                1,
            ) as _;
        }
        "zwp_relative_pointer_manager_v1" => {
            display.pointer_context.relative_pointer_manager = display.client.wl_registry_bind(
                registry,
                name,
                &extensions::cursor::zwp_relative_pointer_manager_v1_interface as _,
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
            assert!(!display.seat.is_null());
            SEAT_LISTENER.capabilities = seat_handle_capabilities;
            (display.client.wl_proxy_add_listener)(
                display.seat as _,
                &SEAT_LISTENER as *const _ as _,
                data,
            );
        }
        "wl_data_device_manager" => {
            display.data_device_manager = display.client.wl_registry_bind(
                registry,
                name,
                display.client.wl_data_device_manager_interface,
                3,
            ) as _;
            assert!(!display.data_device_manager.is_null());
        }

        _ => {}
    }
}

unsafe extern "C" fn xdg_wm_base_handle_ping(
    data: *mut std::ffi::c_void,
    toplevel: *mut extensions::xdg_shell::xdg_wm_base,
    serial: u32,
) {
    assert!(!data.is_null());
    let payload: &mut WaylandPayload = &mut *(data as *mut _);

    wl_request!(
        payload.client,
        toplevel,
        extensions::xdg_shell::xdg_wm_base::pong,
        serial
    );
}

unsafe extern "C" fn data_device_handle_data_offer(
    data: *mut ::core::ffi::c_void,
    data_device: *mut wl_data_device,
    data_offer: *mut wl_data_offer,
) {
    DATA_OFFER_LISTENER.source_actions = drag_n_drop::data_offer_handle_source_actions;
    let display: &mut WaylandPayload = &mut *(data as *mut _);
    assert_eq!(data_device, display.data_device);
    (display.client.wl_proxy_add_listener)(
        data_offer as _,
        &DATA_OFFER_LISTENER as *const _ as _,
        data,
    );
}

pub fn run<F>(conf: &crate::conf::Conf, f: &mut Option<F>) -> Option<()>
where
    F: 'static + FnOnce() -> Box<dyn EventHandler>,
{
    unsafe {
        let client = LibWaylandClient::try_load().ok()?;
        let egl = LibWaylandEgl::try_load().ok()?;
        let xkb = LibXkbCommon::try_load().ok()?;

        let wdisplay = (client.wl_display_connect)(std::ptr::null_mut());
        if wdisplay.is_null() {
            eprintln!("Failed to connect to Wayland display.");
            return None;
        }

        let registry: *mut wl_registry = wl_request_constructor!(
            client,
            wdisplay,
            WL_DISPLAY_GET_REGISTRY,
            client.wl_registry_interface
        );
        assert!(!registry.is_null());

        let xkb_context = (xkb.xkb_context_new)(0);

        let mut display = WaylandPayload {
            client,
            display: wdisplay,
            registry,
            egl,
            xkb,
            compositor: std::ptr::null_mut(),
            subcompositor: std::ptr::null_mut(),
            xdg_toplevel: std::ptr::null_mut(),
            xdg_wm_base: std::ptr::null_mut(),
            surface: std::ptr::null_mut(),
            viewporter: std::ptr::null_mut(),
            shm: std::ptr::null_mut(),
            seat: std::ptr::null_mut(),
            data_device_manager: std::ptr::null_mut(),
            data_device: std::ptr::null_mut(),
            xkb_context,
            keymap: Default::default(),
            xkb_state: std::ptr::null_mut(),
            egl_window: std::ptr::null_mut(),
            keyboard: std::ptr::null_mut(),
            touch: std::ptr::null_mut(),
            touch_positions: HashMap::new(),
            focused_window: std::ptr::null_mut(),
            decoration_manager: std::ptr::null_mut(),
            decorations: decorations::Decorations::None,
            events: Vec::new(),
            pointer_context: PointerContext::new(),
            keyboard_context: KeyboardContext::new(),
            drag_n_drop: Default::default(),
            update_requested: true,
        };

        let mut registry_listener = wl_registry_listener::dummy();
        registry_listener.global = registry_add_object;
        (display.client.wl_proxy_add_listener)(
            display.registry as _,
            &registry_listener as *const _ as _,
            &mut display as *mut _ as _,
        );

        let (tx, rx) = std::sync::mpsc::channel();
        let clipboard = Box::new(clipboard::WaylandClipboard::new(&mut display as *mut _));
        crate::set_display(NativeDisplayData {
            high_dpi: conf.high_dpi,
            dpi_scale: 1., // At this point dpi_scale is not known to us
            blocking_event_loop: conf.platform.blocking_event_loop,
            ..NativeDisplayData::new(conf.window_width, conf.window_height, tx, clipboard)
        });

        (display.client.wl_display_dispatch)(display.display);
        (display.client.wl_display_dispatch)(display.display);

        display.init_data_device();
        display.init_pointer_context();

        let mut libegl = egl::LibEgl::try_load().ok()?;
        let (context, config, egl_display) = egl::create_egl_context(
            &mut libegl,
            wdisplay as *mut _,
            conf.platform.framebuffer_alpha,
            conf.sample_count,
        )
        .unwrap();

        {
            // At this point we have been told the dpi_scale
            let d = crate::native_display().try_lock().unwrap();
            display.egl_window = (display.egl.wl_egl_window_create)(
                display.surface as _,
                d.screen_width,
                d.screen_height,
            );
            wl_request!(
                display.client,
                display.surface,
                WL_SURFACE_SET_BUFFER_SCALE,
                d.dpi_scale as i32
            );
        }

        let egl_surface = (libegl.eglCreateWindowSurface)(
            egl_display,
            config,
            display.egl_window as _,
            std::ptr::null_mut(),
        );

        if egl_surface.is_null() {
            // == EGL_NO_SURFACE
            panic!("surface creation failed");
        }
        if (libegl.eglMakeCurrent)(egl_display, egl_surface, egl_surface, context) == 0 {
            panic!("eglMakeCurrent failed");
        }

        if (libegl.eglSwapInterval)(egl_display, conf.platform.swap_interval.unwrap_or(1)) == 0 {
            eprintln!("eglSwapInterval failed");
        }

        crate::native::gl::load_gl_funcs(|proc| {
            let name = std::ffi::CString::new(proc).unwrap();
            (libegl.eglGetProcAddress)(name.as_ptr() as _)
        });

        display.decorations =
            decorations::Decorations::new(&mut display, conf.platform.wayland_decorations);
        assert!(!display.xdg_toplevel.is_null());

        display.decorations.set_title(
            &mut display.client,
            display.xdg_toplevel,
            conf.window_title.as_str(),
        );

        let wm_class = std::ffi::CString::new(conf.platform.linux_wm_class).unwrap();
        wl_request!(
            display.client,
            display.xdg_toplevel,
            extensions::xdg_shell::xdg_toplevel::set_app_id,
            wm_class.as_ptr()
        );

        if conf.fullscreen {
            display.set_fullscreen(true);
        }

        wl_request!(display.client, display.surface, WL_SURFACE_COMMIT);
        (display.client.wl_display_dispatch)(display.display);
        (display.client.wl_display_dispatch)(display.display);

        let mut event_handler = (f.take().unwrap())();

        while !crate::native_display().try_lock().unwrap().quit_ordered {
            while let Ok(request) = rx.try_recv() {
                match request {
                    Request::SetFullscreen(full) => {
                        display.set_fullscreen(full);
                    }
                    Request::ScheduleUpdate => display.update_requested = true,
                    Request::SetMouseCursor(icon) => {
                        display
                            .pointer_context
                            .set_cursor(&mut display.client, Some(icon));
                    }
                    Request::SetCursorGrab(grab) => {
                        let payload = &mut display as *mut _ as _;
                        display.pointer_context.set_grab(payload, grab);
                    }
                    Request::ShowMouse(show) => {
                        display.pointer_context.set_cursor(
                            &mut display.client,
                            show.then_some(crate::CursorIcon::Default),
                        );
                    }
                    // TODO: implement the other events
                    _ => (),
                }
            }

            // If `blocking_event_loop` is set but an update is requested, we should still poll the
            // new events but continue without blocking
            let blocking = conf.platform.blocking_event_loop && !display.update_requested;
            display.poll_new_event(blocking);

            for event in display.events.drain(..) {
                match event {
                    WaylandEvent::KeyDown(keycode, keymods, repeat) => {
                        event_handler.key_down_event(keycode, keymods, repeat)
                    }
                    WaylandEvent::KeyUp(keycode, keymods) => {
                        event_handler.key_up_event(keycode, keymods)
                    }
                    WaylandEvent::Char(chr, keymods, repeat) => {
                        event_handler.char_event(chr, keymods, repeat)
                    }
                    WaylandEvent::PointerMotion(x, y) => {
                        event_handler.mouse_motion_event(x, y);
                    }
                    WaylandEvent::RawMotion(dx, dy) => {
                        event_handler.raw_mouse_motion(dx, dy);
                    }
                    WaylandEvent::PointerButton(button, state) => {
                        let (x, y) = display.pointer_context.position;
                        if state {
                            event_handler.mouse_button_down_event(button, x, y);
                        } else {
                            event_handler.mouse_button_up_event(button, x, y);
                        }
                    }
                    WaylandEvent::PointerAxis(x, y) => event_handler.mouse_wheel_event(x, y),
                    WaylandEvent::Touch(phase, id, x, y) => {
                        event_handler.touch_event(phase, id, x, y)
                    }
                    WaylandEvent::Resize(width, height) => {
                        event_handler.resize_event(width, height)
                    }
                    WaylandEvent::WindowMinimized => event_handler.window_minimized_event(),
                    WaylandEvent::WindowRestored => event_handler.window_restored_event(),
                    WaylandEvent::FilesDropped(filenames) => {
                        let mut d = crate::native_display().try_lock().unwrap();
                        d.dropped_files = Default::default();
                        for filename in filenames.lines() {
                            let path = std::path::PathBuf::from(filename);
                            if let Ok(bytes) = std::fs::read(&path) {
                                d.dropped_files.paths.push(path);
                                d.dropped_files.bytes.push(bytes);
                            }
                        }
                        // drop d since files_dropped_event is likely to need access to it
                        drop(d);
                        event_handler.files_dropped_event();
                    }
                }
            }

            {
                let d = crate::native_display().try_lock().unwrap();
                if d.quit_requested && !d.quit_ordered {
                    drop(d);
                    event_handler.quit_requested_event();
                    let mut d = crate::native_display().try_lock().unwrap();
                    if d.quit_requested {
                        d.quit_ordered = true
                    }
                }
            }

            if !conf.platform.blocking_event_loop || display.update_requested {
                display.update_requested = false;
                event_handler.update();
                event_handler.draw();
                (libegl.eglSwapBuffers)(egl_display, egl_surface);
            }
        }
    }

    Some(())
}
