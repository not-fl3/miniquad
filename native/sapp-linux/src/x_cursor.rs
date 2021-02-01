use crate::x::{Display, Window};
use crate::{_sapp_x11_display, _sapp_x11_window};

#[derive(Copy, Clone)]
#[repr(C)]
struct XColor {
    pub pixel: libc::c_ulong,
    pub red: libc::c_ushort,
    pub green: libc::c_ushort,
    pub blue: libc::c_ushort,
    pub flags: libc::c_char,
    pub pad: libc::c_char,
}
type XID = libc::c_ulong;
type Pixmap = XID;
type Drawable = XID;

pub type Cursor = XID;

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

extern "C" {
    fn XCreateFontCursor(_: *mut Display, _: libc::c_ushort) -> Cursor;

    fn XCreateBitmapFromData(
        _: *mut Display,
        _: Drawable,
        _: *const libc::c_char,
        _: libc::c_uint,
        _: libc::c_uint,
    ) -> Pixmap;

    fn XCreatePixmapCursor(
        _: *mut Display,
        _: Pixmap,
        _: Pixmap,
        _: *mut XColor,
        _: *mut XColor,
        _: libc::c_uint,
        _: libc::c_uint,
    ) -> Cursor;

    fn XFreePixmap(_: *mut Display, _: Pixmap) -> libc::c_int;

    pub fn XDefineCursor(_: *mut Display, _: Window, _: Cursor) -> libc::c_int;
}

pub unsafe fn create_empty_cursor() -> Cursor {
    let mut cursor_color = XColor {
        pixel: 0 as libc::c_int as libc::c_ulong,
        red: 0,
        green: 0,
        blue: 0,
        flags: 0,
        pad: 0,
    };

    let mut cursor_color_data: [libc::c_char; 1] = [0 as libc::c_int as libc::c_char];
    let mut cursor_pixmap = XCreateBitmapFromData(
        _sapp_x11_display,
        _sapp_x11_window,
        cursor_color_data.as_mut_ptr(),
        1 as libc::c_int as libc::c_uint,
        1 as libc::c_int as libc::c_uint,
    );
    let mut empty_cursor = XCreatePixmapCursor(
        _sapp_x11_display,
        cursor_pixmap,
        cursor_pixmap,
        &mut cursor_color,
        &mut cursor_color,
        0 as libc::c_int as libc::c_uint,
        0 as libc::c_int as libc::c_uint,
    );
    XFreePixmap(_sapp_x11_display, cursor_pixmap);

    empty_cursor
}

pub unsafe fn load_cursor(shape: libc::c_ushort) -> Cursor {
    XCreateFontCursor(_sapp_x11_display, shape)
}

pub unsafe fn set_cursor(cursor: Cursor) {
    XDefineCursor(_sapp_x11_display, _sapp_x11_window, cursor);
}
