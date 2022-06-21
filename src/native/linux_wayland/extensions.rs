#![allow(unused_variables, dead_code, non_upper_case_globals)]

pub mod viewporter;
pub mod xdg_decoration;
pub mod xdg_shell;

#[macro_export]
macro_rules! count {
    () => (0usize);
    ( $x:tt $($xs:tt)* ) => (1usize + crate::count!($($xs)*));
}

#[macro_export]
macro_rules! method_consts {
    ($x:expr, ()) => {
    };
    ($x:expr, ($next:ident, $($rest:ident,)*)) => {
        pub const $next: u32 = $x;
        crate::method_consts!(($x + 1), ($($rest,)*));
    };
}

#[macro_export]
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

                    pub static mut METHOD_ARGUMENTS_TYPES: [*const wl_interface; crate::count!($($method_argument_name)*)] = [$(unsafe { &$method_argument_name as *const _},)*];

                }
            )*

            static mut requests: [wl_message; crate::count!($($method_name)*)] = [$(wl_message {
                name: concat!(stringify!($method_name), '\0').as_ptr() as _,
                signature: concat!($method_sign, '\0').as_ptr() as _,
                types: unsafe { $method_name::METHOD_ARGUMENTS_TYPES.as_ptr() as _ }
            }), *];

            static mut events: [wl_message; crate::count!($($event_name)*)] = [$(wl_message {
                name: concat!($event_name, '\0').as_ptr() as _,
                signature: concat!($event_sign, '\0').as_ptr() as _,
                types: std::ptr::null_mut()
            }),*];

            pub static mut $name: wl_interface = wl_interface {
                name: concat!(stringify!($struct_name), '\0').as_ptr() as *const _,
                version: $version,
                method_count: crate::count!($($method_name)*) as i32,
                methods: unsafe { requests.as_ptr() },
                event_count: crate::count!($($event_name)*) as i32,
                events: unsafe { events.as_ptr() },
            };
        }

        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct $struct_name {
            _unused: [u8; 0],
        }

        impl $struct_name {
            crate::method_consts!(0, ($($method_name,)*));
        }
        pub use $name::$name;
    };
}

/// Redifinition some interfaces from wayland-protocol.c, to have them available
/// in compile time, to allow other interfaces use them as their arguments.
pub mod wayland_protocol {
    use super::super::{wl_interface, wl_message};

    wayland_interface!(
        wl_output_interface,
        wl_output,
        3,
        [(release, "3", ())],
        [
            ("geometry", "iiiiissi"),
            ("mode", "uiii"),
            ("done", "2"),
            ("scale", "2i")
        ]
    );

    wayland_interface!(
        wl_seat_interface,
        wl_seat,
        7,
        [
            (get_pointer, "n", ()),
            (get_keyboard, "n", ()),
            (get_touch, "n", ()),
            (release, "5", ())
        ],
        [("capabilities", "u"), ("name", "2s")]
    );
}
