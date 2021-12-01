use super::libx11::{Cursor, Display, LibX11, Window, XColor};

pub unsafe fn create_empty_cursor(
    display: *mut Display,
    root: Window,
    libx11: &mut LibX11,
) -> Cursor {
    let mut cursor_color = XColor {
        pixel: 0 as libc::c_int as libc::c_ulong,
        red: 0,
        green: 0,
        blue: 0,
        flags: 0,
        pad: 0,
    };

    let mut cursor_color_data: [libc::c_char; 1] = [0 as libc::c_int as libc::c_char];
    let cursor_pixmap = (libx11.XCreateBitmapFromData)(
        display,
        root,
        cursor_color_data.as_mut_ptr(),
        1 as libc::c_int as libc::c_uint,
        1 as libc::c_int as libc::c_uint,
    );
    let empty_cursor = (libx11.XCreatePixmapCursor)(
        display,
        cursor_pixmap,
        cursor_pixmap,
        &mut cursor_color,
        &mut cursor_color,
        0 as libc::c_int as libc::c_uint,
        0 as libc::c_int as libc::c_uint,
    );
    (libx11.XFreePixmap)(display, cursor_pixmap);

    empty_cursor
}
