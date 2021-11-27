#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]

pub mod clipboard;
pub mod gl;
mod rand;

pub use gl::*;
pub use rand::*;

use winapi::{
    shared::{
        hidusage::{HID_USAGE_GENERIC_MOUSE, HID_USAGE_GENERIC_POINTER},
        minwindef::{DWORD, HINSTANCE, HIWORD, INT, LOWORD, LPARAM, LRESULT, PROC, UINT, WPARAM},
        ntdef::{HRESULT, LPCSTR, NULL},
        windef::{HCURSOR, HDC, HGLRC, HICON, HMONITOR, HWND, POINT, RECT},
        windowsx::{GET_X_LPARAM, GET_Y_LPARAM},
    },
    um::{
        errhandlingapi::GetLastError,
        libloaderapi::{FreeLibrary, GetModuleHandleW, GetProcAddress, LoadLibraryA},
        shellscalingapi::{
            MDT_EFFECTIVE_DPI, MONITOR_DPI_TYPE, PROCESS_DPI_AWARENESS, PROCESS_DPI_UNAWARE,
            PROCESS_SYSTEM_DPI_AWARE,
        },
        wingdi::{
            ChoosePixelFormat, CreateBitmap, CreateDIBSection, DeleteObject, DescribePixelFormat,
            SetPixelFormat, SwapBuffers, BITMAPINFO, BITMAPV5HEADER, BI_BITFIELDS, DIB_RGB_COLORS,
            PFD_DOUBLEBUFFER, PFD_DRAW_TO_WINDOW, PFD_SUPPORT_OPENGL, PFD_TYPE_RGBA,
            PIXELFORMATDESCRIPTOR,
        },
        winuser::{
            AdjustWindowRectEx, ClientToScreen, ClipCursor, CreateIconIndirect, CreateWindowExW,
            DefWindowProcW, DestroyWindow, DispatchMessageW, GetClientRect,
            GetCursorInfo, GetDC, GetKeyState, GetRawInputData, GetSystemMetrics, LoadCursorW,
            LoadIconW, MonitorFromPoint, PeekMessageW, PostMessageW, PostQuitMessage,
            RegisterClassW, RegisterRawInputDevices, ReleaseDC, SendMessageW, SetCursor, SetRect,
            SetWindowLongPtrA, SetWindowPos, ShowCursor, ShowWindow, TrackMouseEvent,
            TranslateMessage, UnregisterClassW, CS_HREDRAW, CS_OWNDC, CS_VREDRAW, CURSORINFO,
            CURSOR_SHOWING, CW_USEDEFAULT, GWL_STYLE, HTCLIENT, HWND_TOP, ICONINFO, ICON_BIG,
            ICON_SMALL, IDC_ARROW, IDC_CROSS, IDC_HAND, IDC_HELP, IDC_IBEAM, IDC_NO, IDC_SIZEALL,
            IDC_SIZENESW, IDC_SIZENS, IDC_SIZENWSE, IDC_SIZEWE, IDC_WAIT, IDI_WINLOGO,
            MONITOR_DEFAULTTONEAREST, MOUSE_MOVE_ABSOLUTE, MSG, PM_REMOVE, RAWINPUT,
            RAWINPUTDEVICE, RAWINPUTHEADER, RIDEV_REMOVE, RID_INPUT, SC_KEYMENU, SC_MONITORPOWER,
            SC_SCREENSAVE, SIZE_MINIMIZED, SM_CXICON, SM_CXSCREEN, SM_CXSMICON, SM_CYICON,
            SM_CYSCREEN, SM_CYSMICON, SWP_FRAMECHANGED, SWP_NOMOVE, SW_HIDE, SW_SHOW, TME_LEAVE,
            TRACKMOUSEEVENT, VK_CONTROL, VK_LWIN, VK_MENU, VK_RWIN, VK_SHIFT, WM_CHAR, WM_CLOSE,
            WM_ERASEBKGND, WM_INPUT, WM_KEYDOWN, WM_KEYUP, WM_LBUTTONDOWN, WM_LBUTTONUP,
            WM_MBUTTONDOWN, WM_MBUTTONUP, WM_MOUSEHWHEEL, WM_MOUSELEAVE, WM_MOUSEMOVE,
            WM_MOUSEWHEEL, WM_MOVE, WM_QUIT, WM_RBUTTONDOWN, WM_RBUTTONUP, WM_SETCURSOR,
            WM_SETICON, WM_SIZE, WM_SYSCOMMAND, WM_SYSKEYDOWN, WM_SYSKEYUP, WNDCLASSW, WS_CAPTION,
            WS_CLIPCHILDREN, WS_CLIPSIBLINGS, WS_EX_APPWINDOW, WS_EX_OVERLAPPEDWINDOW,
            WS_EX_WINDOWEDGE, WS_MAXIMIZEBOX, WS_MINIMIZEBOX, WS_POPUP, WS_SIZEBOX, WS_SYSMENU,
            WS_VISIBLE,
        },
    },
};

pub type sapp_event_type = u32;
pub const sapp_event_type__SAPP_EVENTTYPE_FORCE_U32: sapp_event_type = 2147483647;
pub const sapp_event_type__SAPP_EVENTTYPE_NUM: sapp_event_type = 22;
pub const sapp_event_type_SAPP_EVENTTYPE_RAW_DEVICE: sapp_event_type = 21;
pub const sapp_event_type_SAPP_EVENTTYPE_QUIT_REQUESTED: sapp_event_type = 20;
pub const sapp_event_type_SAPP_EVENTTYPE_UPDATE_CURSOR: sapp_event_type = 19;
pub const sapp_event_type_SAPP_EVENTTYPE_RESUMED: sapp_event_type = 18;
pub const sapp_event_type_SAPP_EVENTTYPE_SUSPENDED: sapp_event_type = 17;
pub const sapp_event_type_SAPP_EVENTTYPE_RESTORED: sapp_event_type = 16;
pub const sapp_event_type_SAPP_EVENTTYPE_ICONIFIED: sapp_event_type = 15;
pub const sapp_event_type_SAPP_EVENTTYPE_RESIZED: sapp_event_type = 14;
pub const sapp_event_type_SAPP_EVENTTYPE_TOUCHES_CANCELLED: sapp_event_type = 13;
pub const sapp_event_type_SAPP_EVENTTYPE_TOUCHES_ENDED: sapp_event_type = 12;
pub const sapp_event_type_SAPP_EVENTTYPE_TOUCHES_MOVED: sapp_event_type = 11;
pub const sapp_event_type_SAPP_EVENTTYPE_TOUCHES_BEGAN: sapp_event_type = 10;
pub const sapp_event_type_SAPP_EVENTTYPE_MOUSE_LEAVE: sapp_event_type = 9;
pub const sapp_event_type_SAPP_EVENTTYPE_MOUSE_ENTER: sapp_event_type = 8;
pub const sapp_event_type_SAPP_EVENTTYPE_MOUSE_MOVE: sapp_event_type = 7;
pub const sapp_event_type_SAPP_EVENTTYPE_MOUSE_SCROLL: sapp_event_type = 6;
pub const sapp_event_type_SAPP_EVENTTYPE_MOUSE_UP: sapp_event_type = 5;
pub const sapp_event_type_SAPP_EVENTTYPE_MOUSE_DOWN: sapp_event_type = 4;
pub const sapp_event_type_SAPP_EVENTTYPE_CHAR: sapp_event_type = 3;
pub const sapp_event_type_SAPP_EVENTTYPE_KEY_UP: sapp_event_type = 2;
pub const sapp_event_type_SAPP_EVENTTYPE_KEY_DOWN: sapp_event_type = 1;
pub const sapp_event_type_SAPP_EVENTTYPE_INVALID: sapp_event_type = 0;

