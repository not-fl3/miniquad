
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
pub enum Loading {
    /// No progressbar at all, no html special requirements
    No,
    /// Will look for some specific html elements and show default progress bar
    Embedded,
    //Custom(Box<dyn LoadingPage>),
}

#[derive(Debug)]
pub struct Conf {
    pub cache: Cache,
    pub loading: Loading,
    pub window_title: String,
    /// the preferred width of the window, ignored on wasm/android
    pub window_width: i32,
    /// the preferred height of the window, ignored on wasm/android
    pub window_height: i32,
    /// whether the rendering canvas is full-resolution on HighDPI displays
    pub high_dpi: bool,
    /// whether the window should be created in fullscreen mode, ignored on wasm/android
    pub fullscreen: bool,
}

impl Default for Conf {
    fn default() -> Conf {
        Conf {
            cache: Cache::No,
            loading: Loading::No,
            window_title: "".to_owned(),
            window_width: 800,
            window_height: 600,
            high_dpi: false,
            fullscreen: false
        }
    }
}
