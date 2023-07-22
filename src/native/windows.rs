use crate::{
    conf::{Conf, Icon},
    event::{KeyMods, MouseButton},
    native::{NativeDisplayData, Request},
    CursorIcon, EventHandler,
};

use winapi::{
    shared::{
        hidusage::{HID_USAGE_GENERIC_MOUSE, HID_USAGE_PAGE_GENERIC},
        minwindef::{DWORD, HIWORD, LOWORD, LPARAM, LRESULT, UINT, WPARAM},
        ntdef::NULL,
        windef::{HCURSOR, HDC, HICON, HWND, POINT, RECT},
        windowsx::{GET_X_LPARAM, GET_Y_LPARAM},
    },
    um::{
        libloaderapi::{GetModuleHandleW, GetProcAddress},
        shellscalingapi::*,
        wingdi::*,
        winuser::*,
    },
};

mod clipboard;
mod keycodes;
mod libopengl32;
mod wgl;

use libopengl32::LibOpengl32;

pub(crate) struct WindowsDisplay {
    fullscreen: bool,
    dpi_aware: bool,
    window_resizable: bool,
    cursor_grabbed: bool,
    iconified: bool,
    content_scale: f32,
    window_scale: f32,
    mouse_scale: f32,
    show_cursor: bool,
    user_cursor: bool,
    mouse_x: f32,
    mouse_y: f32,
    cursor: HCURSOR,
    libopengl32: LibOpengl32,
    _msg_wnd: HWND,
    msg_dc: HDC,
    wnd: HWND,
    dc: HDC,
    event_handler: Option<Box<dyn EventHandler>>,
}

impl WindowsDisplay {
    fn set_cursor_grab(&mut self, grab: bool) {
        self.cursor_grabbed = grab;
        unsafe {
            if grab {
                update_clip_rect(self.wnd);
            } else {
                ClipCursor(NULL as _);
            }
        }
    }
    fn show_mouse(&mut self, shown: bool) {
        if self.show_cursor != shown {
            self.show_cursor = shown;
            unsafe { ShowCursor(shown.into()) };
        }
    }
    fn set_mouse_cursor(&mut self, cursor_icon: CursorIcon) {
        let cursor_name = match cursor_icon {
            CursorIcon::Default => IDC_ARROW,
            CursorIcon::Help => IDC_HELP,
            CursorIcon::Pointer => IDC_HAND,
            CursorIcon::Wait => IDC_WAIT,
            CursorIcon::Crosshair => IDC_CROSS,
            CursorIcon::Text => IDC_IBEAM,
            CursorIcon::Move => IDC_SIZEALL,
            CursorIcon::NotAllowed => IDC_NO,
            CursorIcon::EWResize => IDC_SIZEWE,
            CursorIcon::NSResize => IDC_SIZENS,
            CursorIcon::NESWResize => IDC_SIZENESW,
            CursorIcon::NWSEResize => IDC_SIZENWSE,
        };
        self.cursor = unsafe { LoadCursorW(NULL as _, cursor_name) };
        unsafe { SetCursor(self.cursor) };

        self.user_cursor = cursor_icon != CursorIcon::Default;
    }
    fn set_window_size(&mut self, new_width: u32, new_height: u32) {
        let mut x = 0;
        let mut y = 0;
        let mut final_new_width = new_width;
        let mut final_new_height = new_height;

        let mut rect: RECT = unsafe { std::mem::zeroed() };
        if unsafe { GetClientRect(self.wnd, &mut rect as *mut _ as _) } != 0 {
            x = rect.left;
            y = rect.bottom;
        }

        rect.right = (rect.left + new_width as i32) as _;
        rect.top = (rect.bottom - new_height as i32) as _;

        let win_style = get_win_style(self.fullscreen, self.window_resizable);
        let win_style_ex: DWORD = unsafe { GetWindowLongA(self.wnd, GWL_EXSTYLE) as _ };
        if unsafe {
            AdjustWindowRectEx(
                &mut rect as *mut _ as _,
                win_style,
                false as _,
                win_style_ex,
            )
        } != 0
        {
            final_new_width = (rect.right - rect.left) as _;
            final_new_height = (rect.bottom - rect.top) as _;
        }

        unsafe {
            SetWindowPos(
                self.wnd,
                HWND_TOP,
                x,
                y,
                final_new_width as i32,
                final_new_height as i32,
                SWP_NOMOVE,
            )
        };
    }