pub type sapp_keycode = u32;
pub const sapp_keycode_SAPP_KEYCODE_MENU: sapp_keycode = 348;
pub const sapp_keycode_SAPP_KEYCODE_RIGHT_SUPER: sapp_keycode = 347;
pub const sapp_keycode_SAPP_KEYCODE_RIGHT_ALT: sapp_keycode = 346;
pub const sapp_keycode_SAPP_KEYCODE_RIGHT_CONTROL: sapp_keycode = 345;
pub const sapp_keycode_SAPP_KEYCODE_RIGHT_SHIFT: sapp_keycode = 344;
pub const sapp_keycode_SAPP_KEYCODE_LEFT_SUPER: sapp_keycode = 343;
pub const sapp_keycode_SAPP_KEYCODE_LEFT_ALT: sapp_keycode = 342;
pub const sapp_keycode_SAPP_KEYCODE_LEFT_CONTROL: sapp_keycode = 341;
pub const sapp_keycode_SAPP_KEYCODE_LEFT_SHIFT: sapp_keycode = 340;
pub const sapp_keycode_SAPP_KEYCODE_KP_EQUAL: sapp_keycode = 336;
pub const sapp_keycode_SAPP_KEYCODE_KP_ENTER: sapp_keycode = 335;
pub const sapp_keycode_SAPP_KEYCODE_KP_ADD: sapp_keycode = 334;
pub const sapp_keycode_SAPP_KEYCODE_KP_SUBTRACT: sapp_keycode = 333;
pub const sapp_keycode_SAPP_KEYCODE_KP_MULTIPLY: sapp_keycode = 332;
pub const sapp_keycode_SAPP_KEYCODE_KP_DIVIDE: sapp_keycode = 331;
pub const sapp_keycode_SAPP_KEYCODE_KP_DECIMAL: sapp_keycode = 330;
pub const sapp_keycode_SAPP_KEYCODE_KP_9: sapp_keycode = 329;
pub const sapp_keycode_SAPP_KEYCODE_KP_8: sapp_keycode = 328;
pub const sapp_keycode_SAPP_KEYCODE_KP_7: sapp_keycode = 327;
pub const sapp_keycode_SAPP_KEYCODE_KP_6: sapp_keycode = 326;
pub const sapp_keycode_SAPP_KEYCODE_KP_5: sapp_keycode = 325;
pub const sapp_keycode_SAPP_KEYCODE_KP_4: sapp_keycode = 324;
pub const sapp_keycode_SAPP_KEYCODE_KP_3: sapp_keycode = 323;
pub const sapp_keycode_SAPP_KEYCODE_KP_2: sapp_keycode = 322;
pub const sapp_keycode_SAPP_KEYCODE_KP_1: sapp_keycode = 321;
pub const sapp_keycode_SAPP_KEYCODE_KP_0: sapp_keycode = 320;
pub const sapp_keycode_SAPP_KEYCODE_F25: sapp_keycode = 314;
pub const sapp_keycode_SAPP_KEYCODE_F24: sapp_keycode = 313;
pub const sapp_keycode_SAPP_KEYCODE_F23: sapp_keycode = 312;
pub const sapp_keycode_SAPP_KEYCODE_F22: sapp_keycode = 311;
pub const sapp_keycode_SAPP_KEYCODE_F21: sapp_keycode = 310;
pub const sapp_keycode_SAPP_KEYCODE_F20: sapp_keycode = 309;
pub const sapp_keycode_SAPP_KEYCODE_F19: sapp_keycode = 308;
pub const sapp_keycode_SAPP_KEYCODE_F18: sapp_keycode = 307;
pub const sapp_keycode_SAPP_KEYCODE_F17: sapp_keycode = 306;
pub const sapp_keycode_SAPP_KEYCODE_F16: sapp_keycode = 305;
pub const sapp_keycode_SAPP_KEYCODE_F15: sapp_keycode = 304;
pub const sapp_keycode_SAPP_KEYCODE_F14: sapp_keycode = 303;
pub const sapp_keycode_SAPP_KEYCODE_F13: sapp_keycode = 302;
pub const sapp_keycode_SAPP_KEYCODE_F12: sapp_keycode = 301;
pub const sapp_keycode_SAPP_KEYCODE_F11: sapp_keycode = 300;
pub const sapp_keycode_SAPP_KEYCODE_F10: sapp_keycode = 299;
pub const sapp_keycode_SAPP_KEYCODE_F9: sapp_keycode = 298;
pub const sapp_keycode_SAPP_KEYCODE_F8: sapp_keycode = 297;
pub const sapp_keycode_SAPP_KEYCODE_F7: sapp_keycode = 296;
pub const sapp_keycode_SAPP_KEYCODE_F6: sapp_keycode = 295;
pub const sapp_keycode_SAPP_KEYCODE_F5: sapp_keycode = 294;
pub const sapp_keycode_SAPP_KEYCODE_F4: sapp_keycode = 293;
pub const sapp_keycode_SAPP_KEYCODE_F3: sapp_keycode = 292;
pub const sapp_keycode_SAPP_KEYCODE_F2: sapp_keycode = 291;
pub const sapp_keycode_SAPP_KEYCODE_F1: sapp_keycode = 290;
pub const sapp_keycode_SAPP_KEYCODE_PAUSE: sapp_keycode = 284;
pub const sapp_keycode_SAPP_KEYCODE_PRINT_SCREEN: sapp_keycode = 283;
pub const sapp_keycode_SAPP_KEYCODE_NUM_LOCK: sapp_keycode = 282;
pub const sapp_keycode_SAPP_KEYCODE_SCROLL_LOCK: sapp_keycode = 281;
pub const sapp_keycode_SAPP_KEYCODE_CAPS_LOCK: sapp_keycode = 280;
pub const sapp_keycode_SAPP_KEYCODE_END: sapp_keycode = 269;
pub const sapp_keycode_SAPP_KEYCODE_HOME: sapp_keycode = 268;
pub const sapp_keycode_SAPP_KEYCODE_PAGE_DOWN: sapp_keycode = 267;
pub const sapp_keycode_SAPP_KEYCODE_PAGE_UP: sapp_keycode = 266;
pub const sapp_keycode_SAPP_KEYCODE_UP: sapp_keycode = 265;
pub const sapp_keycode_SAPP_KEYCODE_DOWN: sapp_keycode = 264;
pub const sapp_keycode_SAPP_KEYCODE_LEFT: sapp_keycode = 263;
pub const sapp_keycode_SAPP_KEYCODE_RIGHT: sapp_keycode = 262;
pub const sapp_keycode_SAPP_KEYCODE_DELETE: sapp_keycode = 261;
pub const sapp_keycode_SAPP_KEYCODE_INSERT: sapp_keycode = 260;
pub const sapp_keycode_SAPP_KEYCODE_BACKSPACE: sapp_keycode = 259;
pub const sapp_keycode_SAPP_KEYCODE_TAB: sapp_keycode = 258;
pub const sapp_keycode_SAPP_KEYCODE_ENTER: sapp_keycode = 257;
pub const sapp_keycode_SAPP_KEYCODE_ESCAPE: sapp_keycode = 256;
pub const sapp_keycode_SAPP_KEYCODE_WORLD_2: sapp_keycode = 162;
pub const sapp_keycode_SAPP_KEYCODE_WORLD_1: sapp_keycode = 161;
pub const sapp_keycode_SAPP_KEYCODE_GRAVE_ACCENT: sapp_keycode = 96;
pub const sapp_keycode_SAPP_KEYCODE_RIGHT_BRACKET: sapp_keycode = 93;
pub const sapp_keycode_SAPP_KEYCODE_BACKSLASH: sapp_keycode = 92;
pub const sapp_keycode_SAPP_KEYCODE_LEFT_BRACKET: sapp_keycode = 91;
pub const sapp_keycode_SAPP_KEYCODE_Z: sapp_keycode = 90;
pub const sapp_keycode_SAPP_KEYCODE_Y: sapp_keycode = 89;
pub const sapp_keycode_SAPP_KEYCODE_X: sapp_keycode = 88;
pub const sapp_keycode_SAPP_KEYCODE_W: sapp_keycode = 87;
pub const sapp_keycode_SAPP_KEYCODE_V: sapp_keycode = 86;
pub const sapp_keycode_SAPP_KEYCODE_U: sapp_keycode = 85;
pub const sapp_keycode_SAPP_KEYCODE_T: sapp_keycode = 84;
pub const sapp_keycode_SAPP_KEYCODE_S: sapp_keycode = 83;
pub const sapp_keycode_SAPP_KEYCODE_R: sapp_keycode = 82;
pub const sapp_keycode_SAPP_KEYCODE_Q: sapp_keycode = 81;
pub const sapp_keycode_SAPP_KEYCODE_P: sapp_keycode = 80;
pub const sapp_keycode_SAPP_KEYCODE_O: sapp_keycode = 79;
pub const sapp_keycode_SAPP_KEYCODE_N: sapp_keycode = 78;
pub const sapp_keycode_SAPP_KEYCODE_M: sapp_keycode = 77;
pub const sapp_keycode_SAPP_KEYCODE_L: sapp_keycode = 76;
pub const sapp_keycode_SAPP_KEYCODE_K: sapp_keycode = 75;
pub const sapp_keycode_SAPP_KEYCODE_J: sapp_keycode = 74;
pub const sapp_keycode_SAPP_KEYCODE_I: sapp_keycode = 73;
pub const sapp_keycode_SAPP_KEYCODE_H: sapp_keycode = 72;
pub const sapp_keycode_SAPP_KEYCODE_G: sapp_keycode = 71;
pub const sapp_keycode_SAPP_KEYCODE_F: sapp_keycode = 70;
pub const sapp_keycode_SAPP_KEYCODE_E: sapp_keycode = 69;
pub const sapp_keycode_SAPP_KEYCODE_D: sapp_keycode = 68;
pub const sapp_keycode_SAPP_KEYCODE_C: sapp_keycode = 67;
pub const sapp_keycode_SAPP_KEYCODE_B: sapp_keycode = 66;
pub const sapp_keycode_SAPP_KEYCODE_A: sapp_keycode = 65;
pub const sapp_keycode_SAPP_KEYCODE_EQUAL: sapp_keycode = 61;
pub const sapp_keycode_SAPP_KEYCODE_SEMICOLON: sapp_keycode = 59;
pub const sapp_keycode_SAPP_KEYCODE_9: sapp_keycode = 57;
pub const sapp_keycode_SAPP_KEYCODE_8: sapp_keycode = 56;
pub const sapp_keycode_SAPP_KEYCODE_7: sapp_keycode = 55;
pub const sapp_keycode_SAPP_KEYCODE_6: sapp_keycode = 54;
pub const sapp_keycode_SAPP_KEYCODE_5: sapp_keycode = 53;
pub const sapp_keycode_SAPP_KEYCODE_4: sapp_keycode = 52;
pub const sapp_keycode_SAPP_KEYCODE_3: sapp_keycode = 51;
pub const sapp_keycode_SAPP_KEYCODE_2: sapp_keycode = 50;
pub const sapp_keycode_SAPP_KEYCODE_1: sapp_keycode = 49;
pub const sapp_keycode_SAPP_KEYCODE_0: sapp_keycode = 48;
pub const sapp_keycode_SAPP_KEYCODE_SLASH: sapp_keycode = 47;
pub const sapp_keycode_SAPP_KEYCODE_PERIOD: sapp_keycode = 46;
pub const sapp_keycode_SAPP_KEYCODE_MINUS: sapp_keycode = 45;
pub const sapp_keycode_SAPP_KEYCODE_COMMA: sapp_keycode = 44;
pub const sapp_keycode_SAPP_KEYCODE_APOSTROPHE: sapp_keycode = 39;
pub const sapp_keycode_SAPP_KEYCODE_SPACE: sapp_keycode = 32;
pub const sapp_keycode_SAPP_KEYCODE_INVALID: sapp_keycode = 0;

#[derive(Copy, Clone, Default)]
pub struct sapp_touchpoint {
    pub identifier: u64,
    pub pos_x: f32,
    pub pos_y: f32,
    pub changed: bool,
}

pub type sapp_mousebutton = i32;
pub const sapp_mousebutton_SAPP_MOUSEBUTTON_MIDDLE: sapp_mousebutton = 2;
pub const sapp_mousebutton_SAPP_MOUSEBUTTON_RIGHT: sapp_mousebutton = 1;
pub const sapp_mousebutton_SAPP_MOUSEBUTTON_LEFT: sapp_mousebutton = 0;
pub const sapp_mousebutton_SAPP_MOUSEBUTTON_INVALID: sapp_mousebutton = -1;

pub const SAPP_MODIFIER_SHIFT: u32 = 1 << 0;
pub const SAPP_MODIFIER_CTRL: u32 = 1 << 1;
pub const SAPP_MODIFIER_ALT: u32 = 1 << 2;
pub const SAPP_MODIFIER_SUPER: u32 = 1 << 3;

pub const SAPP_CURSOR_DEFAULT: u32 = 0;
pub const SAPP_CURSOR_HELP: u32 = 1;
pub const SAPP_CURSOR_POINTER: u32 = 2;
pub const SAPP_CURSOR_WAIT: u32 = 3;
pub const SAPP_CURSOR_CROSSHAIR: u32 = 4;
pub const SAPP_CURSOR_TEXT: u32 = 5;
pub const SAPP_CURSOR_MOVE: u32 = 6;
pub const SAPP_CURSOR_NOTALLOWED: u32 = 7;
pub const SAPP_CURSOR_EWRESIZE: u32 = 8;
pub const SAPP_CURSOR_NSRESIZE: u32 = 9;
pub const SAPP_CURSOR_NESWRESIZE: u32 = 10;
pub const SAPP_CURSOR_NWSERESIZE: u32 = 11;

#[derive(Copy, Clone, Default)]
pub struct sapp_event {
    pub frame_count: u64,
    pub type_: sapp_event_type,
    pub key_code: sapp_keycode,
    pub char_code: u32,
    pub key_repeat: bool,
    pub modifiers: u32,
    pub mouse_button: sapp_mousebutton,
    pub mouse_x: f32,
    pub mouse_y: f32,
    pub mouse_dx: f32,
    pub mouse_dy: f32,
    pub scroll_x: f32,
    pub scroll_y: f32,
    pub num_touches: i32,
    pub touches: [sapp_touchpoint; 8],
    pub window_width: i32,
    pub window_height: i32,
    pub framebuffer_width: i32,
    pub framebuffer_height: i32,
}

#[derive(Copy, Clone)]
pub struct sapp_icon {
    pub small: *const u8,
    pub medium: *const u8,
    pub big: *const u8,
}

#[derive(Copy, Clone)]
pub struct sapp_desc {
    pub init_cb: Option<unsafe extern "C" fn() -> ()>,
    pub frame_cb: Option<unsafe extern "C" fn() -> ()>,
    pub cleanup_cb: Option<unsafe extern "C" fn() -> ()>,
    pub event_cb: Option<unsafe extern "C" fn(_: *const sapp_event) -> ()>,
    pub fail_cb: Option<unsafe extern "C" fn(_: *const i8) -> ()>,
    pub user_data: *mut std::ffi::c_void,
    pub init_userdata_cb: Option<unsafe extern "C" fn(_: *mut std::ffi::c_void) -> ()>,
    pub frame_userdata_cb: Option<unsafe extern "C" fn(_: *mut std::ffi::c_void) -> ()>,
    pub cleanup_userdata_cb: Option<unsafe extern "C" fn(_: *mut std::ffi::c_void) -> ()>,
    pub event_userdata_cb:
        Option<unsafe extern "C" fn(_: *const sapp_event, _: *mut std::ffi::c_void) -> ()>,
    pub fail_userdata_cb:
        Option<unsafe extern "C" fn(_: *const i8, _: *mut std::ffi::c_void) -> ()>,
    pub width: i32,
    pub height: i32,
    pub window_resizable: bool,
    pub sample_count: i32,
    pub swap_interval: i32,
    pub high_dpi: bool,
    pub fullscreen: bool,
    pub alpha: bool,
    pub window_title: *const i8,
    pub user_cursor: bool,
    pub html5_canvas_name: *const i8,
    pub html5_canvas_resize: bool,
    pub html5_preserve_drawing_buffer: bool,
    pub html5_premultiplied_alpha: bool,
    pub html5_ask_leave_site: bool,
    pub ios_keyboard_resizes_canvas: bool,
    pub gl_force_gles2: bool,
    pub icon: Option<sapp_icon>,
}

