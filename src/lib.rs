pub mod conf;
mod event;
pub mod fs;
pub mod graphics;

pub mod native;

#[cfg(feature = "log-impl")]
pub mod log;

pub use event::*;

pub use graphics::*;

mod default_icon;

pub mod date {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn now() -> f64 {
        use std::time::SystemTime;

        let time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_else(|e| panic!("{}", e));
        time.as_secs_f64()
    }

    #[cfg(target_arch = "wasm32")]
    pub fn now() -> f64 {
        use crate::native;

        unsafe { native::wasm::now() }
    }
}

use std::cell::RefCell;
thread_local! {
    static NATIVE_DISPLAY: RefCell<Option<fn (&mut dyn FnMut(&mut dyn crate::native::NativeDisplay))>> = RefCell::new(None);
}
pub(crate) fn with_native_display(f: &mut dyn FnMut(&mut dyn crate::native::NativeDisplay)) {
    NATIVE_DISPLAY.with(|d| (d.borrow().as_ref().unwrap())(f))
}

// I wish "with_native_display" could be generic over return value, but function
// pointer to a generic function requires way too much unsafe :(
macro_rules! with_native_display {
    ($target:ident, $f:expr) => {{
        let mut res = Default::default();

        with_native_display(&mut |$target| {
            res = $f;
        });

        res
    }};
}

/// Window and associated to window rendering context related functions.
/// in macroquad <= 0.3, it was ctx.screen_size(). Now it is window::screen_size()
pub mod window {
    use super::*;

    /// The current framebuffer size in pixels
    /// NOTE: [High DPI Rendering](../conf/index.html#high-dpi-rendering)
    pub fn screen_size() -> (f32, f32) {
        with_native_display!(d, d.screen_size())
    }

    /// The dpi scaling factor (window pixels to framebuffer pixels)
    /// NOTE: [High DPI Rendering](../conf/index.html#high-dpi-rendering)
    pub fn dpi_scale() -> f32 {
        with_native_display!(d, d.dpi_scale())
    }

    /// True when high_dpi was requested and actually running in a high-dpi scenario
    /// NOTE: [High DPI Rendering](../conf/index.html#high-dpi-rendering)
    pub fn high_dpi() -> bool {
        with_native_display!(d, d.high_dpi())
    }

    /// This function simply quits the application without
    /// giving the user a chance to intervene. Usually this might
    /// be called when the user clicks the 'Ok' button in a 'Really Quit?'
    /// dialog box
    /// Window might not be actually closed right away (exit(0) might not
    /// happen in the order_quit implmentation) and execution might continue for some time after
    /// But the window is going to be inevitably closed at some point.
    pub fn order_quit() {
        with_native_display!(d, d.order_quit())
    }

    /// Calling request_quit() will trigger "quit_requested_event" event , giving
    /// the user code a chance to intervene and cancel the pending quit process
    /// (for instance to show a 'Really Quit?' dialog box).
    /// If the event handler callback does nothing, the application will be quit as usual.
    /// To prevent this, call the function "cancel_quit()"" from inside the event handler.
    pub fn request_quit() {
        with_native_display!(d, d.request_quit())
    }

    /// Cancels a pending quit request, either initiated
    /// by the user clicking the window close button, or programmatically
    /// by calling "request_quit()". The only place where calling this
    /// function makes sense is from inside the event handler callback when
    /// the "quit_requested_event" event has been received
    pub fn cancel_quit() {
        with_native_display!(d, d.cancel_quit())
    }

    /// Capture mouse cursor to the current window
    /// On WASM this will automatically hide cursor
    /// On desktop this will bound cursor to windows border
    /// NOTICE: on desktop cursor will not be automatically released after window lost focus
    ///         so set_cursor_grab(false) on window's focus lost is recommended.
    /// TODO: implement window focus events
    pub fn set_cursor_grab(grab: bool) {
        with_native_display!(d, d.set_cursor_grab(grab))
    }

    /// Show or hide the mouse cursor
    pub fn show_mouse(shown: bool) {
        with_native_display!(d, d.show_mouse(shown))
    }

    /// Set the mouse cursor icon.
    pub fn set_mouse_cursor(cursor_icon: CursorIcon) {
        with_native_display!(d, d.set_mouse_cursor(cursor_icon))
    }