    fn set_fullscreen(&mut self, fullscreen: bool) {
        self.fullscreen = fullscreen as _;

        let win_style: DWORD = get_win_style(self.fullscreen, self.window_resizable);

        unsafe {
            #[cfg(target_arch = "x86_64")]
            SetWindowLongPtrA(self.wnd, GWL_STYLE, win_style as _);
            #[cfg(target_arch = "i686")]
            SetWindowLong(self.wnd, GWL_STYLE, win_style as _);

            if self.fullscreen {
                SetWindowPos(
                    self.wnd,
                    HWND_TOP,
                    0,
                    0,
                    GetSystemMetrics(SM_CXSCREEN),
                    GetSystemMetrics(SM_CYSCREEN),
                    SWP_FRAMECHANGED,
                );
            } else {
                let (w, h) = {
                    let d = crate::native_display().lock().unwrap();
                    (d.screen_width, d.screen_height)
                };

                SetWindowPos(
                    self.wnd,
                    HWND_TOP,
                    0,
                    0,
                    // this is probably not correct: with high dpi content_width and window_width are actually different..
                    w,
                    h,
                    SWP_FRAMECHANGED,
                );
            }

            ShowWindow(self.wnd, SW_SHOW);
        };
    }
}

fn get_win_style(is_fullscreen: bool, is_resizable: bool) -> DWORD {
    if is_fullscreen {
        WS_POPUP | WS_SYSMENU | WS_VISIBLE
    } else {
        let mut win_style: DWORD =
            WS_CLIPSIBLINGS | WS_CLIPCHILDREN | WS_CAPTION | WS_SYSMENU | WS_MINIMIZEBOX;

        if is_resizable {
            win_style |= WS_MAXIMIZEBOX | WS_SIZEBOX;
        }

        win_style
    }
}

unsafe fn update_clip_rect(hwnd: HWND) {
    // Retrieve the screen coordinates of the client area,
    // and convert them into client coordinates.
    let mut rect: RECT = std::mem::zeroed();

    GetClientRect(hwnd, &mut rect as *mut _ as _);
    let mut upper_left = POINT {
        x: rect.left,
        y: rect.top,
    };
    let mut lower_right = POINT {
        x: rect.right,
        y: rect.bottom,
    };

    ClientToScreen(hwnd, &mut upper_left as *mut _ as _);
    ClientToScreen(hwnd, &mut lower_right as *mut _ as _);

    SetRect(
        &mut rect as *mut _ as _,
        upper_left.x,
        upper_left.y,
        lower_right.x,
        lower_right.y,
    );
    ClipCursor(&mut rect as *mut _ as _);
}

unsafe fn key_mods() -> KeyMods {
    let mut mods = KeyMods::default();

    if GetKeyState(VK_SHIFT) as u32 & (1u32 << 31) != 0 {
        mods.shift = true;
    }
    if GetKeyState(VK_CONTROL) as u32 & (1u32 << 31) != 0 {
        mods.ctrl = true;
    }
    if GetKeyState(VK_MENU) as u32 & (1u32 << 31) != 0 {
        mods.alt = true;
    }
    if (GetKeyState(VK_LWIN) | GetKeyState(VK_RWIN)) as u32 & (1u32 << 31) != 0 {
        mods.logo = true;
    }

    mods
}