#[derive(Clone)]
pub struct _sapp_state {
    pub valid: bool,
    pub window_width: i32,
    pub window_height: i32,
    pub framebuffer_width: i32,
    pub framebuffer_height: i32,
    pub sample_count: i32,
    pub swap_interval: i32,
    pub dpi_scale: f32,
    pub gles2_fallback: bool,
    pub first_frame: bool,
    pub init_called: bool,
    pub cleanup_called: bool,
    pub quit_requested: bool,
    pub quit_ordered: bool,
    pub cursor_grabbed: bool,
    pub window_title: String,
    pub frame_count: u64,
    pub mouse_x: f32,
    pub mouse_y: f32,
    pub win32_mouse_tracked: bool,
    pub onscreen_keyboard_shown: bool,
    pub event: sapp_event,
    pub desc: sapp_desc,
    pub keycodes: [sapp_keycode; 512],
}

static mut _sapp_win32_window_scale: f32 = 1.;
static mut _sapp_win32_content_scale: f32 = 1.;
static mut _sapp_win32_mouse_scale: f32 = 1.;
static mut _sapp_win32_iconified: bool = false;
static mut _sapp_win32_in_create_window: bool = false;
static mut _sapp_win32_hwnd: HWND = std::ptr::null_mut();
static mut _sapp_win32_dc: HDC = std::ptr::null_mut();
static mut _sapp_win32_dpi_aware: bool = false;
static mut _sapp_opengl32: HINSTANCE = std::ptr::null_mut();
static mut _sapp_gl_ctx: HGLRC = std::ptr::null_mut();
static mut _sapp_win32_msg_hwnd: HWND = std::ptr::null_mut();
static mut _sapp_win32_msg_dc: HDC = std::ptr::null_mut();
static mut _sapp_cursor: HCURSOR = std::ptr::null_mut();
static mut _sapp_ext_swap_control: bool = false;
static mut _sapp_arb_multisample: bool = false;
static mut _sapp_arb_pixel_format: bool = false;
static mut _sapp_arb_create_context: bool = false;
static mut _sapp_arb_create_context_profile: bool = false;

static mut _sapp_win32_setprocessdpiaware: Option<extern "system" fn() -> bool> = None;
static mut _sapp_win32_setprocessdpiawareness: Option<
    extern "system" fn(_: PROCESS_DPI_AWARENESS) -> HRESULT,
> = None;
static mut _sapp_win32_getdpiformonitor: Option<
    extern "system" fn(_: HMONITOR, _: MONITOR_DPI_TYPE, _: *mut UINT, _: *mut UINT) -> HRESULT,
> = None;
static mut _sapp_wglCreateContext: Option<extern "system" fn(_: HDC) -> HGLRC> = None;
static mut _sapp_wglDeleteContext: Option<extern "system" fn(_: HGLRC) -> bool> = None;
static mut _sapp_wglGetProcAddress: Option<extern "system" fn(_: LPCSTR) -> PROC> = None;
static mut _sapp_wglGetCurrentDC: Option<extern "system" fn() -> HDC> = None;
static mut _sapp_wglMakeCurrent: Option<extern "system" fn(_: HDC, _: HGLRC) -> bool> = None;

static mut _sapp_GetPixelFormatAttribivARB: Option<
    extern "system" fn(_: HDC, _: INT, _: INT, _: UINT, _: *const INT, _: *mut INT) -> bool,
> = None;
static mut _sapp_GetExtensionsStringEXT: Option<extern "system" fn() -> *const i8> = None;
static mut _sapp_GetExtensionsStringARB: Option<extern "system" fn(_: HDC) -> *const i8> = None;
static mut _sapp_CreateContextAttribsARB: Option<
    extern "system" fn(_: HDC, _: HGLRC, _: *const INT) -> HGLRC,
> = None;
static mut _sapp_SwapIntervalEXT: Option<extern "system" fn(_: INT) -> bool> = None;

static mut _sapp: _sapp_state = _sapp_state {
    valid: false,
    window_width: 0,
    window_height: 0,
    framebuffer_width: 0,
    framebuffer_height: 0,
    sample_count: 0,
    swap_interval: 0,
    dpi_scale: 0.,
    gles2_fallback: false,
    first_frame: false,
    init_called: false,
    cleanup_called: false,
    quit_requested: false,
    quit_ordered: false,
    cursor_grabbed: false,
    window_title: String::new(),
    frame_count: 0,
    mouse_x: 0.,
    mouse_y: 0.,
    win32_mouse_tracked: false,
    onscreen_keyboard_shown: false,
    event: sapp_event {
        frame_count: 0,
        type_: sapp_event_type_SAPP_EVENTTYPE_INVALID,
        key_code: sapp_keycode_SAPP_KEYCODE_INVALID,
        char_code: 0,
        key_repeat: false,
        modifiers: 0,
        mouse_button: sapp_mousebutton_SAPP_MOUSEBUTTON_LEFT,
        mouse_x: 0.,
        mouse_y: 0.,
        mouse_dx: 0.,
        mouse_dy: 0.,
        scroll_x: 0.,
        scroll_y: 0.,
        num_touches: 0,
        touches: [sapp_touchpoint {
            identifier: 0,
            pos_x: 0.,
            pos_y: 0.,
            changed: false,
        }; 8],
        window_width: 0,
        window_height: 0,
        framebuffer_width: 0,
        framebuffer_height: 0,
    },
    desc: sapp_desc {
        init_cb: None,
        frame_cb: None,
        cleanup_cb: None,
        event_cb: None,
        fail_cb: None,
        user_data: 0 as *const std::ffi::c_void as *mut std::ffi::c_void,
        init_userdata_cb: None,
        frame_userdata_cb: None,
        cleanup_userdata_cb: None,
        event_userdata_cb: None,
        fail_userdata_cb: None,
        width: 0,
        height: 0,
        window_resizable: false,
        sample_count: 0,
        swap_interval: 0,
        high_dpi: false,
        fullscreen: false,
        alpha: false,
        window_title: 0 as *const i8,
        user_cursor: false,
        html5_canvas_name: 0 as *const i8,
        html5_canvas_resize: false,
        html5_preserve_drawing_buffer: false,
        html5_premultiplied_alpha: false,
        html5_ask_leave_site: false,
        ios_keyboard_resizes_canvas: false,
        gl_force_gles2: false,
        icon: None,
    },
    keycodes: [sapp_keycode_SAPP_KEYCODE_INVALID; 512],
};

#[no_mangle]
pub unsafe extern "C" fn sapp_dpi_scale() -> f32 {
    _sapp.dpi_scale
}
#[no_mangle]
pub unsafe extern "C" fn sapp_high_dpi() -> bool {
    _sapp.desc.high_dpi && (_sapp.dpi_scale > 1.5)
}
#[no_mangle]
pub unsafe extern "C" fn sapp_height() -> i32 {
    if _sapp.framebuffer_height > 0 {
        _sapp.framebuffer_height
    } else {
        1
    }
}

#[no_mangle]
pub unsafe extern "C" fn sapp_width() -> i32 {
    if _sapp.framebuffer_width > 0 {
        _sapp.framebuffer_width
    } else {
        1
    }
}

#[no_mangle]
pub unsafe extern "C" fn sapp_quit() {
    _sapp.quit_ordered = true;
}

#[no_mangle]
pub unsafe extern "C" fn sapp_cancel_quit() {
    _sapp.quit_requested = false;
}

#[no_mangle]
pub unsafe extern "C" fn sapp_request_quit() {
    _sapp.quit_requested = true;
}

pub unsafe fn sapp_mouse_shown() -> bool {
    let mut cursor_info: CURSORINFO = std::mem::zeroed();
    cursor_info.cbSize = std::mem::size_of::<CURSORINFO>() as _;
    GetCursorInfo(&mut cursor_info as *mut _);

    cursor_info.flags & CURSOR_SHOWING != 0
}

pub unsafe fn sapp_set_cursor_grab(grab: bool) {
    if grab == _sapp.cursor_grabbed {
        return;
    }

    _sapp.cursor_grabbed = grab;

    let mut rid: RAWINPUTDEVICE = RAWINPUTDEVICE {
        usUsagePage: HID_USAGE_GENERIC_POINTER,
        usUsage: HID_USAGE_GENERIC_MOUSE,
        dwFlags: if grab { 0 } else { RIDEV_REMOVE },
        hwndTarget: if grab { _sapp_win32_hwnd } else { NULL as _ },
    };

    if RegisterRawInputDevices(
        &mut rid as *mut _ as _,
        1,
        std::mem::size_of::<RAWINPUTDEVICE>() as _,
    ) != 1
    {
        if grab {
            panic!("failed to register raw input device");
        } else {
            panic!("failed to remove raw input device");
        }
    }

    if !grab {
        ClipCursor(NULL as _);
    }
}

pub unsafe fn sapp_show_mouse(shown: bool) {
    ShowCursor(shown as _);
}

pub unsafe fn sapp_set_mouse_cursor(cursor_icon: u32) {
    let cursor_name = match cursor_icon {
        SAPP_CURSOR_DEFAULT => IDC_ARROW,
        SAPP_CURSOR_HELP => IDC_HELP,
        SAPP_CURSOR_POINTER => IDC_HAND,
        SAPP_CURSOR_WAIT => IDC_WAIT,
        SAPP_CURSOR_CROSSHAIR => IDC_CROSS,
        SAPP_CURSOR_TEXT => IDC_IBEAM,
        SAPP_CURSOR_MOVE => IDC_SIZEALL,
        SAPP_CURSOR_NOTALLOWED => IDC_NO,
        SAPP_CURSOR_EWRESIZE => IDC_SIZEWE,
        SAPP_CURSOR_NSRESIZE => IDC_SIZENS,
        SAPP_CURSOR_NESWRESIZE => IDC_SIZENESW,
        SAPP_CURSOR_NWSERESIZE => IDC_SIZENWSE,
        _ => return,
    };
    _sapp_cursor = LoadCursorW(NULL as _, cursor_name);
    SetCursor(_sapp_cursor);

    _sapp.desc.user_cursor = cursor_icon != SAPP_CURSOR_DEFAULT;
}

pub unsafe fn sapp_set_window_size(new_width: u32, new_height: u32) {
    let win_style: DWORD = get_win_style();
    let win_ex_style: DWORD = WS_EX_APPWINDOW | WS_EX_WINDOWEDGE;

    let mut rect: RECT = std::mem::zeroed();
    if _sapp.desc.fullscreen {
        rect.right = GetSystemMetrics(SM_CXSCREEN);
        rect.bottom = GetSystemMetrics(SM_CYSCREEN);
    } else {
        rect.right = (new_width as f32 * _sapp_win32_window_scale) as _;
        rect.bottom = (new_height as f32 * _sapp_win32_window_scale) as _;
    }
    AdjustWindowRectEx(&rect as *const _ as _, win_style, false as _, win_ex_style);
    SetWindowPos(
        _sapp_win32_hwnd,
        HWND_TOP,
        0,
        0,
        rect.right - rect.left,
        rect.bottom - rect.top,
        SWP_NOMOVE,
    );
}