    /// Set the application's window size.
    pub fn set_window_size(new_width: u32, new_height: u32) {
        with_native_display!(d, d.set_window_size(new_width, new_height))
    }

    pub fn set_fullscreen(fullscreen: bool) {
        with_native_display!(d, d.set_fullscreen(fullscreen))
    }

    /// Get current OS clipboard value
    pub fn clipboard_get() -> Option<String> {
        with_native_display!(d, d.clipboard_get())
    }

    /// Save value to OS clipboard
    pub fn clipboard_set(data: &str) {
        with_native_display!(d, d.clipboard_set(data))
    }
    pub fn dropped_file_count() -> usize {
        with_native_display!(d, d.dropped_file_count())
    }
    pub fn dropped_file_bytes(index: usize) -> Option<Vec<u8>> {
        with_native_display!(d, d.dropped_file_bytes(index))
    }
    pub fn dropped_file_path(index: usize) -> Option<std::path::PathBuf> {
        with_native_display!(d, d.dropped_file_path(index))
    }

    /// Shortcut for `order_quit`. Will add a legacy attribute at some point.
    pub fn quit() {
        with_native_display!(d, d.order_quit())
    }

    /// Show/hide onscreen keyboard.
    /// Only works on Android right now.
    pub fn show_keyboard(show: bool) {
        with_native_display!(d, d.show_keyboard(show))
    }

    /// The same as
    /// ```ignore
    /// if metal {
    ///    Box::new(MetalContext::new())
    /// } else {
    ///   Box::new(GlContext::new())
    /// };
    /// ```
    /// but under #[cfg] gate to avoid MetalContext on non-apple platforms
    pub fn new_rendering_backend() -> Box<dyn RenderingBackend> {
        #[cfg(target_vendor = "apple")]
        {
            if with_native_display!(d, d.apple_gfx_api() == conf::AppleGfxApi::Metal) {
                Box::new(MetalContext::new())
            } else {
                Box::new(GlContext::new())
            }
        }
        #[cfg(not(target_vendor = "apple"))]
        Box::new(GlContext::new())
    }

    #[cfg(target_vendor = "apple")]
    pub unsafe fn apple_view() -> Option<crate::native::apple::frameworks::ObjcId> {
        with_native_display!(d, d.apple_view())
    }

    #[cfg(target_os = "ios")]
    pub unsafe fn apple_view_ctrl() -> Option<crate::native::apple::frameworks::ObjcId> {
        with_native_display!(d, d.apple_view_ctrl())
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
pub enum CursorIcon {
    Default,
    Help,
    Pointer,
    Wait,
    Crosshair,
    Text,
    Move,
    NotAllowed,
    EWResize,
    NSResize,
    NESWResize,
    NWSEResize,
}

/// Start miniquad.
pub fn start<F>(conf: conf::Conf, f: F)
where
    F: 'static + FnOnce() -> Box<dyn EventHandler>,
{
    #[cfg(target_os = "linux")]
    {
        let mut f = Some(f);
        let f = &mut f;
        match conf.platform.linux_backend {
            conf::LinuxBackend::X11Only => {
                native::linux_x11::run(&conf, f).expect("X11 backend failed")
            }
            conf::LinuxBackend::WaylandOnly => {
                native::linux_wayland::run(&conf, f).expect("Wayland backend failed")
            }
            conf::LinuxBackend::X11WithWaylandFallback => {
                if native::linux_x11::run(&conf, f).is_none() {
                    println!("Failed to initialize through X11! Trying wayland instead");
                    native::linux_wayland::run(&conf, f);
                }
            }
            conf::LinuxBackend::WaylandWithX11Fallback => {
                if native::linux_wayland::run(&conf, f).is_none() {
                    println!("Failed to initialize through wayland! Trying X11 instead");
                    native::linux_x11::run(&conf, f);
                }
            }
        }
    }

    #[cfg(target_os = "android")]
    unsafe {
        native::android::run(conf, f);
    }

    #[cfg(target_arch = "wasm32")]
    {
        native::wasm::run(&conf, f);
    }

    #[cfg(target_os = "windows")]
    {
        native::windows::run(&conf, f);
    }

    #[cfg(target_os = "macos")]
    unsafe {
        native::macos::run(conf, f);
    }

    #[cfg(target_os = "ios")]
    unsafe {
        native::ios::run(conf, f);
    }
}
