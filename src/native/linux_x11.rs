// Spiritual successor of an X11 part of https://github.com/floooh/sokol/blob/master/sokol_app.h

mod clipboard;
mod glx;
mod keycodes;
pub mod libx11;
mod libx11_ex;
mod x_cursor;
mod xi_input;

use crate::{
    event::EventHandler,
    native::{egl, gl, NativeDisplayData, Request},
    CursorIcon,
};

use libx11::*;

use std::collections::HashMap;

pub struct X11Display {
    libx11: LibX11,
    libxi: xi_input::LibXi,
    display: *mut Display,
    root: Window,
    window: Window,
    repeated_keycodes: [bool; 256],
    empty_cursor: libx11::Cursor,
    cursor_cache: HashMap<CursorIcon, libx11::Cursor>,
}

impl X11Display {
    unsafe fn process_event(&mut self, event: &mut XEvent, event_handler: &mut dyn EventHandler) {
        match (*event).type_0 {
            2 => {
                let keycode = (*event).xkey.keycode as libc::c_int;
                let key = keycodes::translate_key(&mut self.libx11, self.display, keycode);
                let repeat = self.repeated_keycodes[(keycode & 0xff) as usize];
                self.repeated_keycodes[(keycode & 0xff) as usize] = true;
                let mods = keycodes::translate_mod((*event).xkey.state as libc::c_int);
                let mut keysym: KeySym = 0;
                (self.libx11.XLookupString)(
                    &mut (*event).xkey,
                    std::ptr::null_mut(),
                    0 as libc::c_int,
                    &mut keysym,
                    std::ptr::null_mut(),
                );
                let chr = keycodes::keysym_to_unicode(keysym);
                if chr > 0 {
                    if let Some(chr) = std::char::from_u32(chr as u32) {
                        event_handler.char_event(chr, mods, repeat);
                    }
                }
                event_handler.key_down_event(key, mods, repeat);
            }
            3 => {
                let keycode = (*event).xkey.keycode;
                let key = keycodes::translate_key(&mut self.libx11, self.display, keycode as _);
                self.repeated_keycodes[(keycode & 0xff) as usize] = false;
                let mods = keycodes::translate_mod((*event).xkey.state as libc::c_int);
                event_handler.key_up_event(key, mods);
            }
            4 => {
                let btn = keycodes::translate_mouse_button((*event).xbutton.button as _);
                let x = (*event).xmotion.x as libc::c_float;
                let y = (*event).xmotion.y as libc::c_float;

                if btn != crate::event::MouseButton::Unknown {
                    event_handler.mouse_button_down_event(btn, x, y);
                } else {
                    match (*event).xbutton.button {
                        4 => {
                            event_handler.mouse_wheel_event(0.0, 1.0);
                        }
                        5 => {
                            event_handler.mouse_wheel_event(0.0, -1.0);
                        }
                        6 => {
                            event_handler.mouse_wheel_event(1.0, 0.0);
                        }
                        7 => {
                            event_handler.mouse_wheel_event(-1.0, 0.0);
                        }
                        _ => {}
                    }
                }
            }
            5 => {
                let btn = keycodes::translate_mouse_button((*event).xbutton.button as _);
                let x = (*event).xmotion.x as libc::c_float;
                let y = (*event).xmotion.y as libc::c_float;

                if btn != crate::event::MouseButton::Unknown {
                    event_handler.mouse_button_up_event(btn, x, y);
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
                event_handler.mouse_motion_event(x, y);
            }
            9 => {
                event_handler.window_restored_event();
            }
            10 => {
                event_handler.window_minimized_event();
            }
            22 => {
                let mut d = crate::native_display().try_lock().unwrap();
                if (*event).xconfigure.width != d.screen_width
                    || (*event).xconfigure.height != d.screen_height
                {
                    let width = (*event).xconfigure.width;
                    let height = (*event).xconfigure.height;
                    d.screen_width = width;
                    d.screen_height = height;
                    drop(d);
                    event_handler.resize_event(width as _, height as _);
                }
            }
            33 => {
                let mut d = crate::native_display().try_lock().unwrap();
                if (*event).xclient.message_type == self.libx11.extensions.wm_protocols {
                    let protocol = (*event).xclient.data.l[0 as libc::c_int as usize] as Atom;
                    if protocol == self.libx11.extensions.wm_delete_window {
                        d.quit_requested = true;
                    }
                }
            }
            // SelectionRequest
            30 => {
                // // some other app is waiting for clibpoard content
                // // need to make appropriate XSelectionEvent - response for this request
                // // only UTF8_STRING request is actually supported
                clipboard::respond_to_clipboard_request(&mut self.libx11, self.display, event);
            }
            // SelectionClear
            29 => {}
            17 => {}

            // GenericEvent
            35 if Some((*event).xcookie.extension) == self.libxi.xi_extension_opcode => {
                if (*event).xcookie.evtype == xi_input::XI_RawMotion {
                    let (dx, dy) = self.libxi.read_cookie(&mut (*event).xcookie, self.display);
                    event_handler.raw_mouse_motion(dx as f32, dy as f32);
                }
            }
            _ => {}
        };

        let mut d = crate::native_display().try_lock().unwrap();
        if d.quit_requested && !d.quit_ordered {
            drop(d);
            event_handler.quit_requested_event();
            let mut d = crate::native_display().try_lock().unwrap();
            if d.quit_requested {
                d.quit_ordered = true
            }
        }
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

    unsafe fn set_window_size(&mut self, window: Window, new_width: i32, new_height: i32) {
        (self.libx11.XResizeWindow)(self.display, window, new_width, new_height);
        (self.libx11.XFlush)(self.display);
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
    pub unsafe fn set_cursor(&mut self, window: Window, cursor: Option<CursorIcon>) {
        let libx11 = &mut self.libx11;
        let display = self.display;

        let cursor = match cursor {
            None => self.empty_cursor,
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

    fn process_request(&mut self, request: Request) {
        use Request::*;
        unsafe {
            match request {
                SetCursorGrab(grab) => self.set_cursor_grab(self.window, grab),
                ShowMouse(show) => self.show_mouse(show),
                SetMouseCursor(icon) => self.set_cursor(self.window, Some(icon)),
                SetWindowSize {
                    new_width,
                    new_height,
                } => self.set_window_size(self.window, new_width as _, new_height as _),
                SetFullscreen(fullscreen) => self.set_fullscreen(self.window, fullscreen),
                ShowKeyboard(show) => {
                    eprintln!("Not implemented for X11")
                }
            }
        }
    }
}

unsafe fn glx_main_loop<F>(
    mut display: X11Display,
    conf: &crate::conf::Conf,
    f: &mut Option<F>,
    screen: i32,
) -> Result<(), X11Display>
where
    F: 'static + FnOnce() -> Box<dyn EventHandler>,
{
    let mut glx = match glx::Glx::init(&mut display.libx11, display.display, screen) {
        Some(glx) => glx,
        _ => return Err(display),
    };
    let visual = glx.visual;
    let depth = glx.depth;
    display.window =
        display
            .libx11
            .create_window(display.root, display.display, visual, depth, conf);

    let (glx_context, glx_window) = glx.create_context(display.display, display.window);
    glx.swap_interval(
        display.display,
        glx_window,
        glx_context,
        conf.platform.swap_interval.unwrap_or(1),
    );
    gl::load_gl_funcs(|proc| glx.libgl.get_procaddr(proc));

    display.libx11.show_window(display.display, display.window);

    (display.libx11.XFlush)(display.display);

    let (w, h) = display
        .libx11
        .query_window_size(display.display, display.window);

    let (tx, rx) = std::sync::mpsc::channel();
    let clipboard = Box::new(clipboard::X11Clipboard::new(
        display.libx11.clone(),
        display.display,
        display.window,
    ));
    crate::set_display(NativeDisplayData {
        high_dpi: conf.high_dpi,
        dpi_scale: display.libx11.update_system_dpi(display.display),
        ..NativeDisplayData::new(w, h, tx, clipboard)
    });
    if conf.fullscreen {
        display.set_fullscreen(display.window, true);
    }

    let mut event_handler = (f.take().unwrap())();

    while !crate::native_display().try_lock().unwrap().quit_ordered {
        while let Ok(request) = rx.try_recv() {
            display.process_request(request);
        }
        glx.make_current(display.display, glx_window, glx_context);
        let count = (display.libx11.XPending)(display.display);

        for _ in 0..count {
            let mut xevent = _XEvent { type_0: 0 };
            (display.libx11.XNextEvent)(display.display, &mut xevent);
            display.process_event(&mut xevent, &mut *event_handler);
        }

        event_handler.update();
        event_handler.draw();

        glx.swap_buffers(display.display, glx_window);
        (display.libx11.XFlush)(display.display);
    }

    glx.destroy_context(display.display, glx_window, glx_context);
    (display.libx11.XUnmapWindow)(display.display, display.window);
    (display.libx11.XDestroyWindow)(display.display, display.window);
    (display.libx11.XCloseDisplay)(display.display);

    Ok(())
}

unsafe fn egl_main_loop<F>(
    mut display: X11Display,
    conf: &crate::conf::Conf,
    f: &mut Option<F>,
) -> Result<(), X11Display>
where
    F: 'static + FnOnce() -> Box<dyn EventHandler>,
{
    let mut egl_lib = match egl::LibEgl::try_load() {
        Some(glx) => glx,
        _ => return Err(display),
    };

    display.window =
        display
            .libx11
            .create_window(display.root, display.display, std::ptr::null_mut(), 0, conf);

    let (context, config, egl_display) = egl::create_egl_context(
        &mut egl_lib,
        display.display as *mut _,
        conf.platform.framebuffer_alpha,
    )
    .unwrap();

    let egl_surface = (egl_lib.eglCreateWindowSurface.unwrap())(
        egl_display,
        config,
        display.window,
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

    display.libx11.show_window(display.display, display.window);
    let (w, h) = display
        .libx11
        .query_window_size(display.display, display.window);

    let (tx, rx) = std::sync::mpsc::channel();
    let clipboard = Box::new(clipboard::X11Clipboard::new(
        display.libx11.clone(),
        display.display,
        display.window,
    ));
    crate::set_display(NativeDisplayData {
        high_dpi: conf.high_dpi,
        dpi_scale: display.libx11.update_system_dpi(display.display),
        ..NativeDisplayData::new(w, h, tx, clipboard)
    });
    if conf.fullscreen {
        display.set_fullscreen(display.window, true)
    }

    (display.libx11.XFlush)(display.display);

    let mut event_handler = (f.take().unwrap())();

    while !crate::native_display().try_lock().unwrap().quit_ordered {
        while let Ok(request) = rx.try_recv() {
            display.process_request(request);
        }

        let count = (display.libx11.XPending)(display.display);

        for _ in 0..count {
            let mut xevent = _XEvent { type_0: 0 };
            (display.libx11.XNextEvent)(display.display, &mut xevent);
            display.process_event(&mut xevent, &mut *event_handler);
        }

        event_handler.update();
        event_handler.draw();

        (egl_lib.eglSwapBuffers.unwrap())(egl_display, egl_surface);
        (display.libx11.XFlush)(display.display);
    }

    (display.libx11.XUnmapWindow)(display.display, display.window);
    (display.libx11.XDestroyWindow)(display.display, display.window);
    (display.libx11.XCloseDisplay)(display.display);

    Ok(())
}

pub fn run<F>(conf: &crate::conf::Conf, f: &mut Option<F>) -> Option<()>
where
    F: 'static + FnOnce() -> Box<dyn EventHandler>,
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

        libx11.load_extensions(x11_display);
        let mut display = X11Display {
            empty_cursor: x_cursor::create_empty_cursor(x11_display, x11_root, &mut libx11),
            display: x11_display,
            root: x11_root,
            window: 0,
            libx11,
            libxi,
            repeated_keycodes: [false; 256],
            cursor_cache: HashMap::new(),
        };

        display
            .libxi
            .query_xi_extension(&mut display.libx11, display.display);

        match conf.platform.linux_x11_gl {
            crate::conf::LinuxX11Gl::GLXOnly => {
                glx_main_loop(display, &conf, f, x11_screen).ok().unwrap();
            }
            crate::conf::LinuxX11Gl::EGLOnly => {
                egl_main_loop(display, &conf, f).ok().unwrap();
            }
            crate::conf::LinuxX11Gl::GLXWithEGLFallback => {
                if let Err(display) = glx_main_loop(display, &conf, f, x11_screen) {
                    egl_main_loop(display, &conf, f).ok().unwrap();
                }
            }
            crate::conf::LinuxX11Gl::EGLWithGLXFallback => {
                if let Err(display) = egl_main_loop(display, &conf, f) {
                    glx_main_loop(display, &conf, f, x11_screen).ok().unwrap();
                }
            }
        }
    }
    Some(())
}
