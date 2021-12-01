#[derive(Debug)]
pub enum Error {
    DlOpenError,
    DlSymError,
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
            let path = CString::new(path).unwrap();

            let module = unsafe { dlopen(path.as_ptr(), RTLD_LAZY | RTLD_LOCAL) };
            if module.is_null() {
                Err(Error::DlOpenError)
            } else {
                Ok(Module(unsafe { NonNull::new_unchecked(module) }))
            }
        }

        pub fn get_symbol<F: Sized>(&self, name: &str) -> Result<F, Error> {
            let name = CString::new(name).unwrap();

            let symbol = unsafe { dlsym(self.0.as_ptr(), name.as_ptr()) };

            if symbol.is_null() {
                return Err(Error::DlSymError);
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
            let path = std::ffi::CString::new(path).unwrap();
            let library = unsafe { LoadLibraryA(path.as_ptr()) };

            if library.is_null() {
                return Err(Error::DlOpenError);
            }
            Ok(Self(library))
        }
        pub fn get_symbol<F: Sized>(&self, name: &str) -> Result<F, Error> {
            let name = std::ffi::CString::new(name).unwrap();
            let proc = unsafe { GetProcAddress(self.0, name.as_ptr() as *const _) };

            if proc.is_null() {
                return Err(Error::DlSymError);
            }
            return Ok(unsafe { std::mem::transmute_copy(&proc) });
        }
    }

    impl Drop for Module {
        fn drop(&mut self) {
            unsafe { FreeLibrary(self.0) };
        }
    }
}

#[cfg(any(target_os = "linux", target_os = "android"))]
pub use linux::*;

#[cfg(target_os = "windows")]
pub use windows::Module;
