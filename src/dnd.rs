//! File drag & drop abstraction

#[cfg(target_arch = "wasm32")]
pub use sapp_wasm::{
    dropped_file_count,
    dropped_file_bytes,
    dropped_file_path
};

#[cfg(target_os = "windows")]
pub use sapp_windows::{
    dropped_file_count,
    dropped_file_bytes,
    dropped_file_path
};

#[cfg(target_os = "linux")]
pub use sapp_linux::{
    dropped_file_count,
    dropped_file_bytes,
    dropped_file_path
};

#[cfg(not(any(target_arch = "wasm32", target_os = "windows", target_os = "linux")))]
mod dummy {
    pub fn dropped_file_count() -> usize {
        0
    }

    pub fn dropped_file_bytes(index: usize) -> Option<Vec<u8>> {
        None
    }

    pub fn dropped_file_path(index: usize) -> Option<std::path::PathBuf> {
        None
    }
}
#[cfg(not(any(target_arch = "wasm32", target_os = "windows", target_os = "linux")))]
pub use dummy::*;