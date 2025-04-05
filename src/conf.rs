//! Context creation configuration
//!
//! A [`Conf`] struct is used to describe a hardware and platform specific setup,
//! mostly video display settings.
//!
//! ## High DPI rendering
//!
//! You can set the [`Conf::high_dpi`] flag during initialization to request
//! a full-resolution framebuffer on HighDPI displays. The default behaviour
//! is `high_dpi = false`, this means that the application will
//! render to a lower-resolution framebuffer on HighDPI displays and the
//! rendered content will be upscaled by the window system composer.
//! In a HighDPI scenario, you still request the same window size during
//! [`miniquad::start`][super::start], but the framebuffer sizes returned by
//! [`screen_size`] will be scaled up according to the DPI scaling ratio.
//! You can also get a DPI scaling factor with the function [`dpi_scale`].
//!
//! Here's an example on a Mac with Retina display:
//! ```ignore
//! Conf {
//!   width = 640,
//!   height = 480,
//!   high_dpi = true,
//!   .. Default::default()
//! };
//! ```
//!
//! The functions [`screen_size`] and [`dpi_scale`] will return the following values:
//! ```bash
//! screen_size -> (1280, 960)
//! dpi_scale   -> 2.0
//! ```
//!
//! If the `high_dpi` flag is false, or you're not running on a Retina display,
//! the values would be:
//! ```bash
//! screen_size -> (640, 480)
//! dpi_scale   -> 1.0
//! ```
//!
//! [`dpi_scale`]: super::window::dpi_scale
//! [`screen_size`]: super::window::screen_size

/// Specifies how to load an OpenGL context on X11 in Linux.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum LinuxX11Gl {
    /// Use `libGLX.so` (or `libGLX.so.0`) exclusively. Panics if unavailable.
    GLXOnly,
    /// Use `libEGL.so` (or `libEGL.so.0`) exclusively. Panics if unavailable.
    EGLOnly,
    /// Prefer `libGLX`; fall back to `libEGL` if `libGLX` is unavailable.
    /// This is the default choice.
    #[default]
    GLXWithEGLFallback,
    /// Prefer `libEGL`; fall back to `libGLX` if `libEGL` is unavailable.
    EGLWithGLXFallback,
}

/// On Linux, the backend used for windowing and event handling.
///
/// Defaults to `X11Only`. The Wayland implementation is currently unstable
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum LinuxBackend {
    /// Use only the X11 backend. Panics if unavailable. This is the default choice.
    #[default]
    X11Only,
    /// Use only the Wayland backend. Panics if unavailable.
    WaylandOnly,
    /// Prefer X11, fall back to Wayland if X11 is unavailable.
    X11WithWaylandFallback,
    /// Prefer Wayland, fall back to X11 if Wayland is unavailable.
    WaylandWithX11Fallback,
}

/// On Apple platforms, choose the rendering API for creating contexts.
///
/// Miniquad always links to Metal.framework (assuming it's present),
/// and links to OpenGL dynamically only if required.
///
/// Defaults to AppleGfxApi::GL for legacy reasons.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum AppleGfxApi {
    /// Use OpenGL for Apple platforms. This is the default choice.
    #[default]
    OpenGl,
    /// Use Metal for Apple platforms.
    Metal,
}

/// On the Web, specify which WebGL version to use.
///
/// While miniquad itself only uses WebGL 1 features, a WebGL 2 context allows to:
/// - Use GLES3 shaders.
/// - Do raw WebGL2 OpenGL calls.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum WebGLVersion {
    /// Use WebGL 1.0. This is the default choice.
    #[default]
    WebGL1,
    /// Use WebGL 2.0.
    WebGL2,
}

/// On Wayland, specify how to draw client-side decoration (CSD) if server-side decoration (SSD) is
/// not supported (e.g., on GNOME).
///
/// Defaults to ServerWithLibDecorFallback
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum WaylandDecorations {
    /// If SSD is not supported, will try to load `libdecor` to draw CSD. This is the default
    /// choice.
    #[default]
    ServerWithLibDecorFallback,
    /// If SSD is not supported, draw a light gray border.
    ServerWithMiniquadFallback,
    /// If SSD is not supported, no CSD will be drawn.
    ServerOnly,
}

