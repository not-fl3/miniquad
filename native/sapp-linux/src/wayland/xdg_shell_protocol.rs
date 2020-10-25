use crate::wayland::wayland_client::{wl_seat_interface, wl_output_interface, wl_interface, wl_message, wl_array};

#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct xdg_surface_listener {
    pub configure: Option<
        unsafe extern "C" fn(_: *mut std::ffi::c_void, _: *mut xdg_surface, _: u32) -> (),
    >,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct xdg_toplevel_listener {
    pub configure: Option<
        unsafe extern "C" fn(
            _: *mut std::ffi::c_void,
            _: *mut xdg_toplevel,
            _: i32,
            _: i32,
            _: *mut wl_array,
        ) -> (),
    >,
    pub close:
        Option<unsafe extern "C" fn(_: *mut std::ffi::c_void, _: *mut xdg_toplevel) -> ()>,
}

macro_rules! count {
    () => (0usize);
    ( $x:tt $($xs:tt)* ) => (1usize + count!($($xs)*));
}

macro_rules! method_consts {
    ($x:expr, ()) => {
    };
    ($x:expr, ($next:ident, $($rest:ident,)*)) => {
        pub const $next: u32 = $x;
        method_consts!(($x + 1), ($($rest,)*));
    };
}
macro_rules! wayland_interface {
    ($name:ident, $struct_name:ident, $version:expr,
     [
         $(
             ($method_name:ident, $method_sign:expr, ($($method_argument_name:expr),*))
         ),*
     ],
     [
         $(
             ($event_name:expr, $event_sign:expr)
         ),*
     ]) => {
        mod $name {
            use super::*;

            $(
                mod $method_name {
                    use super::*;

                    pub static mut METHOD_ARGUMENTS_TYPES: [*const wl_interface; count!($($method_argument_name)*)] = [$(unsafe { &$method_argument_name as *const _},)*];

                }
            )*

            static mut requests: [wl_message; count!($($method_name)*)] = [$(wl_message {
                name: concat!(stringify!($method_name), '\0').as_ptr() as _,
                signature: concat!($method_sign, '\0').as_ptr() as _,
                types: unsafe { $method_name::METHOD_ARGUMENTS_TYPES.as_ptr() as _ }
            }), *];

            static mut events: [wl_message; count!($($event_name)*)] = [$(wl_message {
                name: concat!($event_name, '\0').as_ptr() as _,
                signature: concat!($event_sign, '\0').as_ptr() as _,
                types: std::ptr::null_mut()
            }),*];

            pub static mut $name: wl_interface = wl_interface {
                name: concat!(stringify!($struct_name), '\0').as_ptr() as *const i8,
                version: $version,
                method_count: count!($($method_name)*) as i32,
                methods: unsafe { requests.as_ptr() },
                event_count: count!($($event_name)*) as i32,
                events: unsafe { events.as_ptr() },
            };
        }

        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct $struct_name {
            _unused: [u8; 0],
        }

        impl $struct_name {
            method_consts!(0, ($($method_name,)*));
        }
        pub use $name::$name;
    };
}

wayland_interface!(
    xdg_wm_base_interface,
    xdg_wm_base,
    3,
    [
        (destroy, "", ()),
        (create_positioner, "n", (xdg_positioner_interface)),
        (get_xdg_surface, "no", (xdg_surface_interface)),
        (pong, "", ())
    ],
    [("ping", "u")]
);

wayland_interface!(
    xdg_surface_interface,
    xdg_surface,
    3,
    [
        (destroy, "", ()),
        (get_toplevel, "n", (xdg_toplevel_interface)),
        (get_popup, "n?oo", ()),
        (set_window_geometry, "iiii", ()),
        (ack_configure, "u", ())
    ],
    [("configure", "u")]
);

wayland_interface!(
    xdg_toplevel_interface,
    xdg_toplevel,
    3,
    [
        (destroy, "", ()),
        (set_parent, "?o", (xdg_toplevel_interface)),
        (set_title, "s", ()),
        (set_app_id, "s", ()),
        (show_window_menu, "ouii", (xdg_popup_interface)),
        (r#move, "ou", (wl_seat_interface)),
        (resize, "ouu", (wl_seat_interface)),
        (set_max_size, "ii", ()),
        (set_min_size, "ii", ()),
        (set_maximized, "", ()),
        (unset_maximized, "", ()),
        (set_fullscreen, "", (wl_output_interface)),
        (unset_fullscreen, "", ()),
        (set_minimized, "", ())
    ],
    [("configure", "iia"), ("close", "")]
);

wayland_interface!(
    xdg_positioner_interface,
    xdg_positioner,
    3,
    [
        (destroy, "", ()),
        (set_size, "ii", ()),
        (set_anchor_rect, "iiii", ()),
        (set_anchor, "u", ()),
        (set_gravity, "u", ()),
        (set_constraint_adjustment, "u", ()),
        (set_offset, "ii", ()),
        (set_reactive, "3", ()),
        (set_parent_size, "3ii", ()),
        (set_parent_configure, "3u", ())
    ],
    []
);

wayland_interface!(
    xdg_popup_interface,
    xdg_popup,
    3,
    [
        (destroy, "", ()),
        (grab, "ou", (wl_seat_interface)),
        (reposition, "3ou", (xdg_positioner_interface))
    ],
    [
        ("configure", "iiii"),
        ("popup_done", "")
    ]
);
