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
pub enum Cache {
    /// No preloading at all, filesystem::open will always panic.
    No,
    /// Load /index.txt first, and cache all the files specified.
    /// Game will not start until all the files will be cached
    Index,
    /// Same as Index, but with the files list instead of index.txt
    List(Vec<&'static str>),
    /// Tar archive contents, usually from include_bytes!
    Tar(&'static [u8]),
}

#[derive(Debug)]
pub struct Conf {
    pub cache: Cache,
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
}

impl Default for Conf {
    fn default() -> Conf {
        Conf {
            cache: Cache::No,
            window_title: "".to_owned(),
            window_width: 800,
            window_height: 600,
            high_dpi: false,
            fullscreen: false,
            sample_count: 1,
            window_resizable: true,
        }
    }
}