unsafe extern "system" fn win32_wndproc(
    hwnd: HWND,
    umsg: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    let display_ptr: isize;

    #[cfg(target_arch = "x86_64")]
    {
        display_ptr = GetWindowLongPtrA(hwnd, GWLP_USERDATA)
    }

    #[cfg(target_arch = "i686")]
    {
        display_ptr = GetWindowLong(hwnd, GWLP_USERDATA)
    }

    if display_ptr == 0 {
        return DefWindowProcW(hwnd, umsg, wparam, lparam);
    }
    let payload = &mut *(display_ptr as *mut WindowsDisplay);
    let event_handler = payload.event_handler.as_mut().unwrap();

    match umsg {
        WM_CLOSE => {
            let mut quit_requested = false;

            let mut d = crate::native_display().lock().unwrap();
            // only give user a chance to intervene when sapp_quit() wasn't already called
            if !d.quit_ordered {
                // if window should be closed and event handling is enabled, give user code
                // a change to intervene via sapp_cancel_quit()
                d.quit_requested = true;
                quit_requested = true;
                // if user code hasn't intervened, quit the app
                if d.quit_requested {
                    d.quit_ordered = true;
                }
            }
            if d.quit_ordered {
                PostQuitMessage(0);
            }

            if quit_requested {
                drop(d);
                event_handler.quit_requested_event();
            }
            return 0;
        }
        WM_SYSCOMMAND => {
            match wparam & 0xFFF0 {
                SC_SCREENSAVE | SC_MONITORPOWER => {
                    if payload.fullscreen {
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
            if payload.cursor_grabbed {
                update_clip_rect(hwnd);
            }

            let iconified = wparam == SIZE_MINIMIZED;
            if iconified != payload.iconified {
                payload.iconified = iconified;
                if iconified {
                    event_handler.window_minimized_event();
                } else {
                    event_handler.window_restored_event();
                }
            }
        }
        WM_SETCURSOR => {
            if payload.user_cursor && LOWORD(lparam as _) == HTCLIENT as _ {
                SetCursor(payload.cursor);

                return 1;
            }
        }
        WM_LBUTTONDOWN => {
            let mouse_x = payload.mouse_x;
            let mouse_y = payload.mouse_y;
            event_handler.mouse_button_down_event(MouseButton::Left, mouse_x, mouse_y);
        }
        WM_RBUTTONDOWN => {
            let mouse_x = payload.mouse_x;
            let mouse_y = payload.mouse_y;

            event_handler.mouse_button_down_event(MouseButton::Right, mouse_x, mouse_y);
        }
        WM_MBUTTONDOWN => {
            let mouse_x = payload.mouse_x;
            let mouse_y = payload.mouse_y;

            event_handler.mouse_button_down_event(MouseButton::Middle, mouse_x, mouse_y);
        }
        WM_LBUTTONUP => {
            let mouse_x = payload.mouse_x;
            let mouse_y = payload.mouse_y;

            event_handler.mouse_button_up_event(MouseButton::Left, mouse_x, mouse_y);
        }
        WM_RBUTTONUP => {
            let mouse_x = payload.mouse_x;
            let mouse_y = payload.mouse_y;

            event_handler.mouse_button_up_event(MouseButton::Right, mouse_x, mouse_y);
        }
        WM_MBUTTONUP => {
            let mouse_x = payload.mouse_x;
            let mouse_y = payload.mouse_y;

            event_handler.mouse_button_up_event(MouseButton::Middle, mouse_x, mouse_y);
        }

        WM_MOUSEMOVE => {
            payload.mouse_x = GET_X_LPARAM(lparam) as f32 * payload.mouse_scale;
            payload.mouse_y = GET_Y_LPARAM(lparam) as f32 * payload.mouse_scale;
            // mouse enter was not handled by miniquad anyway
            // if !_sapp.win32_mouse_tracked {
            //     _sapp.win32_mouse_tracked = true;

            //     let mut tme: TRACKMOUSEEVENT = std::mem::zeroed();

            //     tme.cbSize = std::mem::size_of_val(&tme) as _;
            //     tme.dwFlags = TME_LEAVE;
            //     tme.hwndTrack = wnd;
            //     TrackMouseEvent(&mut tme as *mut _);
            //     _sapp_win32_mouse_event(
            //         sapp_event_type_SAPP_EVENTTYPE_MOUSE_ENTER,
            //         sapp_mousebutton_SAPP_MOUSEBUTTON_INVALID,
            //     );
            // }

            let mouse_x = payload.mouse_x;
            let mouse_y = payload.mouse_y;

            event_handler.mouse_motion_event(mouse_x, mouse_y);
        }

        WM_MOVE if payload.cursor_grabbed => {
            update_clip_rect(hwnd);
        }

        WM_INPUT => {
            let mut data: RAWINPUT = std::mem::zeroed();
            let mut size = std::mem::size_of::<RAWINPUT>();
            let get_succeed = GetRawInputData(
                lparam as _,
                RID_INPUT,
                &mut data as *mut _ as _,
                &mut size as *mut _ as _,
                std::mem::size_of::<RAWINPUTHEADER>() as _,
            );
            if get_succeed as i32 == -1 {
                panic!("failed to retrieve raw input data");
            }

            let mouse_scale = payload.mouse_scale;
            let mut dx = data.data.mouse().lLastX as f32 * mouse_scale;
            let mut dy = data.data.mouse().lLastY as f32 * mouse_scale;

            // convert from normalised absolute coordinates
            if (data.data.mouse().usFlags & MOUSE_MOVE_ABSOLUTE) == MOUSE_MOVE_ABSOLUTE {
                let (width, height) = {
                    let d = crate::native_display().lock().unwrap();
                    (d.screen_width as f32, d.screen_height as f32)
                };

                dx = dx / 65535.0 * width;
                dy = dy / 65535.0 * height;
            }

            event_handler.raw_mouse_motion(dx as f32, dy as f32);
        }

        WM_MOUSELEAVE => {
            // mouse leave was not handled by miniquad anyway
            // _sapp.win32_mouse_tracked = false;
            // _sapp_win32_mouse_event(
            //     sapp_event_type_SAPP_EVENTTYPE_MOUSE_LEAVE,
            //     sapp_mousebutton_SAPP_MOUSEBUTTON_INVALID,
            // );
        }
        WM_MOUSEWHEEL => {
            event_handler.mouse_wheel_event(0.0, (HIWORD(wparam as _) as i16) as f32);
        }

        WM_MOUSEHWHEEL => {
            event_handler.mouse_wheel_event((HIWORD(wparam as _) as i16) as f32, 0.0);
        }
        WM_CHAR => {
            let chr = wparam as u32;
            let repeat = !!(lparam & 0x40000000) != 0;
            let mods = key_mods();
            if chr > 0 {
                if let Some(chr) = std::char::from_u32(chr as u32) {
                    event_handler.char_event(chr, mods, repeat);
                }
            }
        }
        WM_KEYDOWN | WM_SYSKEYDOWN => {
            let keycode = HIWORD(lparam as _) as u32 & 0x1FF;
            let keycode = keycodes::translate_keycode(keycode);
            let mods = key_mods();
            let repeat = !!(lparam & 0x40000000) != 0;
            event_handler.key_down_event(keycode, mods, repeat);
        }
        WM_KEYUP | WM_SYSKEYUP => {
            let keycode = HIWORD(lparam as _) as u32 & 0x1FF;
            let keycode = keycodes::translate_keycode(keycode);
            let mods = key_mods();
            event_handler.key_up_event(keycode, mods);
        }

        _ => {}
    }

    DefWindowProcW(hwnd, umsg, wparam, lparam)
}

unsafe fn create_win_icon_from_image(width: u32, height: u32, colors: &[u8]) -> Option<HICON> {
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

    for i in 0..width as usize * height as usize {
        *(target as *mut u8).offset(i as isize * 4 + 0) = colors[i * 4 + 2];
        *(target as *mut u8).offset(i as isize * 4 + 1) = colors[i * 4 + 1];
        *(target as *mut u8).offset(i as isize * 4 + 2) = colors[i * 4 + 0];
        *(target as *mut u8).offset(i as isize * 4 + 3) = colors[i * 4 + 3];
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

unsafe fn set_icon(wnd: HWND, icon: &Icon) {
    let big_icon_w = GetSystemMetrics(SM_CXICON);
    let big_icon_h = GetSystemMetrics(SM_CYICON);
    let small_icon_w = GetSystemMetrics(SM_CXSMICON);
    let small_icon_h = GetSystemMetrics(SM_CYSMICON);

    let big_icon = if big_icon_w * big_icon_h >= 64 * 64 {
        (&icon.big[..], 64, 64)
    } else {
        (&icon.medium[..], 32, 32)
    };

    let small_icon = if small_icon_w * small_icon_h <= 16 * 16 {
        (&icon.small[..], 16, 16)
    } else {
        (&icon.medium[..], 32, 32)
    };

    let big_icon = create_win_icon_from_image(big_icon.1, big_icon.2, big_icon.0);
    let small_icon = create_win_icon_from_image(small_icon.1, small_icon.2, small_icon.0);
    if let Some(icon) = big_icon {
        SendMessageW(wnd, WM_SETICON, ICON_BIG as _, icon as LPARAM);
    }
    if let Some(icon) = small_icon {
        SendMessageW(wnd, WM_SETICON, ICON_SMALL as _, icon as LPARAM);
    }
}

unsafe fn create_window(
    window_title: &str,
    fullscreen: bool,
    resizable: bool,
    width: i32,
    height: i32,
) -> (HWND, HDC) {
    let mut wndclassw: WNDCLASSW = std::mem::zeroed();

    wndclassw.style = CS_HREDRAW | CS_VREDRAW | CS_OWNDC;
    wndclassw.lpfnWndProc = Some(win32_wndproc);
    wndclassw.hInstance = GetModuleHandleW(NULL as _);
    wndclassw.hCursor = LoadCursorW(NULL as _, IDC_ARROW);
    wndclassw.hIcon = LoadIconW(NULL as _, IDI_WINLOGO);
    let class_name = "MINIQUADAPP\0".encode_utf16().collect::<Vec<u16>>();
    wndclassw.lpszClassName = class_name.as_ptr() as _;
    wndclassw.cbWndExtra = std::mem::size_of::<*mut std::ffi::c_void>() as i32;
    RegisterClassW(&wndclassw);

    let win_style: DWORD;
    let win_ex_style: DWORD = WS_EX_APPWINDOW | WS_EX_WINDOWEDGE;
    let mut rect = RECT {
        left: 0,
        top: 0,
        right: 0,
        bottom: 0,
    };

    if fullscreen {
        win_style = WS_POPUP | WS_SYSMENU | WS_VISIBLE;
        rect.right = GetSystemMetrics(SM_CXSCREEN);
        rect.bottom = GetSystemMetrics(SM_CYSCREEN);
    } else {
        win_style = if resizable {
            WS_CLIPSIBLINGS
                | WS_CLIPCHILDREN
                | WS_CAPTION
                | WS_SYSMENU
                | WS_MINIMIZEBOX
                | WS_MAXIMIZEBOX
                | WS_SIZEBOX
        } else {
            WS_CLIPSIBLINGS | WS_CLIPCHILDREN | WS_CAPTION | WS_SYSMENU | WS_MINIMIZEBOX
        };

        rect.right = width;
        rect.bottom = height;
    }

    AdjustWindowRectEx(&rect as *const _ as _, win_style, false as _, win_ex_style);
    let win_width = rect.right - rect.left;
    let win_height = rect.bottom - rect.top;
    let class_name = "MINIQUADAPP\0".encode_utf16().collect::<Vec<u16>>();
    let mut window_name = window_title.encode_utf16().collect::<Vec<u16>>();
    window_name.push(0);
    let hwnd = CreateWindowExW(
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
        NULL as _,                   // lparam
    );
    assert!(hwnd.is_null() == false);

    let mut rawinputdevice: RAWINPUTDEVICE = std::mem::zeroed();
    rawinputdevice.usUsagePage = HID_USAGE_PAGE_GENERIC;
    rawinputdevice.usUsage = HID_USAGE_GENERIC_MOUSE;
    rawinputdevice.hwndTarget = NULL as _;
    let register_succeed = RegisterRawInputDevices(
        &rawinputdevice as *const _,
        1,
        std::mem::size_of::<RAWINPUTDEVICE>() as _,
    );
    assert!(
        register_succeed == 1,
        "Win32: failed to register for raw mouse input!"
    );

    ShowWindow(hwnd, SW_SHOW);
    let dc = GetDC(hwnd);
    assert!(dc.is_null() == false);

    (hwnd, dc)
}

unsafe fn create_msg_window() -> (HWND, HDC) {
    let class_name = "MINIQUADAPP\0".encode_utf16().collect::<Vec<u16>>();
    let window_name = "miniquad message window\0"
        .encode_utf16()
        .collect::<Vec<u16>>();
    let msg_hwnd = CreateWindowExW(
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
        msg_hwnd.is_null() == false,
        "Win32: failed to create helper window!"
    );
    ShowWindow(msg_hwnd, SW_HIDE);
    let mut msg = std::mem::zeroed();
    while PeekMessageW(&mut msg as _, msg_hwnd, 0, 0, PM_REMOVE) != 0 {
        TranslateMessage(&msg);
        DispatchMessageW(&msg);
    }
    let msg_dc = GetDC(msg_hwnd);
    assert!(
        msg_dc.is_null() == false,
        "Win32: failed to obtain helper window DC!"
    );

    (msg_hwnd, msg_dc)
}

impl WindowsDisplay {
    unsafe fn get_proc_address(&mut self, proc: &str) -> Option<unsafe extern "C" fn() -> ()> {
        let proc = std::ffi::CString::new(proc).unwrap();
        let mut proc_ptr = (self.libopengl32.wglGetProcAddress)(proc.as_ptr());
        if proc_ptr.is_null() {
            proc_ptr = GetProcAddress(self.libopengl32.module.0, proc.as_ptr());
        }
        if proc_ptr.is_null() {
            eprintln!("Load GL func {:?} failed.", proc);
            return None;
        }
        Some(std::mem::transmute(proc_ptr))
    }

    /// updates current window and framebuffer size from the window's client rect,
    /// returns true if size has changed
    unsafe fn update_dimensions(&mut self, hwnd: HWND) -> bool {
        let mut d = crate::native_display().lock().unwrap();
        let mut rect: RECT = std::mem::zeroed();

        if GetClientRect(hwnd, &mut rect as *mut _ as _) != 0 {
            let window_width = ((rect.right - rect.left) as f32 / self.window_scale) as i32;
            let window_height = ((rect.bottom - rect.top) as f32 / self.window_scale) as i32;

            // prevent a framebuffer size of 0 when window is minimized
            let fb_width = ((window_width as f32 * self.content_scale) as i32).max(1);
            let fb_height = ((window_height as f32 * self.content_scale) as i32).max(1);
            if fb_width != d.screen_width || fb_height != d.screen_height {
                d.screen_width = fb_width;
                d.screen_height = fb_height;
                return true;
            }
        } else {
            d.screen_width = 1;
            d.screen_height = 1;
        }
        return false;
    }

    unsafe fn init_dpi(&mut self, high_dpi: bool) {
        self.dpi_aware = high_dpi;
        // get dpi scale factor for main monitor
        if self.dpi_aware {
            let pt = POINT { x: 1, y: 1 };
            let hm = MonitorFromPoint(pt, MONITOR_DEFAULTTONEAREST);
            let mut dpix: UINT = 0;
            let mut dpiy: UINT = 0;
            let hr = GetDpiForMonitor(
                hm,
                MDT_EFFECTIVE_DPI,
                &mut dpix as *mut _ as _,
                &mut dpiy as *mut _ as _,
            );
            assert_eq!(hr, 0);
            //  clamp window scale to an integer factor
            self.window_scale = dpix as f32 / 96.0;
        }
        if high_dpi {
            self.content_scale = self.window_scale;
            self.mouse_scale = 1.0;
        } else {
            self.content_scale = 1.0;
            self.mouse_scale = 1.0 / self.window_scale;
        }
    }

    fn process_request(&mut self, request: Request) {
        use Request::*;
        unsafe {
            match request {
                SetCursorGrab(grab) => self.set_cursor_grab(grab),
                ShowMouse(show) => self.show_mouse(show),
                SetMouseCursor(icon) => self.set_mouse_cursor(icon),
                SetWindowSize {
                    new_width,
                    new_height,
                } => self.set_window_size(new_width as _, new_height as _),
                SetFullscreen(fullscreen) => self.set_fullscreen(fullscreen),
                ShowKeyboard(show) => {
                    eprintln!("Not implemented for windows")
                }
            }
        }
    }
}

pub fn run<F>(conf: &Conf, f: F)
where
    F: 'static + FnOnce() -> Box<dyn EventHandler>,
{
    unsafe {
        if conf.high_dpi {
            SetProcessDPIAware();
        }
        let (wnd, dc) = create_window(
            &conf.window_title,
            conf.fullscreen,
            conf.window_resizable,
            conf.window_width as _,
            conf.window_height as _,
        );
        if let Some(icon) = &conf.icon {
            set_icon(wnd, icon);
        }

        let libopengl32 = LibOpengl32::try_load().expect("Failed to load opengl32.dll.");

        let (msg_wnd, msg_dc) = create_msg_window();
        let mut display = WindowsDisplay {
            fullscreen: false,
            dpi_aware: false,
            window_resizable: conf.window_resizable,
            cursor_grabbed: false,
            iconified: false,
            content_scale: 1.,
            mouse_scale: 1.,
            window_scale: 1.,
            mouse_x: 0.,
            mouse_y: 0.,
            show_cursor: true,
            user_cursor: false,
            cursor: std::ptr::null_mut(),
            libopengl32,
            _msg_wnd: msg_wnd,
            msg_dc,
            wnd,
            dc,
            event_handler: None,
        };

        let (tx, rx) = std::sync::mpsc::channel();
        let clipboard = Box::new(clipboard::WindowsClipboard::new());
        crate::set_display(NativeDisplayData {
            high_dpi: conf.high_dpi,
            dpi_scale: display.window_scale,
            ..NativeDisplayData::new(conf.window_width, conf.window_height, tx, clipboard)
        });

        display.update_dimensions(wnd);
        display.init_dpi(conf.high_dpi);

        let mut wgl = wgl::Wgl::new(&mut display);
        let gl_ctx = wgl.create_context(
            &mut display,
            conf.sample_count,
            conf.platform.swap_interval.unwrap_or(1),
        );

        super::gl::load_gl_funcs(|proc| display.get_proc_address(proc));

        display.event_handler = Some(f());

        #[cfg(target_arch = "x86_64")]
        SetWindowLongPtrA(wnd, GWLP_USERDATA, &mut display as *mut _ as isize);
        #[cfg(target_arch = "i686")]
        SetWindowLong(wnd, GWLP_USERDATA, &mut display as *mut _ as isize);

        let mut done = false;
        while !(done || crate::native_display().lock().unwrap().quit_ordered) {
            while let Ok(request) = rx.try_recv() {
                display.process_request(request);
            }

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

            display.event_handler.as_mut().unwrap().update();
            display.event_handler.as_mut().unwrap().draw();

            SwapBuffers(display.dc);

            if display.update_dimensions(wnd) {
                let d = crate::native_display().lock().unwrap();
                let width = d.screen_width as f32;
                let height = d.screen_height as f32;
                drop(d);
                display
                    .event_handler
                    .as_mut()
                    .unwrap()
                    .resize_event(width, height);
            }
            if crate::native_display().lock().unwrap().quit_requested {
                PostMessageW(display.wnd, WM_CLOSE, 0, 0);
            }
        }

        (display.libopengl32.wglDeleteContext)(gl_ctx);
        DestroyWindow(wnd);
    }
}
