use crate::x::{Display, Window};
use crate::{_sapp_x11_display, _sapp_x11_window};

#[derive(Copy, Clone)]
#[repr(C)]
struct XColor {
    pub pixel: cty::c_ulong,
    pub red: cty::c_ushort,
    pub green: cty::c_ushort,
    pub blue: cty::c_ushort,
    pub flags: cty::c_char,
    pub pad: cty::c_char,
}
type XID = cty::c_ulong;
type Pixmap = XID;
type Drawable = XID;

pub type Cursor = XID;

extern "C" {
    #[no_mangle]
    fn XCreateBitmapFromData(
        _: *mut Display,
        _: Drawable,
        _: *const cty::c_char,
        _: cty::c_uint,
        _: cty::c_uint,
    ) -> Pixmap;

    #[no_mangle]
    fn XCreatePixmapCursor(
        _: *mut Display,
        _: Pixmap,
        _: Pixmap,
        _: *mut XColor,
        _: *mut XColor,
        _: cty::c_uint,
        _: cty::c_uint,
    ) -> Cursor;

    #[no_mangle]
    fn XFreePixmap(_: *mut Display, _: Pixmap) -> cty::c_int;

    #[no_mangle]
    pub fn XDefineCursor(_: *mut Display, _: Window, _: Cursor) -> cty::c_int;
}

pub unsafe fn create_empty_cursor() -> Cursor {
    let mut cursor_color = XColor {
        pixel: 0 as cty::c_int as cty::c_ulong,
        red: 0,
        green: 0,
        blue: 0,
        flags: 0,
        pad: 0,
    };

    let mut cursor_color_data: [cty::c_char; 1] = [0 as cty::c_int as cty::c_char];
    let mut cursor_pixmap = XCreateBitmapFromData(
        _sapp_x11_display,
        _sapp_x11_window,
        cursor_color_data.as_mut_ptr(),
        1 as cty::c_int as cty::c_uint,
        1 as cty::c_int as cty::c_uint,
    );
    let mut empty_cursor = XCreatePixmapCursor(
        _sapp_x11_display,
        cursor_pixmap,
        cursor_pixmap,
        &mut cursor_color,
        &mut cursor_color,
        0 as cty::c_int as cty::c_uint,
        0 as cty::c_int as cty::c_uint,
    );
    XFreePixmap(_sapp_x11_display, cursor_pixmap);

    empty_cursor
}

pub unsafe fn set_cursor(cursor: Cursor) {
    XDefineCursor(_sapp_x11_display, _sapp_x11_window, cursor);
}
