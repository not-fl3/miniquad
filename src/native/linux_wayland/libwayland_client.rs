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

pub type wl_shm_format = ::std::os::raw::c_uint;

pub const wl_shm_format_WL_SHM_FORMAT_ARGB8888: wl_shm_format = 0;

pub type wl_fixed_t = i32;

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
    pub data: *mut ::std::os::raw::c_void,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union wl_argument {
    pub i: i32,
    pub u: u32,
    pub f: wl_fixed_t,
    pub s: *const ::std::os::raw::c_char,
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
    pub name: *const ::std::os::raw::c_char,
    pub signature: *const ::std::os::raw::c_char,
    pub types: *mut *const wl_interface,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct wl_interface {
    pub name: *const ::std::os::raw::c_char,
    pub version: ::std::os::raw::c_int,
    pub method_count: ::std::os::raw::c_int,
    pub methods: *const wl_message,
    pub event_count: ::std::os::raw::c_int,
    pub events: *const wl_message,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct wl_registry_listener {
    pub global: ::std::option::Option<
        unsafe extern "C" fn(
            data: *mut ::std::os::raw::c_void,
            wl_registry: *mut wl_registry,
            name: u32,
            interface: *const ::std::os::raw::c_char,
            version: u32,
        ),
    >,
    pub global_remove: ::std::option::Option<
        unsafe extern "C" fn(
            data: *mut ::std::os::raw::c_void,
            wl_registry: *mut wl_registry,
            name: u32,
        ),
    >,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct wl_callback_listener {
    pub done: ::std::option::Option<
        unsafe extern "C" fn(
            data: *mut ::std::os::raw::c_void,
            wl_callback: *mut wl_callback,
            callback_data: u32,
        ),
    >,
}
pub const wl_seat_capability_WL_SEAT_CAPABILITY_POINTER: wl_seat_capability = 1;
pub const wl_seat_capability_WL_SEAT_CAPABILITY_KEYBOARD: wl_seat_capability = 2;
pub const wl_seat_capability_WL_SEAT_CAPABILITY_TOUCH: wl_seat_capability = 4;
pub type wl_seat_capability = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct wl_seat_listener {
    pub capabilities: ::std::option::Option<
        unsafe extern "C" fn(
            data: *mut ::std::os::raw::c_void,
            wl_seat: *mut wl_seat,
            capabilities: u32,
        ),
    >,
    pub name: ::std::option::Option<
        unsafe extern "C" fn(
            data: *mut ::std::os::raw::c_void,
            wl_seat: *mut wl_seat,
            name: *const ::std::os::raw::c_char,
        ),
    >,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct wl_keyboard_listener {
    pub keymap: ::std::option::Option<
        unsafe extern "C" fn(
            data: *mut ::std::os::raw::c_void,
            wl_keyboard: *mut wl_keyboard,
            format: u32,
            fd: i32,
            size: u32,
        ),
    >,
    pub enter: ::std::option::Option<
        unsafe extern "C" fn(
            data: *mut ::std::os::raw::c_void,
            wl_keyboard: *mut wl_keyboard,
            serial: u32,
            surface: *mut wl_surface,
            keys: *mut wl_array,
        ),
    >,
    pub leave: ::std::option::Option<
        unsafe extern "C" fn(
            data: *mut ::std::os::raw::c_void,
            wl_keyboard: *mut wl_keyboard,
            serial: u32,
            surface: *mut wl_surface,
        ),
    >,
    pub key: ::std::option::Option<
        unsafe extern "C" fn(
            data: *mut ::std::os::raw::c_void,
            wl_keyboard: *mut wl_keyboard,
            serial: u32,
            time: u32,
            key: u32,
            state: u32,
        ),
    >,
    pub modifiers: ::std::option::Option<
        unsafe extern "C" fn(
            data: *mut ::std::os::raw::c_void,
            wl_keyboard: *mut wl_keyboard,
            serial: u32,
            mods_depressed: u32,
            mods_latched: u32,
            mods_locked: u32,
            group: u32,
        ),
    >,
    pub repeat_info: ::std::option::Option<
        unsafe extern "C" fn(
            data: *mut ::std::os::raw::c_void,
            wl_keyboard: *mut wl_keyboard,
            rate: i32,
            delay: i32,
        ),
    >,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct wl_pointer_listener {
    pub enter: ::std::option::Option<
        unsafe extern "C" fn(
            data: *mut ::std::os::raw::c_void,
            wl_pointer: *mut wl_pointer,
            serial: u32,
            surface: *mut wl_surface,
            surface_x: i32,
            surface_y: i32,
        ),
    >,
    pub leave: ::std::option::Option<
        unsafe extern "C" fn(
            data: *mut ::std::os::raw::c_void,
            wl_pointer: *mut wl_pointer,
            serial: u32,
            surface: *mut wl_surface,
        ),
    >,
    pub motion: ::std::option::Option<
        unsafe extern "C" fn(
            data: *mut ::std::os::raw::c_void,
            wl_pointer: *mut wl_pointer,
            time: u32,
            surface_x: i32,
            surface_y: i32,
        ),
    >,
    pub button: ::std::option::Option<
        unsafe extern "C" fn(
            data: *mut ::std::os::raw::c_void,
            wl_pointer: *mut wl_pointer,
            serial: u32,
            time: u32,
            button: u32,
            state: u32,
        ),
    >,
    pub axis: ::std::option::Option<
        unsafe extern "C" fn(
            data: *mut ::std::os::raw::c_void,
            wl_pointer: *mut wl_pointer,
            time: u32,
            axis: u32,
            value: i32,
        ),
    >,
    pub frame: ::std::option::Option<
        unsafe extern "C" fn(data: *mut ::std::os::raw::c_void, wl_pointer: *mut wl_pointer),
    >,
    pub axis_source: ::std::option::Option<
        unsafe extern "C" fn(
            data: *mut ::std::os::raw::c_void,
            wl_pointer: *mut wl_pointer,
            axis_source: u32,
        ),
    >,
    pub axis_stop: ::std::option::Option<
        unsafe extern "C" fn(
            data: *mut ::std::os::raw::c_void,
            wl_pointer: *mut wl_pointer,
            time: u32,
            axis: u32,
        ),
    >,
    pub axis_discrete: ::std::option::Option<
        unsafe extern "C" fn(
            data: *mut ::std::os::raw::c_void,
            wl_pointer: *mut wl_pointer,
            axis: u32,
            discrete: i32,
        ),
    >,
    pub axis_value120: ::std::option::Option<
        unsafe extern "C" fn(
            data: *mut ::std::os::raw::c_void,
            wl_pointer: *mut wl_pointer,
            axis: u32,
            value120: i32,
        ),
    >,
    pub axis_relative_direction: ::std::option::Option<
        unsafe extern "C" fn(
            data: *mut ::std::os::raw::c_void,
            wl_pointer: *mut wl_pointer,
            axis: u32,
            direction: u32,
        ),
    >,
}

pub type wl_display_connect =
    unsafe extern "C" fn(name: *const ::std::os::raw::c_char) -> *mut wl_display;
pub type wl_proxy_destroy = unsafe extern "C" fn(proxy: *mut wl_proxy);
pub type wl_proxy_marshal = unsafe extern "C" fn(p: *mut wl_proxy, opcode: u32, ...);
pub type wl_proxy_marshal_constructor = unsafe extern "C" fn(
    proxy: *mut wl_proxy,
    opcode: u32,
    interface: *const wl_interface,
    ...
) -> *mut wl_proxy;
pub type wl_proxy_marshal_constructor_versioned = unsafe extern "C" fn(
    proxy: *mut wl_proxy,
    opcode: u32,
    interface: *const wl_interface,
    version: u32,
    ...
) -> *mut wl_proxy;
pub type wl_proxy_add_listener = unsafe extern "C" fn(
    proxy: *mut wl_proxy,
    implementation: *mut ::std::option::Option<unsafe extern "C" fn()>,
    data: *mut ::std::os::raw::c_void,
) -> ::std::os::raw::c_int;

pub type wl_display_roundtrip =
    unsafe extern "C" fn(display: *mut wl_display) -> ::std::os::raw::c_int;
pub type wl_display_dispatch_pending =
    unsafe extern "C" fn(display: *mut wl_display) -> ::std::os::raw::c_int;

#[derive(Clone)]
pub struct LibWaylandClient {
    _module: std::rc::Rc<crate::native::module::Module>,
    pub wl_display_connect: wl_display_connect,
    pub wl_proxy_destroy: wl_proxy_destroy,
    pub wl_proxy_marshal: wl_proxy_marshal,
    pub wl_proxy_marshal_constructor: wl_proxy_marshal_constructor,
    pub wl_proxy_marshal_constructor_versioned: wl_proxy_marshal_constructor_versioned,
    pub wl_display_dispatch_pending: wl_display_dispatch_pending,
    pub wl_proxy_add_listener: wl_proxy_add_listener,
    pub wl_display_roundtrip: wl_display_roundtrip,
    pub wl_registry_interface: *mut wl_interface,
    pub wl_compositor_interface: *mut wl_interface,
    pub wl_subcompositor_interface: *mut wl_interface,
    pub wl_surface_interface: *mut wl_interface,
    pub wl_subsurface_interface: *mut wl_interface,
    pub wl_buffer_interface: *mut wl_interface,
    pub wl_seat_interface: *mut wl_interface,
    pub wl_shm_interface: *mut wl_interface,
    pub wl_shm_pool_interface: *mut wl_interface,
    pub wl_keyboard_interface: *mut wl_interface,
    pub wl_pointer_interface: *mut wl_interface,
}

impl LibWaylandClient {
    pub fn try_load() -> Option<LibWaylandClient> {
        crate::native::module::Module::load("libwayland-client.so")
            .or_else(|_| crate::native::module::Module::load("libwayland-client.so.0"))
            .map(|module| LibWaylandClient {
                wl_display_connect: module.get_symbol("wl_display_connect").unwrap(),
                wl_proxy_add_listener: module.get_symbol("wl_proxy_add_listener").unwrap(),
                wl_display_dispatch_pending: module
                    .get_symbol("wl_display_dispatch_pending")
                    .unwrap(),

                wl_proxy_destroy: module.get_symbol("wl_proxy_destroy").unwrap(),
                wl_proxy_marshal: module.get_symbol("wl_proxy_marshal").unwrap(),
                wl_proxy_marshal_constructor: module
                    .get_symbol("wl_proxy_marshal_constructor")
                    .unwrap(),
                wl_proxy_marshal_constructor_versioned: module
                    .get_symbol("wl_proxy_marshal_constructor_versioned")
                    .unwrap(),
                wl_display_roundtrip: module.get_symbol("wl_display_roundtrip").unwrap(),

                wl_registry_interface: module.get_symbol("wl_registry_interface").unwrap(),
                wl_compositor_interface: module.get_symbol("wl_compositor_interface").unwrap(),
                wl_subcompositor_interface: module
                    .get_symbol("wl_subcompositor_interface")
                    .unwrap(),
                wl_surface_interface: module.get_symbol("wl_surface_interface").unwrap(),
                wl_subsurface_interface: module.get_symbol("wl_subsurface_interface").unwrap(),
                wl_buffer_interface: module.get_symbol("wl_buffer_interface").unwrap(),
                wl_seat_interface: module.get_symbol("wl_seat_interface").unwrap(),
                wl_shm_interface: module.get_symbol("wl_shm_interface").unwrap(),
                wl_shm_pool_interface: module.get_symbol("wl_shm_pool_interface").unwrap(),
                wl_keyboard_interface: module.get_symbol("wl_keyboard_interface").unwrap(),
                wl_pointer_interface: module.get_symbol("wl_pointer_interface").unwrap(),

                _module: std::rc::Rc::new(module),
            })
            .ok()
    }

    pub unsafe fn wl_registry_bind(
        &mut self,
        wl_registry: *const wl_registry,
        name: u32,
        interface: *const wl_interface,
        version: u32,
    ) -> *mut std::ffi::c_void {
        let id: *mut wl_proxy;

        id = (self.wl_proxy_marshal_constructor_versioned)(
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
}
