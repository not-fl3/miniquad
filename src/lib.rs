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

/// Main miniquad struct, the only struct exposed to the event handler.
/// All the drawing and window manipulation are done through the Context.
///
/// (impl details) a and b fields are here due to unresolved problem
/// of relationship between GraphicsContext and NativeDisplay
/// Should we expose both of them to the user? Should one of them be
/// consumed by another (technically very complicated, they have a bit different lifetimes)? Or, maybe, just expose both to each EventHandler's functions?
/// From implementation perspective "fn update(&mut self, context: &mut GraphicsContext, &mut dyn NativeDisplay)" feels really nice
/// But looks a bit heavy in the trait declaration and is a bit too much of a breaking change
/// Conclusion for now - leave it with a/b, think during 0.4 and make a final decision in 0.5
pub struct Context<'a, 'b> {
    pub(crate) a: &'a mut GraphicsContext,
    pub(crate) b: &'b mut dyn NativeDisplay,
}

impl<'a, 'b> Context<'a, 'b> {
    pub(crate) fn new(a: &'a mut GraphicsContext, b: &'b mut dyn NativeDisplay) -> Context<'a, 'b> {
        Context { a, b }
    }
    /// This function simply quits the application without
    /// giving the user a chance to intervene. Usually this might
    /// be called when the user clicks the 'Ok' button in a 'Really Quit?'
    /// dialog box
    /// Window might not be actually closed right away (exit(0) might not
    /// happen in the order_quit implmentation) and execution might continue for some time after
    /// But the window is going to be inevitably closed at some point.
    pub fn order_quit(&mut self) {
        self.b.order_quit();
    }

    /// Calling request_quit() will trigger "quit_requested_event" event , giving
    /// the user code a chance to intervene and cancel the pending quit process
    /// (for instance to show a 'Really Quit?' dialog box).
    /// If the event handler callback does nothing, the application will be quit as usual.
    /// To prevent this, call the function "cancel_quit()"" from inside the event handler.
    pub fn request_quit(&mut self) {
        self.b.request_quit();
    }

    /// Cancels a pending quit request, either initiated
    /// by the user clicking the window close button, or programmatically
    /// by calling "request_quit()". The only place where calling this
    /// function makes sense is from inside the event handler callback when
    /// the "quit_requested_event" event has been received
    pub fn cancel_quit(&mut self) {
        self.b.cancel_quit();
    }

    /// Capture mouse cursor to the current window
    /// On WASM this will automatically hide cursor
    /// On desktop this will bound cursor to windows border
    /// NOTICE: on desktop cursor will not be automatically released after window lost focus
    ///         so set_cursor_grab(false) on window's focus lost is recommended.
    /// TODO: implement window focus events
    pub fn set_cursor_grab(&mut self, grab: bool) {
        self.b.set_cursor_grab(grab);
    }

    /// Show or hide the mouse cursor
    pub fn show_mouse(&mut self, shown: bool) {
        self.b.show_mouse(shown);
    }

    /// Set the mouse cursor icon.
    pub fn set_mouse_cursor(&mut self, cursor_icon: CursorIcon) {
        self.b.set_mouse_cursor(cursor_icon);
    }

    /// Set the application's window size.
    pub fn set_window_size(&mut self, new_width: u32, new_height: u32) {
        self.b.set_window_size(new_width, new_height);
    }

    pub fn set_fullscreen(&mut self, fullscreen: bool) {
        self.b.set_fullscreen(fullscreen);
    }

    /// Get current OS clipboard value
    pub fn clipboard_get(&mut self) -> Option<String> {
        self.b.clipboard_get()
    }

    /// Save value to OS clipboard
    pub fn clipboard_set(&mut self, data: &str) {
        self.b.clipboard_set(data);
    }
    pub fn dropped_file_count(&mut self) -> usize {
        self.b.dropped_file_count()
    }
    pub fn dropped_file_bytes(&mut self, index: usize) -> Option<Vec<u8>> {
        self.b.dropped_file_bytes(index)
    }
    pub fn dropped_file_path(&mut self, index: usize) -> Option<std::path::PathBuf> {
        self.b.dropped_file_path(index)
    }

    /// Shortcut for `order_quit`. Will add a legacy attribute at some point.
    pub fn quit(&mut self) {
        self.b.order_quit()
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
/// Initialization callback will be called when miniquad's Context is ready.
/// User can take ownership on Context and store it in user Code. Or return it back
/// to miniquad and give miniquad ownership on Context.
///
/// Variant wth EventHandler:
/// ```no_run
/// # use miniquad::*;
/// struct Stage;
///
/// impl EventHandler for Stage {
///     fn update(&mut self, _ctx: &mut Context) {}
///     fn draw(&mut self, _ctx: &mut Context) {}
/// }
/// fn main() {
///     miniquad::start(conf::Conf::default(), |ctx| UserData::owning(Stage, ctx));
/// }
/// ```
///
/// Variant wth EventHandlerFree:
/// ```no_run
/// # use miniquad::*;
/// struct Stage {
///     ctx: Context,
/// }
/// impl EventHandlerFree for Stage {
///     fn update(&mut self) {}
///     fn draw(&mut self) {}
/// }
/// fn main() {
///     miniquad::start(conf::Conf::default(), |ctx| UserData::free(Stage { ctx }));
/// }
/// ```
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
    {
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
}
