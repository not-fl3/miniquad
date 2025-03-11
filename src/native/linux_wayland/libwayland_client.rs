#![allow(non_camel_case_types, non_upper_case_globals, dead_code)]

pub const WL_DISPLAY_SYNC: u32 = 0;
pub const WL_DISPLAY_GET_REGISTRY: u32 = 1;
pub const WL_DISPLAY_ERROR_SINCE_VERSION: u32 = 1;
pub const WL_DISPLAY_DELETE_ID_SINCE_VERSION: u32 = 1;
pub const WL_DISPLAY_SYNC_SINCE_VERSION: u32 = 1;
pub const WL_DISPLAY_GET_REGISTRY_SINCE_VERSION: u32 = 1;
pub const WL_REGISTRY_BIND: u32 = 0;
pub const WL_REGISTRY_GLOBAL_SINCE_VERSION: u32 = 1;
pub const WL_REGISTRY_GLOBAL_REMOVE_SINCE_VERSION: u32 = 1;
pub const WL_REGISTRY_BIND_SINCE_VERSION: u32 = 1;
pub const WL_CALLBACK_DONE_SINCE_VERSION: u32 = 1;
pub const WL_COMPOSITOR_CREATE_SURFACE: u32 = 0;
pub const WL_COMPOSITOR_CREATE_REGION: u32 = 1;
pub const WL_COMPOSITOR_CREATE_SURFACE_SINCE_VERSION: u32 = 1;
pub const WL_COMPOSITOR_CREATE_REGION_SINCE_VERSION: u32 = 1;
pub const WL_SHM_POOL_CREATE_BUFFER: u32 = 0;
pub const WL_SHM_POOL_DESTROY: u32 = 1;
pub const WL_SHM_POOL_RESIZE: u32 = 2;
pub const WL_SHM_POOL_CREATE_BUFFER_SINCE_VERSION: u32 = 1;
pub const WL_SHM_POOL_DESTROY_SINCE_VERSION: u32 = 1;
pub const WL_SHM_POOL_RESIZE_SINCE_VERSION: u32 = 1;
pub const WL_SHM_CREATE_POOL: u32 = 0;
pub const WL_SHM_FORMAT_SINCE_VERSION: u32 = 1;
pub const WL_SHM_CREATE_POOL_SINCE_VERSION: u32 = 1;
pub const WL_BUFFER_DESTROY: u32 = 0;
pub const WL_BUFFER_RELEASE_SINCE_VERSION: u32 = 1;
pub const WL_BUFFER_DESTROY_SINCE_VERSION: u32 = 1;
pub const WL_DATA_OFFER_ACCEPT: u32 = 0;
pub const WL_DATA_OFFER_RECEIVE: u32 = 1;
pub const WL_DATA_OFFER_DESTROY: u32 = 2;
pub const WL_DATA_OFFER_FINISH: u32 = 3;
pub const WL_DATA_OFFER_SET_ACTIONS: u32 = 4;
pub const WL_DATA_OFFER_OFFER_SINCE_VERSION: u32 = 1;
pub const WL_DATA_OFFER_SOURCE_ACTIONS_SINCE_VERSION: u32 = 3;
pub const WL_DATA_OFFER_ACTION_SINCE_VERSION: u32 = 3;
pub const WL_DATA_OFFER_ACCEPT_SINCE_VERSION: u32 = 1;
pub const WL_DATA_OFFER_RECEIVE_SINCE_VERSION: u32 = 1;
pub const WL_DATA_OFFER_DESTROY_SINCE_VERSION: u32 = 1;
pub const WL_DATA_OFFER_FINISH_SINCE_VERSION: u32 = 3;
pub const WL_DATA_OFFER_SET_ACTIONS_SINCE_VERSION: u32 = 3;
pub const WL_DATA_SOURCE_OFFER: u32 = 0;
pub const WL_DATA_SOURCE_DESTROY: u32 = 1;
pub const WL_DATA_SOURCE_SET_ACTIONS: u32 = 2;
pub const WL_DATA_SOURCE_TARGET_SINCE_VERSION: u32 = 1;
pub const WL_DATA_SOURCE_SEND_SINCE_VERSION: u32 = 1;
pub const WL_DATA_SOURCE_CANCELLED_SINCE_VERSION: u32 = 1;
pub const WL_DATA_SOURCE_DND_DROP_PERFORMED_SINCE_VERSION: u32 = 3;
pub const WL_DATA_SOURCE_DND_FINISHED_SINCE_VERSION: u32 = 3;
pub const WL_DATA_SOURCE_ACTION_SINCE_VERSION: u32 = 3;
pub const WL_DATA_SOURCE_OFFER_SINCE_VERSION: u32 = 1;
pub const WL_DATA_SOURCE_DESTROY_SINCE_VERSION: u32 = 1;
pub const WL_DATA_SOURCE_SET_ACTIONS_SINCE_VERSION: u32 = 3;
pub const WL_DATA_DEVICE_START_DRAG: u32 = 0;
pub const WL_DATA_DEVICE_SET_SELECTION: u32 = 1;
pub const WL_DATA_DEVICE_RELEASE: u32 = 2;
pub const WL_DATA_DEVICE_DATA_OFFER_SINCE_VERSION: u32 = 1;
pub const WL_DATA_DEVICE_ENTER_SINCE_VERSION: u32 = 1;
pub const WL_DATA_DEVICE_LEAVE_SINCE_VERSION: u32 = 1;
pub const WL_DATA_DEVICE_MOTION_SINCE_VERSION: u32 = 1;
pub const WL_DATA_DEVICE_DROP_SINCE_VERSION: u32 = 1;
pub const WL_DATA_DEVICE_SELECTION_SINCE_VERSION: u32 = 1;
pub const WL_DATA_DEVICE_START_DRAG_SINCE_VERSION: u32 = 1;
pub const WL_DATA_DEVICE_SET_SELECTION_SINCE_VERSION: u32 = 1;
pub const WL_DATA_DEVICE_RELEASE_SINCE_VERSION: u32 = 2;
pub const WL_DATA_DEVICE_MANAGER_CREATE_DATA_SOURCE: u32 = 0;
pub const WL_DATA_DEVICE_MANAGER_GET_DATA_DEVICE: u32 = 1;
pub const WL_DATA_DEVICE_MANAGER_CREATE_DATA_SOURCE_SINCE_VERSION: u32 = 1;
pub const WL_DATA_DEVICE_MANAGER_GET_DATA_DEVICE_SINCE_VERSION: u32 = 1;
pub const WL_DATA_DEVICE_MANAGER_DND_ACTION_COPY: u32 = 1;
pub const WL_SHELL_GET_SHELL_SURFACE: u32 = 0;
pub const WL_SHELL_GET_SHELL_SURFACE_SINCE_VERSION: u32 = 1;
pub const WL_SHELL_SURFACE_PONG: u32 = 0;
pub const WL_SHELL_SURFACE_MOVE: u32 = 1;
pub const WL_SHELL_SURFACE_RESIZE: u32 = 2;
pub const WL_SHELL_SURFACE_SET_TOPLEVEL: u32 = 3;
pub const WL_SHELL_SURFACE_SET_TRANSIENT: u32 = 4;
pub const WL_SHELL_SURFACE_SET_FULLSCREEN: u32 = 5;
pub const WL_SHELL_SURFACE_SET_POPUP: u32 = 6;
pub const WL_SHELL_SURFACE_SET_MAXIMIZED: u32 = 7;
pub const WL_SHELL_SURFACE_SET_TITLE: u32 = 8;
pub const WL_SHELL_SURFACE_SET_CLASS: u32 = 9;
pub const WL_SHELL_SURFACE_PING_SINCE_VERSION: u32 = 1;
pub const WL_SHELL_SURFACE_CONFIGURE_SINCE_VERSION: u32 = 1;
pub const WL_SHELL_SURFACE_POPUP_DONE_SINCE_VERSION: u32 = 1;
pub const WL_SHELL_SURFACE_PONG_SINCE_VERSION: u32 = 1;
pub const WL_SHELL_SURFACE_MOVE_SINCE_VERSION: u32 = 1;
pub const WL_SHELL_SURFACE_RESIZE_SINCE_VERSION: u32 = 1;
pub const WL_SHELL_SURFACE_SET_TOPLEVEL_SINCE_VERSION: u32 = 1;
pub const WL_SHELL_SURFACE_SET_TRANSIENT_SINCE_VERSION: u32 = 1;
pub const WL_SHELL_SURFACE_SET_FULLSCREEN_SINCE_VERSION: u32 = 1;
pub const WL_SHELL_SURFACE_SET_POPUP_SINCE_VERSION: u32 = 1;
pub const WL_SHELL_SURFACE_SET_MAXIMIZED_SINCE_VERSION: u32 = 1;
pub const WL_SHELL_SURFACE_SET_TITLE_SINCE_VERSION: u32 = 1;
pub const WL_SHELL_SURFACE_SET_CLASS_SINCE_VERSION: u32 = 1;
pub const WL_SURFACE_DESTROY: u32 = 0;
pub const WL_SURFACE_ATTACH: u32 = 1;
pub const WL_SURFACE_DAMAGE: u32 = 2;
pub const WL_SURFACE_FRAME: u32 = 3;
pub const WL_SURFACE_SET_OPAQUE_REGION: u32 = 4;
pub const WL_SURFACE_SET_INPUT_REGION: u32 = 5;
pub const WL_SURFACE_COMMIT: u32 = 6;
pub const WL_SURFACE_SET_BUFFER_TRANSFORM: u32 = 7;
pub const WL_SURFACE_SET_BUFFER_SCALE: u32 = 8;
pub const WL_SURFACE_DAMAGE_BUFFER: u32 = 9;
pub const WL_SURFACE_ENTER_SINCE_VERSION: u32 = 1;
pub const WL_SURFACE_LEAVE_SINCE_VERSION: u32 = 1;
pub const WL_SURFACE_DESTROY_SINCE_VERSION: u32 = 1;
pub const WL_SURFACE_ATTACH_SINCE_VERSION: u32 = 1;
pub const WL_SURFACE_DAMAGE_SINCE_VERSION: u32 = 1;
pub const WL_SURFACE_FRAME_SINCE_VERSION: u32 = 1;
pub const WL_SURFACE_SET_OPAQUE_REGION_SINCE_VERSION: u32 = 1;
pub const WL_SURFACE_SET_INPUT_REGION_SINCE_VERSION: u32 = 1;
pub const WL_SURFACE_COMMIT_SINCE_VERSION: u32 = 1;
pub const WL_SURFACE_SET_BUFFER_TRANSFORM_SINCE_VERSION: u32 = 2;
pub const WL_SURFACE_SET_BUFFER_SCALE_SINCE_VERSION: u32 = 3;
pub const WL_SURFACE_DAMAGE_BUFFER_SINCE_VERSION: u32 = 4;
pub const WL_SEAT_GET_POINTER: u32 = 0;
pub const WL_SEAT_GET_KEYBOARD: u32 = 1;
pub const WL_SEAT_GET_TOUCH: u32 = 2;
pub const WL_SEAT_RELEASE: u32 = 3;
pub const WL_SEAT_CAPABILITIES_SINCE_VERSION: u32 = 1;
pub const WL_SEAT_NAME_SINCE_VERSION: u32 = 2;
pub const WL_SEAT_GET_POINTER_SINCE_VERSION: u32 = 1;
pub const WL_SEAT_GET_KEYBOARD_SINCE_VERSION: u32 = 1;
pub const WL_SEAT_GET_TOUCH_SINCE_VERSION: u32 = 1;
pub const WL_SEAT_RELEASE_SINCE_VERSION: u32 = 5;
pub const WL_POINTER_AXIS_SOURCE_WHEEL_TILT_SINCE_VERSION: u32 = 6;
pub const WL_POINTER_SET_CURSOR: u32 = 0;
pub const WL_POINTER_RELEASE: u32 = 1;
pub const WL_POINTER_ENTER_SINCE_VERSION: u32 = 1;
pub const WL_POINTER_LEAVE_SINCE_VERSION: u32 = 1;
pub const WL_POINTER_MOTION_SINCE_VERSION: u32 = 1;
pub const WL_POINTER_BUTTON_SINCE_VERSION: u32 = 1;
pub const WL_POINTER_AXIS_SINCE_VERSION: u32 = 1;
pub const WL_POINTER_FRAME_SINCE_VERSION: u32 = 5;
pub const WL_POINTER_AXIS_SOURCE_SINCE_VERSION: u32 = 5;
pub const WL_POINTER_AXIS_STOP_SINCE_VERSION: u32 = 5;
pub const WL_POINTER_AXIS_DISCRETE_SINCE_VERSION: u32 = 5;
pub const WL_POINTER_SET_CURSOR_SINCE_VERSION: u32 = 1;
pub const WL_POINTER_RELEASE_SINCE_VERSION: u32 = 3;
pub const WL_KEYBOARD_RELEASE: u32 = 0;
pub const WL_KEYBOARD_KEYMAP_SINCE_VERSION: u32 = 1;
pub const WL_KEYBOARD_ENTER_SINCE_VERSION: u32 = 1;
pub const WL_KEYBOARD_LEAVE_SINCE_VERSION: u32 = 1;
pub const WL_KEYBOARD_KEY_SINCE_VERSION: u32 = 1;
pub const WL_KEYBOARD_MODIFIERS_SINCE_VERSION: u32 = 1;
pub const WL_KEYBOARD_REPEAT_INFO_SINCE_VERSION: u32 = 4;
pub const WL_KEYBOARD_RELEASE_SINCE_VERSION: u32 = 3;
pub const WL_TOUCH_RELEASE: u32 = 0;
pub const WL_TOUCH_DOWN_SINCE_VERSION: u32 = 1;
pub const WL_TOUCH_UP_SINCE_VERSION: u32 = 1;
pub const WL_TOUCH_MOTION_SINCE_VERSION: u32 = 1;
pub const WL_TOUCH_FRAME_SINCE_VERSION: u32 = 1;
pub const WL_TOUCH_CANCEL_SINCE_VERSION: u32 = 1;
pub const WL_TOUCH_SHAPE_SINCE_VERSION: u32 = 6;
pub const WL_TOUCH_ORIENTATION_SINCE_VERSION: u32 = 6;
pub const WL_TOUCH_RELEASE_SINCE_VERSION: u32 = 3;
pub const WL_OUTPUT_RELEASE: u32 = 0;
pub const WL_OUTPUT_GEOMETRY_SINCE_VERSION: u32 = 1;
pub const WL_OUTPUT_MODE_SINCE_VERSION: u32 = 1;
pub const WL_OUTPUT_DONE_SINCE_VERSION: u32 = 2;
pub const WL_OUTPUT_SCALE_SINCE_VERSION: u32 = 2;
pub const WL_OUTPUT_RELEASE_SINCE_VERSION: u32 = 3;
pub const WL_REGION_DESTROY: u32 = 0;
pub const WL_REGION_ADD: u32 = 1;
pub const WL_REGION_SUBTRACT: u32 = 2;
pub const WL_REGION_DESTROY_SINCE_VERSION: u32 = 1;
pub const WL_REGION_ADD_SINCE_VERSION: u32 = 1;
pub const WL_REGION_SUBTRACT_SINCE_VERSION: u32 = 1;
pub const WL_SUBCOMPOSITOR_DESTROY: u32 = 0;
pub const WL_SUBCOMPOSITOR_GET_SUBSURFACE: u32 = 1;
pub const WL_SUBCOMPOSITOR_DESTROY_SINCE_VERSION: u32 = 1;
pub const WL_SUBCOMPOSITOR_GET_SUBSURFACE_SINCE_VERSION: u32 = 1;
pub const WL_SUBSURFACE_DESTROY: u32 = 0;
pub const WL_SUBSURFACE_SET_POSITION: u32 = 1;
pub const WL_SUBSURFACE_PLACE_ABOVE: u32 = 2;
pub const WL_SUBSURFACE_PLACE_BELOW: u32 = 3;
pub const WL_SUBSURFACE_SET_SYNC: u32 = 4;
pub const WL_SUBSURFACE_SET_DESYNC: u32 = 5;
pub const WL_SUBSURFACE_DESTROY_SINCE_VERSION: u32 = 1;
pub const WL_SUBSURFACE_SET_POSITION_SINCE_VERSION: u32 = 1;
pub const WL_SUBSURFACE_PLACE_ABOVE_SINCE_VERSION: u32 = 1;
pub const WL_SUBSURFACE_PLACE_BELOW_SINCE_VERSION: u32 = 1;
pub const WL_SUBSURFACE_SET_SYNC_SINCE_VERSION: u32 = 1;
pub const WL_SUBSURFACE_SET_DESYNC_SINCE_VERSION: u32 = 1;