pub unsafe fn sapp_is_fullscreen() -> bool {
    _sapp.desc.fullscreen as _
}

unsafe fn get_win_style() -> DWORD {
    if _sapp.desc.fullscreen {
        WS_POPUP | WS_SYSMENU | WS_VISIBLE
    } else {
        let mut win_style: DWORD =
            WS_CLIPSIBLINGS | WS_CLIPCHILDREN | WS_CAPTION | WS_SYSMENU | WS_MINIMIZEBOX;

        if _sapp.desc.window_resizable {
            win_style |= WS_MAXIMIZEBOX | WS_SIZEBOX;
        }

        win_style
    }
}

pub unsafe fn sapp_set_fullscreen(fullscreen: bool) {
    _sapp.desc.fullscreen = fullscreen as _;

    let win_style: DWORD = get_win_style();

    SetWindowLongPtrA(_sapp_win32_hwnd, GWL_STYLE, win_style as _);

    if _sapp.desc.fullscreen {
        SetWindowPos(
            _sapp_win32_hwnd,
            HWND_TOP,
            0,
            0,
            GetSystemMetrics(SM_CXSCREEN),
            GetSystemMetrics(SM_CYSCREEN),
            SWP_FRAMECHANGED,
        );
    } else {
        sapp_set_window_size(_sapp.desc.width as _, _sapp.desc.height as _);
    }

    ShowWindow(_sapp_win32_hwnd, SW_SHOW);
}

unsafe fn _sapp_init_event(type_: sapp_event_type) {
    _sapp.event = std::mem::zeroed();
    _sapp.event.type_ = type_;
    _sapp.event.frame_count = _sapp.frame_count;
    _sapp.event.mouse_button = sapp_mousebutton_SAPP_MOUSEBUTTON_INVALID;
    _sapp.event.window_width = _sapp.window_width;
    _sapp.event.window_height = _sapp.window_height;
    _sapp.event.framebuffer_width = _sapp.framebuffer_width;
    _sapp.event.framebuffer_height = _sapp.framebuffer_height;
}

unsafe fn _sapp_events_enabled() -> bool {
    // only send events when an event callback is set, and the init function was called
    (_sapp.desc.event_cb.is_some() || _sapp.desc.event_userdata_cb.is_some()) && _sapp.init_called
}

unsafe fn _sapp_call_event(e: *const sapp_event) {
    if !_sapp.cleanup_called {
        if _sapp.desc.event_cb.is_some() {
            _sapp.desc.event_cb.expect("non-null function pointer")(e);
        } else if _sapp.desc.event_userdata_cb.is_some() {
            _sapp
                .desc
                .event_userdata_cb
                .expect("non-null function pointer")(e, _sapp.desc.user_data);
        }
    };
}

unsafe fn _sapp_win32_app_event(type_: sapp_event_type) {
    if _sapp_events_enabled() {
        _sapp_init_event(type_);
        _sapp_call_event(&_sapp.event);
    }
}

unsafe fn _sapp_win32_mods() -> u32 {
    let mut mods = 0;

    if GetKeyState(VK_SHIFT) as u32 & (1u32 << 31) != 0 {
        mods |= SAPP_MODIFIER_SHIFT;
    }
    if GetKeyState(VK_CONTROL) as u32 & (1u32 << 31) != 0 {
        mods |= SAPP_MODIFIER_CTRL;
    }
    if GetKeyState(VK_MENU) as u32 & (1u32 << 31) != 0 {
        mods |= SAPP_MODIFIER_ALT;
    }
    if (GetKeyState(VK_LWIN) | GetKeyState(VK_RWIN)) as u32 & (1u32 << 31) != 0 {
        mods |= SAPP_MODIFIER_SUPER;
    }

    mods
}

unsafe fn _sapp_win32_mouse_event(type_: sapp_event_type, btn: sapp_mousebutton) {
    if _sapp_events_enabled() {
        _sapp_init_event(type_);
        _sapp.event.modifiers = _sapp_win32_mods();
        _sapp.event.mouse_button = btn;
        _sapp.event.mouse_x = _sapp.mouse_x;
        _sapp.event.mouse_y = _sapp.mouse_y;
        _sapp_call_event(&_sapp.event);
    }
}

unsafe fn _sapp_win32_scroll_event(x: f32, y: f32) {
    if _sapp_events_enabled() {
        _sapp_init_event(sapp_event_type_SAPP_EVENTTYPE_MOUSE_SCROLL);
        _sapp.event.modifiers = _sapp_win32_mods();
        _sapp.event.scroll_x = -x / 30.0;
        _sapp.event.scroll_y = y / 30.0;
        _sapp_call_event(&_sapp.event);
    }
}

unsafe fn _sapp_win32_char_event(c: u32, repeat: bool) {
    if _sapp_events_enabled() && c >= 32 {
        _sapp_init_event(sapp_event_type_SAPP_EVENTTYPE_CHAR);
        _sapp.event.modifiers = _sapp_win32_mods();
        _sapp.event.char_code = c;
        _sapp.event.key_repeat = repeat;
        _sapp_call_event(&_sapp.event);
    }
}

unsafe fn _sapp_win32_key_event(type_: sapp_event_type, vk: u32, repeat: bool) {
    if _sapp_events_enabled() && vk < _sapp.keycodes.len() as _ {
        _sapp_init_event(type_);
        _sapp.event.modifiers = _sapp_win32_mods();
        _sapp.event.key_code = _sapp.keycodes[vk as usize];
        _sapp.event.key_repeat = repeat;
        _sapp_call_event(&_sapp.event);
    }
}

unsafe fn update_clip_rect(hWnd: HWND) {
    // Retrieve the screen coordinates of the client area,
    // and convert them into client coordinates.
    let mut rect: RECT = std::mem::zeroed();

    GetClientRect(hWnd, &mut rect as *mut _ as _);
    let mut upper_left = POINT {
        x: rect.left,
        y: rect.top,
    };
    let mut lower_right = POINT {
        x: rect.right,
        y: rect.bottom,
    };

    ClientToScreen(hWnd, &mut upper_left as *mut _ as _);
    ClientToScreen(hWnd, &mut lower_right as *mut _ as _);

    SetRect(
        &mut rect as *mut _ as _,
        upper_left.x,
        upper_left.y,
        lower_right.x,
        lower_right.y,
    );
    ClipCursor(&mut rect as *mut _ as _);
}

unsafe extern "system" fn win32_wndproc(
    hWnd: HWND,
    uMsg: UINT,
    wParam: WPARAM,
    lParam: LPARAM,
) -> LRESULT {
    // FIXME: refresh rendering during resize with a WM_TIMER event
    if _sapp_win32_in_create_window == false {
        match uMsg {
            WM_CLOSE => {
                // only give user a chance to intervene when sapp_quit() wasn't already called
                if !_sapp.quit_ordered {
                    // if window should be closed and event handling is enabled, give user code
                    // a change to intervene via sapp_cancel_quit()
                    _sapp.quit_requested = true;
                    _sapp_win32_app_event(sapp_event_type_SAPP_EVENTTYPE_QUIT_REQUESTED);
                    // if user code hasn't intervened, quit the app
                    if _sapp.quit_requested {
                        _sapp.quit_ordered = true;
                    }
                }
                if _sapp.quit_ordered {
                    PostQuitMessage(0);
                }
                return 0;
            }
            WM_SYSCOMMAND => {
                match wParam & 0xFFF0 {
                    SC_SCREENSAVE | SC_MONITORPOWER => {
                        if _sapp.desc.fullscreen {
                            // disable screen saver and blanking in fullscreen mode
                            return 0;
                        }
                    }
                    SC_KEYMENU => {
                        // user trying to access menu via ALT
                        return 0;
                    }
                    _ => {}
                }
            }
            WM_ERASEBKGND => {
                return 1;
            }
            WM_SIZE => {
                if _sapp.cursor_grabbed {
                    update_clip_rect(hWnd);
                }

                let iconified = wParam == SIZE_MINIMIZED;
                if iconified != _sapp_win32_iconified {
                    _sapp_win32_iconified = iconified;
                    if iconified {
                        _sapp_win32_app_event(sapp_event_type_SAPP_EVENTTYPE_ICONIFIED);
                    } else {
                        _sapp_win32_app_event(sapp_event_type_SAPP_EVENTTYPE_RESTORED);
                    }
                }
            }
            WM_SETCURSOR => {
                if _sapp.desc.user_cursor {
                    if LOWORD(lParam as _) == HTCLIENT as _ {
                        SetCursor(_sapp_cursor);
                        _sapp_win32_app_event(sapp_event_type_SAPP_EVENTTYPE_UPDATE_CURSOR);
                        return 1;
                    }
                }
            }
            WM_LBUTTONDOWN => _sapp_win32_mouse_event(
                sapp_event_type_SAPP_EVENTTYPE_MOUSE_DOWN,
                sapp_mousebutton_SAPP_MOUSEBUTTON_LEFT,
            ),
            WM_RBUTTONDOWN => _sapp_win32_mouse_event(
                sapp_event_type_SAPP_EVENTTYPE_MOUSE_DOWN,
                sapp_mousebutton_SAPP_MOUSEBUTTON_RIGHT,
            ),
            WM_MBUTTONDOWN => _sapp_win32_mouse_event(
                sapp_event_type_SAPP_EVENTTYPE_MOUSE_DOWN,
                sapp_mousebutton_SAPP_MOUSEBUTTON_MIDDLE,
            ),
            WM_LBUTTONUP => _sapp_win32_mouse_event(
                sapp_event_type_SAPP_EVENTTYPE_MOUSE_UP,
                sapp_mousebutton_SAPP_MOUSEBUTTON_LEFT,
            ),
            WM_RBUTTONUP => _sapp_win32_mouse_event(
                sapp_event_type_SAPP_EVENTTYPE_MOUSE_UP,
                sapp_mousebutton_SAPP_MOUSEBUTTON_RIGHT,
            ),
            WM_MBUTTONUP => _sapp_win32_mouse_event(
                sapp_event_type_SAPP_EVENTTYPE_MOUSE_UP,
                sapp_mousebutton_SAPP_MOUSEBUTTON_MIDDLE,
            ),

            WM_MOUSEMOVE => {
                _sapp.mouse_x = GET_X_LPARAM(lParam) as f32 * _sapp_win32_mouse_scale;
                _sapp.mouse_y = GET_Y_LPARAM(lParam) as f32 * _sapp_win32_mouse_scale;
                if !_sapp.win32_mouse_tracked {
                    _sapp.win32_mouse_tracked = true;

                    let mut tme: TRACKMOUSEEVENT = std::mem::zeroed();

                    tme.cbSize = std::mem::size_of_val(&tme) as _;
                    tme.dwFlags = TME_LEAVE;
                    tme.hwndTrack = _sapp_win32_hwnd;
                    TrackMouseEvent(&mut tme as *mut _);
                    _sapp_win32_mouse_event(
                        sapp_event_type_SAPP_EVENTTYPE_MOUSE_ENTER,
                        sapp_mousebutton_SAPP_MOUSEBUTTON_INVALID,
                    );
                }

                _sapp_win32_mouse_event(
                    sapp_event_type_SAPP_EVENTTYPE_MOUSE_MOVE,
                    sapp_mousebutton_SAPP_MOUSEBUTTON_INVALID,
                );
            }

            WM_MOVE if _sapp.cursor_grabbed => {
                update_clip_rect(hWnd);
            }

            WM_INPUT => {
                let mut data: RAWINPUT = std::mem::zeroed();
                let mut size = std::mem::size_of::<RAWINPUT>();
                let get_succeed = GetRawInputData(
                    lParam as _,
                    RID_INPUT,
                    &mut data as *mut _ as _,
                    &mut size as *mut _ as _,
                    std::mem::size_of::<RAWINPUTHEADER>() as _,
                );
                if get_succeed as i32 == -1 {
                    panic!("failed to retrieve raw input data");
                }

                if data.data.mouse().usFlags & MOUSE_MOVE_ABSOLUTE == 1 {
                    unimplemented!("Got MOUSE_MOVE_ABSOLUTE on WM_INPUT, related issue: https://github.com/not-fl3/miniquad/issues/165");
                }

                _sapp_init_event(sapp_event_type_SAPP_EVENTTYPE_RAW_DEVICE);
                _sapp.event.mouse_dx = data.data.mouse().lLastX as f32 * _sapp_win32_mouse_scale;
                _sapp.event.mouse_dy = data.data.mouse().lLastY as f32 * _sapp_win32_mouse_scale;
                _sapp_call_event(&_sapp.event);

                update_clip_rect(hWnd);
            }

            WM_MOUSELEAVE => {
                _sapp.win32_mouse_tracked = false;
                _sapp_win32_mouse_event(
                    sapp_event_type_SAPP_EVENTTYPE_MOUSE_LEAVE,
                    sapp_mousebutton_SAPP_MOUSEBUTTON_INVALID,
                );
            }
            WM_MOUSEWHEEL => {
                _sapp_win32_scroll_event(0.0, (HIWORD(wParam as _) as i16) as f32);
            }

            WM_MOUSEHWHEEL => {
                _sapp_win32_scroll_event((HIWORD(wParam as _) as i16) as f32, 0.0);
            }
            WM_CHAR => _sapp_win32_char_event(wParam as _, !!(lParam & 0x40000000) != 0),
            WM_KEYDOWN | WM_SYSKEYDOWN => _sapp_win32_key_event(
                sapp_event_type_SAPP_EVENTTYPE_KEY_DOWN,
                HIWORD(lParam as _) as u32 & 0x1FF,
                !!(lParam & 0x40000000) != 0,
            ),
            WM_KEYUP | WM_SYSKEYUP => _sapp_win32_key_event(
                sapp_event_type_SAPP_EVENTTYPE_KEY_UP,
                HIWORD(lParam as _) as u32 & 0x1FF,
                false,
            ),
            _ => {}
        }
    }

    DefWindowProcW(hWnd, uMsg, wParam, lParam)
}

