#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Error {
    DlOpenError(String),
    DlSymError(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DlOpenError(msg) => write!(f, "Shared library open error:\n{msg}"),
            Self::DlSymError(msg) => write!(f, "Shared library symlink error:\n{msg}"),
        }
    }
}

#[cfg(any(target_os = "linux", target_os = "android"))]
pub mod linux {
    use super::Error;
    use libc::{dlclose, dlopen, dlsym, RTLD_LAZY, RTLD_LOCAL};
    use std::{
        ffi::{c_void, CString},
        ptr::NonNull,
    };

    pub struct Module(NonNull<c_void>);

    impl Module {
        pub fn load(path: &str) -> Result<Self, Error> {
            let cpath = CString::new(path).unwrap();
            let module = unsafe { dlopen(cpath.as_ptr(), RTLD_LAZY | RTLD_LOCAL) };
            if module.is_null() {
                Err(Error::DlOpenError(path.to_string()))
            } else {
                Ok(Module(unsafe { NonNull::new_unchecked(module) }))
            }
        }

        pub fn get_symbol<F: Sized>(&self, name: &str) -> Result<F, Error> {
            let cname = CString::new(name).unwrap();
            let symbol = unsafe { dlsym(self.0.as_ptr(), cname.as_ptr()) };
            if symbol.is_null() {
                return Err(Error::DlSymError(name.to_string()));
            }
            Ok(unsafe { std::mem::transmute_copy::<_, F>(&symbol) })
        }
    }

    impl Drop for Module {
        fn drop(&mut self) {
            unsafe { dlclose(self.0.as_ptr()) };
        }
    }
}

#[cfg(target_os = "windows")]
mod windows {
    use super::Error;
    use winapi::{
        shared::minwindef::HINSTANCE,
        um::libloaderapi::{FreeLibrary, GetProcAddress, LoadLibraryA},
    };

    pub struct Module(pub HINSTANCE);

    impl Module {
        pub fn load(path: &str) -> Result<Self, Error> {
            let cpath = std::ffi::CString::new(path).unwrap();
            let library = unsafe { LoadLibraryA(cpath.as_ptr()) };
            if library.is_null() {
                return Err(Error::DlOpenError(path.to_string()));
            }
            Ok(Self(library))
        }
        pub fn get_symbol<F: Sized>(&self, name: &str) -> Result<F, Error> {
            let cname = std::ffi::CString::new(name).unwrap();
            let proc = unsafe { GetProcAddress(self.0, cname.as_ptr() as *const _) };
            if proc.is_null() {
                return Err(Error::DlSymError(name.to_string()));
            }
            Ok(unsafe { std::mem::transmute_copy(&proc) })
        }
    }

    impl Drop for Module {
        fn drop(&mut self) {
            unsafe { FreeLibrary(self.0) };
        }
    }
}

use std::fmt::Display;

#[cfg(any(target_os = "linux", target_os = "android"))]
pub use linux::*;

#[cfg(target_os = "windows")]
pub use windows::Module;
