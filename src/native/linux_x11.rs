//!
//! Spiritual successor of an X11 part of https://github.com/floooh/sokol/blob/master/sokol_app.h

mod clipboard;
mod glx;
mod keycodes;
pub mod libx11;
mod x_cursor;
mod xi_input;

use crate::{
    event::EventHandler,
    gl,
    graphics::GraphicsContext,
    native::{egl, NativeDisplayData},
    Context, CursorIcon,
};

use libx11::*;

use std::collections::HashMap;

pub struct Dummy;

struct X11Extensions {
    utf8_string: Atom,
    wm_protocols: Atom,
    wm_delete_window: Atom,
    _wm_state: Atom,
    net_wm_name: Atom,
    net_wm_icon_name: Atom,
}

impl X11Extensions {
    pub unsafe fn new(libx11: &mut LibX11, display: *mut Display) -> X11Extensions {
        X11Extensions {
            utf8_string: (libx11.XInternAtom)(
                display,
                b"UTF8_STRING\x00" as *const u8 as *const libc::c_char,
                false as _,
            ),
            wm_protocols: (libx11.XInternAtom)(
                display,
                b"WM_PROTOCOLS\x00" as *const u8 as *const libc::c_char,
                false as _,
            ),
            wm_delete_window: (libx11.XInternAtom)(
                display,
                b"WM_DELETE_WINDOW\x00" as *const u8 as *const libc::c_char,
                false as _,
            ),
            _wm_state: (libx11.XInternAtom)(
                display,
                b"WM_STATE\x00" as *const u8 as *const libc::c_char,
                false as _,
            ),
            net_wm_name: (libx11.XInternAtom)(
                display,
                b"_NET_WM_NAME\x00" as *const u8 as *const libc::c_char,
                false as _,
            ),
            net_wm_icon_name: (libx11.XInternAtom)(
                display,
                b"_NET_WM_ICON_NAME\x00" as *const u8 as *const libc::c_char,
                false as _,
            ),
        }
    }
}

pub struct X11Display {
    libx11: LibX11,
    libxi: xi_input::LibXi,
    screen: i32,
    display: *mut Display,
    root: Window,
    window: Window,
    dpi_scale: f32,
    extensions: X11Extensions,
    xi_extension_opcode: Option<i32>,
    repeated_keycodes: [bool; 256],
    empty_cursor: Option<libx11::Cursor>,
    cursor_cache: HashMap<CursorIcon, libx11::Cursor>,
    data: NativeDisplayData,
}

impl crate::native::NativeDisplay for X11Display {
    fn screen_size(&self) -> (f32, f32) {
        (self.data.screen_width as _, self.data.screen_height as _)
    }
    fn dpi_scale(&self) -> f32 {
        self.data.dpi_scale
    }
    fn high_dpi(&self) -> bool {
        self.data.high_dpi
    }
    fn order_quit(&mut self) {
        self.data.quit_ordered = true;
    }
    fn request_quit(&mut self) {
        self.data.quit_requested = true;
    }
    fn cancel_quit(&mut self) {
        self.data.quit_requested = false;
    }
    fn set_cursor_grab(&mut self, grab: bool) {
        unsafe {
            self.set_cursor_grab(self.window, grab);
        }
    }

    fn show_mouse(&mut self, shown: bool) {
        unsafe {
            if shown {
                self.set_cursor(self.window, Some(CursorIcon::Default));
            } else {
                self.set_cursor(self.window, None);
            }
        }
    }

    fn set_mouse_cursor(&mut self, cursor_icon: CursorIcon) {
        unsafe {
            self.set_cursor(self.window, Some(cursor_icon));
        }
    }

    fn set_window_size(&mut self, _new_width: u32, _new_height: u32) {
        println!("set_window_size not implemented on linux/x11")
    }

    fn set_fullscreen(&mut self, fullscreen: bool) {
        unsafe {
            self.set_fullscreen(self.window, fullscreen);
        }
    }

    fn clipboard_get(&mut self) -> Option<String> {
        use std::ffi::CString;

        let bufname = CString::new("CLIPBOARD").unwrap();
        let fmtname = CString::new("UTF8_STRING").unwrap();

        unsafe { clipboard::get_clipboard(self, bufname.as_ptr(), fmtname.as_ptr()) }
    }