unsafe fn init_state(desc: *const sapp_desc) {
    fn _sapp_def<T: PartialEq>(x: T, default: T, zero: T) -> T {
        if x == zero {
            default
        } else {
            x
        }
    }

    _sapp.desc = *desc;
    _sapp.first_frame = true;
    _sapp.window_width = _sapp_def(_sapp.desc.width, 640, 0);
    _sapp.window_height = _sapp_def(_sapp.desc.height, 480, 0);
    _sapp.framebuffer_width = _sapp.window_width;
    _sapp.framebuffer_height = _sapp.window_height;
    _sapp.sample_count = _sapp_def(_sapp.desc.sample_count, 1, 0);
    _sapp.swap_interval = _sapp_def(_sapp.desc.swap_interval, 1, 0);
    if _sapp.desc.window_title.is_null() {
        _sapp.window_title = "sokol_app".to_string();
    } else {
        _sapp.window_title = std::ffi::CStr::from_ptr(_sapp.desc.window_title)
            .to_string_lossy()
            .to_string();
    }
    _sapp.dpi_scale = 1.0;
}

unsafe fn _sapp_call_init() {
    if _sapp.desc.init_cb.is_some() {
        _sapp.desc.init_cb.expect("non-null function pointer")();
    } else if _sapp.desc.init_userdata_cb.is_some() {
        _sapp
            .desc
            .init_userdata_cb
            .expect("non-null function pointer")(_sapp.desc.user_data);
    }
    _sapp.init_called = true;
}

unsafe fn _sapp_call_frame() {
    if _sapp.init_called as i32 != 0 && !_sapp.cleanup_called {
        if _sapp.desc.frame_cb.is_some() {
            _sapp.desc.frame_cb.expect("non-null function pointer")();
        } else if _sapp.desc.frame_userdata_cb.is_some() {
            _sapp
                .desc
                .frame_userdata_cb
                .expect("non-null function pointer")(_sapp.desc.user_data);
        }
    };
}

unsafe fn _sapp_call_cleanup() {
    if !_sapp.cleanup_called {
        if _sapp.desc.cleanup_cb.is_some() {
            _sapp.desc.cleanup_cb.unwrap()();
        } else if _sapp.desc.cleanup_userdata_cb.is_some() {
            _sapp.desc.cleanup_userdata_cb.unwrap()(_sapp.desc.user_data);
        }
        _sapp.cleanup_called = true;
    }
}

unsafe fn _sapp_frame() {
    if _sapp.first_frame {
        _sapp.first_frame = false;
        _sapp_call_init();
    }
    _sapp_call_frame();
    _sapp.frame_count = _sapp.frame_count.wrapping_add(1);
}

unsafe fn init_keytable() {
    // same as GLFW
    _sapp.keycodes[0x00B] = sapp_keycode_SAPP_KEYCODE_0;
    _sapp.keycodes[0x002] = sapp_keycode_SAPP_KEYCODE_1;
    _sapp.keycodes[0x003] = sapp_keycode_SAPP_KEYCODE_2;
    _sapp.keycodes[0x004] = sapp_keycode_SAPP_KEYCODE_3;
    _sapp.keycodes[0x005] = sapp_keycode_SAPP_KEYCODE_4;
    _sapp.keycodes[0x006] = sapp_keycode_SAPP_KEYCODE_5;
    _sapp.keycodes[0x007] = sapp_keycode_SAPP_KEYCODE_6;
    _sapp.keycodes[0x008] = sapp_keycode_SAPP_KEYCODE_7;
    _sapp.keycodes[0x009] = sapp_keycode_SAPP_KEYCODE_8;
    _sapp.keycodes[0x00A] = sapp_keycode_SAPP_KEYCODE_9;
    _sapp.keycodes[0x01E] = sapp_keycode_SAPP_KEYCODE_A;
    _sapp.keycodes[0x030] = sapp_keycode_SAPP_KEYCODE_B;
    _sapp.keycodes[0x02E] = sapp_keycode_SAPP_KEYCODE_C;
    _sapp.keycodes[0x020] = sapp_keycode_SAPP_KEYCODE_D;
    _sapp.keycodes[0x012] = sapp_keycode_SAPP_KEYCODE_E;
    _sapp.keycodes[0x021] = sapp_keycode_SAPP_KEYCODE_F;
    _sapp.keycodes[0x022] = sapp_keycode_SAPP_KEYCODE_G;
    _sapp.keycodes[0x023] = sapp_keycode_SAPP_KEYCODE_H;
    _sapp.keycodes[0x017] = sapp_keycode_SAPP_KEYCODE_I;
    _sapp.keycodes[0x024] = sapp_keycode_SAPP_KEYCODE_J;
    _sapp.keycodes[0x025] = sapp_keycode_SAPP_KEYCODE_K;
    _sapp.keycodes[0x026] = sapp_keycode_SAPP_KEYCODE_L;
    _sapp.keycodes[0x032] = sapp_keycode_SAPP_KEYCODE_M;
    _sapp.keycodes[0x031] = sapp_keycode_SAPP_KEYCODE_N;
    _sapp.keycodes[0x018] = sapp_keycode_SAPP_KEYCODE_O;
    _sapp.keycodes[0x019] = sapp_keycode_SAPP_KEYCODE_P;
    _sapp.keycodes[0x010] = sapp_keycode_SAPP_KEYCODE_Q;
    _sapp.keycodes[0x013] = sapp_keycode_SAPP_KEYCODE_R;
    _sapp.keycodes[0x01F] = sapp_keycode_SAPP_KEYCODE_S;
    _sapp.keycodes[0x014] = sapp_keycode_SAPP_KEYCODE_T;
    _sapp.keycodes[0x016] = sapp_keycode_SAPP_KEYCODE_U;
    _sapp.keycodes[0x02F] = sapp_keycode_SAPP_KEYCODE_V;
    _sapp.keycodes[0x011] = sapp_keycode_SAPP_KEYCODE_W;
    _sapp.keycodes[0x02D] = sapp_keycode_SAPP_KEYCODE_X;
    _sapp.keycodes[0x015] = sapp_keycode_SAPP_KEYCODE_Y;
    _sapp.keycodes[0x02C] = sapp_keycode_SAPP_KEYCODE_Z;
    _sapp.keycodes[0x028] = sapp_keycode_SAPP_KEYCODE_APOSTROPHE;
    _sapp.keycodes[0x02B] = sapp_keycode_SAPP_KEYCODE_BACKSLASH;
    _sapp.keycodes[0x033] = sapp_keycode_SAPP_KEYCODE_COMMA;
    _sapp.keycodes[0x00D] = sapp_keycode_SAPP_KEYCODE_EQUAL;
    _sapp.keycodes[0x029] = sapp_keycode_SAPP_KEYCODE_GRAVE_ACCENT;
    _sapp.keycodes[0x01A] = sapp_keycode_SAPP_KEYCODE_LEFT_BRACKET;
    _sapp.keycodes[0x00C] = sapp_keycode_SAPP_KEYCODE_MINUS;
    _sapp.keycodes[0x034] = sapp_keycode_SAPP_KEYCODE_PERIOD;
    _sapp.keycodes[0x01B] = sapp_keycode_SAPP_KEYCODE_RIGHT_BRACKET;
    _sapp.keycodes[0x027] = sapp_keycode_SAPP_KEYCODE_SEMICOLON;
    _sapp.keycodes[0x035] = sapp_keycode_SAPP_KEYCODE_SLASH;
    _sapp.keycodes[0x056] = sapp_keycode_SAPP_KEYCODE_WORLD_2;
    _sapp.keycodes[0x00E] = sapp_keycode_SAPP_KEYCODE_BACKSPACE;
    _sapp.keycodes[0x153] = sapp_keycode_SAPP_KEYCODE_DELETE;
    _sapp.keycodes[0x14F] = sapp_keycode_SAPP_KEYCODE_END;
    _sapp.keycodes[0x01C] = sapp_keycode_SAPP_KEYCODE_ENTER;
    _sapp.keycodes[0x001] = sapp_keycode_SAPP_KEYCODE_ESCAPE;
    _sapp.keycodes[0x147] = sapp_keycode_SAPP_KEYCODE_HOME;
    _sapp.keycodes[0x152] = sapp_keycode_SAPP_KEYCODE_INSERT;
    _sapp.keycodes[0x15D] = sapp_keycode_SAPP_KEYCODE_MENU;
    _sapp.keycodes[0x151] = sapp_keycode_SAPP_KEYCODE_PAGE_DOWN;
    _sapp.keycodes[0x149] = sapp_keycode_SAPP_KEYCODE_PAGE_UP;
    _sapp.keycodes[0x045] = sapp_keycode_SAPP_KEYCODE_PAUSE;
    _sapp.keycodes[0x146] = sapp_keycode_SAPP_KEYCODE_PAUSE;
    _sapp.keycodes[0x039] = sapp_keycode_SAPP_KEYCODE_SPACE;
    _sapp.keycodes[0x00F] = sapp_keycode_SAPP_KEYCODE_TAB;
    _sapp.keycodes[0x03A] = sapp_keycode_SAPP_KEYCODE_CAPS_LOCK;
    _sapp.keycodes[0x145] = sapp_keycode_SAPP_KEYCODE_NUM_LOCK;
    _sapp.keycodes[0x046] = sapp_keycode_SAPP_KEYCODE_SCROLL_LOCK;
    _sapp.keycodes[0x03B] = sapp_keycode_SAPP_KEYCODE_F1;
    _sapp.keycodes[0x03C] = sapp_keycode_SAPP_KEYCODE_F2;
    _sapp.keycodes[0x03D] = sapp_keycode_SAPP_KEYCODE_F3;
    _sapp.keycodes[0x03E] = sapp_keycode_SAPP_KEYCODE_F4;
    _sapp.keycodes[0x03F] = sapp_keycode_SAPP_KEYCODE_F5;
    _sapp.keycodes[0x040] = sapp_keycode_SAPP_KEYCODE_F6;
    _sapp.keycodes[0x041] = sapp_keycode_SAPP_KEYCODE_F7;
    _sapp.keycodes[0x042] = sapp_keycode_SAPP_KEYCODE_F8;
    _sapp.keycodes[0x043] = sapp_keycode_SAPP_KEYCODE_F9;
    _sapp.keycodes[0x044] = sapp_keycode_SAPP_KEYCODE_F10;
    _sapp.keycodes[0x057] = sapp_keycode_SAPP_KEYCODE_F11;
    _sapp.keycodes[0x058] = sapp_keycode_SAPP_KEYCODE_F12;
    _sapp.keycodes[0x064] = sapp_keycode_SAPP_KEYCODE_F13;
    _sapp.keycodes[0x065] = sapp_keycode_SAPP_KEYCODE_F14;
    _sapp.keycodes[0x066] = sapp_keycode_SAPP_KEYCODE_F15;
    _sapp.keycodes[0x067] = sapp_keycode_SAPP_KEYCODE_F16;
    _sapp.keycodes[0x068] = sapp_keycode_SAPP_KEYCODE_F17;
    _sapp.keycodes[0x069] = sapp_keycode_SAPP_KEYCODE_F18;
    _sapp.keycodes[0x06A] = sapp_keycode_SAPP_KEYCODE_F19;
    _sapp.keycodes[0x06B] = sapp_keycode_SAPP_KEYCODE_F20;
    _sapp.keycodes[0x06C] = sapp_keycode_SAPP_KEYCODE_F21;
    _sapp.keycodes[0x06D] = sapp_keycode_SAPP_KEYCODE_F22;
    _sapp.keycodes[0x06E] = sapp_keycode_SAPP_KEYCODE_F23;
    _sapp.keycodes[0x076] = sapp_keycode_SAPP_KEYCODE_F24;
    _sapp.keycodes[0x038] = sapp_keycode_SAPP_KEYCODE_LEFT_ALT;
    _sapp.keycodes[0x01D] = sapp_keycode_SAPP_KEYCODE_LEFT_CONTROL;
    _sapp.keycodes[0x02A] = sapp_keycode_SAPP_KEYCODE_LEFT_SHIFT;
    _sapp.keycodes[0x15B] = sapp_keycode_SAPP_KEYCODE_LEFT_SUPER;
    _sapp.keycodes[0x137] = sapp_keycode_SAPP_KEYCODE_PRINT_SCREEN;
    _sapp.keycodes[0x138] = sapp_keycode_SAPP_KEYCODE_RIGHT_ALT;
    _sapp.keycodes[0x11D] = sapp_keycode_SAPP_KEYCODE_RIGHT_CONTROL;
    _sapp.keycodes[0x036] = sapp_keycode_SAPP_KEYCODE_RIGHT_SHIFT;
    _sapp.keycodes[0x15C] = sapp_keycode_SAPP_KEYCODE_RIGHT_SUPER;
    _sapp.keycodes[0x150] = sapp_keycode_SAPP_KEYCODE_DOWN;
    _sapp.keycodes[0x14B] = sapp_keycode_SAPP_KEYCODE_LEFT;
    _sapp.keycodes[0x14D] = sapp_keycode_SAPP_KEYCODE_RIGHT;
    _sapp.keycodes[0x148] = sapp_keycode_SAPP_KEYCODE_UP;
    _sapp.keycodes[0x052] = sapp_keycode_SAPP_KEYCODE_KP_0;
    _sapp.keycodes[0x04F] = sapp_keycode_SAPP_KEYCODE_KP_1;
    _sapp.keycodes[0x050] = sapp_keycode_SAPP_KEYCODE_KP_2;
    _sapp.keycodes[0x051] = sapp_keycode_SAPP_KEYCODE_KP_3;
    _sapp.keycodes[0x04B] = sapp_keycode_SAPP_KEYCODE_KP_4;
    _sapp.keycodes[0x04C] = sapp_keycode_SAPP_KEYCODE_KP_5;
    _sapp.keycodes[0x04D] = sapp_keycode_SAPP_KEYCODE_KP_6;
    _sapp.keycodes[0x047] = sapp_keycode_SAPP_KEYCODE_KP_7;
    _sapp.keycodes[0x048] = sapp_keycode_SAPP_KEYCODE_KP_8;
    _sapp.keycodes[0x049] = sapp_keycode_SAPP_KEYCODE_KP_9;
    _sapp.keycodes[0x04E] = sapp_keycode_SAPP_KEYCODE_KP_ADD;
    _sapp.keycodes[0x053] = sapp_keycode_SAPP_KEYCODE_KP_DECIMAL;
    _sapp.keycodes[0x135] = sapp_keycode_SAPP_KEYCODE_KP_DIVIDE;
    _sapp.keycodes[0x11C] = sapp_keycode_SAPP_KEYCODE_KP_ENTER;
    _sapp.keycodes[0x037] = sapp_keycode_SAPP_KEYCODE_KP_MULTIPLY;
    _sapp.keycodes[0x04A] = sapp_keycode_SAPP_KEYCODE_KP_SUBTRACT;
}

