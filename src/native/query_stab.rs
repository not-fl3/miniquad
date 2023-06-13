#![allow(non_snake_case, dead_code)]

use crate::native::gl::{GLenum, GLint, GLuint, GLuint64};

pub const GL_TIME_ELAPSED: u32 = 35007;

pub unsafe fn glGetQueryObjectui64v(_id: GLuint, _pname: GLenum, _params: *mut GLuint64) {
    unimplemented!();
}

pub unsafe fn glGetQueryObjectiv(_id: GLuint, _pname: GLenum, _params: *mut GLint) {
    unimplemented!();
}