    fn clipboard_set(&mut self, data: &str) {
        use std::ffi::CString;

        let bufname = CString::new("CLIPBOARD").unwrap();

        unsafe {
            clipboard::claim_clipboard_ownership(self, bufname.as_ptr(), data.to_owned());
        };
    }

    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl X11Display {
    unsafe fn update_system_dpi(&mut self) {
        let rms = (self.libx11.XResourceManagerString)(self.display);
        if !rms.is_null() {
            let db = (self.libx11.XrmGetStringDatabase)(rms);
            if !db.is_null() {
                let mut value = XrmValue {
                    size: 0,
                    addr: 0 as *mut libc::c_char,
                };
                let mut type_ = std::ptr::null_mut();
                if (self.libx11.XrmGetResource)(
                    db,
                    b"Xft.dpi\x00".as_ptr() as _,
                    b"Xft.Dpi\x00".as_ptr() as _,
                    &mut type_,
                    &mut value,
                ) != 0
                {
                    if !type_.is_null() && libc::strcmp(type_, b"String\x00".as_ptr() as _) == 0 {
                        self.dpi_scale = libc::atof(value.addr as *const _) as f32 / 96.0;
                    }
                }
                (self.libx11.XrmDestroyDatabase)(db);
            }
        };
    }

    unsafe fn grab_error_handler(&mut self) {
        pub unsafe extern "C" fn _sapp_x11_error_handler(
            mut _display: *mut Display,
            event: *mut XErrorEvent,
        ) -> libc::c_int {
            println!("Error: {}", (*event).error_code);
            return 0 as libc::c_int;
        }

        (self.libx11.XSetErrorHandler)(Some(
            _sapp_x11_error_handler
                as unsafe extern "C" fn(_: *mut Display, _: *mut XErrorEvent) -> libc::c_int,
        ));
    }
    unsafe fn release_error_handler(&mut self) {
        (self.libx11.XSync)(self.display, false as _);
        (self.libx11.XSetErrorHandler)(None);
    }
    unsafe fn create_window(
        &mut self,
        visual: *mut Visual,
        depth: libc::c_int,
        conf: &crate::conf::Conf,
    ) -> Window {
        let mut wa = XSetWindowAttributes {
            background_pixmap: 0,
            background_pixel: 0,
            border_pixmap: 0,
            border_pixel: 0,
            bit_gravity: 0,
            win_gravity: 0,
            backing_store: 0,
            backing_planes: 0,
            backing_pixel: 0,
            save_under: 0,
            event_mask: 0,
            do_not_propagate_mask: 0,
            override_redirect: 0,
            colormap: 0,
            cursor: 0,
        };
        libc::memset(
            &mut wa as *mut XSetWindowAttributes as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<XSetWindowAttributes>() as _,
        );
        let wamask = (CWBorderPixel | CWColormap | CWEventMask) as u32;

        if !visual.is_null() {
            let colormap =
                (self.libx11.XCreateColormap)(self.display, self.root, visual, AllocNone);
            wa.colormap = colormap;
        }
        wa.border_pixel = 0 as libc::c_int as libc::c_ulong;
        wa.event_mask = StructureNotifyMask
            | KeyPressMask
            | KeyReleaseMask
            | PointerMotionMask
            | ButtonPressMask
            | ButtonReleaseMask
            | ExposureMask
            | FocusChangeMask
            | VisibilityChangeMask
            | EnterWindowMask
            | LeaveWindowMask
            | PropertyChangeMask;
        self.grab_error_handler();

        let window = (self.libx11.XCreateWindow)(
            self.display,
            self.root,
            0 as libc::c_int,
            0 as libc::c_int,
            conf.window_width as _,
            conf.window_height as _,
            0 as libc::c_int as libc::c_uint,
            depth,
            InputOutput as libc::c_uint,
            visual,
            wamask as libc::c_ulong,
            &mut wa,
        );
        self.release_error_handler();
        assert!(window != 0, "X11: Failed to create window");

        self.xi_extension_opcode = self
            .libxi
            .query_xi_extension(&mut self.libx11, self.display);

        let empty_cursor = x_cursor::create_empty_cursor(self.display, self.root, &mut self.libx11);
        self.empty_cursor = Some(empty_cursor);

        let mut protocols: [Atom; 1] = [self.extensions.wm_delete_window];
        (self.libx11.XSetWMProtocols)(
            self.display,
            window,
            protocols.as_mut_ptr(),
            1 as libc::c_int,
        );
        let mut hints = (self.libx11.XAllocSizeHints)();
        (*hints).flags |= PWinGravity;
        if conf.window_resizable == false {
            (*hints).flags |= PMinSize | PMaxSize;
            (*hints).min_width = conf.window_width;
            (*hints).min_height = conf.window_height;
            (*hints).max_width = conf.window_width;
            (*hints).max_height = conf.window_height;
        }
        (*hints).win_gravity = StaticGravity;
        (self.libx11.XSetWMNormalHints)(self.display, window, hints);
        (self.libx11.XFree)(hints as *mut libc::c_void);

        self.update_window_title(window, &conf.window_title);

        window
    }

    unsafe fn show_window(&mut self, window: Window) {
        (self.libx11.XMapWindow)(self.display, window);
        (self.libx11.XRaiseWindow)(self.display, window);
        (self.libx11.XFlush)(self.display);
    }

    unsafe fn update_window_title(&mut self, window: Window, title: &str) {
        let c_title = std::ffi::CString::new(title).unwrap();

        (self.libx11.Xutf8SetWMProperties)(
            self.display,
            window,
            c_title.as_ptr(),
            c_title.as_ptr(),
            std::ptr::null_mut(),
            0 as libc::c_int,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        );
        (self.libx11.XChangeProperty)(
            self.display,
            window,
            self.extensions.net_wm_name,
            self.extensions.utf8_string,
            8 as libc::c_int,
            PropModeReplace,
            c_title.as_ptr() as *mut libc::c_uchar,
            libc::strlen(c_title.as_ptr()) as libc::c_int,
        );
        (self.libx11.XChangeProperty)(
            self.display,
            window,
            self.extensions.net_wm_icon_name,
            self.extensions.utf8_string,
            8 as libc::c_int,
            PropModeReplace,
            c_title.as_ptr() as *mut libc::c_uchar,
            libc::strlen(c_title.as_ptr()) as libc::c_int,
        );
        (self.libx11.XFlush)(self.display);
    }

    // TODO: _fullscreen is not used, this function always setting window fullscreen
    // should be able to able to go back from fullscreen to windowed instead
    unsafe fn set_fullscreen(&mut self, window: Window, _fullscreen: bool) {
        let wm_state = (self.libx11.XInternAtom)(
            self.display,
            b"_NET_WM_STATE\x00" as *const u8 as *const _,
            false as _,
        );
        let wm_fullscreen = (self.libx11.XInternAtom)(
            self.display,
            b"_NET_WM_STATE_FULLSCREEN\x00" as *const u8 as *const _,
            false as _,
        );

        // this is the first method to make window fullscreen
        // hide it, change _NET_WM_STATE_FULLSCREEN property and than show it back
        // someone on stackoverflow mentioned that this is not working on ubuntu/unity though
        {
            (self.libx11.XLowerWindow)(self.display, window);
            (self.libx11.XUnmapWindow)(self.display, window);
            (self.libx11.XSync)(self.display, false as _);

            let mut atoms: [Atom; 2] = [wm_fullscreen, 0 as _];
            (self.libx11.XChangeProperty)(
                self.display,
                window,
                wm_state,
                4 as _,
                32,
                PropModeReplace,
                atoms.as_mut_ptr() as *mut _ as *mut _,
                1,
            );
            (self.libx11.XMapWindow)(self.display, window);
            (self.libx11.XRaiseWindow)(self.display, window);
            (self.libx11.XFlush)(self.display);
        }

        // however, this is X, so just in case - the second method
        // send ClientMessage to the window with request to change property to fullscreen
        {
            let mut data = [0isize; 5];

            data[0] = 1;
            data[1] = wm_fullscreen as isize;
            data[2] = 0;

            let mut ev = XClientMessageEvent {
                type_0: 33,
                serial: 0,
                send_event: true as _,
                message_type: wm_state,
                window: window,
                display: self.display,
                format: 32,
                data: ClientMessageData {
                    l: std::mem::transmute(data),
                },
            };
            (self.libx11.XSendEvent)(
                self.display as _,
                self.root,
                false as _,
                (1048576 | 131072) as _,
                &mut ev as *mut XClientMessageEvent as *mut _,
            );
        }
    }

    pub unsafe fn set_cursor_grab(&mut self, window: Window, grab: bool) {
        (self.libx11.XUngrabPointer)(self.display, 0);

        if grab {
            (self.libx11.XGrabPointer)(
                self.display,
                window,
                true as _,
                (ButtonPressMask
                    | ButtonReleaseMask
                    | EnterWindowMask
                    | LeaveWindowMask
                    | PointerMotionMask
                    | PointerMotionHintMask
                    | Button1MotionMask
                    | Button2MotionMask
                    | Button3MotionMask
                    | Button4MotionMask
                    | Button5MotionMask
                    | ButtonMotionMask
                    | KeymapStateMask) as libc::c_uint,
                GrabModeAsync,
                GrabModeAsync,
                window,
                0,
                0, // CurrentTime
            );
        }

        (self.libx11.XFlush)(self.display);
    }
    unsafe fn query_window_size(&mut self, window: Window) -> (i32, i32) {
        let mut attribs: XWindowAttributes = std::mem::zeroed();
        (self.libx11.XGetWindowAttributes)(self.display, window, &mut attribs);
        (attribs.width, attribs.height)
    }

    unsafe fn process_event(
        &mut self,
        context: &mut GraphicsContext,
        event_handler: &mut dyn EventHandler,
        event: &mut XEvent,
    ) {
        match (*event).type_0 {
            2 => {
                let keycode = (*event).xkey.keycode as libc::c_int;
                let key = self.translate_key(keycode);
                let repeat = self.repeated_keycodes[(keycode & 0xff) as usize];
                self.repeated_keycodes[(keycode & 0xff) as usize] = true;
                let mods = self.translate_mod((*event).xkey.state as libc::c_int);
                if key != crate::event::KeyCode::Unknown {
                    event_handler.key_down_event(
                        context.with_display(&mut *self),
                        key,
                        mods,
                        repeat,
                    );
                }
                let mut keysym: KeySym = 0;
                (self.libx11.XLookupString)(
                    &mut (*event).xkey,
                    std::ptr::null_mut(),
                    0 as libc::c_int,
                    &mut keysym,
                    std::ptr::null_mut(),
                );
                let chr = self.keysym_to_unicode(keysym);
                if chr > 0 {
                    if let Some(chr) = std::char::from_u32(chr as u32) {
                        event_handler.char_event(
                            context.with_display(&mut *self),
                            chr,
                            mods,
                            repeat,
                        );
                    }
                }
            }
            3 => {
                let keycode = (*event).xkey.keycode;
                let key = self.translate_key(keycode as _);
                self.repeated_keycodes[(keycode & 0xff) as usize] = false;
                if key != crate::event::KeyCode::Unknown {
                    let mods = self.translate_mod((*event).xkey.state as libc::c_int);
                    event_handler.key_up_event(context.with_display(&mut *self), key, mods);
                }
            }
            4 => {
                let btn = self.translate_mouse_button((*event).xbutton.button as _);
                let x = (*event).xmotion.x as libc::c_float;
                let y = (*event).xmotion.y as libc::c_float;

                if btn != crate::event::MouseButton::Unknown {
                    event_handler.mouse_button_down_event(
                        context.with_display(&mut *self),
                        btn,
                        x,
                        y,
                    );
                } else {
                    match (*event).xbutton.button {
                        4 => {
                            event_handler.mouse_wheel_event(
                                context.with_display(&mut *self),
                                0.0,
                                1.0,
                            );
                        }
                        5 => {
                            event_handler.mouse_wheel_event(
                                context.with_display(&mut *self),
                                0.0,
                                -1.0,
                            );
                        }
                        6 => {
                            event_handler.mouse_wheel_event(
                                context.with_display(&mut *self),
                                1.0,
                                0.0,
                            );
                        }
                        7 => {
                            event_handler.mouse_wheel_event(
                                context.with_display(&mut *self),
                                -1.0,
                                0.0,
                            );
                        }
                        _ => {}
                    }
                }
            }
            5 => {
                let btn = self.translate_mouse_button((*event).xbutton.button as _);
                let x = (*event).xmotion.x as libc::c_float;
                let y = (*event).xmotion.y as libc::c_float;

                if btn != crate::event::MouseButton::Unknown {
                    event_handler.mouse_button_up_event(
                        context.with_display(&mut *self),
                        btn,
                        x,
                        y,
                    );
                }
            }
            7 => {
                // Mouse Enter
            }
            8 => {
                // Mouse Leave
            }
            6 => {
                let x = (*event).xmotion.x as libc::c_float;
                let y = (*event).xmotion.y as libc::c_float;
                event_handler.mouse_motion_event(context.with_display(&mut *self), x, y);
            }
            22 => {
                if (*event).xconfigure.width != self.data.screen_width
                    || (*event).xconfigure.height != self.data.screen_height
                {
                    let width = (*event).xconfigure.width;
                    let height = (*event).xconfigure.height;
                    self.data.screen_width = width;
                    self.data.screen_height = height;
                    event_handler.resize_event(
                        context.with_display(&mut *self),
                        width as f32,
                        height as f32,
                    );
                }
            }
            33 => {
                if (*event).xclient.message_type == self.extensions.wm_protocols {
                    let protocol = (*event).xclient.data.l[0 as libc::c_int as usize] as Atom;
                    if protocol == self.extensions.wm_delete_window {
                        self.data.quit_requested = true
                    }
                }
            }
            // SelectionRequest
            30 => {
                // // some other app is waiting for clibpoard content
                // // need to make appropriate XSelectionEvent - response for this request
                // // only UTF8_STRING request is actually supported
                clipboard::respond_to_clipboard_request(self, event);
            }
            // SelectionClear
            29 => {}
            17 => {}

            // GenericEvent
            35 if Some((*event).xcookie.extension) == self.xi_extension_opcode => {
                if (*event).xcookie.evtype == xi_input::XI_RawMotion {
                    let (dx, dy) = self.libxi.read_cookie(&mut (*event).xcookie, self.display);
                    event_handler.raw_mouse_motion(
                        context.with_display(&mut *self),
                        dx as f32,
                        dy as f32,
                    );
                }
            }
            _ => {}
        };

        if self.data.quit_requested && !self.data.quit_ordered {
            event_handler.quit_requested_event(context.with_display(&mut *self));
            if self.data.quit_requested {
                self.data.quit_ordered = true
            }
        }
    }

    pub unsafe fn set_cursor(&mut self, window: Window, cursor: Option<CursorIcon>) {
        let libx11 = &mut self.libx11;
        let display = self.display;

        let cursor = match cursor {
            None => {
                // empty_cursor was created during create_window
                self.empty_cursor.unwrap()
            }
            Some(cursor_icon) => *self.cursor_cache.entry(cursor_icon).or_insert_with(|| {
                (libx11.XCreateFontCursor)(
                    display,
                    match cursor_icon {
                        CursorIcon::Default => libx11::XC_left_ptr,
                        CursorIcon::Help => libx11::XC_question_arrow,
                        CursorIcon::Pointer => libx11::XC_hand2,
                        CursorIcon::Wait => libx11::XC_watch,
                        CursorIcon::Crosshair => libx11::XC_crosshair,
                        CursorIcon::Text => libx11::XC_xterm,
                        CursorIcon::Move => libx11::XC_fleur,
                        CursorIcon::NotAllowed => libx11::XC_pirate,
                        CursorIcon::EWResize => libx11::XC_sb_h_double_arrow,
                        CursorIcon::NSResize => libx11::XC_sb_v_double_arrow,
                        CursorIcon::NESWResize => libx11::XC_top_right_corner,
                        CursorIcon::NWSEResize => libx11::XC_top_left_corner,
                    },
                )
            }),
        };
        (libx11.XDefineCursor)(display, window, cursor);
    }

    // pub unsafe fn process_requests(&mut self, window: Window, event_handler: &mut super::UserData) {
    //     let context = data.get_context();

    //     if let Some(cursor) = self.data.cursor_requested.take() {
    //         self.set_cursor(window, Some(cursor));
    //     }
    //     match self.data.show_mouse_requested.take() {
    //         Some(true) => self.set_cursor(window, Some(CursorIcon::Default)),
    //         Some(false) => self.set_cursor(window, None),
    //         None => {}
    //     }
    //     if let Some(fullscreen) = self.data.fullscreen_requested.take() {
    //         self.set_fullscreen(window, fullscreen);
    //     }
    //     if let Some(grab) = self.data.cursor_grab_requested.take() {
    //         self.set_cursor_grab(window, grab);
    //     }
}

unsafe fn glx_main_loop<F>(
    mut display: X11Display,
    conf: &crate::conf::Conf,
    f: &mut Option<F>,
) -> Result<(), X11Display>
where
    F: 'static + FnOnce(&mut Context) -> Box<dyn EventHandler>,
{
    let mut glx = match glx::Glx::init(&mut display) {
        Some(glx) => glx,
        _ => return Err(display),
    };
    let visual = glx.visual;
    let depth = glx.depth;
    let window = display.create_window(visual, depth, conf);
    display.window = window;
    let (glx_context, glx_window) = glx.create_context(&mut display, window);
    glx.swap_interval(
        &mut display,
        glx_window,
        glx_context,
        conf.platform.swap_interval.unwrap_or(1),
    );
    gl::load_gl_funcs(|proc| glx.libgl.get_procaddr(proc));

    display.show_window(window);

    if conf.fullscreen {
        display.set_fullscreen(window, true);
    }

    (display.libx11.XFlush)(display.display);

    let (w, h) = display.query_window_size(window);

    display.data.screen_width = w;
    display.data.screen_height = h;

    let mut context = GraphicsContext::new(gl::is_gl2());

    let mut data = (f.take().unwrap())(context.with_display(&mut display));

    while !display.data.quit_ordered {
        {
            glx.make_current(&mut display, glx_window, glx_context);

            let count = (display.libx11.XPending)(display.display);
            for _ in 0..count {
                let mut event = _XEvent { type_0: 0 };
                (display.libx11.XNextEvent)(display.display, &mut event);

                display.process_event(&mut context, &mut *data, &mut event);
            }
        }

        data.update(context.with_display(&mut display));
        data.draw(context.with_display(&mut display));

        glx.swap_buffers(&mut display, glx_window);

        (display.libx11.XFlush)(display.display);
        //display.process_requests(window, &mut data);
    }

    glx.destroy_context(&mut display, glx_window, glx_context);

    (display.libx11.XUnmapWindow)(display.display, window);
    (display.libx11.XDestroyWindow)(display.display, window);
    (display.libx11.XCloseDisplay)(display.display);

    Ok(())
}

unsafe fn egl_main_loop<F>(
    mut display: X11Display,
    conf: &crate::conf::Conf,
    f: &mut Option<F>,
) -> Result<(), X11Display>
where
    F: 'static + FnOnce(&mut Context) -> Box<dyn EventHandler>,
{
    let mut egl_lib = match egl::LibEgl::try_load() {
        Some(glx) => glx,
        _ => return Err(display),
    };

    let window = display.create_window(std::ptr::null_mut(), 0, conf);
    display.window = window;
    let (context, config, egl_display) = egl::create_egl_context(
        &mut egl_lib,
        display.display as *mut _,
        conf.platform.framebuffer_alpha,
    )
    .unwrap();

    let egl_surface = (egl_lib.eglCreateWindowSurface.unwrap())(
        egl_display,
        config,
        window,
        std::ptr::null_mut(),
    );

    if egl_surface == /* EGL_NO_SURFACE  */ std::ptr::null_mut() {
        panic!("surface creation failed");
    }
    if (egl_lib.eglMakeCurrent.unwrap())(egl_display, egl_surface, egl_surface, context) == 0 {
        panic!("eglMakeCurrent failed");
    }

    crate::native::gl::load_gl_funcs(|proc| {
        let name = std::ffi::CString::new(proc).unwrap();
        egl_lib
            .eglGetProcAddress
            .expect("non-null function pointer")(name.as_ptr() as _)
    });

    display.show_window(window);

    if conf.fullscreen {
        display.set_fullscreen(window, true);
    }

    (display.libx11.XFlush)(display.display);

    let mut context = GraphicsContext::new(gl::is_gl2());

    let (w, h) = display.query_window_size(window);
    display.data.screen_width = w;
    display.data.screen_height = h;

    let mut data = (f.take().unwrap())(context.with_display(&mut display));

    while !display.data.quit_ordered {
        let count = (display.libx11.XPending)(display.display);
        for _ in 0..count {
            let mut event = _XEvent { type_0: 0 };
            (display.libx11.XNextEvent)(display.display, &mut event);

            display.process_event(&mut context, &mut *data, &mut event);
        }

        data.update(context.with_display(&mut display));
        data.draw(context.with_display(&mut display));

        (egl_lib.eglSwapBuffers.unwrap())(egl_display, egl_surface);
        (display.libx11.XFlush)(display.display);

        //display.process_requests(window, &mut data);
    }

    // (display.libx11.XUnmapWindow)(display.display, window);
    // (display.libx11.XDestroyWindow)(display.display, window);
    // (display.libx11.XCloseDisplay)(display.display);

    Ok(())
}

pub fn run<F>(conf: &crate::conf::Conf, f: &mut Option<F>) -> Option<()>
where
    F: 'static + FnOnce(&mut Context) -> Box<dyn EventHandler>,
{
    unsafe {
        let mut libx11 = LibX11::try_load()?;
        let libxi = xi_input::LibXi::try_load()?;

        (libx11.XInitThreads)();
        (libx11.XrmInitialize)();

        let x11_display = (libx11.XOpenDisplay)(std::ptr::null());
        if x11_display.is_null() {
            panic!("XOpenDisplay() failed!");
        }

        // screen selection process. The place to do something about
        // proper multi-monitor support
        let x11_screen = (*(x11_display as _XPrivDisplay)).default_screen;
        let x11_root = (*(*(x11_display as _XPrivDisplay))
            .screens
            .offset(x11_screen as isize))
        .root;

        // https://linux.die.net/man/3/xkbsetdetectableautorepeat
        // TLDR: Xkb allows clients to request detectable auto-repeat.
        // If a client requests and the server supports DetectableAutoRepeat,
        // Xkb generates KeyRelease events only when the key is physically
        // released. If DetectableAutoRepeat is not supported or has not been
        // requested, the server synthesizes a KeyRelease event for each
        // repeating KeyPress event it generates.
        (libx11.XkbSetDetectableAutoRepeat)(x11_display, true as _, std::ptr::null_mut());

        let extensions = X11Extensions::new(&mut libx11, x11_display);
        let mut display = X11Display {
            display: x11_display,
            screen: x11_screen,
            root: x11_root,
            window: 0,
            libx11,
            libxi,
            dpi_scale: 1.0,
            extensions,
            xi_extension_opcode: None,
            repeated_keycodes: [false; 256],
            empty_cursor: None,
            cursor_cache: HashMap::new(),
            data: Default::default(),
        };
        display.update_system_dpi();

        match conf.platform.linux_x11_gl {
            crate::conf::LinuxX11Gl::GLXOnly => {
                glx_main_loop(display, &conf, f).ok().unwrap();
            }
            crate::conf::LinuxX11Gl::EGLOnly => {
                egl_main_loop(display, &conf, f).ok().unwrap();
            }
            crate::conf::LinuxX11Gl::GLXWithEGLFallback => {
                if let Err(display) = glx_main_loop(display, &conf, f) {
                    egl_main_loop(display, &conf, f).ok().unwrap();
                }
            }
            crate::conf::LinuxX11Gl::EGLWithGLXFallback => {
                if let Err(display) = egl_main_loop(display, &conf, f) {
                    glx_main_loop(display, &conf, f).ok().unwrap();
                }
            }
        }
    }
    Some(())
}