/// updates current window and framebuffer size from the window's client rect,
/// returns true if size has changed
unsafe fn update_dimensions() -> bool {
    let mut rect: RECT = std::mem::zeroed();

    if GetClientRect(_sapp_win32_hwnd, &mut rect as *mut _ as _) != 0 {
        _sapp.window_width = ((rect.right - rect.left) as f32 / _sapp_win32_window_scale) as i32;
        _sapp.window_height = ((rect.bottom - rect.top) as f32 / _sapp_win32_window_scale) as i32;
        let fb_width = (_sapp.window_width as f32 * _sapp_win32_content_scale) as i32;
        let fb_height = (_sapp.window_height as f32 * _sapp_win32_content_scale) as i32;
        if fb_width != _sapp.framebuffer_width || fb_height != _sapp.framebuffer_height {
            _sapp.framebuffer_width =
                (_sapp.window_width as f32 * _sapp_win32_content_scale) as i32;
            _sapp.framebuffer_height =
                (_sapp.window_height as f32 * _sapp_win32_content_scale) as i32;

            // prevent a framebuffer size of 0 when window is minimized
            if _sapp.framebuffer_width == 0 {
                _sapp.framebuffer_width = 1;
            }
            if _sapp.framebuffer_height == 0 {
                _sapp.framebuffer_height = 1;
            }
            return true;
        }
    } else {
        _sapp.window_width = 1;
        _sapp.window_height = 1;
        _sapp.framebuffer_width = 1;
        _sapp.framebuffer_height = 1;
    }
    return false;
}

unsafe fn init_dpi() {
    assert!(_sapp_win32_setprocessdpiaware.is_none());
    assert!(_sapp_win32_setprocessdpiawareness.is_none());
    assert!(_sapp_win32_getdpiformonitor.is_none());

    let user32 = LoadLibraryA(b"user32.dll\0".as_ptr() as *const _);

    if user32.is_null() == false {
        _sapp_win32_setprocessdpiaware = get_proc_address(user32, b"SetProcessDPIAware\0");
    }

    let shcore = LoadLibraryA(b"shcore.dll\0".as_ptr() as *const _);

    if shcore.is_null() == false {
        _sapp_win32_setprocessdpiawareness = get_proc_address(shcore, b"SetProcessDpiAwareness\0");
        _sapp_win32_getdpiformonitor = get_proc_address(shcore, b"GetDpiForMonitor\0");
    }

    if let Some(setprocessdpiawareness) = _sapp_win32_setprocessdpiawareness {
        // if the app didn't request HighDPI rendering, let Windows do the upscaling
        let mut process_dpi_awareness = PROCESS_SYSTEM_DPI_AWARE;
        _sapp_win32_dpi_aware = true;
        if !_sapp.desc.high_dpi {
            process_dpi_awareness = PROCESS_DPI_UNAWARE;
            _sapp_win32_dpi_aware = false;
        }
        setprocessdpiawareness(process_dpi_awareness);
    } else if let Some(setprocessdpiaware) = _sapp_win32_setprocessdpiaware {
        setprocessdpiaware();
        _sapp_win32_dpi_aware = true;
    }
    // get dpi scale factor for main monitor
    if let Some(getdpiformonitor) = _sapp_win32_getdpiformonitor {
        if _sapp_win32_dpi_aware {
            let pt = POINT { x: 1, y: 1 };
            let hm = MonitorFromPoint(pt, MONITOR_DEFAULTTONEAREST);
            let mut dpix: UINT = 0;
            let mut dpiy: UINT = 0;
            let hr = getdpiformonitor(
                hm,
                MDT_EFFECTIVE_DPI,
                &mut dpix as *mut _ as _,
                &mut dpiy as *mut _ as _,
            );
            assert_eq!(hr, 0);
            //  clamp window scale to an integer factor
            _sapp_win32_window_scale = dpix as f32 / 96.0;
        }
    } else {
        _sapp_win32_window_scale = 1.0;
    }
    if _sapp.desc.high_dpi {
        _sapp_win32_content_scale = _sapp_win32_window_scale;
        _sapp_win32_mouse_scale = 1.0;
    } else {
        _sapp_win32_content_scale = 1.0;
        _sapp_win32_mouse_scale = 1.0 / _sapp_win32_window_scale;
    }
    _sapp.dpi_scale = _sapp_win32_content_scale;
    if user32.is_null() == false {
        FreeLibrary(user32);
    }
    if shcore.is_null() == false {
        FreeLibrary(shcore);
    }
}

unsafe fn create_win_icon_from_image(width: u32, height: u32, colors: *const u8) -> Option<HICON> {
    let mut bi: BITMAPV5HEADER = std::mem::zeroed();

    bi.bV5Size = std::mem::size_of::<BITMAPV5HEADER>() as _;
    bi.bV5Width = width as i32;
    bi.bV5Height = -(height as i32); // NOTE the '-' here to indicate that origin is top-left
    bi.bV5Planes = 1;
    bi.bV5BitCount = 32;
    bi.bV5Compression = BI_BITFIELDS;
    bi.bV5RedMask = 0x00FF0000;
    bi.bV5GreenMask = 0x0000FF00;
    bi.bV5BlueMask = 0x000000FF;
    bi.bV5AlphaMask = 0xFF000000;

    let mut target = std::ptr::null_mut();
    // const uint8_t* source = (const uint8_t*)desc->pixels.ptr;

    let dc = GetDC(std::ptr::null_mut());
    let color = CreateDIBSection(
        dc,
        &bi as *const _ as *const BITMAPINFO,
        DIB_RGB_COLORS,
        &mut target,
        std::ptr::null_mut(),
        0,
    );
    ReleaseDC(std::ptr::null_mut(), dc);
    if color.is_null() {
        return None;
    }
    assert!(target.is_null() == false);

    let mask = CreateBitmap(width as _, height as _, 1, 1, std::ptr::null());
    if mask.is_null() {
        DeleteObject(color as *mut _);
        return None;
    }

    let source = std::slice::from_raw_parts(colors, width as usize * height as usize * 4);
    for i in 0..width as usize * height as usize {
        *(target as *mut u8).offset(i as isize * 4 + 0) = source[i * 4 + 2];
        *(target as *mut u8).offset(i as isize * 4 + 1) = source[i * 4 + 1];
        *(target as *mut u8).offset(i as isize * 4 + 2) = source[i * 4 + 0];
        *(target as *mut u8).offset(i as isize * 4 + 3) = source[i * 4 + 3];
    }

    let mut icon_info: ICONINFO = std::mem::zeroed();
    icon_info.fIcon = 1;
    icon_info.xHotspot = 0;
    icon_info.yHotspot = 0;
    icon_info.hbmMask = mask;
    icon_info.hbmColor = color;
    let icon_handle = CreateIconIndirect(&mut icon_info);
    DeleteObject(color as *mut _);
    DeleteObject(mask as *mut _);

    Some(icon_handle)
}