pub const ENOMEM: c_int = 12;
pub const EFAULT: c_int = 14;
pub const EINVAL: c_int = 22;
pub const EPROTO: c_int = 71;

pub type wl_shm_format = ::core::ffi::c_uint;

pub const wl_shm_format_WL_SHM_FORMAT_ARGB8888: wl_shm_format = 0;

pub type wl_fixed_t = ::core::ffi::c_int;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct wl_object {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct wl_array {
    pub size: libc::size_t,
    pub alloc: libc::size_t,
    pub data: *mut ::core::ffi::c_void,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union wl_argument {
    pub i: i32,
    pub u: u32,
    pub f: wl_fixed_t,
    pub s: *const ::core::ffi::c_char,
    pub o: *mut wl_object,
    pub n: u32,
    pub a: *mut wl_array,
    pub h: i32,
    _union_align: u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct wl_proxy {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct wl_display {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct wl_event_queue {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct wl_buffer {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct wl_callback {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct wl_compositor {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct wl_data_device {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct wl_data_device_manager {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct wl_data_offer {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct wl_data_source {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct wl_keyboard {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct wl_output {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct wl_pointer {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct wl_region {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct wl_registry {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct wl_seat {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct wl_shell {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct wl_shell_surface {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct wl_shm {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct wl_shm_pool {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct wl_subcompositor {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct wl_subsurface {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct wl_surface {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct wl_touch {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct wl_message {
    pub name: *const ::core::ffi::c_char,
    pub signature: *const ::core::ffi::c_char,
    pub types: *mut *const wl_interface,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct wl_interface {
    pub name: *const ::core::ffi::c_char,
    pub version: ::core::ffi::c_int,
    pub method_count: ::core::ffi::c_int,
    pub methods: *const wl_message,
    pub event_count: ::core::ffi::c_int,
    pub events: *const wl_message,
}

#[macro_export]
macro_rules! wl_listener {
    ($name_listener:ident, $name:ident, $name_dummy:ident,
    $(fn $event:ident($($arg_name:ident: $arg_ty:ty),*$(,)?)),*$(,)?
    ) => {
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        /// Wayland event listener
        pub struct $name_listener {
            $(pub $event: unsafe extern "C" fn(data: *mut core::ffi::c_void, $name: *mut $name, $($arg_name: $arg_ty),*)),*
        }
        /// Implementation for the dummy event handlers
        mod $name_dummy {
            use super::*;
            $(pub unsafe extern "C" fn $event(_: *mut core::ffi::c_void, _: *mut $name, $(_: $arg_ty),*) {})*
        }
        impl $name_listener {
            /// Create a listener with dummy event handlers
            pub const fn dummy() -> Self {
                Self {
                    $($event: $name_dummy::$event),*
                }
            }
        }
    };
}
pub const wl_seat_capability_WL_SEAT_CAPABILITY_POINTER: wl_seat_capability = 1;
pub const wl_seat_capability_WL_SEAT_CAPABILITY_KEYBOARD: wl_seat_capability = 2;
pub const wl_seat_capability_WL_SEAT_CAPABILITY_TOUCH: wl_seat_capability = 4;
pub type wl_seat_capability = ::core::ffi::c_uint;

use core::ffi::{c_char, c_int, c_uint, c_void};

pub type wl_output_subpixel = c_int;
pub type wl_output_transform = c_int;
pub type wl_output_mode = c_uint;
pub type wl_keyboard_keymap_format = c_uint;
pub type wl_keyboard_key_state = c_uint;
pub type wl_pointer_button_state = c_uint;
pub type wl_pointer_axis = c_uint;
pub type wl_pointer_axis_source = c_uint;
pub type wl_pointer_axis_relative_direction = c_uint;
pub type wl_data_device_manager_dnd_action = c_uint;

wl_listener!(
    wl_registry_listener,
    wl_registry,
    wl_registry_dummy,
    fn global(name: c_uint, interface: *const c_char, version: c_uint),
    fn global_remove(name: c_uint),
);

wl_listener!(
    wl_callback_listener,
    wl_callback,
    wl_callback_dummy,
    fn done(callback_data: c_uint),
);

wl_listener!(
    wl_seat_listener,
    wl_seat,
    wl_seat_dummy,
    fn capabilities(capabilities: wl_seat_capability),
    fn name(name: *const c_char),
);

wl_listener!(
    wl_output_listener,
    wl_output,
    wl_output_dummy,
    fn geometry(
        x: c_int,
        y: c_int,
        physical_width: c_int,
        physical_height: c_int,
        subpixel: wl_output_subpixel,
        make: *const c_char,
        model: *const c_char,
        transform: wl_output_transform,
    ),
    fn mode(
        flags: wl_output_mode,
        width: c_int,
        height: c_int,
        refresh: c_int,
    ),
    fn done(),
    fn scale(factor: c_int),
    fn name(name: *const c_char),
    fn description(description: *const c_char),
);

wl_listener!(
    wl_surface_listener,
    wl_surface,
    wl_surface_dummy,
    fn enter(output: *mut wl_output),
    fn leave(output: *mut wl_output),
    fn preferred_buffer_scale(factor: c_int),
    fn preferred_buffer_transform(transform: wl_output_transform),
);

wl_listener!(
    wl_keyboard_listener,
    wl_keyboard,
    wl_keyboard_dummy,
    fn keymap(format: wl_keyboard_keymap_format, fd: c_int, size: c_uint),
    fn enter(serial: c_uint, surface: *mut wl_surface, keys: *mut wl_array),
    fn leave(serial: c_uint, surface: *mut wl_surface),
    fn key(serial: c_uint, time: c_uint, key: c_uint, state: wl_keyboard_key_state),
    fn modifiers(
        serial: c_uint,
        mods_depressed: c_uint,
        mods_latched: c_uint,
        mods_locked: c_uint,
        group: c_uint,
    ),
    fn repeat_info(rate: c_int, delay: c_int),
);

wl_listener!(
    wl_pointer_listener,
    wl_pointer,
    wl_pointer_dummy,
    fn enter(
        serial: c_uint,
        surface: *mut wl_surface,
        surface_x: wl_fixed_t,
        surface_y: wl_fixed_t,
    ),
    fn leave(serial: c_uint, surface: *mut wl_surface),
    fn motion(time: c_uint, surface_x: wl_fixed_t, surface_y: wl_fixed_t),
    fn button(
        serial: c_uint,
        time: c_uint,
        button: c_uint,
        state: wl_pointer_button_state,
    ),
    fn axis(time: c_uint, axis: wl_pointer_axis, value: wl_fixed_t),
    fn frame(),
    fn axis_source(axis_source: wl_pointer_axis_source),
    fn axis_stop(time: c_uint, axis: wl_pointer_axis),
    fn axis_discrete(axis: wl_pointer_axis, discrete: c_int),
    fn axis_value120(axis: wl_pointer_axis, value120: c_int),
    fn axis_relative_direction(
        axis: wl_pointer_axis,
        direction: wl_pointer_axis_relative_direction,
    ),
);

wl_listener!(
    wl_touch_listener,
    wl_touch,
    wl_touch_dummy,
    fn down(
        serial: c_uint,
        time: c_uint,
        surface: *mut wl_surface,
        id: c_int,
        x: wl_fixed_t,
        y: wl_fixed_t,
    ),
    fn up(serial: c_uint, time: c_uint, id: c_int),
    fn motion(time: c_uint, id: c_int, x: wl_fixed_t, y: wl_fixed_t),
    fn frame(),
    fn cancel(),
    fn shape(id: c_int, major: wl_fixed_t, minor: wl_fixed_t),
    fn orientation(id: c_int, orientation: wl_fixed_t),
);

wl_listener!(
    wl_data_device_listener,
    wl_data_device,
    wl_data_device_dummy,
    fn data_offer(id: *mut wl_data_offer),
    fn enter(
        serial: c_uint,
        surface: *mut wl_surface,
        x: wl_fixed_t,
        y: wl_fixed_t,
        id: *mut wl_data_offer,
    ),
    fn leave(),
    fn motion(time: c_uint, x: wl_fixed_t, y: wl_fixed_t),
    fn drop(),
    fn selection(id: *mut wl_data_offer),
);

wl_listener!(
    wl_data_offer_listener,
    wl_data_offer,
    wl_data_offer_dummy,
    fn offer(mime_type: *const c_char),
    fn source_actions(source_actions: wl_data_device_manager_dnd_action),
    fn action(dnd_action: wl_data_device_manager_dnd_action),
);

wl_listener!(
    wl_data_source_listener,
    wl_data_source,
    wl_data_source_dummy,
    fn target(mime_type: *const c_char),
    fn send(mime_type: *const c_char, fd: c_int),
    fn cancelled(),
    fn dnd_drop_performed(),
    fn dnd_finished(),
    fn action(dnd_action: wl_data_device_manager_dnd_action),
);

crate::declare_module!(
    LibWaylandClient,
    "libwayland-client.so",
    "libwayland-client.so.0",
    ...
    pub wl_registry_interface: *mut wl_interface,
    pub wl_compositor_interface: *mut wl_interface,
    pub wl_subcompositor_interface: *mut wl_interface,
    pub wl_surface_interface: *mut wl_interface,
    pub wl_subsurface_interface: *mut wl_interface,
    pub wl_buffer_interface: *mut wl_interface,
    pub wl_seat_interface: *mut wl_interface,
    pub wl_shm_interface: *mut wl_interface,
    pub wl_shm_pool_interface: *mut wl_interface,
    pub wl_output_interface: *mut wl_interface,
    pub wl_keyboard_interface: *mut wl_interface,
    pub wl_pointer_interface: *mut wl_interface,
    pub wl_touch_interface: *mut wl_interface,
    pub wl_data_device_manager_interface: *mut wl_interface,
    pub wl_data_device_interface: *mut wl_interface,
    pub wl_data_source_interface: *mut wl_interface,
    ...
    pub fn wl_display_connect(*const c_char) -> *mut wl_display,
    pub fn wl_display_disconnect(*mut wl_display),
    pub fn wl_display_get_fd(*mut wl_display) -> c_int,
    pub fn wl_display_create_queue(*mut wl_display) -> *mut wl_event_queue,
    pub fn wl_display_create_queue_with_name(*mut wl_display, *const c_char) -> *mut wl_event_queue,
    pub fn wl_event_queue_get_name(*mut wl_event_queue) -> *const c_char,
    pub fn wl_event_queue_destroy(*mut wl_event_queue),
    pub fn wl_display_flush(*mut wl_display),
    pub fn wl_display_roundtrip(*mut wl_display) -> c_int,
    pub fn wl_display_roundtrip_queue(*mut wl_display, *mut wl_event_queue) -> c_int,
    pub fn wl_display_prepare_read(*mut wl_display) -> c_int,
    pub fn wl_display_prepare_read_queue(*mut wl_display, *mut wl_event_queue) -> c_int,
    pub fn wl_display_read_events(*mut wl_display) -> c_int,
    pub fn wl_display_cancel_read(*mut wl_display),
    pub fn wl_display_dispatch(*mut wl_display) -> c_int,
    pub fn wl_display_dispatch_pending(*mut wl_display) -> c_int,
    pub fn wl_display_dispatch_queue(*mut wl_display, *mut wl_event_queue) -> c_int,
    pub fn wl_display_dispatch_queue_pending(*mut wl_display, *mut wl_event_queue) -> c_int,
    pub fn wl_display_get_error(*mut wl_display) -> c_int,
    pub fn wl_display_get_protocol_error(*mut wl_display, *mut *const wl_interface, *mut c_uint) -> c_uint,
    pub fn wl_proxy_set_queue(*mut wl_proxy, *mut wl_event_queue),
    pub fn wl_proxy_get_queue(*mut wl_proxy) -> *mut wl_event_queue,
    pub fn wl_proxy_add_listener(*mut wl_proxy, *mut Option<unsafe extern "C" fn()>, *mut c_void) -> c_int,
    pub fn wl_proxy_destroy(*mut wl_proxy),
    pub fn wl_proxy_get_version(*mut wl_proxy) -> c_uint,
    ...
    pub fn wl_proxy_marshal(*mut wl_proxy, c_uint, ...),
    pub fn wl_proxy_marshal_constructor(*mut wl_proxy, c_uint, *const wl_interface, ...) -> *mut wl_proxy,
    pub fn wl_proxy_marshal_constructor_versioned(*mut wl_proxy, c_uint, *const wl_interface, c_uint, ...) -> *mut wl_proxy,
    pub fn wl_proxy_marshal_flags(*mut wl_proxy, c_uint, *const wl_interface, c_uint, c_uint, ...) -> *mut wl_proxy,
    ...
);

impl LibWaylandClient {
    pub unsafe fn wl_registry_bind(
        &mut self,
        wl_registry: *const wl_registry,
        name: c_uint,
        interface: *const wl_interface,
        version: c_uint,
    ) -> *mut c_void {
        let id: *mut wl_proxy = (self.wl_proxy_marshal_constructor_versioned)(
            wl_registry as _,
            WL_REGISTRY_BIND,
            interface as _,
            version,
            name,
            (*interface).name,
            version,
        );
        id as *mut _
    }
    pub unsafe fn data_offer_receive(
        &mut self,
        display: *mut wl_display,
        data_offer: *mut wl_data_offer,
        mime_type: *const c_char,
    ) -> Option<Vec<u8>> {
        let mut fds: [c_int; 2] = [0; 2];
        assert_eq!(libc::pipe(fds.as_mut_ptr()), 0);
        (self.wl_proxy_marshal)(data_offer as _, WL_DATA_OFFER_RECEIVE, mime_type, fds[1]);
        libc::close(fds[1]);
        (self.wl_display_roundtrip)(display);
        let mut bytes = Vec::new();
        loop {
            let mut buf = [0_u8; 1024];
            let n = libc::read(fds[0], buf.as_mut_ptr() as _, buf.len());
            match n {
                n if n > 0 => bytes.extend_from_slice(&buf[..n as usize]),
                0 => break,
                _ => return None,
            }
        }
        libc::close(fds[0]);
        Some(bytes)
    }
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
