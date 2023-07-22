// little helpers for LibX11
use super::*;

impl LibX11 {
    pub unsafe fn update_system_dpi(&mut self, display: *mut Display) -> f32 {
        let mut dpi_scale = 1.;
        let rms = (self.XResourceManagerString)(display);
        if !rms.is_null() {
            let db = (self.XrmGetStringDatabase)(rms);
            if !db.is_null() {
                let mut value = XrmValue {
                    size: 0,
                    addr: 0 as *mut libc::c_char,
                };
                let mut type_ = std::ptr::null_mut();
                if (self.XrmGetResource)(
                    db,
                    b"Xft.dpi\x00".as_ptr() as _,
                    b"Xft.Dpi\x00".as_ptr() as _,
                    &mut type_,
                    &mut value,
                ) != 0
                {
                    if !type_.is_null() && libc::strcmp(type_, b"String\x00".as_ptr() as _) == 0 {
                        dpi_scale = libc::atof(value.addr as *const _) as f32 / 96.0;
                    }
                }
                (self.XrmDestroyDatabase)(db);
            }
        };
        dpi_scale
    }

    pub unsafe fn grab_error_handler(&mut self) {
        pub unsafe extern "C" fn _sapp_x11_error_handler(
            mut _display: *mut Display,
            event: *mut XErrorEvent,
        ) -> libc::c_int {
            eprintln!("Error: {}", (*event).error_code);
            return 0 as libc::c_int;
        }

        (self.XSetErrorHandler)(Some(
            _sapp_x11_error_handler
                as unsafe extern "C" fn(_: *mut Display, _: *mut XErrorEvent) -> libc::c_int,
        ));
    }

    pub unsafe fn release_error_handler(&mut self, display: *mut Display) {
        (self.XSync)(display, false as _);
        (self.XSetErrorHandler)(None);
    }

    pub unsafe fn query_window_size(
        &mut self,
        display: *mut Display,
        window: Window,
    ) -> (i32, i32) {
        let mut attribs: XWindowAttributes = std::mem::zeroed();
        (self.XGetWindowAttributes)(display, window, &mut attribs);
        (attribs.width, attribs.height)
    }

    pub unsafe fn update_window_title(
        &mut self,
        display: *mut Display,
        window: Window,
        title: &str,
    ) {
        let c_title = std::ffi::CString::new(title).unwrap();

        (self.Xutf8SetWMProperties)(
            display,
            window,
            c_title.as_ptr(),
            c_title.as_ptr(),
            std::ptr::null_mut(),
            0 as libc::c_int,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        );
        (self.XChangeProperty)(
            display,
            window,
            self.extensions.net_wm_name,
            self.extensions.utf8_string,
            8 as libc::c_int,
            PropModeReplace,
            c_title.as_ptr() as *mut libc::c_uchar,
            libc::strlen(c_title.as_ptr()) as libc::c_int,
        );
        (self.XChangeProperty)(
            display,
            window,
            self.extensions.net_wm_icon_name,
            self.extensions.utf8_string,
            8 as libc::c_int,
            PropModeReplace,
            c_title.as_ptr() as *mut libc::c_uchar,
            libc::strlen(c_title.as_ptr()) as libc::c_int,
        );
        (self.XFlush)(display);
    }

    pub unsafe fn create_window(
        &mut self,
        root: Window,
        display: *mut Display,
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
            let colormap = (self.XCreateColormap)(display, root, visual, AllocNone);
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

        let window = (self.XCreateWindow)(
            display,
            root,
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
        self.release_error_handler(display);
        assert!(window != 0, "X11: Failed to create window");

        let mut protocols: [Atom; 1] = [self.extensions.wm_delete_window];
        (self.XSetWMProtocols)(display, window, protocols.as_mut_ptr(), 1 as libc::c_int);
        let mut hints = (self.XAllocSizeHints)();
        (*hints).flags |= PWinGravity;
        if conf.window_resizable == false {
            (*hints).flags |= PMinSize | PMaxSize;
            (*hints).min_width = conf.window_width;
            (*hints).min_height = conf.window_height;
            (*hints).max_width = conf.window_width;
            (*hints).max_height = conf.window_height;
        }
        (*hints).win_gravity = StaticGravity;
        (self.XSetWMNormalHints)(display, window, hints);
        (self.XFree)(hints as *mut libc::c_void);

        self.update_window_title(display, window, &conf.window_title);

        window
    }

    pub unsafe fn show_window(&mut self, display: *mut Display, window: Window) {
        (self.XMapWindow)(display, window);
        (self.XRaiseWindow)(display, window);
        (self.XFlush)(display);
    }
}