unsafe fn set_icon(icon: sapp_icon) {
    let big_icon_w = GetSystemMetrics(SM_CXICON);
    let big_icon_h = GetSystemMetrics(SM_CYICON);
    let small_icon_w = GetSystemMetrics(SM_CXSMICON);
    let small_icon_h = GetSystemMetrics(SM_CYSMICON);

    let big_icon = if big_icon_w * big_icon_h >= 64 * 64 {
        (icon.big, 64, 64)
    } else {
        (icon.medium, 32, 32)
    };

    let small_icon = if small_icon_w * small_icon_h <= 16 * 16 {
        (icon.small, 16, 16)
    } else {
        (icon.medium, 32, 32)
    };

    let big_icon = create_win_icon_from_image(big_icon.1, big_icon.2, big_icon.0);
    let small_icon = create_win_icon_from_image(small_icon.1, small_icon.2, small_icon.0);
    if let Some(icon) = big_icon {
        SendMessageW(_sapp_win32_hwnd, WM_SETICON, ICON_BIG as _, icon as LPARAM);
    }
    if let Some(icon) = small_icon {
        SendMessageW(
            _sapp_win32_hwnd,
            WM_SETICON,
            ICON_SMALL as _,
            icon as LPARAM,
        );
    }
}

unsafe fn create_window() {
    let mut wndclassw: WNDCLASSW = std::mem::zeroed();

    wndclassw.style = CS_HREDRAW | CS_VREDRAW | CS_OWNDC;
    wndclassw.lpfnWndProc = Some(win32_wndproc);
    wndclassw.hInstance = GetModuleHandleW(NULL as _);
    wndclassw.hCursor = LoadCursorW(NULL as _, IDC_ARROW);
    wndclassw.hIcon = LoadIconW(NULL as _, IDI_WINLOGO);
    let class_name = "MINIQUADAPP\0".encode_utf16().collect::<Vec<u16>>();
    wndclassw.lpszClassName = class_name.as_ptr() as _;
    RegisterClassW(&wndclassw);

    let win_ex_style: DWORD = WS_EX_APPWINDOW | WS_EX_WINDOWEDGE;
    let mut rect = RECT {
        left: 0,
        top: 0,
        right: 0,
        bottom: 0,
    };

    let win_style: DWORD = get_win_style();
    if _sapp.desc.fullscreen {
        rect.right = GetSystemMetrics(SM_CXSCREEN);
        rect.bottom = GetSystemMetrics(SM_CYSCREEN);
    } else {
        rect.right = (_sapp.window_width as f32 * _sapp_win32_window_scale) as _;
        rect.bottom = (_sapp.window_height as f32 * _sapp_win32_window_scale) as _;
    }

    AdjustWindowRectEx(&rect as *const _ as _, win_style, false as _, win_ex_style);
    let win_width = rect.right - rect.left;
    let win_height = rect.bottom - rect.top;
    _sapp_win32_in_create_window = true;
    let class_name = "MINIQUADAPP\0".encode_utf16().collect::<Vec<u16>>();
    let mut window_name = _sapp.window_title.encode_utf16().collect::<Vec<u16>>();
    window_name.push(0);
    _sapp_win32_hwnd = CreateWindowExW(
        win_ex_style,                // dwExStyle
        class_name.as_ptr(),         // lpClassName
        window_name.as_ptr(),        // lpWindowName
        win_style,                   // dwStyle
        CW_USEDEFAULT,               // X
        CW_USEDEFAULT,               // Y
        win_width,                   // nWidth
        win_height,                  // nHeight
        NULL as _,                   // hWndParent
        NULL as _,                   // hMenu
        GetModuleHandleW(NULL as _), // hInstance
        NULL as _,                   // lParam
    );
    assert!(_sapp_win32_hwnd.is_null() == false);
    ShowWindow(_sapp_win32_hwnd, SW_SHOW);
    _sapp_win32_in_create_window = false;
    let dc = GetDC(_sapp_win32_hwnd);
    assert!(dc.is_null() == false);
    _sapp_win32_dc = dc;
    update_dimensions();
}

unsafe fn destroy_window() {
    DestroyWindow(_sapp_win32_hwnd);
    _sapp_win32_hwnd = std::ptr::null_mut();

    let mut class_name = "MINIQUADAPP\0".encode_utf16().collect::<Vec<u16>>();

    UnregisterClassW(class_name.as_mut_ptr() as _, GetModuleHandleW(NULL as _));
}

unsafe fn get_proc_address<T>(lib: HINSTANCE, proc: &[u8]) -> Option<T> {
    let proc = GetProcAddress(lib, proc.as_ptr() as *const _);

    if proc.is_null() {
        return None;
    }
    return Some(std::mem::transmute_copy(&proc));
}

unsafe fn get_wgl_proc_address<T>(proc: &[u8]) -> Option<T> {
    let proc = _sapp_wglGetProcAddress.unwrap()(proc.as_ptr() as *const _);

    if proc.is_null() {
        return None;
    }
    return Some(std::mem::transmute_copy(&proc));
}

unsafe fn _sapp_wgl_ext_supported(ext: &str) -> bool {
    if let Some(getExtensionsStringEXT) = _sapp_GetExtensionsStringEXT {
        let extensions = getExtensionsStringEXT();

        if extensions.is_null() == false {
            let extensions_string = std::ffi::CStr::from_ptr(extensions).to_string_lossy();
            if extensions_string.contains(ext) {
                return true;
            }
        }
    }

    if let Some(getExtensionsStringARB) = _sapp_GetExtensionsStringARB {
        let extensions = getExtensionsStringARB(_sapp_wglGetCurrentDC.unwrap()());
        if extensions.is_null() == false {
            let extensions_string = std::ffi::CStr::from_ptr(extensions).to_string_lossy();

            if extensions_string.contains(ext) {
                return true;
            }
        }
    }
    return false;
}

unsafe fn wgl_load_extensions() {
    assert!(_sapp_win32_msg_dc.is_null() == false);

    let mut pfd: PIXELFORMATDESCRIPTOR = std::mem::zeroed();
    pfd.nSize = std::mem::size_of_val(&pfd) as _;
    pfd.nVersion = 1;
    pfd.dwFlags = PFD_DRAW_TO_WINDOW | PFD_SUPPORT_OPENGL | PFD_DOUBLEBUFFER;
    pfd.iPixelType = PFD_TYPE_RGBA;
    pfd.cColorBits = 24;
    if SetPixelFormat(
        _sapp_win32_msg_dc,
        ChoosePixelFormat(_sapp_win32_msg_dc, &pfd),
        &pfd,
    ) == 0
    {
        panic!("WGL: failed to set pixel format for dummy context");
    }
    let rc = _sapp_wglCreateContext.unwrap()(_sapp_win32_msg_dc);
    if rc.is_null() {
        panic!("WGL: Failed to create dummy context");
    }
    if _sapp_wglMakeCurrent.unwrap()(_sapp_win32_msg_dc, rc) == false {
        panic!("WGL: Failed to make context current");
    }
    _sapp_GetExtensionsStringEXT = get_wgl_proc_address(b"wglGetExtensionsStringEXT\0");
    _sapp_GetExtensionsStringARB = get_wgl_proc_address(b"wglGetExtensionsStringARB\0");
    _sapp_CreateContextAttribsARB = get_wgl_proc_address(b"wglCreateContextAttribsARB\0");
    _sapp_SwapIntervalEXT = get_wgl_proc_address(b"wglSwapIntervalEXT\0");
    _sapp_GetPixelFormatAttribivARB = get_wgl_proc_address(b"wglGetPixelFormatAttribivARB\0");
    _sapp_arb_multisample = _sapp_wgl_ext_supported("WGL_ARB_multisample");
    _sapp_arb_create_context = _sapp_wgl_ext_supported("WGL_ARB_create_context");
    _sapp_arb_create_context_profile = _sapp_wgl_ext_supported("WGL_ARB_create_context_profile");
    _sapp_ext_swap_control = _sapp_wgl_ext_supported("WGL_EXT_swap_control");
    _sapp_arb_pixel_format = _sapp_wgl_ext_supported("WGL_ARB_pixel_format");
    _sapp_wglMakeCurrent.unwrap()(_sapp_win32_msg_dc, std::ptr::null_mut() as _);
    _sapp_wglDeleteContext.unwrap()(rc);
}

#[derive(Copy, Clone)]
pub struct _sapp_gl_fbconfig {
    pub red_bits: i32,
    pub green_bits: i32,
    pub blue_bits: i32,
    pub alpha_bits: i32,
    pub depth_bits: i32,
    pub stencil_bits: i32,
    pub samples: i32,
    pub doublebuffer: bool,
    pub handle: u32,
}

impl Default for _sapp_gl_fbconfig {
    fn default() -> Self {
        // -1 means "don't care"
        _sapp_gl_fbconfig {
            red_bits: -1,
            green_bits: -1,
            blue_bits: -1,
            alpha_bits: -1,
            depth_bits: -1,
            stencil_bits: -1,
            samples: -1,
            doublebuffer: false,
            handle: 0,
        }
    }
}
pub unsafe fn _sapp_gl_choose_fbconfig(
    desired: &mut _sapp_gl_fbconfig,
    alternatives: &[_sapp_gl_fbconfig],
) -> Option<usize> {
    let mut missing: i32;
    let mut least_missing: i32 = 1000000;
    let mut color_diff: i32;
    let mut least_color_diff: i32 = 10000000;
    let mut extra_diff: i32;
    let mut least_extra_diff: i32 = 10000000;
    let mut closest = None;

    for (i, current) in alternatives.iter().enumerate() {
        if desired.doublebuffer == current.doublebuffer {
            missing = 0;
            if desired.alpha_bits > 0 && current.alpha_bits == 0 {
                missing += 1;
            }
            if desired.depth_bits > 0 && current.depth_bits == 0 {
                missing += 1;
            }
            if desired.stencil_bits > 0 && current.stencil_bits == 0 {
                missing += 1;
            }
            if desired.samples > 0 && current.samples == 0 {
                // Technically, several multisampling buffers could be
                //  involved, but that's a lower level implentation detail and
                //  not important to us here, so we count them as one

                missing += 1;
            }

            // These polynomials make many small channel size differences matter
            //  less than one large channel size difference
            //  Calculate color channel size difference value

            color_diff = 0;
            if desired.red_bits != -1 {
                color_diff +=
                    (desired.red_bits - current.red_bits) * (desired.red_bits - current.red_bits);
            }
            if desired.green_bits != -1 {
                color_diff += (desired.green_bits - current.green_bits)
                    * (desired.green_bits - current.green_bits)
            }
            if desired.blue_bits != -1 {
                color_diff += (desired.blue_bits - current.blue_bits)
                    * (desired.blue_bits - current.blue_bits)
            }

            // Calculate non-color channel size difference value
            extra_diff = 0;
            if desired.alpha_bits != -1 {
                extra_diff += (desired.alpha_bits - current.alpha_bits)
                    * (desired.alpha_bits - current.alpha_bits)
            }
            if desired.depth_bits != -1 {
                extra_diff += (desired.depth_bits - current.depth_bits)
                    * (desired.depth_bits - current.depth_bits);
            }
            if desired.stencil_bits != -1 {
                extra_diff = (desired.stencil_bits - current.stencil_bits)
                    * (desired.stencil_bits - current.stencil_bits);
            }
            if desired.samples != -1 {
                extra_diff +=
                    (desired.samples - current.samples) * (desired.samples - current.samples);
            }
            if missing < least_missing {
                closest = Some(i);
            } else if missing == least_missing {
                if color_diff < least_color_diff
                    || color_diff == least_color_diff && extra_diff < least_extra_diff
                {
                    closest = Some(i);
                }
            }

            // Figure out if the current one is better than the best one found so far
            //  Least number of missing buffers is the most important heuristic,
            //  then color buffer size match and lastly size match for other buffers

            if closest.map_or(false, |closest| closest == i) {
                least_missing = missing;
                least_color_diff = color_diff;
                least_extra_diff = extra_diff
            }
        }
    }
    closest
}

