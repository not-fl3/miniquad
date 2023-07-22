use std::sync::mpsc;

#[derive(Default)]
pub(crate) struct DroppedFiles {
    pub paths: Vec<std::path::PathBuf>,
    pub bytes: Vec<Vec<u8>>,
}
pub(crate) struct NativeDisplayData {
    pub screen_width: i32,
    pub screen_height: i32,
    pub dpi_scale: f32,
    pub high_dpi: bool,
    pub quit_requested: bool,
    pub quit_ordered: bool,
    pub native_requests: mpsc::Sender<Request>,
    pub clipboard: Box<dyn Clipboard>,
    pub dropped_files: DroppedFiles,

    #[cfg(target_vendor = "apple")]
    pub view: crate::native::apple::frameworks::ObjcId,
    #[cfg(target_os = "ios")]
    pub view_ctrl: crate::native::apple::frameworks::ObjcId,
    #[cfg(target_vendor = "apple")]
    pub gfx_api: crate::conf::AppleGfxApi,
}
#[cfg(target_vendor = "apple")]
unsafe impl Send for NativeDisplayData {}
#[cfg(target_vendor = "apple")]
unsafe impl Sync for NativeDisplayData {}

impl NativeDisplayData {
    pub fn new(
        screen_width: i32,
        screen_height: i32,
        native_requests: mpsc::Sender<Request>,
        clipboard: Box<dyn Clipboard>,
    ) -> NativeDisplayData {
        NativeDisplayData {
            screen_width,
            screen_height,
            dpi_scale: 1.,
            high_dpi: false,
            quit_requested: false,
            quit_ordered: false,
            native_requests,
            clipboard,
            dropped_files: Default::default(),
            #[cfg(target_vendor = "apple")]
            gfx_api: crate::conf::AppleGfxApi::OpenGl,
            #[cfg(target_vendor = "apple")]
            view: std::ptr::null_mut(),
            #[cfg(target_os = "ios")]
            view_ctrl: std::ptr::null_mut(),
        }
    }
}

#[derive(Debug)]
pub(crate) enum Request {
    SetCursorGrab(bool),
    ShowMouse(bool),
    SetMouseCursor(crate::CursorIcon),
    SetWindowSize { new_width: u32, new_height: u32 },
    SetFullscreen(bool),
    ShowKeyboard(bool),
}

pub trait Clipboard: Send + Sync {
    fn get(&mut self) -> Option<String>;
    fn set(&mut self, string: &str);
}

pub mod module;

#[cfg(target_os = "linux")]
pub mod linux_x11;

#[cfg(target_os = "linux")]
pub mod linux_wayland;

#[cfg(target_os = "android")]
pub mod android;

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "android")]
pub use android::*;

#[cfg(target_arch = "wasm32")]
pub mod wasm;

#[cfg(any(target_os = "macos", target_os = "ios"))]
pub mod apple;

#[cfg(target_os = "macos")]
pub mod macos;

#[cfg(target_os = "ios")]
pub mod ios;

#[cfg(any(target_os = "android", target_os = "linux"))]
pub mod egl;

// there is no glGetProcAddr on webgl, so its impossible to make "gl" module work
// on macos.. well, there is, but way easier to just statically link to gl
#[cfg(not(target_arch = "wasm32"))]
pub mod gl;

#[cfg(target_arch = "wasm32")]
pub use wasm::webgl as gl;

pub mod query_stab;
