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

pub use native::{gl, NativeDisplay};

pub use graphics::GraphicsContext as Context;

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

impl Context {
    // Updates the display pointer inside the Context
    // Context should always be passed to event handlers through "with_display"
    pub(crate) fn with_display(&mut self, display: &mut dyn NativeDisplay) -> &mut Context {
        self.display = Some(display);
        self
    }

    pub fn display(&self) -> &dyn NativeDisplay {
        unsafe { &*self.display.unwrap() }
    }

    pub fn display_mut(&mut self) -> &mut dyn NativeDisplay {
        unsafe { &mut *self.display.unwrap() }
    }

    /// The current framebuffer size in pixels
    /// NOTE: [High DPI Rendering](../conf/index.html#high-dpi-rendering)
    pub fn screen_size(&self) -> (f32, f32) {
        self.display().screen_size()
    }

    /// The dpi scaling factor (window pixels to framebuffer pixels)
    /// NOTE: [High DPI Rendering](../conf/index.html#high-dpi-rendering)
    pub fn dpi_scale(&self) -> f32 {
        self.display().dpi_scale()
    }

    /// True when high_dpi was requested and actually running in a high-dpi scenario
    /// NOTE: [High DPI Rendering](../conf/index.html#high-dpi-rendering)
    pub fn high_dpi(&self) -> bool {
        self.display().high_dpi()
    }

    /// This function simply quits the application without
    /// giving the user a chance to intervene. Usually this might
    /// be called when the user clicks the 'Ok' button in a 'Really Quit?'
    /// dialog box
    /// Window might not be actually closed right away (exit(0) might not
    /// happen in the order_quit implmentation) and execution might continue for some time after
    /// But the window is going to be inevitably closed at some point.
    pub fn order_quit(&mut self) {
        self.display_mut().order_quit();
    }

    /// Calling request_quit() will trigger "quit_requested_event" event , giving
    /// the user code a chance to intervene and cancel the pending quit process
    /// (for instance to show a 'Really Quit?' dialog box).
    /// If the event handler callback does nothing, the application will be quit as usual.
    /// To prevent this, call the function "cancel_quit()"" from inside the event handler.
    pub fn request_quit(&mut self) {
        self.display_mut().request_quit();
    }

    /// Cancels a pending quit request, either initiated
    /// by the user clicking the window close button, or programmatically
    /// by calling "request_quit()". The only place where calling this
    /// function makes sense is from inside the event handler callback when
    /// the "quit_requested_event" event has been received
    pub fn cancel_quit(&mut self) {
        self.display_mut().cancel_quit();
    }

    /// Capture mouse cursor to the current window
    /// On WASM this will automatically hide cursor
    /// On desktop this will bound cursor to windows border
    /// NOTICE: on desktop cursor will not be automatically released after window lost focus
    ///         so set_cursor_grab(false) on window's focus lost is recommended.
    /// TODO: implement window focus events
    pub fn set_cursor_grab(&mut self, grab: bool) {
        self.display_mut().set_cursor_grab(grab);
    }

    /// Show or hide the mouse cursor
    pub fn show_mouse(&mut self, shown: bool) {
        self.display_mut().show_mouse(shown);
    }

    /// Set the mouse cursor icon.
    pub fn set_mouse_cursor(&mut self, cursor_icon: CursorIcon) {
        self.display_mut().set_mouse_cursor(cursor_icon);
    }

    /// Set the application's window size.
    pub fn set_window_size(&mut self, new_width: u32, new_height: u32) {
        self.display_mut().set_window_size(new_width, new_height);
    }

    pub fn set_fullscreen(&mut self, fullscreen: bool) {
        self.display_mut().set_fullscreen(fullscreen);
    }

    /// Get current OS clipboard value
    pub fn clipboard_get(&mut self) -> Option<String> {
        self.display_mut().clipboard_get()
    }

    /// Save value to OS clipboard
    pub fn clipboard_set(&mut self, data: &str) {
        self.display_mut().clipboard_set(data);
    }
    pub fn dropped_file_count(&mut self) -> usize {
        self.display_mut().dropped_file_count()
    }
    pub fn dropped_file_bytes(&mut self, index: usize) -> Option<Vec<u8>> {
        self.display_mut().dropped_file_bytes(index)
    }
    pub fn dropped_file_path(&mut self, index: usize) -> Option<std::path::PathBuf> {
        self.display_mut().dropped_file_path(index)
    }

    /// Shortcut for `order_quit`. Will add a legacy attribute at some point.
    pub fn quit(&mut self) {
        self.display_mut().order_quit()
    }

    /// Show/hide onscreen keyboard.
    /// Only works on Android right now.
    pub fn show_keyboard(&mut self, show: bool) {
        self.display_mut().show_keyboard(show)
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
    F: 'static + FnOnce(&mut Context) -> Box<dyn EventHandler>,
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