unsafe fn _sapp_wgl_attrib(pixel_format: i32, attrib: i32) -> i32 {
    assert!(_sapp_arb_pixel_format);
    let mut value = 0;
    if !_sapp_GetPixelFormatAttribivARB.unwrap()(
        _sapp_win32_dc,
        pixel_format,
        0,
        1,
        &attrib,
        &mut value as *mut _,
    ) {
        panic!("WGL: Failed to retrieve pixel format attribute");
    }
    return value;
}

unsafe fn wgl_find_pixel_format() -> u32 {
    assert!(_sapp_win32_dc.is_null() == false);
    assert!(_sapp_arb_pixel_format);
    // const _sapp_gl_fbconfig* closest;

    let native_count = _sapp_wgl_attrib(1, WGL_NUMBER_PIXEL_FORMATS_ARB as _);
    // _sapp_gl_fbconfig* usable_configs = (_sapp_gl_fbconfig*) SOKOL_CALLOC(native_count, sizeof(_sapp_gl_fbconfig));
    let mut usable_configs = vec![_sapp_gl_fbconfig::default(); native_count as usize];

    let mut usable_count = 0;
    for i in 0..native_count {
        let n = i + 1;
        let u = &mut usable_configs[usable_count];
        *u = Default::default();
        if _sapp_wgl_attrib(n, WGL_SUPPORT_OPENGL_ARB as _) == 0
            || _sapp_wgl_attrib(n, WGL_DRAW_TO_WINDOW_ARB as _) == 0
        {
            continue;
        }
        if _sapp_wgl_attrib(n, WGL_PIXEL_TYPE_ARB as _) != WGL_TYPE_RGBA_ARB as _ {
            continue;
        }
        if _sapp_wgl_attrib(n, WGL_ACCELERATION_ARB as _) == WGL_NO_ACCELERATION_ARB as _ {
            continue;
        }
        u.red_bits = _sapp_wgl_attrib(n, WGL_RED_BITS_ARB as _);
        u.green_bits = _sapp_wgl_attrib(n, WGL_GREEN_BITS_ARB as _);
        u.blue_bits = _sapp_wgl_attrib(n, WGL_BLUE_BITS_ARB as _);
        u.alpha_bits = _sapp_wgl_attrib(n, WGL_ALPHA_BITS_ARB as _);
        u.depth_bits = _sapp_wgl_attrib(n, WGL_DEPTH_BITS_ARB as _);
        u.stencil_bits = _sapp_wgl_attrib(n, WGL_STENCIL_BITS_ARB as _);
        if _sapp_wgl_attrib(n, WGL_DOUBLE_BUFFER_ARB as _) != 0 {
            u.doublebuffer = true;
        }
        if _sapp_arb_multisample {
            u.samples = _sapp_wgl_attrib(n, WGL_SAMPLES_ARB as _);
        }
        u.handle = n as _;
        usable_count += 1;
    }
    assert!(usable_count > 0);

    let mut desired = _sapp_gl_fbconfig::default();
    desired.red_bits = 8;
    desired.green_bits = 8;
    desired.blue_bits = 8;
    desired.alpha_bits = 8;
    desired.depth_bits = 24;
    desired.stencil_bits = 8;
    desired.doublebuffer = true;
    desired.samples = if _sapp.sample_count > 1 {
        _sapp.sample_count
    } else {
        0
    };
    let closest = _sapp_gl_choose_fbconfig(&mut desired, &usable_configs[..]);
    let mut pixel_format = 0;
    if let Some(closest) = closest {
        pixel_format = usable_configs[closest].handle;
    }
    pixel_format
}

unsafe fn wgl_create_context() {
    let pixel_format = wgl_find_pixel_format();
    if 0 == pixel_format {
        panic!("WGL: Didn't find matching pixel format.");
    }
    let mut pfd: PIXELFORMATDESCRIPTOR = std::mem::zeroed();
    if DescribePixelFormat(
        _sapp_win32_dc,
        pixel_format as _,
        std::mem::size_of_val(&pfd) as _,
        &mut pfd as *mut _ as _,
    ) == 0
    {
        panic!("WGL: Failed to retrieve PFD for selected pixel format!");
    }
    if SetPixelFormat(_sapp_win32_dc, pixel_format as _, &pfd) == 0 {
        panic!("WGL: Failed to set selected pixel format!");
    }
    if !_sapp_arb_create_context {
        panic!("WGL: ARB_create_context required!\n");
    }
    if !_sapp_arb_create_context_profile {
        panic!("WGL: ARB_create_context_profile required!");
    }
    let attrs = [
        WGL_CONTEXT_MAJOR_VERSION_ARB,
        3,
        WGL_CONTEXT_MINOR_VERSION_ARB,
        3,
        WGL_CONTEXT_FLAGS_ARB,
        WGL_CONTEXT_FORWARD_COMPATIBLE_BIT_ARB,
        WGL_CONTEXT_PROFILE_MASK_ARB,
        WGL_CONTEXT_CORE_PROFILE_BIT_ARB,
        0,
        0,
    ];
    _sapp_gl_ctx = _sapp_CreateContextAttribsARB.unwrap()(
        _sapp_win32_dc,
        std::ptr::null_mut(),
        attrs.as_ptr() as *const _,
    );
    if _sapp_gl_ctx.is_null() {
        let err = GetLastError();
        if err == (0xc0070000 | ERROR_INVALID_VERSION_ARB) {
            panic!("WGL: Driver does not support OpenGL version 3.3");
        } else if err == (0xc0070000 | ERROR_INVALID_PROFILE_ARB) {
            panic!("WGL: Driver does not support the requested OpenGL profile");
        } else if err == (0xc0070000 | ERROR_INCOMPATIBLE_DEVICE_CONTEXTS_ARB) {
            panic!("WGL: The share context is not compatible with the requested context");
        } else {
            panic!("WGL: Failed to create OpenGL context");
        }
    }
    _sapp_wglMakeCurrent.unwrap()(_sapp_win32_dc, _sapp_gl_ctx);
    if _sapp_ext_swap_control {
        /* FIXME: DwmIsCompositionEnabled() (see GLFW) */
        _sapp_SwapIntervalEXT.unwrap()(_sapp.swap_interval);
    }
}

unsafe fn wgl_shutdown() {
    assert!(_sapp_opengl32.is_null() == false);
    assert!(_sapp_win32_msg_hwnd.is_null() == false);
    DestroyWindow(_sapp_win32_msg_hwnd);
    _sapp_win32_msg_hwnd = std::ptr::null_mut();
    FreeLibrary(_sapp_opengl32);
    _sapp_opengl32 = std::ptr::null_mut();
}

unsafe fn wgl_init() {
    _sapp_opengl32 = LoadLibraryA(b"opengl32.dll\0".as_ptr() as *const _);

    if _sapp_opengl32.is_null() {
        panic!("Failed to load opengl32.dll");
    }

    _sapp_wglCreateContext = get_proc_address(_sapp_opengl32, b"wglCreateContext\0");
    assert!(_sapp_wglCreateContext.is_some());
    _sapp_wglDeleteContext = get_proc_address(_sapp_opengl32, b"wglDeleteContext\0");
    assert!(_sapp_wglDeleteContext.is_some());

    _sapp_wglGetProcAddress = get_proc_address(_sapp_opengl32, b"wglGetProcAddress\0");
    assert!(_sapp_wglGetProcAddress.is_some());
    _sapp_wglGetCurrentDC = get_proc_address(_sapp_opengl32, b"wglGetCurrentDC\0");
    assert!(_sapp_wglGetCurrentDC.is_some());
    _sapp_wglMakeCurrent = get_proc_address(_sapp_opengl32, b"wglMakeCurrent\0");
    assert!(_sapp_wglMakeCurrent.is_some());

    let class_name = "MINIQUADAPP\0".encode_utf16().collect::<Vec<u16>>();
    let window_name = "miniquad message window\0"
        .encode_utf16()
        .collect::<Vec<u16>>();
    _sapp_win32_msg_hwnd = CreateWindowExW(
        WS_EX_OVERLAPPEDWINDOW,
        class_name.as_ptr() as _,
        window_name.as_ptr() as _,
        WS_CLIPSIBLINGS | WS_CLIPCHILDREN,
        0,
        0,
        1,
        1,
        NULL as _,
        NULL as _,
        GetModuleHandleW(NULL as _),
        NULL,
    );
    assert!(
        _sapp_win32_msg_hwnd.is_null() == false,
        "Win32: failed to create helper window!"
    );
    ShowWindow(_sapp_win32_msg_hwnd, SW_HIDE);
    let mut msg = std::mem::zeroed();
    while PeekMessageW(&mut msg as _, _sapp_win32_msg_hwnd, 0, 0, PM_REMOVE) != 0 {
        TranslateMessage(&msg);
        DispatchMessageW(&msg);
    }
    _sapp_win32_msg_dc = GetDC(_sapp_win32_msg_hwnd);
    assert!(
        _sapp_win32_msg_dc.is_null() == false,
        "Win32: failed to obtain helper window DC!"
    );
}

unsafe fn wgl_swap_buffers() {
    assert!(_sapp_win32_dc.is_null() == false);
    // FIXME: DwmIsCompositionEnabled? (see GLFW)
    SwapBuffers(_sapp_win32_dc);
}

unsafe fn wgl_destroy_context() {
    assert!(_sapp_gl_ctx.is_null() == false);
    _sapp_wglDeleteContext.unwrap()(_sapp_gl_ctx);
    _sapp_gl_ctx = std::ptr::null_mut();
}

pub unsafe fn sapp_run(desc: *const sapp_desc) -> i32 {
    init_state(desc);
    init_keytable();
    init_dpi();
    create_window();
    if let Some(icon) = (&*desc).icon {
        set_icon(icon);
    }

    wgl_init();
    wgl_load_extensions();
    wgl_create_context();

    gl::load_gl_funcs();

    _sapp.valid = true;

    let mut done = false;
    while !(done || _sapp.quit_ordered) {
        let mut msg: MSG = std::mem::zeroed();
        while PeekMessageW(&mut msg as *mut _ as _, NULL as _, 0, 0, PM_REMOVE) != 0 {
            if WM_QUIT == msg.message {
                done = true;
                continue;
            } else {
                TranslateMessage(&mut msg as *mut _ as _);
                DispatchMessageW(&mut msg as *mut _ as _);
            }
        }
        _sapp_frame();
        wgl_swap_buffers();

        if update_dimensions() {
            _sapp_win32_app_event(sapp_event_type_SAPP_EVENTTYPE_RESIZED);
        }
        if _sapp.quit_requested {
            PostMessageW(_sapp_win32_hwnd, WM_CLOSE, 0, 0);
        }
    }
    _sapp_call_cleanup();

    wgl_destroy_context();
    wgl_shutdown();
    destroy_window();

    return 0 as i32;
}

pub unsafe fn sapp_is_elapsed_timer_supported() -> bool {
    return true;
}
