pub use bits_dlfcn_h::{RTLD_GLOBAL, RTLD_LAZY};
pub use dlfcn_h::{dlopen, dlsym};
pub use stdlib_h::atof;
pub use string_h::{memset, strcmp, strlen, strstr};
pub use XKBlib_h::XkbSetDetectableAutoRepeat;
pub use X_h::{
    AllocNone, Atom, Button1MotionMask, Button2MotionMask, Button3MotionMask, Button4MotionMask,
    Button5MotionMask, ButtonMotionMask, ButtonPressMask, ButtonReleaseMask, CWBorderPixel,
    CWColormap, CWEventMask, Colormap, ControlMask, Cursor, EnterWindowMask, ExposureMask,
    FocusChangeMask, GrabModeAsync, InputOutput, IsViewable, KeyCode, KeyPressMask, KeyReleaseMask,
    KeySym, KeymapStateMask, LeaveWindowMask, Mod1Mask, Mod4Mask, Pixmap, PointerMotionHintMask,
    PointerMotionMask, PropModeReplace, PropertyChangeMask, PropertyNewValue, ShiftMask,
    StaticGravity, StructureNotifyMask, Success, VisibilityChangeMask, Window, XID,
};
pub use Xlib_h::{
    ClientMessageData, Display, Screen, Visual, XChangeProperty, XClientMessageEvent,
    XCloseDisplay, XCreateColormap, XCreateWindow, XDestroyWindow, XErrorEvent, XErrorHandler,
    XEvent, XFlush, XFree, XFreeColormap, XGetKeyboardMapping, XGetWindowAttributes,
    XGetWindowProperty, XGrabPointer, XInitThreads, XInternAtom, XKeyEvent, XLowerWindow,
    XMapWindow, XNextEvent, XOpenDisplay, XPending, XPointer, XRaiseWindow, XResourceManagerString,
    XSelectionEvent, XSelectionRequestEvent, XSetErrorHandler, XSetWMProtocols,
    XSetWindowAttributes, XSync, XUngrabPointer, XUnmapWindow, XWindowAttributes, XrmInitialize,
    _XEvent, _XPrivDisplay, _XrmHashBucketRec,
};
pub use Xmd_h::CARD32;
pub use Xresource_h::{
    XrmDatabase, XrmDestroyDatabase, XrmGetResource, XrmGetStringDatabase, XrmValue,
};
pub use Xutil_h::{
    IconicState, NormalState, PWinGravity, WithdrawnState, XAllocSizeHints, XClassHint,
    XComposeStatus, XLookupString, XSetWMNormalHints, XSizeHints, XVisualInfo, XWMHints,
    Xutf8SetWMProperties,
};
pub type __GLXcontext = ();
pub type __GLXFBConfig = ();

