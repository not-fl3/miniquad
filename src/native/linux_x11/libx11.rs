#![allow(non_snake_case, non_upper_case_globals, dead_code)]
pub use X_h::*;
pub use Xlib_h::*;
pub use Xresource_h::*;
pub use Xutil_h::*;

pub mod Xlib_h {
    pub type Display = _XDisplay;
    pub type XEvent = _XEvent;

    #[derive(Copy, Clone)]
    #[repr(C)]
    pub union _XEvent {
        pub type_0: libc::c_int,
        pub xany: XAnyEvent,
        pub xkey: XKeyEvent,
        pub xbutton: XButtonEvent,
        pub xmotion: XMotionEvent,
        pub xcrossing: XCrossingEvent,
        pub xfocus: XFocusChangeEvent,
        pub xexpose: XExposeEvent,
        pub xgraphicsexpose: XGraphicsExposeEvent,
        pub xnoexpose: XNoExposeEvent,
        pub xvisibility: XVisibilityEvent,
        pub xcreatewindow: XCreateWindowEvent,
        pub xdestroywindow: XDestroyWindowEvent,
        pub xunmap: XUnmapEvent,
        pub xmap: XMapEvent,
        pub xmaprequest: XMapRequestEvent,
        pub xreparent: XReparentEvent,
        pub xconfigure: XConfigureEvent,
        pub xgravity: XGravityEvent,
        pub xresizerequest: XResizeRequestEvent,
        pub xconfigurerequest: XConfigureRequestEvent,
        pub xcirculate: XCirculateEvent,
        pub xcirculaterequest: XCirculateRequestEvent,
        pub xproperty: XPropertyEvent,
        pub xselectionclear: XSelectionClearEvent,
        pub xselectionrequest: XSelectionRequestEvent,
        pub xselection: XSelectionEvent,
        pub xcolormap: XColormapEvent,
        pub xclient: XClientMessageEvent,
        pub xmapping: XMappingEvent,
        pub xerror: XErrorEvent,
        pub xkeymap: XKeymapEvent,
        pub xgeneric: XGenericEvent,
        pub xcookie: XGenericEventCookie,
        pub pad: [libc::c_long; 24],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct XGenericEventCookie {
        pub type_0: libc::c_int,
        pub serial: libc::c_ulong,
        pub send_event: libc::c_int,
        pub display: *mut Display,
        pub extension: libc::c_int,
        pub evtype: libc::c_int,
        pub cookie: libc::c_uint,
        pub data: *mut libc::c_void,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct XGenericEvent {
        pub type_0: libc::c_int,
        pub serial: libc::c_ulong,
        pub send_event: libc::c_int,
        pub display: *mut Display,
        pub extension: libc::c_int,
        pub evtype: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct XKeymapEvent {
        pub type_0: libc::c_int,
        pub serial: libc::c_ulong,
        pub send_event: libc::c_int,
        pub display: *mut Display,
        pub window: Window,
        pub key_vector: [libc::c_char; 32],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct XErrorEvent {
        pub type_0: libc::c_int,
        pub display: *mut Display,
        pub resourceid: XID,
        pub serial: libc::c_ulong,
        pub error_code: libc::c_uchar,
        pub request_code: libc::c_uchar,
        pub minor_code: libc::c_uchar,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct XMappingEvent {
        pub type_0: libc::c_int,
        pub serial: libc::c_ulong,
        pub send_event: libc::c_int,
        pub display: *mut Display,
        pub window: Window,
        pub request: libc::c_int,
        pub first_keycode: libc::c_int,
        pub count: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct XClientMessageEvent {
        pub type_0: libc::c_int,
        pub serial: libc::c_ulong,
        pub send_event: libc::c_int,
        pub display: *mut Display,
        pub window: Window,
        pub message_type: Atom,
        pub format: libc::c_int,
        pub data: ClientMessageData,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub union ClientMessageData {
        pub b: [libc::c_char; 20],
        pub s: [libc::c_short; 10],
        pub l: [libc::c_long; 5],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct XColormapEvent {
        pub type_0: libc::c_int,
        pub serial: libc::c_ulong,
        pub send_event: libc::c_int,
        pub display: *mut Display,
        pub window: Window,
        pub colormap: Colormap,
        pub new: libc::c_int,
        pub state: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct XSelectionEvent {
        pub type_0: libc::c_int,
        pub serial: libc::c_ulong,
        pub send_event: libc::c_int,
        pub display: *mut Display,
        pub requestor: Window,
        pub selection: Atom,
        pub target: Atom,
        pub property: Atom,
        pub time: Time,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct XSelectionRequestEvent {
        pub type_0: libc::c_int,
        pub serial: libc::c_ulong,
        pub send_event: libc::c_int,
        pub display: *mut Display,
        pub owner: Window,
        pub requestor: Window,
        pub selection: Atom,
        pub target: Atom,
        pub property: Atom,
        pub time: Time,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct XSelectionClearEvent {
        pub type_0: libc::c_int,
        pub serial: libc::c_ulong,
        pub send_event: libc::c_int,
        pub display: *mut Display,
        pub window: Window,
        pub selection: Atom,
        pub time: Time,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct XPropertyEvent {
        pub type_0: libc::c_int,
        pub serial: libc::c_ulong,
        pub send_event: libc::c_int,
        pub display: *mut Display,
        pub window: Window,
        pub atom: Atom,
        pub time: Time,
        pub state: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct XCirculateRequestEvent {
        pub type_0: libc::c_int,
        pub serial: libc::c_ulong,
        pub send_event: libc::c_int,
        pub display: *mut Display,
        pub parent: Window,
        pub window: Window,
        pub place: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct XCirculateEvent {
        pub type_0: libc::c_int,
        pub serial: libc::c_ulong,
        pub send_event: libc::c_int,
        pub display: *mut Display,
        pub event: Window,
        pub window: Window,
        pub place: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct XConfigureRequestEvent {
        pub type_0: libc::c_int,
        pub serial: libc::c_ulong,
        pub send_event: libc::c_int,
        pub display: *mut Display,
        pub parent: Window,
        pub window: Window,
        pub x: libc::c_int,
        pub y: libc::c_int,
        pub width: libc::c_int,
        pub height: libc::c_int,
        pub border_width: libc::c_int,
        pub above: Window,
        pub detail: libc::c_int,
        pub value_mask: libc::c_ulong,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct XResizeRequestEvent {
        pub type_0: libc::c_int,
        pub serial: libc::c_ulong,
        pub send_event: libc::c_int,
        pub display: *mut Display,
        pub window: Window,
        pub width: libc::c_int,
        pub height: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct XGravityEvent {
        pub type_0: libc::c_int,
        pub serial: libc::c_ulong,
        pub send_event: libc::c_int,
        pub display: *mut Display,
        pub event: Window,
        pub window: Window,
        pub x: libc::c_int,
        pub y: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct XConfigureEvent {
        pub type_0: libc::c_int,
        pub serial: libc::c_ulong,
        pub send_event: libc::c_int,
        pub display: *mut Display,
        pub event: Window,
        pub window: Window,
        pub x: libc::c_int,
        pub y: libc::c_int,
        pub width: libc::c_int,
        pub height: libc::c_int,
        pub border_width: libc::c_int,
        pub above: Window,
        pub override_redirect: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct XReparentEvent {
        pub type_0: libc::c_int,
        pub serial: libc::c_ulong,
        pub send_event: libc::c_int,
        pub display: *mut Display,
        pub event: Window,
        pub window: Window,
        pub parent: Window,
        pub x: libc::c_int,
        pub y: libc::c_int,
        pub override_redirect: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct XMapRequestEvent {
        pub type_0: libc::c_int,
        pub serial: libc::c_ulong,
        pub send_event: libc::c_int,
        pub display: *mut Display,
        pub parent: Window,
        pub window: Window,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct XMapEvent {
        pub type_0: libc::c_int,
        pub serial: libc::c_ulong,
        pub send_event: libc::c_int,
        pub display: *mut Display,
        pub event: Window,
        pub window: Window,
        pub override_redirect: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct XUnmapEvent {
        pub type_0: libc::c_int,
        pub serial: libc::c_ulong,
        pub send_event: libc::c_int,
        pub display: *mut Display,
        pub event: Window,
        pub window: Window,
        pub from_configure: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct XDestroyWindowEvent {
        pub type_0: libc::c_int,
        pub serial: libc::c_ulong,
        pub send_event: libc::c_int,
        pub display: *mut Display,
        pub event: Window,
        pub window: Window,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct XCreateWindowEvent {
        pub type_0: libc::c_int,
        pub serial: libc::c_ulong,
        pub send_event: libc::c_int,
        pub display: *mut Display,
        pub parent: Window,
        pub window: Window,
        pub x: libc::c_int,
        pub y: libc::c_int,
        pub width: libc::c_int,
        pub height: libc::c_int,
        pub border_width: libc::c_int,
        pub override_redirect: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct XVisibilityEvent {
        pub type_0: libc::c_int,
        pub serial: libc::c_ulong,
        pub send_event: libc::c_int,
        pub display: *mut Display,
        pub window: Window,
        pub state: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct XNoExposeEvent {
        pub type_0: libc::c_int,
        pub serial: libc::c_ulong,
        pub send_event: libc::c_int,
        pub display: *mut Display,
        pub drawable: Drawable,
        pub major_code: libc::c_int,
        pub minor_code: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct XGraphicsExposeEvent {
        pub type_0: libc::c_int,
        pub serial: libc::c_ulong,
        pub send_event: libc::c_int,
        pub display: *mut Display,
        pub drawable: Drawable,
        pub x: libc::c_int,
        pub y: libc::c_int,
        pub width: libc::c_int,
        pub height: libc::c_int,
        pub count: libc::c_int,
        pub major_code: libc::c_int,
        pub minor_code: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct XExposeEvent {
        pub type_0: libc::c_int,
        pub serial: libc::c_ulong,
        pub send_event: libc::c_int,
        pub display: *mut Display,
        pub window: Window,
        pub x: libc::c_int,
        pub y: libc::c_int,
        pub width: libc::c_int,
        pub height: libc::c_int,
        pub count: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct XFocusChangeEvent {
        pub type_0: libc::c_int,
        pub serial: libc::c_ulong,
        pub send_event: libc::c_int,
        pub display: *mut Display,
        pub window: Window,
        pub mode: libc::c_int,
        pub detail: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct XCrossingEvent {
        pub type_0: libc::c_int,
        pub serial: libc::c_ulong,
        pub send_event: libc::c_int,
        pub display: *mut Display,
        pub window: Window,
        pub root: Window,
        pub subwindow: Window,
        pub time: Time,
        pub x: libc::c_int,
        pub y: libc::c_int,
        pub x_root: libc::c_int,
        pub y_root: libc::c_int,
        pub mode: libc::c_int,
        pub detail: libc::c_int,
        pub same_screen: libc::c_int,
        pub focus: libc::c_int,
        pub state: libc::c_uint,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct XMotionEvent {
        pub type_0: libc::c_int,
        pub serial: libc::c_ulong,
        pub send_event: libc::c_int,
        pub display: *mut Display,
        pub window: Window,
        pub root: Window,
        pub subwindow: Window,
        pub time: Time,
        pub x: libc::c_int,
        pub y: libc::c_int,
        pub x_root: libc::c_int,
        pub y_root: libc::c_int,
        pub state: libc::c_uint,
        pub is_hint: libc::c_char,
        pub same_screen: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct XButtonEvent {
        pub type_0: libc::c_int,
        pub serial: libc::c_ulong,
        pub send_event: libc::c_int,
        pub display: *mut Display,
        pub window: Window,
        pub root: Window,
        pub subwindow: Window,
        pub time: Time,
        pub x: libc::c_int,
        pub y: libc::c_int,
        pub x_root: libc::c_int,
        pub y_root: libc::c_int,
        pub state: libc::c_uint,
        pub button: libc::c_uint,
        pub same_screen: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct XKeyEvent {
        pub type_0: libc::c_int,
        pub serial: libc::c_ulong,
        pub send_event: libc::c_int,
        pub display: *mut Display,
        pub window: Window,
        pub root: Window,
        pub subwindow: Window,
        pub time: Time,
        pub x: libc::c_int,
        pub y: libc::c_int,
        pub x_root: libc::c_int,
        pub y_root: libc::c_int,
        pub state: libc::c_uint,
        pub keycode: libc::c_uint,
        pub same_screen: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct XAnyEvent {
        pub type_0: libc::c_int,
        pub serial: libc::c_ulong,
        pub send_event: libc::c_int,
        pub display: *mut Display,
        pub window: Window,
    }
    pub type XPointer = *mut libc::c_char;
    pub type XErrorHandler =
        Option<unsafe extern "C" fn(_: *mut Display, _: *mut XErrorEvent) -> libc::c_int>;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct XWindowAttributes {
        pub x: libc::c_int,
        pub y: libc::c_int,
        pub width: libc::c_int,
        pub height: libc::c_int,
        pub border_width: libc::c_int,
        pub depth: libc::c_int,
        pub visual: *mut Visual,
        pub root: Window,
        pub class: libc::c_int,
        pub bit_gravity: libc::c_int,
        pub win_gravity: libc::c_int,
        pub backing_store: libc::c_int,
        pub backing_planes: libc::c_ulong,
        pub backing_pixel: libc::c_ulong,
        pub save_under: libc::c_int,
        pub colormap: Colormap,
        pub map_installed: libc::c_int,
        pub map_state: libc::c_int,
        pub all_event_masks: libc::c_long,
        pub your_event_mask: libc::c_long,
        pub do_not_propagate_mask: libc::c_long,
        pub override_redirect: libc::c_int,
        pub screen: *mut Screen,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct Screen {
        pub ext_data: *mut XExtData,
        pub display: *mut _XDisplay,
        pub root: Window,
        pub width: libc::c_int,
        pub height: libc::c_int,
        pub mwidth: libc::c_int,
        pub mheight: libc::c_int,
        pub ndepths: libc::c_int,
        pub depths: *mut Depth,
        pub root_depth: libc::c_int,
        pub root_visual: *mut Visual,
        pub default_gc: GC,
        pub cmap: Colormap,
        pub white_pixel: libc::c_ulong,
        pub black_pixel: libc::c_ulong,
        pub max_maps: libc::c_int,
        pub min_maps: libc::c_int,
        pub backing_store: libc::c_int,
        pub save_unders: libc::c_int,
        pub root_input_mask: libc::c_long,
    }
    pub type GC = *mut _XGC;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct Visual {
        pub ext_data: *mut XExtData,
        pub visualid: VisualID,
        pub class: libc::c_int,
        pub red_mask: libc::c_ulong,
        pub green_mask: libc::c_ulong,
        pub blue_mask: libc::c_ulong,
        pub bits_per_rgb: libc::c_int,
        pub map_entries: libc::c_int,
    }
    pub type XExtData = _XExtData;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _XExtData {
        pub number: libc::c_int,
        pub next: *mut _XExtData,
        pub free_private: Option<unsafe extern "C" fn(_: *mut _XExtData) -> libc::c_int>,
        pub private_data: XPointer,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct Depth {
        pub depth: libc::c_int,
        pub nvisuals: libc::c_int,
        pub visuals: *mut Visual,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct XSetWindowAttributes {
        pub background_pixmap: Pixmap,
        pub background_pixel: libc::c_ulong,
        pub border_pixmap: Pixmap,
        pub border_pixel: libc::c_ulong,
        pub bit_gravity: libc::c_int,
        pub win_gravity: libc::c_int,
        pub backing_store: libc::c_int,
        pub backing_planes: libc::c_ulong,
        pub backing_pixel: libc::c_ulong,
        pub save_under: libc::c_int,
        pub event_mask: libc::c_long,
        pub do_not_propagate_mask: libc::c_long,
        pub override_redirect: libc::c_int,
        pub colormap: Colormap,
        pub cursor: Cursor,
    }
    pub type _XPrivDisplay = *mut C2RustUnnamed_3;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct C2RustUnnamed_3 {
        pub ext_data: *mut XExtData,
        pub private1: *mut _XPrivate,
        pub fd: libc::c_int,
        pub private2: libc::c_int,
        pub proto_major_version: libc::c_int,
        pub proto_minor_version: libc::c_int,
        pub vendor: *mut libc::c_char,
        pub private3: XID,
        pub private4: XID,
        pub private5: XID,
        pub private6: libc::c_int,
        pub resource_alloc: Option<unsafe extern "C" fn(_: *mut _XDisplay) -> XID>,
        pub byte_order: libc::c_int,
        pub bitmap_unit: libc::c_int,
        pub bitmap_pad: libc::c_int,
        pub bitmap_bit_order: libc::c_int,
        pub nformats: libc::c_int,
        pub pixmap_format: *mut ScreenFormat,
        pub private8: libc::c_int,
        pub release: libc::c_int,
        pub private9: *mut _XPrivate,
        pub private10: *mut _XPrivate,
        pub qlen: libc::c_int,
        pub last_request_read: libc::c_ulong,
        pub request: libc::c_ulong,
        pub private11: XPointer,
        pub private12: XPointer,
        pub private13: XPointer,
        pub private14: XPointer,
        pub max_request_size: libc::c_uint,
        pub db: *mut _XrmHashBucketRec,
        pub private15: Option<unsafe extern "C" fn(_: *mut _XDisplay) -> libc::c_int>,
        pub display_name: *mut libc::c_char,
        pub default_screen: libc::c_int,
        pub nscreens: libc::c_int,
        pub screens: *mut Screen,
        pub motion_buffer: libc::c_ulong,
        pub private16: libc::c_ulong,
        pub min_keycode: libc::c_int,
        pub max_keycode: libc::c_int,
        pub private17: XPointer,
        pub private18: XPointer,
        pub private19: libc::c_int,
        pub xdefaults: *mut libc::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct ScreenFormat {
        pub ext_data: *mut XExtData,
        pub depth: libc::c_int,
        pub bits_per_pixel: libc::c_int,
        pub scanline_pad: libc::c_int,
    }

    use super::X_h::{Atom, Colormap, Cursor, Drawable, Pixmap, Time, VisualID, Window, XID};
    pub type _XDisplay = ();
    pub type _XGC = ();
    pub type _XrmHashBucketRec = ();
    pub type _XPrivate = ();
}

pub mod X_h {
    pub type Colormap = XID;
    pub type XID = libc::c_ulong;
    pub type Window = XID;
    pub type Atom = libc::c_ulong;
    pub type Time = libc::c_ulong;
    pub type Drawable = XID;
    pub type Pixmap = XID;
    pub type KeySym = XID;
    pub type KeyCode = libc::c_uchar;
    pub type VisualID = libc::c_ulong;
    pub type Cursor = XID;
    pub const AllocNone: libc::c_int = 0 as libc::c_int;
    pub const StructureNotifyMask: libc::c_long = (1 as libc::c_long) << 17 as libc::c_int;
    pub const SubstructureNotifyMask: libc::c_long = (1 as libc::c_long) << 19 as libc::c_int;
    pub const SubstructureRedirectMask: libc::c_long = (1 as libc::c_long) << 20 as libc::c_int;
    pub const NoEventMask: libc::c_long = 0 as libc::c_long;
    pub const KeyPressMask: libc::c_long = (1 as libc::c_long) << 0 as libc::c_int;
    pub const KeyReleaseMask: libc::c_long = (1 as libc::c_long) << 1 as libc::c_int;
    pub const PointerMotionMask: libc::c_long = (1 as libc::c_long) << 6 as libc::c_int;
    pub const PointerMotionHintMask: libc::c_long = (1 as libc::c_long) << 7 as libc::c_int;
    pub const Button1MotionMask: libc::c_long = (1 as libc::c_long) << 8 as libc::c_int;
    pub const Button2MotionMask: libc::c_long = (1 as libc::c_long) << 9 as libc::c_int;
    pub const Button3MotionMask: libc::c_long = (1 as libc::c_long) << 10 as libc::c_int;
    pub const Button4MotionMask: libc::c_long = (1 as libc::c_long) << 11 as libc::c_int;
    pub const Button5MotionMask: libc::c_long = (1 as libc::c_long) << 12 as libc::c_int;
    pub const ButtonMotionMask: libc::c_long = (1 as libc::c_long) << 13 as libc::c_int;
    pub const KeymapStateMask: libc::c_long = (1 as libc::c_long) << 14 as libc::c_int;

    pub const GrabModeAsync: libc::c_int = 1 as libc::c_int;

    pub const ButtonPressMask: libc::c_long = (1 as libc::c_long) << 2 as libc::c_int;
    pub const ButtonReleaseMask: libc::c_long = (1 as libc::c_long) << 3 as libc::c_int;
    pub const ExposureMask: libc::c_long = (1 as libc::c_long) << 15 as libc::c_int;
    pub const FocusChangeMask: libc::c_long = (1 as libc::c_long) << 21 as libc::c_int;
    pub const VisibilityChangeMask: libc::c_long = (1 as libc::c_long) << 16 as libc::c_int;
    pub const EnterWindowMask: libc::c_long = (1 as libc::c_long) << 4 as libc::c_int;
    pub const LeaveWindowMask: libc::c_long = (1 as libc::c_long) << 5 as libc::c_int;
    pub const PropertyChangeMask: libc::c_long = (1 as libc::c_long) << 22 as libc::c_int;
    pub const InputOutput: libc::c_int = 1 as libc::c_int;
    pub const CWBorderPixel: libc::c_long = (1 as libc::c_long) << 3 as libc::c_int;
    pub const CWColormap: libc::c_long = (1 as libc::c_long) << 13 as libc::c_int;
    pub const CWEventMask: libc::c_long = (1 as libc::c_long) << 11 as libc::c_int;
    pub const StaticGravity: libc::c_int = 10 as libc::c_int;
    pub const PropModeReplace: libc::c_int = 0 as libc::c_int;
    pub const Success: libc::c_int = 0 as libc::c_int;
    pub const IsViewable: libc::c_int = 2 as libc::c_int;
    pub const ShiftMask: libc::c_int = (1 as libc::c_int) << 0 as libc::c_int;
    pub const ControlMask: libc::c_int = (1 as libc::c_int) << 2 as libc::c_int;
    pub const Mod1Mask: libc::c_int = (1 as libc::c_int) << 3 as libc::c_int;
    pub const Mod4Mask: libc::c_int = (1 as libc::c_int) << 6 as libc::c_int;

    pub const AnyPropertyType: libc::c_ulong = 0 as libc::c_ulong;
    pub const CurrentTime: libc::c_long = 0 as libc::c_long;

    pub const PropertyNotify: libc::c_int = 28 as libc::c_int;
    pub const SelectionRequest: libc::c_int = 30 as libc::c_int;
    pub const SelectionNotify: libc::c_int = 31 as libc::c_int;
    pub const ClientMessage: libc::c_int = 33 as libc::c_int;

    pub const PropertyNewValue: libc::c_int = 0 as libc::c_int;
}

pub mod Xutil_h {
    pub type XComposeStatus = _XComposeStatus;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _XComposeStatus {
        pub compose_ptr: XPointer,
        pub chars_matched: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct XClassHint {
        pub res_name: *mut libc::c_char,
        pub res_class: *mut libc::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct XWMHints {
        pub flags: libc::c_long,
        pub input: libc::c_int,
        pub initial_state: libc::c_int,
        pub icon_pixmap: Pixmap,
        pub icon_window: Window,
        pub icon_x: libc::c_int,
        pub icon_y: libc::c_int,
        pub icon_mask: Pixmap,
        pub window_group: XID,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct XSizeHints {
        pub flags: libc::c_long,
        pub x: libc::c_int,
        pub y: libc::c_int,
        pub width: libc::c_int,
        pub height: libc::c_int,
        pub min_width: libc::c_int,
        pub min_height: libc::c_int,
        pub max_width: libc::c_int,
        pub max_height: libc::c_int,
        pub width_inc: libc::c_int,
        pub height_inc: libc::c_int,
        pub min_aspect: C2RustUnnamed_2,
        pub max_aspect: C2RustUnnamed_2,
        pub base_width: libc::c_int,
        pub base_height: libc::c_int,
        pub win_gravity: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct C2RustUnnamed_2 {
        pub x: libc::c_int,
        pub y: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct XVisualInfo {
        pub visual: *mut Visual,
        pub visualid: VisualID,
        pub screen: libc::c_int,
        pub depth: libc::c_int,
        pub class: libc::c_int,
        pub red_mask: libc::c_ulong,
        pub green_mask: libc::c_ulong,
        pub blue_mask: libc::c_ulong,
        pub colormap_size: libc::c_int,
        pub bits_per_rgb: libc::c_int,
    }
    pub const PMinSize: libc::c_long = (1 as libc::c_long) << 4 as libc::c_int;
    pub const PMaxSize: libc::c_long = (1 as libc::c_long) << 5 as libc::c_int;
    pub const PWinGravity: libc::c_long = (1 as libc::c_long) << 9 as libc::c_int;
    pub const IconicState: libc::c_int = 3 as libc::c_int;
    pub const WithdrawnState: libc::c_int = 0 as libc::c_int;
    pub const NormalState: libc::c_int = 1 as libc::c_int;
    use super::X_h::{Pixmap, VisualID, Window, XID};
    use super::Xlib_h::{Visual, XPointer};
}

pub mod Xresource_h {
    pub type XrmDatabase = *mut _XrmHashBucketRec;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct XrmValue {
        pub size: libc::c_uint,
        pub addr: XPointer,
    }
    use super::Xlib_h::{XPointer, _XrmHashBucketRec};
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XColor {
    pub pixel: libc::c_ulong,
    pub red: libc::c_ushort,
    pub green: libc::c_ushort,
    pub blue: libc::c_ushort,
    pub flags: libc::c_char,
    pub pad: libc::c_char,
}

// See https://tronche.com/gui/x/xlib/appendix/b/
pub const XC_crosshair: libc::c_ushort = 34;
pub const XC_fleur: libc::c_ushort = 52;
pub const XC_hand2: libc::c_ushort = 60;
pub const XC_left_ptr: libc::c_ushort = 68;
pub const XC_pirate: libc::c_ushort = 88;
pub const XC_question_arrow: libc::c_ushort = 92;
pub const XC_sb_h_double_arrow: libc::c_ushort = 108;
pub const XC_sb_v_double_arrow: libc::c_ushort = 116;
pub const XC_top_left_corner: libc::c_ushort = 134;
pub const XC_top_right_corner: libc::c_ushort = 136;
pub const XC_watch: libc::c_ushort = 150;
pub const XC_xterm: libc::c_ushort = 152;

macro_rules! declare_atoms {
    ($($name:ident: $atom:literal),*$(,)?) => {
        #[derive(Clone, Default)]
        pub struct X11Extensions {
            $(pub $name: Atom,)*
        }
        impl X11Extensions {
            pub unsafe fn load(libx11: &mut LibX11, display: *mut Display) -> Self {
                Self {
                    $($name: {
                        let atom = std::ffi::CString::new($atom).unwrap();
                        (libx11.XInternAtom)(display, atom.as_ptr(), false as _)
                    },)*
                }
            }
        }
    }
}

declare_atoms!(
    utf8_string: "UTF8_STRING",
    wm_protocols: "WM_PROTOCOLS",
    wm_delete_window: "WM_DELETE_WINDOW",
    _wm_state: "WM_STATE",
    net_wm_name: "_NET_WM_NAME",
    net_wm_icon_name: "_NET_WM_ICON_NAME",
    net_wm_icon: "_NET_WM_ICON",
    cardinal: "CARDINAL",
    // clipboard
    clipboard: "CLIPBOARD",
    xsel_data: "XSEL_DATA",
    incr: "INCR",
    // drag_n_drop related
    xdnd_action_copy: "XdndActionCopy",
    xdnd_aware: "XdndAware",
    xdnd_drop: "XdndDrop",
    xdnd_enter: "XdndEnter",
    xdnd_finished: "XdndFinished",
    xdnd_position: "XdndPosition",
    xdnd_selection: "XdndSelection",
    xdnd_status: "XdndStatus",
    xdnd_type_list: "XdndTypeList",
);

use core::ffi::{c_char, c_int, c_long, c_uchar, c_uint, c_ulong, c_ushort, c_void};
crate::declare_module!(
    LibX11,
    "libX11.so",
    "libX11.so.6",
    ...
    ...
    pub fn XSetWMNormalHints(*mut Display, Window, *mut XSizeHints),
    pub fn XAllocSizeHints() -> *mut XSizeHints,
    pub fn XAllocClassHint() -> *mut XClassHint,
    pub fn XSetClassHint(*mut Display, Window, *mut XClassHint),
    pub fn Xutf8SetWMProperties(*mut Display, Window, *const c_char, *const c_char, *mut *mut c_char, c_int, *mut XSizeHints, *mut XWMHints, *mut XClassHint),
    pub fn XLookupString(*mut XKeyEvent, *mut c_char, c_int, *mut KeySym, *mut XComposeStatus) -> c_int,
    pub fn XInitThreads() -> c_int,
    pub fn XrmInitialize(),
    pub fn XOpenDisplay(*const c_char) -> *mut Display,
    pub fn XResourceManagerString(*mut Display) -> *mut c_char,
    pub fn XInternAtom(*mut Display, *const c_char, c_int) -> Atom,
    pub fn XGetAtomName(*mut Display, Atom) -> *mut c_char,
    pub fn XCreateColormap (*mut Display, Window, *mut Visual, c_int) -> Colormap,
    pub fn XCreateWindow(*mut Display, Window, c_int, c_int, c_uint, c_uint, c_uint, c_int, c_uint, *mut Visual, c_ulong, *mut XSetWindowAttributes) -> Window,
    pub fn XSetWMProtocols (*mut Display, Window, *mut Atom, c_int) -> c_int,
    pub fn XChangeProperty(*mut Display, Window, Atom, Atom, c_int, c_int, *const c_uchar, c_int) -> c_int,
    pub fn XSync(*mut Display, c_int) -> c_int,
    pub fn XSetErrorHandler(XErrorHandler) -> XErrorHandler,
    pub fn XGetWindowAttributes(*mut Display, Window, *mut XWindowAttributes) -> c_int,
    pub fn XMapWindow(*mut Display, Window) -> c_int,
    pub fn XLowerWindow(*mut Display, Window) -> c_int,
    pub fn XRaiseWindow(*mut Display, Window) -> c_int,
    pub fn XResizeWindow(*mut Display, Window, c_int, c_int) -> c_int,
    pub fn XMoveWindow(*mut Display, Window, c_int, c_int) -> c_int,
    pub fn XPending(*mut Display) -> c_int,
    pub fn XNextEvent(*mut Display, *mut XEvent) -> c_int,
    pub fn XGetKeyboardMapping(*mut Display, KeyCode, c_int, *mut c_int) -> *mut KeySym,
    pub fn XGetWindowProperty(*mut Display, Window, Atom, c_long, c_long, c_int, Atom, *mut Atom, *mut c_int, *mut c_ulong, *mut c_ulong, *mut *mut c_uchar) -> c_int,
    pub fn XDeleteProperty(*mut Display, Window, Atom) -> c_int,
    pub fn XFree(*mut c_void) -> c_int,
    pub fn XUnmapWindow(*mut Display, Window) -> c_int,
    pub fn XDestroyWindow(*mut Display, Window) -> c_int,
    pub fn XFreeColormap(*mut Display, Colormap) -> c_int,
    pub fn XFlush(*mut Display) -> c_int,
    pub fn XCloseDisplay(*mut Display) -> c_int,
    pub fn XGrabPointer(*mut Display, Window, c_int, c_uint, c_int, c_int, Window, Cursor, Time) -> c_int,
    pub fn XUngrabPointer(*mut Display, Time) -> c_int,
    pub fn XSendEvent(*mut Display, Window, c_int, c_long, *mut XEvent) -> c_int,
    pub fn XrmGetResource(XrmDatabase, *const c_char, *const c_char, *mut *mut c_char, *mut XrmValue) -> c_int,
    pub fn XrmDestroyDatabase(XrmDatabase),
    pub fn XrmGetStringDatabase(*const c_char) -> XrmDatabase,
    pub fn XkbSetDetectableAutoRepeat(*mut Display, c_int, *mut c_int) -> c_int,
    pub fn XQueryExtension(*mut Display, *const c_char, *mut c_int, *mut c_int, *mut c_int) -> c_int,
    pub fn XConvertSelection(*mut Display, Atom, Atom, Atom, Window, Time) -> c_int,
    pub fn XSetSelectionOwner(*mut Display, Atom, Window, Time) -> c_int,
    pub fn XCreateFontCursor(*mut Display, c_ushort) -> Cursor,
    pub fn XCreateBitmapFromData(*mut Display, Drawable, *const c_char, c_uint, c_uint) -> Pixmap,
    pub fn XCreatePixmapCursor(*mut Display, Pixmap, Pixmap, *mut XColor, *mut XColor, c_uint, c_uint) -> Cursor,
    pub fn XFreePixmap(*mut Display, Pixmap) -> c_int,
    pub fn XDefineCursor(*mut Display, Window, Cursor) -> c_int,
    ...
    ...
    pub extensions: X11Extensions,
);

impl LibX11 {
    pub unsafe fn load_extensions(&mut self, display: *mut Display) {
        self.extensions = X11Extensions::load(self, display);
    }
}

crate::declare_module!(
    LibXkbCommon,
    "libxkbcommon.so",
    "libxkbcommon.so.0",
    "libxkbcommon.so.0.0.0",
    "libxkbcommon.so.0.0.0.0",
    ...
    ...
    pub fn xkb_keysym_to_utf32(u32) -> u32,
    ...
    ...
);