/// Platform-specific settings.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Platform {
    /// Determines how to load an OpenGL context on X11 (via GLX or EGL).
    pub linux_x11_gl: LinuxX11Gl,

    /// Specifies which Linux window system (X11 or Wayland) is preferred or used.
    pub linux_backend: LinuxBackend,

    /// Specifies which WebGL version to use on the Web (1.0. or 2.0).
    pub webgl_version: WebGLVersion,

    /// Defines which rendering API to use on Apple platforms (Metal or OpenGL).
    pub apple_gfx_api: AppleGfxApi,

    /// Optional swap interval (vertical sync).
    ///
    /// Note that this is highly platform- and driver-dependent.
    /// There is no guarantee the FPS will match the specified `swap_interval`.
    /// In other words, `swap_interval` is only a hint to the GPU driver and
    /// not a reliable way to limit the game's FPS.
    pub swap_interval: Option<i32>,

    /// If `true`, the event loop will block until [`schedule_update`] is called.
    ///
    /// This can reduce CPU usage to nearly zero while waiting for events.
    ///
    /// It is recommended to call `schedule_update` at the end of `resize_event`
    /// or any relevant mouse/keyboard input.
    ///
    /// `schedule_update` may be used from other threads to "wake up" the window.
    ///
    /// [`schedule_update`]: super::window::schedule_update
    pub blocking_event_loop: bool,

    /// If `true`, the framebuffer includes an alpha channel.
    /// Currently supported only on Android.
    ///
    /// - TODO: Make it works on web, on web it should make a transparent HTML5 canvas
    /// - TODO: Document(and check) what does it actually mean on android. Transparent window?
    pub framebuffer_alpha: bool,

    /// On Wayland, specifies how to draw client-side decoration (CSD) if server-side decoration (SSD) is
    /// not supported (e.g., on GNOME).
    pub wayland_decorations: WaylandDecorations,

    /// Set the `WM_CLASS` window property on X11 and the `app_id` on Wayland. This is used
    /// by gnome to determine the window icon (together with an external `.desktop` file).
    // in fact `WM_CLASS` contains two strings "instance name" and "class name"
    // for most purposes they are the same so we just use class name for simplicity
    // https://unix.stackexchange.com/questions/494169/
    pub linux_wm_class: &'static str,
}

impl Default for Platform {
    fn default() -> Platform {
        Platform {
            linux_x11_gl: LinuxX11Gl::default(),
            linux_backend: LinuxBackend::default(),
            apple_gfx_api: AppleGfxApi::default(),
            webgl_version: WebGLVersion::default(),
            blocking_event_loop: false,
            swap_interval: None,
            framebuffer_alpha: false,
            wayland_decorations: WaylandDecorations::default(),
            linux_wm_class: "miniquad-application",
        }
    }
}

/// Describes a hardware and platform-specific setup.
#[derive(Debug)]
pub struct Conf {
    /// Window title. Defaults to an empty string.
    pub window_title: String,

    /// Preferred window width (ignored on WASM/Android).
    /// Defaults to `800`.
    pub window_width: i32,

    /// Preferred window height (ignored on WASM/Android).
    /// Defaults to `600`.
    pub window_height: i32,

    /// If `true`, the rendering canvas is scaled for HighDPI displays.
    /// Defaults to `false`.
    pub high_dpi: bool,

    /// If `true`, create the window in fullscreen mode (ignored on WASM/Android).
    /// Defaults to `false`.
    pub fullscreen: bool,

    /// MSAA sample count.
    /// Defaults to `1`.
    pub sample_count: i32,

    /// If `true`, the user can resize the window.
    pub window_resizable: bool,

    /// Optional icon data used by the OS where applicable:
    /// - On Windows, taskbar/title bar icon
    /// - On macOS, Dock/title bar icon
    /// - TODO: Favicon on HTML5
    /// - TODO: Taskbar/title bar icon on Linux (depends on WM)
    /// - Note: on gnome, icon is determined using `WM_CLASS` (can be set under [`Platform`]) and
    ///   an external `.desktop` file
    pub icon: Option<Icon>,

    /// Platform-specific hints (e.g., context creation, driver settings).
    pub platform: Platform,
}

/// Icon image in three levels of detail.
#[derive(Clone)]
pub struct Icon {
    /// 16 * 16 image of RGBA pixels (each 4 * u8) in row-major order.
    pub small: [u8; 16 * 16 * 4],
    /// 32 x 32 image of RGBA pixels (each 4 * u8) in row-major order.
    pub medium: [u8; 32 * 32 * 4],
    /// 64 x 64 image of RGBA pixels (each 4 * u8) in row-major order.
    pub big: [u8; 64 * 64 * 4],
}

impl Icon {
    pub fn miniquad_logo() -> Icon {
        Icon {
            small: crate::default_icon::SMALL,
            medium: crate::default_icon::MEDIUM,
            big: crate::default_icon::BIG,
        }
    }
}
// Printing 64x64 array with a default formatter is not meaningfull,
// so debug will skip the data fields of an Icon
impl std::fmt::Debug for Icon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Icon").finish()
    }
}

// reasonable defaults for PC and mobiles are slightly different
#[cfg(not(target_os = "android"))]
impl Default for Conf {
    fn default() -> Conf {
        Conf {
            window_title: "".to_owned(),
            window_width: 800,
            window_height: 600,
            high_dpi: false,
            fullscreen: false,
            sample_count: 1,
            window_resizable: true,
            icon: Some(Icon::miniquad_logo()),
            platform: Default::default(),
        }
    }
}

#[cfg(target_os = "android")]
impl Default for Conf {
    fn default() -> Conf {
        Conf {
            window_title: "".to_owned(),
            window_width: 800,
            window_height: 600,
            high_dpi: true,
            fullscreen: true, //
            sample_count: 1,
            window_resizable: false, //
            icon: Some(Icon::miniquad_logo()),
            platform: Default::default(),
        }
    }
}