pub mod Xlib_h {
    pub type Display = _XDisplay;
    pub type XEvent = _XEvent;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub union _XEvent {
        pub type_0: cty::c_int,
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
        pub pad: [cty::c_long; 24],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct XGenericEventCookie {
        pub type_0: cty::c_int,
        pub serial: cty::c_ulong,
        pub send_event: cty::c_int,
        pub display: *mut Display,
        pub extension: cty::c_int,
        pub evtype: cty::c_int,
        pub cookie: cty::c_uint,
        pub data: *mut cty::c_void,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct XGenericEvent {
        pub type_0: cty::c_int,
        pub serial: cty::c_ulong,
        pub send_event: cty::c_int,
        pub display: *mut Display,
        pub extension: cty::c_int,
        pub evtype: cty::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct XKeymapEvent {
        pub type_0: cty::c_int,
        pub serial: cty::c_ulong,
        pub send_event: cty::c_int,
        pub display: *mut Display,
        pub window: Window,
        pub key_vector: [cty::c_char; 32],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct XErrorEvent {
        pub type_0: cty::c_int,
        pub display: *mut Display,
        pub resourceid: XID,
        pub serial: cty::c_ulong,
        pub error_code: cty::c_uchar,
        pub request_code: cty::c_uchar,
        pub minor_code: cty::c_uchar,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct XMappingEvent {
        pub type_0: cty::c_int,
        pub serial: cty::c_ulong,
        pub send_event: cty::c_int,
        pub display: *mut Display,
        pub window: Window,
        pub request: cty::c_int,
        pub first_keycode: cty::c_int,
        pub count: cty::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct XClientMessageEvent {
        pub type_0: cty::c_int,
        pub serial: cty::c_ulong,
        pub send_event: cty::c_int,
        pub display: *mut Display,
        pub window: Window,
        pub message_type: Atom,
        pub format: cty::c_int,
        pub data: ClientMessageData,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub union ClientMessageData {
        pub b: [cty::c_char; 20],
        pub s: [cty::c_short; 10],
        pub l: [cty::c_long; 5],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct XColormapEvent {
        pub type_0: cty::c_int,
        pub serial: cty::c_ulong,
        pub send_event: cty::c_int,
        pub display: *mut Display,
        pub window: Window,
        pub colormap: Colormap,
        pub new: cty::c_int,
        pub state: cty::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct XSelectionEvent {
        pub type_0: cty::c_int,
        pub serial: cty::c_ulong,
        pub send_event: cty::c_int,
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
        pub type_0: cty::c_int,
        pub serial: cty::c_ulong,
        pub send_event: cty::c_int,
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
        pub type_0: cty::c_int,
        pub serial: cty::c_ulong,
        pub send_event: cty::c_int,
        pub display: *mut Display,
        pub window: Window,
        pub selection: Atom,
        pub time: Time,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct XPropertyEvent {
        pub type_0: cty::c_int,
        pub serial: cty::c_ulong,
        pub send_event: cty::c_int,
        pub display: *mut Display,
        pub window: Window,
        pub atom: Atom,
        pub time: Time,
        pub state: cty::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct XCirculateRequestEvent {
        pub type_0: cty::c_int,
        pub serial: cty::c_ulong,
        pub send_event: cty::c_int,
        pub display: *mut Display,
        pub parent: Window,
        pub window: Window,
        pub place: cty::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct XCirculateEvent {
        pub type_0: cty::c_int,
        pub serial: cty::c_ulong,
        pub send_event: cty::c_int,
        pub display: *mut Display,
        pub event: Window,
        pub window: Window,
        pub place: cty::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct XConfigureRequestEvent {
        pub type_0: cty::c_int,
        pub serial: cty::c_ulong,
        pub send_event: cty::c_int,
        pub display: *mut Display,
        pub parent: Window,
        pub window: Window,
        pub x: cty::c_int,
        pub y: cty::c_int,
        pub width: cty::c_int,
        pub height: cty::c_int,
        pub border_width: cty::c_int,
        pub above: Window,
        pub detail: cty::c_int,
        pub value_mask: cty::c_ulong,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct XResizeRequestEvent {
        pub type_0: cty::c_int,
        pub serial: cty::c_ulong,
        pub send_event: cty::c_int,
        pub display: *mut Display,
        pub window: Window,
        pub width: cty::c_int,
        pub height: cty::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct XGravityEvent {
        pub type_0: cty::c_int,
        pub serial: cty::c_ulong,
        pub send_event: cty::c_int,
        pub display: *mut Display,
        pub event: Window,
        pub window: Window,
        pub x: cty::c_int,
        pub y: cty::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct XConfigureEvent {
        pub type_0: cty::c_int,
        pub serial: cty::c_ulong,
        pub send_event: cty::c_int,
        pub display: *mut Display,
        pub event: Window,
        pub window: Window,
        pub x: cty::c_int,
        pub y: cty::c_int,
        pub width: cty::c_int,
        pub height: cty::c_int,
        pub border_width: cty::c_int,
        pub above: Window,
        pub override_redirect: cty::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct XReparentEvent {
        pub type_0: cty::c_int,
        pub serial: cty::c_ulong,
        pub send_event: cty::c_int,
        pub display: *mut Display,
        pub event: Window,
        pub window: Window,
        pub parent: Window,
        pub x: cty::c_int,
        pub y: cty::c_int,
        pub override_redirect: cty::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct XMapRequestEvent {
        pub type_0: cty::c_int,
        pub serial: cty::c_ulong,
        pub send_event: cty::c_int,
        pub display: *mut Display,
        pub parent: Window,
        pub window: Window,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct XMapEvent {
        pub type_0: cty::c_int,
        pub serial: cty::c_ulong,
        pub send_event: cty::c_int,
        pub display: *mut Display,
        pub event: Window,
        pub window: Window,
        pub override_redirect: cty::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct XUnmapEvent {
        pub type_0: cty::c_int,
        pub serial: cty::c_ulong,
        pub send_event: cty::c_int,
        pub display: *mut Display,
        pub event: Window,
        pub window: Window,
        pub from_configure: cty::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct XDestroyWindowEvent {
        pub type_0: cty::c_int,
        pub serial: cty::c_ulong,
        pub send_event: cty::c_int,
        pub display: *mut Display,
        pub event: Window,
        pub window: Window,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct XCreateWindowEvent {
        pub type_0: cty::c_int,
        pub serial: cty::c_ulong,
        pub send_event: cty::c_int,
        pub display: *mut Display,
        pub parent: Window,
        pub window: Window,
        pub x: cty::c_int,
        pub y: cty::c_int,
        pub width: cty::c_int,
        pub height: cty::c_int,
        pub border_width: cty::c_int,
        pub override_redirect: cty::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct XVisibilityEvent {
        pub type_0: cty::c_int,
        pub serial: cty::c_ulong,
        pub send_event: cty::c_int,
        pub display: *mut Display,
        pub window: Window,
        pub state: cty::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct XNoExposeEvent {
        pub type_0: cty::c_int,
        pub serial: cty::c_ulong,
        pub send_event: cty::c_int,
        pub display: *mut Display,
        pub drawable: Drawable,
        pub major_code: cty::c_int,
        pub minor_code: cty::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct XGraphicsExposeEvent {
        pub type_0: cty::c_int,
        pub serial: cty::c_ulong,
        pub send_event: cty::c_int,
        pub display: *mut Display,
        pub drawable: Drawable,
        pub x: cty::c_int,
        pub y: cty::c_int,
        pub width: cty::c_int,
        pub height: cty::c_int,
        pub count: cty::c_int,
        pub major_code: cty::c_int,
        pub minor_code: cty::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct XExposeEvent {
        pub type_0: cty::c_int,
        pub serial: cty::c_ulong,
        pub send_event: cty::c_int,
        pub display: *mut Display,
        pub window: Window,
        pub x: cty::c_int,
        pub y: cty::c_int,
        pub width: cty::c_int,
        pub height: cty::c_int,
        pub count: cty::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct XFocusChangeEvent {
        pub type_0: cty::c_int,
        pub serial: cty::c_ulong,
        pub send_event: cty::c_int,
        pub display: *mut Display,
        pub window: Window,
        pub mode: cty::c_int,
        pub detail: cty::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct XCrossingEvent {
        pub type_0: cty::c_int,
        pub serial: cty::c_ulong,
        pub send_event: cty::c_int,
        pub display: *mut Display,
        pub window: Window,
        pub root: Window,
        pub subwindow: Window,
        pub time: Time,
        pub x: cty::c_int,
        pub y: cty::c_int,
        pub x_root: cty::c_int,
        pub y_root: cty::c_int,
        pub mode: cty::c_int,
        pub detail: cty::c_int,
        pub same_screen: cty::c_int,
        pub focus: cty::c_int,
        pub state: cty::c_uint,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct XMotionEvent {
        pub type_0: cty::c_int,
        pub serial: cty::c_ulong,
        pub send_event: cty::c_int,
        pub display: *mut Display,
        pub window: Window,
        pub root: Window,
        pub subwindow: Window,
        pub time: Time,
        pub x: cty::c_int,
        pub y: cty::c_int,
        pub x_root: cty::c_int,
        pub y_root: cty::c_int,
        pub state: cty::c_uint,
        pub is_hint: cty::c_char,
        pub same_screen: cty::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct XButtonEvent {
        pub type_0: cty::c_int,
        pub serial: cty::c_ulong,
        pub send_event: cty::c_int,
        pub display: *mut Display,
        pub window: Window,
        pub root: Window,
        pub subwindow: Window,
        pub time: Time,
        pub x: cty::c_int,
        pub y: cty::c_int,
        pub x_root: cty::c_int,
        pub y_root: cty::c_int,
        pub state: cty::c_uint,
        pub button: cty::c_uint,
        pub same_screen: cty::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct XKeyEvent {
        pub type_0: cty::c_int,
        pub serial: cty::c_ulong,
        pub send_event: cty::c_int,
        pub display: *mut Display,
        pub window: Window,
        pub root: Window,
        pub subwindow: Window,
        pub time: Time,
        pub x: cty::c_int,
        pub y: cty::c_int,
        pub x_root: cty::c_int,
        pub y_root: cty::c_int,
        pub state: cty::c_uint,
        pub keycode: cty::c_uint,
        pub same_screen: cty::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct XAnyEvent {
        pub type_0: cty::c_int,
        pub serial: cty::c_ulong,
        pub send_event: cty::c_int,
        pub display: *mut Display,
        pub window: Window,
    }
    pub type XPointer = *mut cty::c_char;
    pub type XErrorHandler =
        Option<unsafe extern "C" fn(_: *mut Display, _: *mut XErrorEvent) -> cty::c_int>;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct XWindowAttributes {
        pub x: cty::c_int,
        pub y: cty::c_int,
        pub width: cty::c_int,
        pub height: cty::c_int,
        pub border_width: cty::c_int,
        pub depth: cty::c_int,
        pub visual: *mut Visual,
        pub root: Window,
        pub class: cty::c_int,
        pub bit_gravity: cty::c_int,
        pub win_gravity: cty::c_int,
        pub backing_store: cty::c_int,
        pub backing_planes: cty::c_ulong,
        pub backing_pixel: cty::c_ulong,
        pub save_under: cty::c_int,
        pub colormap: Colormap,
        pub map_installed: cty::c_int,
        pub map_state: cty::c_int,
        pub all_event_masks: cty::c_long,
        pub your_event_mask: cty::c_long,
        pub do_not_propagate_mask: cty::c_long,
        pub override_redirect: cty::c_int,
        pub screen: *mut Screen,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct Screen {
        pub ext_data: *mut XExtData,
        pub display: *mut _XDisplay,
        pub root: Window,
        pub width: cty::c_int,
        pub height: cty::c_int,
        pub mwidth: cty::c_int,
        pub mheight: cty::c_int,
        pub ndepths: cty::c_int,
        pub depths: *mut Depth,
        pub root_depth: cty::c_int,
        pub root_visual: *mut Visual,
        pub default_gc: GC,
        pub cmap: Colormap,
        pub white_pixel: cty::c_ulong,
        pub black_pixel: cty::c_ulong,
        pub max_maps: cty::c_int,
        pub min_maps: cty::c_int,
        pub backing_store: cty::c_int,
        pub save_unders: cty::c_int,
        pub root_input_mask: cty::c_long,
    }
    pub type GC = *mut _XGC;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct Visual {
        pub ext_data: *mut XExtData,
        pub visualid: VisualID,
        pub class: cty::c_int,
        pub red_mask: cty::c_ulong,
        pub green_mask: cty::c_ulong,
        pub blue_mask: cty::c_ulong,
        pub bits_per_rgb: cty::c_int,
        pub map_entries: cty::c_int,
    }
    pub type XExtData = _XExtData;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _XExtData {
        pub number: cty::c_int,
        pub next: *mut _XExtData,
        pub free_private: Option<unsafe extern "C" fn(_: *mut _XExtData) -> cty::c_int>,
        pub private_data: XPointer,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct Depth {
        pub depth: cty::c_int,
        pub nvisuals: cty::c_int,
        pub visuals: *mut Visual,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct XSetWindowAttributes {
        pub background_pixmap: Pixmap,
        pub background_pixel: cty::c_ulong,
        pub border_pixmap: Pixmap,
        pub border_pixel: cty::c_ulong,
        pub bit_gravity: cty::c_int,
        pub win_gravity: cty::c_int,
        pub backing_store: cty::c_int,
        pub backing_planes: cty::c_ulong,
        pub backing_pixel: cty::c_ulong,
        pub save_under: cty::c_int,
        pub event_mask: cty::c_long,
        pub do_not_propagate_mask: cty::c_long,
        pub override_redirect: cty::c_int,
        pub colormap: Colormap,
        pub cursor: Cursor,
    }
    pub type _XPrivDisplay = *mut C2RustUnnamed_3;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct C2RustUnnamed_3 {
        pub ext_data: *mut XExtData,
        pub private1: *mut _XPrivate,
        pub fd: cty::c_int,
        pub private2: cty::c_int,
        pub proto_major_version: cty::c_int,
        pub proto_minor_version: cty::c_int,
        pub vendor: *mut cty::c_char,
        pub private3: XID,
        pub private4: XID,
        pub private5: XID,
        pub private6: cty::c_int,
        pub resource_alloc: Option<unsafe extern "C" fn(_: *mut _XDisplay) -> XID>,
        pub byte_order: cty::c_int,
        pub bitmap_unit: cty::c_int,
        pub bitmap_pad: cty::c_int,
        pub bitmap_bit_order: cty::c_int,
        pub nformats: cty::c_int,
        pub pixmap_format: *mut ScreenFormat,
        pub private8: cty::c_int,
        pub release: cty::c_int,
        pub private9: *mut _XPrivate,
        pub private10: *mut _XPrivate,
        pub qlen: cty::c_int,
        pub last_request_read: cty::c_ulong,
        pub request: cty::c_ulong,
        pub private11: XPointer,
        pub private12: XPointer,
        pub private13: XPointer,
        pub private14: XPointer,
        pub max_request_size: cty::c_uint,
        pub db: *mut _XrmHashBucketRec,
        pub private15: Option<unsafe extern "C" fn(_: *mut _XDisplay) -> cty::c_int>,
        pub display_name: *mut cty::c_char,
        pub default_screen: cty::c_int,
        pub nscreens: cty::c_int,
        pub screens: *mut Screen,
        pub motion_buffer: cty::c_ulong,
        pub private16: cty::c_ulong,
        pub min_keycode: cty::c_int,
        pub max_keycode: cty::c_int,
        pub private17: XPointer,
        pub private18: XPointer,
        pub private19: cty::c_int,
        pub xdefaults: *mut cty::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct ScreenFormat {
        pub ext_data: *mut XExtData,
        pub depth: cty::c_int,
        pub bits_per_pixel: cty::c_int,
        pub scanline_pad: cty::c_int,
    }

    use super::X_h::{
        Atom, Colormap, Cursor, Drawable, KeyCode, KeySym, Pixmap, Time, VisualID, Window, XID,
    };
    pub type _XDisplay = ();
    pub type _XGC = ();
    pub type _XrmHashBucketRec = ();
    pub type _XPrivate = ();

    extern "C" {
        #[no_mangle]
        pub fn XInitThreads() -> cty::c_int;
        #[no_mangle]
        pub fn XrmInitialize();
        #[no_mangle]
        pub fn XOpenDisplay(_: *const cty::c_char) -> *mut Display;
        #[no_mangle]
        pub fn XResourceManagerString(_: *mut Display) -> *mut cty::c_char;
        #[no_mangle]
        pub fn XInternAtom(_: *mut Display, _: *const cty::c_char, _: cty::c_int) -> Atom;
        #[no_mangle]
        pub fn XCreateColormap(
            _: *mut Display,
            _: Window,
            _: *mut Visual,
            _: cty::c_int,
        ) -> Colormap;
        #[no_mangle]
        pub fn XCreateWindow(
            _: *mut Display,
            _: Window,
            _: cty::c_int,
            _: cty::c_int,
            _: cty::c_uint,
            _: cty::c_uint,
            _: cty::c_uint,
            _: cty::c_int,
            _: cty::c_uint,
            _: *mut Visual,
            _: cty::c_ulong,
            _: *mut XSetWindowAttributes,
        ) -> Window;
        #[no_mangle]
        pub fn XSetWMProtocols(
            _: *mut Display,
            _: Window,
            _: *mut Atom,
            _: cty::c_int,
        ) -> cty::c_int;
        #[no_mangle]
        pub fn XChangeProperty(
            _: *mut Display,
            _: Window,
            _: Atom,
            _: Atom,
            _: cty::c_int,
            _: cty::c_int,
            _: *const cty::c_uchar,
            _: cty::c_int,
        ) -> cty::c_int;
        #[no_mangle]
        pub fn XSync(_: *mut Display, _: cty::c_int) -> cty::c_int;
        #[no_mangle]
        pub fn XSetErrorHandler(_: XErrorHandler) -> XErrorHandler;
        #[no_mangle]
        pub fn XGetWindowAttributes(
            _: *mut Display,
            _: Window,
            _: *mut XWindowAttributes,
        ) -> cty::c_int;
        #[no_mangle]
        pub fn XMapWindow(_: *mut Display, _: Window) -> cty::c_int;
        #[no_mangle]
        pub fn XLowerWindow(_: *mut Display, _: Window) -> cty::c_int;
        #[no_mangle]
        pub fn XRaiseWindow(_: *mut Display, _: Window) -> cty::c_int;
        #[no_mangle]
        pub fn XPending(_: *mut Display) -> cty::c_int;
        #[no_mangle]
        pub fn XNextEvent(_: *mut Display, _: *mut XEvent) -> cty::c_int;
        #[no_mangle]
        pub fn XGetKeyboardMapping(
            _: *mut Display,
            _: KeyCode,
            _: cty::c_int,
            _: *mut cty::c_int,
        ) -> *mut KeySym;
        #[no_mangle]
        pub fn XGetWindowProperty(
            _: *mut Display,
            _: Window,
            _: Atom,
            _: cty::c_long,
            _: cty::c_long,
            _: cty::c_int,
            _: Atom,
            _: *mut Atom,
            _: *mut cty::c_int,
            _: *mut cty::c_ulong,
            _: *mut cty::c_ulong,
            _: *mut *mut cty::c_uchar,
        ) -> cty::c_int;
        #[no_mangle]
        pub fn XFree(_: *mut cty::c_void) -> cty::c_int;
        #[no_mangle]
        pub fn XUnmapWindow(_: *mut Display, _: Window) -> cty::c_int;
        #[no_mangle]
        pub fn XDestroyWindow(_: *mut Display, _: Window) -> cty::c_int;
        #[no_mangle]
        pub fn XFreeColormap(_: *mut Display, _: Colormap) -> cty::c_int;
        #[no_mangle]
        pub fn XFlush(_: *mut Display) -> cty::c_int;
        #[no_mangle]
        pub fn XCloseDisplay(_: *mut Display) -> cty::c_int;
        #[no_mangle]
        pub fn XGrabPointer(
            _: *mut Display,
            _: Window,
            _: cty::c_int,
            _: cty::c_uint,
            _: cty::c_int,
            _: cty::c_int,
            _: Window,
            _: Cursor,
            _: Time,
        ) -> cty::c_int;
        #[no_mangle]
        pub fn XUngrabPointer(_: *mut Display, _: Time) -> cty::c_int;

    }
}
pub mod X_h {
    pub type Colormap = XID;
    pub type XID = cty::c_ulong;
    pub type Window = XID;
    pub type Atom = cty::c_ulong;
    pub type Time = cty::c_ulong;
    pub type Drawable = XID;
    pub type Pixmap = XID;
    pub type KeySym = XID;
    pub type KeyCode = cty::c_uchar;
    pub type VisualID = cty::c_ulong;
    pub type Cursor = XID;
    pub const AllocNone: cty::c_int = 0 as cty::c_int;
    pub const StructureNotifyMask: cty::c_long = (1 as cty::c_long) << 17 as cty::c_int;
    pub const KeyPressMask: cty::c_long = (1 as cty::c_long) << 0 as cty::c_int;
    pub const KeyReleaseMask: cty::c_long = (1 as cty::c_long) << 1 as cty::c_int;
    pub const PointerMotionMask: cty::c_long = (1 as cty::c_long) << 6 as cty::c_int;
    pub const PointerMotionHintMask: cty::c_long = (1 as cty::c_long) << 7 as cty::c_int;
    pub const Button1MotionMask: cty::c_long = (1 as cty::c_long) << 8 as cty::c_int;
    pub const Button2MotionMask: cty::c_long = (1 as cty::c_long) << 9 as cty::c_int;
    pub const Button3MotionMask: cty::c_long = (1 as cty::c_long) << 10 as cty::c_int;
    pub const Button4MotionMask: cty::c_long = (1 as cty::c_long) << 11 as cty::c_int;
    pub const Button5MotionMask: cty::c_long = (1 as cty::c_long) << 12 as cty::c_int;
    pub const ButtonMotionMask: cty::c_long = (1 as cty::c_long) << 13 as cty::c_int;
    pub const KeymapStateMask: cty::c_long = (1 as cty::c_long) << 14 as cty::c_int;

    pub const GrabModeAsync: cty::c_int = 1 as cty::c_int;

    pub const ButtonPressMask: cty::c_long = (1 as cty::c_long) << 2 as cty::c_int;
    pub const ButtonReleaseMask: cty::c_long = (1 as cty::c_long) << 3 as cty::c_int;
    pub const ExposureMask: cty::c_long = (1 as cty::c_long) << 15 as cty::c_int;
    pub const FocusChangeMask: cty::c_long = (1 as cty::c_long) << 21 as cty::c_int;
    pub const VisibilityChangeMask: cty::c_long = (1 as cty::c_long) << 16 as cty::c_int;
    pub const EnterWindowMask: cty::c_long = (1 as cty::c_long) << 4 as cty::c_int;
    pub const LeaveWindowMask: cty::c_long = (1 as cty::c_long) << 5 as cty::c_int;
    pub const PropertyChangeMask: cty::c_long = (1 as cty::c_long) << 22 as cty::c_int;
    pub const InputOutput: cty::c_int = 1 as cty::c_int;
    pub const CWBorderPixel: cty::c_long = (1 as cty::c_long) << 3 as cty::c_int;
    pub const CWColormap: cty::c_long = (1 as cty::c_long) << 13 as cty::c_int;
    pub const CWEventMask: cty::c_long = (1 as cty::c_long) << 11 as cty::c_int;
    pub const StaticGravity: cty::c_int = 10 as cty::c_int;
    pub const PropModeReplace: cty::c_int = 0 as cty::c_int;
    pub const Success: cty::c_int = 0 as cty::c_int;
    pub const IsViewable: cty::c_int = 2 as cty::c_int;
    pub const ShiftMask: cty::c_int = (1 as cty::c_int) << 0 as cty::c_int;
    pub const ControlMask: cty::c_int = (1 as cty::c_int) << 2 as cty::c_int;
    pub const Mod1Mask: cty::c_int = (1 as cty::c_int) << 3 as cty::c_int;
    pub const Mod4Mask: cty::c_int = (1 as cty::c_int) << 6 as cty::c_int;
    pub const PropertyNewValue: cty::c_int = 0 as cty::c_int;
}
pub mod Xmd_h {
    pub type CARD32 = cty::c_uint;
}
pub mod Xutil_h {
    pub type XComposeStatus = _XComposeStatus;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _XComposeStatus {
        pub compose_ptr: XPointer,
        pub chars_matched: cty::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct XClassHint {
        pub res_name: *mut cty::c_char,
        pub res_class: *mut cty::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct XWMHints {
        pub flags: cty::c_long,
        pub input: cty::c_int,
        pub initial_state: cty::c_int,
        pub icon_pixmap: Pixmap,
        pub icon_window: Window,
        pub icon_x: cty::c_int,
        pub icon_y: cty::c_int,
        pub icon_mask: Pixmap,
        pub window_group: XID,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct XSizeHints {
        pub flags: cty::c_long,
        pub x: cty::c_int,
        pub y: cty::c_int,
        pub width: cty::c_int,
        pub height: cty::c_int,
        pub min_width: cty::c_int,
        pub min_height: cty::c_int,
        pub max_width: cty::c_int,
        pub max_height: cty::c_int,
        pub width_inc: cty::c_int,
        pub height_inc: cty::c_int,
        pub min_aspect: C2RustUnnamed_2,
        pub max_aspect: C2RustUnnamed_2,
        pub base_width: cty::c_int,
        pub base_height: cty::c_int,
        pub win_gravity: cty::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct C2RustUnnamed_2 {
        pub x: cty::c_int,
        pub y: cty::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct XVisualInfo {
        pub visual: *mut Visual,
        pub visualid: VisualID,
        pub screen: cty::c_int,
        pub depth: cty::c_int,
        pub class: cty::c_int,
        pub red_mask: cty::c_ulong,
        pub green_mask: cty::c_ulong,
        pub blue_mask: cty::c_ulong,
        pub colormap_size: cty::c_int,
        pub bits_per_rgb: cty::c_int,
    }
    pub const PWinGravity: cty::c_long = (1 as cty::c_long) << 9 as cty::c_int;
    pub const IconicState: cty::c_int = 3 as cty::c_int;
    pub const WithdrawnState: cty::c_int = 0 as cty::c_int;
    pub const NormalState: cty::c_int = 1 as cty::c_int;
    use super::X_h::{KeySym, Pixmap, VisualID, Window, XID};
    use super::Xlib_h::{Display, Visual, XKeyEvent, XPointer};
    extern "C" {
        #[no_mangle]
        pub fn XSetWMNormalHints(_: *mut Display, _: Window, _: *mut XSizeHints);
        #[no_mangle]
        pub fn XAllocSizeHints() -> *mut XSizeHints;
        #[no_mangle]
        pub fn Xutf8SetWMProperties(
            _: *mut Display,
            _: Window,
            _: *const cty::c_char,
            _: *const cty::c_char,
            _: *mut *mut cty::c_char,
            _: cty::c_int,
            _: *mut XSizeHints,
            _: *mut XWMHints,
            _: *mut XClassHint,
        );
        #[no_mangle]
        pub fn XLookupString(
            _: *mut XKeyEvent,
            _: *mut cty::c_char,
            _: cty::c_int,
            _: *mut KeySym,
            _: *mut XComposeStatus,
        ) -> cty::c_int;
    }
}
pub mod Xresource_h {
    pub type XrmDatabase = *mut _XrmHashBucketRec;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct XrmValue {
        pub size: cty::c_uint,
        pub addr: XPointer,
    }
    use super::Xlib_h::{XPointer, _XrmHashBucketRec};
    extern "C" {
        #[no_mangle]
        pub fn XrmGetResource(
            _: XrmDatabase,
            _: *const cty::c_char,
            _: *const cty::c_char,
            _: *mut *mut cty::c_char,
            _: *mut XrmValue,
        ) -> cty::c_int;
        #[no_mangle]
        pub fn XrmDestroyDatabase(_: XrmDatabase);
        #[no_mangle]
        pub fn XrmGetStringDatabase(_: *const cty::c_char) -> XrmDatabase;
    }
}
pub mod XKBlib_h {
    use super::Xlib_h::Display;
    extern "C" {
        #[no_mangle]
        pub fn XkbSetDetectableAutoRepeat(
            _: *mut Display,
            _: cty::c_int,
            _: *mut cty::c_int,
        ) -> cty::c_int;
    }
}
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        pub fn atof(__nptr: *const cty::c_char) -> cty::c_double;

    }
}
pub mod dlfcn_h {
    extern "C" {
        #[no_mangle]
        pub fn dlopen(__file: *const cty::c_char, __mode: cty::c_int) -> *mut cty::c_void;
        #[no_mangle]
        pub fn dlsym(__handle: *mut cty::c_void, __name: *const cty::c_char)
            -> *mut cty::c_void;
    }
}
pub mod bits_dlfcn_h {
    pub const RTLD_LAZY: cty::c_int = 0x1 as cty::c_int;
    pub const RTLD_GLOBAL: cty::c_int = 0x100 as cty::c_int;
}
pub mod string_h {
    extern "C" {
        #[no_mangle]
        pub fn strstr(_: *const cty::c_char, _: *const cty::c_char) -> *mut cty::c_char;
        #[no_mangle]
        pub fn strlen(_: *const cty::c_char) -> cty::c_ulong;
        #[no_mangle]
        pub fn strcmp(_: *const cty::c_char, _: *const cty::c_char) -> cty::c_int;
        #[no_mangle]
        pub fn memset(_: *mut cty::c_void, _: cty::c_int, _: cty::c_ulong) -> *mut cty::c_void;
    }
}

extern "C" {
    #[no_mangle]
    pub fn XSendEvent(
        _: *mut Display,
        _: Window,
        _: cty::c_int,
        _: cty::c_long,
        _: *mut XEvent,
    ) -> cty::c_int;
}
