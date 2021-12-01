#![allow(non_camel_case_types, dead_code, non_snake_case)]

use winapi::shared::{minwindef::*, ntdef::*, windef::*};

pub type wglCreateContext = extern "system" fn(_: HDC) -> HGLRC;
pub type wglDeleteContext = extern "system" fn(_: HGLRC) -> bool;
pub type wglGetProcAddress = extern "system" fn(_: LPCSTR) -> PROC;
pub type wglGetCurrentDC = extern "system" fn() -> HDC;
pub type wglMakeCurrent = extern "system" fn(_: HDC, _: HGLRC) -> bool;

pub struct LibOpengl32 {
    pub module: crate::native::module::Module,
    pub wglCreateContext: wglCreateContext,
    pub wglDeleteContext: wglDeleteContext,
    pub wglGetProcAddress: wglGetProcAddress,
    pub wglGetCurrentDC: wglGetCurrentDC,
    pub wglMakeCurrent: wglMakeCurrent,
}

impl LibOpengl32 {
    pub fn try_load() -> Option<LibOpengl32> {
        crate::native::module::Module::load("opengl32.dll")
            .map(|module| LibOpengl32 {
                wglCreateContext: module.get_symbol("wglCreateContext").unwrap(),
                wglDeleteContext: module.get_symbol("wglDeleteContext").unwrap(),
                wglGetProcAddress: module.get_symbol("wglGetProcAddress").unwrap(),
                wglGetCurrentDC: module.get_symbol("wglGetCurrentDC").unwrap(),
                wglMakeCurrent: module.get_symbol("wglMakeCurrent").unwrap(),
                module,
            })
            .ok()
    }
}
