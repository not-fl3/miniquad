//! Context creation configuration
//!
//! A [`Conf`](struct.Conf.html) struct is used to describe a hardware and platform specific setup,
//! mostly video display settings.
//!
//! ## High DPI rendering
//!
//! You can set the [`Conf::high_dpi`](struct.Conf.html#structfield.high_dpi) flag during initialization to request
//! a full-resolution framebuffer on HighDPI displays. The default behaviour
//! is `high_dpi = false`, this means that the application will
//! render to a lower-resolution framebuffer on HighDPI displays and the
//! rendered content will be upscaled by the window system composer.
//! In a HighDPI scenario, you still request the same window size during
//! [`miniquad::start`](../fn.start.html), but the framebuffer sizes returned by [`Context::screen_size`](../graphics/struct.Context.html#method.screen_size)
//! will be scaled up according to the DPI scaling ratio.
//! You can also get a DPI scaling factor with the function
//! [`Context::dpi_scale`](../graphics/struct.Context.html#method.dpi_scale).
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
//! The functions [`screen_size`](../graphics/struct.Context.html#method.screen_size) and [`dpi_scale`](../graphics/struct.Context.html#method.dpi_scale) will
//! return the following values:
//! ```bash
//! screen_size -> (1280, 960)
//! dpi_scale   -> 2.0
//! ```
//!
//! If the high_dpi flag is false, or you're not running on a Retina display,
//! the values would be:
//! ```bash
//! screen_size -> (640, 480)
//! dpi_scale   -> 1.0
//! ```

#[derive(Debug)]
pub enum LinuxX11Gl {
    /// Use libGLX.so/libGLX.so.0 and its funciton for creating OpenGL context
    /// If there is no libGLX - just panic right away
    GLXOnly,
    /// Use libEGL.so/libEGL.so.0 and its funciton for creating OpenGL context
    /// If there is no libEGL - just panic right away
    EGLOnly,
    /// Use libGLX and if there is not libGLX - try libEGL.
    /// The default option.
    GLXWithEGLFallback,
    /// Use libEGL and if there is not libEGL - try libGLX.
    EGLWithGLXFallback,
}

#[derive(Debug)]
pub enum LinuxBackend {
    X11Only,
    WaylandOnly,
    X11WithWaylandFallback,
    WaylandWithX11Fallback,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum AppleGfxApi {
    OpenGl,
    Metal,
}

/// Platform specific settings.
#[derive(Debug)]
pub struct Platform {
    /// On X11 there are two ways to get OpenGl context: libglx.so and libegl.so
    /// Default is GLXWithEGLFallback - will try to create glx context and if fails -
    /// try EGL. If EGL also fails - panic.
    pub linux_x11_gl: LinuxX11Gl,

    /// Wayland or X11. Defaults to X11WithWaylandFallback - miniquad will try
    /// to load "libX11.so", but if there is no - will try to initialize
    /// through wayland natively. If both  fails (no graphics server at
    /// all, like KMS) - will panic.
    ///
    /// Defaults to X11Only. Wayland implementation is way too unstable right now.
    pub linux_backend: LinuxBackend,

    /// Which rendering context to create, Metal or OpenGL.
    /// Miniquad always links with Metal.framework (assuming it is always present)
    /// but it links with OpenGL dynamically and only if required.
    ///
    /// Defaults to AppleGfxApi::GL for legacy reasons.
    pub apple_gfx_api: AppleGfxApi,

    /// On some platform it is possible to ask the OS for a specific swap interval.
    /// Note that this is highly platform and implementation dependent,
    /// there is no guarantee that FPS will be equal to swap_interval.
    /// In other words - "swap_interval" is a hint for a GPU driver, this is not
    /// the way to limit FPS in the game!
    pub swap_interval: Option<i32>,

    /// Whether the framebuffer should have an alpha channel.
    /// Currently supported only on Android
    /// TODO: Make it works on web, on web it should make a transparent HTML5 canvas
    /// TODO: Document(and check) what does it actually mean on android. Transparent window?
    pub framebuffer_alpha: bool,

    /// Whether to draw the default window decorations on Wayland.
    /// Only works when using the Wayland backend.
    pub wayland_use_fallback_decorations: bool,
}

impl Default for Platform {
    fn default() -> Platform {
        Platform {
            linux_x11_gl: LinuxX11Gl::GLXWithEGLFallback,
            swap_interval: None,
            linux_backend: LinuxBackend::X11Only,
            apple_gfx_api: AppleGfxApi::OpenGl,
            framebuffer_alpha: false,
            wayland_use_fallback_decorations: true,
        }
    }
}

#[derive(Debug)]
pub struct Conf {
    /// Title of the window, defaults to an empty string.
    pub window_title: String,
    /// The preferred width of the window, ignored on wasm/android.
    ///
    /// Default: 800
    pub window_width: i32,
    /// The preferred height of the window, ignored on wasm/android.
    ///
    /// Default: 600
    pub window_height: i32,
    /// Whether the rendering canvas is full-resolution on HighDPI displays.
    ///
    /// Default: false
    pub high_dpi: bool,
    /// Whether the window should be created in fullscreen mode, ignored on wasm/android.
    ///
    /// Default: false
    pub fullscreen: bool,
    /// MSAA sample count
    ///
    /// Default: 1
    pub sample_count: i32,

    /// Determines if the application user can resize the window
    pub window_resizable: bool,

    /// Miniquad allows to change the window icon programmatically.
    /// The icon will be used as
    /// - taskbar and titlebar icons on Windows.
    /// - TODO: favicon on HTML5
    /// - TODO: taskbar and titlebar(highly dependent on the WM) icons on Linux
    /// - TODO: dock and titlebar icon on  MacOs
    pub icon: Option<Icon>,

    /// Platform specific settings. Hints to OS for context creation, driver-specific
    /// settings etc.
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
            fullscreen: true,
            sample_count: 1,
            window_resizable: false,
            icon: Some(Icon::miniquad_logo()),
            platform: Default::default(),
        }
    }
}
