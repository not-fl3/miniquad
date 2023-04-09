#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

pub type GLenum = ::std::os::raw::c_uint;
pub type GLboolean = ::std::os::raw::c_uchar;
pub type GLbitfield = ::std::os::raw::c_uint;
pub type GLvoid = ::std::os::raw::c_void;
pub type GLbyte = ::std::os::raw::c_schar;
pub type GLshort = ::std::os::raw::c_short;
pub type GLint = ::std::os::raw::c_int;
pub type GLubyte = ::std::os::raw::c_uchar;
pub type GLushort = ::std::os::raw::c_ushort;
pub type GLuint = ::std::os::raw::c_uint;
pub type GLuint64 = ::std::os::raw::c_ulonglong;
pub type GLsizei = ::std::os::raw::c_int;
pub type GLchar = ::std::os::raw::c_char;

pub type khronos_ssize_t = ::std::os::raw::c_long;
pub type khronos_usize_t = ::std::os::raw::c_ulong;
pub type khronos_intptr_t = ::std::os::raw::c_long;

pub type GLsizeiptr = khronos_ssize_t;
pub type GLintptr = khronos_intptr_t;

pub type GLfloat = f32;
pub type GLclampf = f32;
pub type GLdouble = f64;
pub type GLclampd = f64;

pub const GL_INT_2_10_10_10_REV: u32 = 0x8D9F;
pub const GL_PROGRAM_POINT_SIZE: u32 = 0x8642;
pub const GL_STENCIL_ATTACHMENT: u32 = 0x8D20;
pub const GL_DEPTH_ATTACHMENT: u32 = 0x8D00;
pub const GL_COLOR_ATTACHMENT2: u32 = 0x8CE2;
pub const GL_COLOR_ATTACHMENT0: u32 = 0x8CE0;
pub const GL_COLOR_ATTACHMENT22: u32 = 0x8CF6;
pub const GL_DRAW_FRAMEBUFFER: u32 = 0x8CA9;
pub const GL_FRAMEBUFFER_COMPLETE: u32 = 0x8CD5;
pub const GL_NUM_EXTENSIONS: u32 = 0x821D;
pub const GL_INFO_LOG_LENGTH: u32 = 0x8B84;
pub const GL_VERTEX_SHADER: u32 = 0x8B31;
pub const GL_INCR: u32 = 0x1E02;
pub const GL_DYNAMIC_DRAW: u32 = 0x88E8;
pub const GL_STATIC_DRAW: u32 = 0x88E4;
pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Z: u32 = 0x8519;
pub const GL_TEXTURE_CUBE_MAP: u32 = 0x8513;
pub const GL_FUNC_SUBTRACT: u32 = 0x800A;
pub const GL_FUNC_REVERSE_SUBTRACT: u32 = 0x800B;
pub const GL_CONSTANT_COLOR: u32 = 0x8001;
pub const GL_DECR_WRAP: u32 = 0x8508;
pub const GL_LINEAR_MIPMAP_LINEAR: u32 = 0x2703;
pub const GL_ELEMENT_ARRAY_BUFFER: u32 = 0x8893;
pub const GL_SHORT: u32 = 0x1402;
pub const GL_DEPTH_TEST: u32 = 0x0B71;
pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Y: u32 = 0x8518;
pub const GL_LINK_STATUS: u32 = 0x8B82;
pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Y: u32 = 0x8517;
pub const GL_SAMPLE_ALPHA_TO_COVERAGE: u32 = 0x809E;
pub const GL_RGBA16F: u32 = 0x881A;
pub const GL_CONSTANT_ALPHA: u32 = 0x8003;
pub const GL_READ_FRAMEBUFFER: u32 = 0x8CA8;
pub const GL_TEXTURE0: u32 = 0x84C0;
pub const GL_TEXTURE_MIN_LOD: u32 = 0x813A;
pub const GL_CLAMP_TO_EDGE: u32 = 0x812F;
pub const GL_UNSIGNED_SHORT_5_6_5: u32 = 0x8363;
pub const GL_TEXTURE_WRAP_R: u32 = 0x8072;
pub const GL_UNSIGNED_SHORT_5_5_5_1: u32 = 0x8034;
pub const GL_NEAREST_MIPMAP_NEAREST: u32 = 0x2700;
pub const GL_UNSIGNED_SHORT_4_4_4_4: u32 = 0x8033;
pub const GL_SRC_ALPHA_SATURATE: u32 = 0x0308;
pub const GL_STREAM_DRAW: u32 = 0x88E0;
pub const GL_ONE: u32 = 1;
pub const GL_NEAREST_MIPMAP_LINEAR: u32 = 0x2702;
pub const GL_RGB10_A2: u32 = 0x8059;
pub const GL_RGBA8: u32 = 0x8058;
pub const GL_COLOR_ATTACHMENT1: u32 = 0x8CE1;
pub const GL_RGBA4: u32 = 0x8056;
pub const GL_RGB8: u32 = 0x8051;
pub const GL_ARRAY_BUFFER: u32 = 0x8892;
pub const GL_STENCIL: u32 = 0x1802;
pub const GL_TEXTURE_2D: u32 = 0x0DE1;
pub const GL_DEPTH: u32 = 0x1801;
pub const GL_FRONT: u32 = 0x0404;
pub const GL_STENCIL_BUFFER_BIT: u32 = 0x00000400;
pub const GL_REPEAT: u32 = 0x2901;
pub const GL_RGBA: u32 = 0x1908;
pub const GL_TEXTURE_CUBE_MAP_POSITIVE_X: u32 = 0x8515;
pub const GL_DECR: u32 = 0x1E03;
pub const GL_FRAGMENT_SHADER: u32 = 0x8B30;
pub const GL_FLOAT: u32 = 0x1406;
pub const GL_TEXTURE_MAX_LOD: u32 = 0x813B;
pub const GL_DEPTH_COMPONENT: u32 = 0x1902;
pub const GL_ONE_MINUS_DST_ALPHA: u32 = 0x0305;
pub const GL_COLOR: u32 = 0x1800;
pub const GL_TEXTURE_2D_ARRAY: u32 = 0x8C1A;
pub const GL_TRIANGLES: u32 = 0x0004;
pub const GL_UNSIGNED_BYTE: u32 = 0x1401;
pub const GL_TEXTURE_MAG_FILTER: u32 = 0x2800;
pub const GL_ONE_MINUS_CONSTANT_ALPHA: u32 = 0x8004;
pub const GL_NONE: u32 = 0;
pub const GL_SRC_COLOR: u32 = 0x0300;
pub const GL_BYTE: u32 = 0x1400;
pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Z: u32 = 0x851A;
pub const GL_LINE_STRIP: u32 = 0x0003;
pub const GL_TEXTURE_3D: u32 = 0x806F;
pub const GL_CW: u32 = 0x0900;
pub const GL_LINEAR: u32 = 0x2601;
pub const GL_RENDERBUFFER: u32 = 0x8D41;
pub const GL_GEQUAL: u32 = 0x0206;
pub const GL_COLOR_BUFFER_BIT: u32 = 0x00004000;
pub const GL_RGBA32F: u32 = 0x8814;
pub const GL_BLEND: u32 = 0x0BE2;
pub const GL_ONE_MINUS_SRC_ALPHA: u32 = 0x0303;
pub const GL_ONE_MINUS_CONSTANT_COLOR: u32 = 0x8002;
pub const GL_TEXTURE_WRAP_T: u32 = 0x2803;
pub const GL_TEXTURE_WRAP_S: u32 = 0x2802;
pub const GL_TEXTURE_MIN_FILTER: u32 = 0x2801;
pub const GL_LINEAR_MIPMAP_NEAREST: u32 = 0x2701;
pub const GL_EXTENSIONS: u32 = 0x1F03;
pub const GL_NO_ERROR: u32 = 0;
pub const GL_REPLACE: u32 = 0x1E01;
pub const GL_KEEP: u32 = 0x1E00;
pub const GL_CCW: u32 = 0x0901;
pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_X: u32 = 0x8516;
pub const GL_RGB: u32 = 0x1907;
pub const GL_TRIANGLE_STRIP: u32 = 0x0005;
pub const GL_FALSE: u32 = 0;
pub const GL_ZERO: u32 = 0;
pub const GL_CULL_FACE: u32 = 0x0B44;
pub const GL_INVERT: u32 = 0x150A;
pub const GL_INT: u32 = 0x1404;
pub const GL_UNSIGNED_INT: u32 = 0x1405;
pub const GL_UNSIGNED_SHORT: u32 = 0x1403;
pub const GL_NEAREST: u32 = 0x2600;
pub const GL_SCISSOR_TEST: u32 = 0x0C11;
pub const GL_LEQUAL: u32 = 0x0203;
pub const GL_STENCIL_TEST: u32 = 0x0B90;
pub const GL_DITHER: u32 = 0x0BD0;
pub const GL_DEPTH_COMPONENT16: u32 = 0x81A5;
pub const GL_EQUAL: u32 = 0x0202;
pub const GL_FRAMEBUFFER: u32 = 0x8D40;
pub const GL_RGB5: u32 = 0x8050;
pub const GL_LINES: u32 = 0x0001;
pub const GL_DEPTH_BUFFER_BIT: u32 = 0x00000100;
pub const GL_SRC_ALPHA: u32 = 0x0302;
pub const GL_INCR_WRAP: u32 = 0x8507;
pub const GL_LESS: u32 = 0x0201;
pub const GL_MULTISAMPLE: u32 = 0x809D;
pub const GL_FRAMEBUFFER_BINDING: u32 = 0x8CA6;
pub const GL_BACK: u32 = 0x0405;
pub const GL_ALWAYS: u32 = 0x0207;
pub const GL_FUNC_ADD: u32 = 0x8006;
pub const GL_ONE_MINUS_DST_COLOR: u32 = 0x0307;
pub const GL_NOTEQUAL: u32 = 0x0205;
pub const GL_DST_COLOR: u32 = 0x0306;
pub const GL_COMPILE_STATUS: u32 = 0x8B81;
pub const GL_RED: u32 = 0x1903;
pub const GL_GREEN: u32 = 6404;
pub const GL_BLUE: u32 = 6405;
pub const GL_ALPHA: u32 = 6406;
pub const GL_LUMINANCE: u32 = 6409;
pub const GL_LUMINANCE_ALPHA: u32 = 6410;
pub const GL_ALPHA_BITS: u32 = 3413;
pub const GL_RED_BITS: u32 = 3410;
pub const GL_GREEN_BITS: u32 = 3411;
pub const GL_BLUE_BITS: u32 = 3412;
pub const GL_INDEX_BITS: u32 = 3409;
pub const GL_SUBPIXEL_BITS: u32 = 3408;
pub const GL_AUX_BUFFERS: u32 = 3072;
pub const GL_READ_BUFFER: u32 = 3074;
pub const GL_DRAW_BUFFER: u32 = 3073;
pub const GL_DOUBLEBUFFER: u32 = 3122;
pub const GL_COLOR_ATTACHMENT3: u32 = 0x8CE3;
pub const GL_DST_ALPHA: u32 = 0x0304;
pub const GL_RGB5_A1: u32 = 0x8057;
pub const GL_GREATER: u32 = 0x0204;
pub const GL_POLYGON_OFFSET_FILL: u32 = 0x8037;
pub const GL_TRUE: u32 = 1;
pub const GL_NEVER: u32 = 0x0200;
pub const GL_POINTS: u32 = 0x0000;
pub const GL_ONE_MINUS_SRC_COLOR: u32 = 0x0301;
pub const GL_MIRRORED_REPEAT: u32 = 0x8370;
pub const GL_MAX_COMBINED_TEXTURE_IMAGE_UNITS: u32 = 0x8B4D;
pub const GL_R11F_G11F_B10F: u32 = 0x8C3A;
pub const GL_UNSIGNED_INT_10F_11F_11F_REV: u32 = 0x8C3B;
pub const GL_RGBA32UI: u32 = 0x8D70;
pub const GL_RGB32UI: u32 = 0x8D71;
pub const GL_RGBA16UI: u32 = 0x8D76;
pub const GL_RGB16UI: u32 = 0x8D77;
pub const GL_RGBA8UI: u32 = 0x8D7C;
pub const GL_RGB8UI: u32 = 0x8D7D;
pub const GL_RGBA32I: u32 = 0x8D82;
pub const GL_RGB32I: u32 = 0x8D83;
pub const GL_RGBA16I: u32 = 0x8D88;
pub const GL_RGB16I: u32 = 0x8D89;
pub const GL_RGBA8I: u32 = 0x8D8E;
pub const GL_RGB8I: u32 = 0x8D8F;
pub const GL_RED_INTEGER: u32 = 0x8D94;
pub const GL_RG: u32 = 0x8227;
pub const GL_RG_INTEGER: u32 = 0x8228;
pub const GL_R8: u32 = 0x8229;
pub const GL_R16: u32 = 0x822A;
pub const GL_RG8: u32 = 0x822B;
pub const GL_RG16: u32 = 0x822C;
pub const GL_R16F: u32 = 0x822D;
pub const GL_R32F: u32 = 0x822E;
pub const GL_RG16F: u32 = 0x822F;
pub const GL_RG32F: u32 = 0x8230;
pub const GL_R8I: u32 = 0x8231;
pub const GL_R8UI: u32 = 0x8232;
pub const GL_R16I: u32 = 0x8233;
pub const GL_R16UI: u32 = 0x8234;
pub const GL_R32I: u32 = 0x8235;
pub const GL_R32UI: u32 = 0x8236;
pub const GL_RG8I: u32 = 0x8237;
pub const GL_RG8UI: u32 = 0x8238;
pub const GL_RG16I: u32 = 0x8239;
pub const GL_RG16UI: u32 = 0x823A;
pub const GL_RG32I: u32 = 0x823B;
pub const GL_RG32UI: u32 = 0x823C;
pub const GL_RGBA_INTEGER: u32 = 0x8D99;
pub const GL_R8_SNORM: u32 = 0x8F94;
pub const GL_RG8_SNORM: u32 = 0x8F95;
pub const GL_RGB8_SNORM: u32 = 0x8F96;
pub const GL_RGBA8_SNORM: u32 = 0x8F97;
pub const GL_R16_SNORM: u32 = 0x8F98;
pub const GL_RG16_SNORM: u32 = 0x8F99;
pub const GL_RGB16_SNORM: u32 = 0x8F9A;
pub const GL_RGBA16_SNORM: u32 = 0x8F9B;
pub const GL_RGBA16: u32 = 0x805B;
pub const GL_MAX_TEXTURE_SIZE: u32 = 0x0D33;
pub const GL_MAX_CUBE_MAP_TEXTURE_SIZE: u32 = 0x851C;
pub const GL_MAX_3D_TEXTURE_SIZE: u32 = 0x8073;
pub const GL_MAX_ARRAY_TEXTURE_LAYERS: u32 = 0x88FF;
pub const GL_MAX_VERTEX_ATTRIBS: u32 = 0x8869;
pub const GL_CLAMP_TO_BORDER: u32 = 0x812D;
pub const GL_TEXTURE_BORDER_COLOR: u32 = 0x1004;
pub const GL_UNPACK_ALIGNMENT: u32 = 3317;
pub const GL_TEXTURE_SWIZZLE_R: u32 = 36418;
pub const GL_TEXTURE_SWIZZLE_G: u32 = 36419;
pub const GL_TEXTURE_SWIZZLE_B: u32 = 36420;
pub const GL_TEXTURE_SWIZZLE_A: u32 = 36421;
pub const GL_TEXTURE_SWIZZLE_RGBA: u32 = 36422;
pub const GL_DRAW_FRAMEBUFFER_BINDING: u32 = 36006;
pub const GL_TIME_ELAPSED: u32 = 35007;
pub const GL_QUERY_RESULT: u32 = 34918;
pub const GL_QUERY_RESULT_AVAILABLE: u32 = 34919;
pub const GL_VENDOR: u32 = 0x1F00;
pub const GL_VERSION: u32 = 0x1F02;

pub const WGL_NUMBER_PIXEL_FORMATS_ARB: u32 = 0x2000;
pub const WGL_SUPPORT_OPENGL_ARB: u32 = 0x2010;
pub const WGL_DRAW_TO_WINDOW_ARB: u32 = 0x2001;
pub const WGL_PIXEL_TYPE_ARB: u32 = 0x2013;
pub const WGL_TYPE_RGBA_ARB: u32 = 0x202b;
pub const WGL_ACCELERATION_ARB: u32 = 0x2003;
pub const WGL_NO_ACCELERATION_ARB: u32 = 0x2025;
pub const WGL_RED_BITS_ARB: u32 = 0x2015;
pub const WGL_RED_SHIFT_ARB: u32 = 0x2016;
pub const WGL_GREEN_BITS_ARB: u32 = 0x2017;
pub const WGL_GREEN_SHIFT_ARB: u32 = 0x2018;
pub const WGL_BLUE_BITS_ARB: u32 = 0x2019;
pub const WGL_BLUE_SHIFT_ARB: u32 = 0x201a;
pub const WGL_ALPHA_BITS_ARB: u32 = 0x201b;
pub const WGL_ALPHA_SHIFT_ARB: u32 = 0x201c;
pub const WGL_ACCUM_BITS_ARB: u32 = 0x201d;
pub const WGL_ACCUM_RED_BITS_ARB: u32 = 0x201e;
pub const WGL_ACCUM_GREEN_BITS_ARB: u32 = 0x201f;
pub const WGL_ACCUM_BLUE_BITS_ARB: u32 = 0x2020;
pub const WGL_ACCUM_ALPHA_BITS_ARB: u32 = 0x2021;
pub const WGL_DEPTH_BITS_ARB: u32 = 0x2022;
pub const WGL_STENCIL_BITS_ARB: u32 = 0x2023;
pub const WGL_AUX_BUFFERS_ARB: u32 = 0x2024;
pub const WGL_STEREO_ARB: u32 = 0x2012;
pub const WGL_DOUBLE_BUFFER_ARB: u32 = 0x2011;
pub const WGL_SAMPLES_ARB: u32 = 0x2042;
pub const WGL_FRAMEBUFFER_SRGB_CAPABLE_ARB: u32 = 0x20a9;
pub const WGL_CONTEXT_DEBUG_BIT_ARB: u32 = 0x00000001;
pub const WGL_CONTEXT_FORWARD_COMPATIBLE_BIT_ARB: u32 = 0x00000002;
pub const WGL_CONTEXT_PROFILE_MASK_ARB: u32 = 0x9126;
pub const WGL_CONTEXT_CORE_PROFILE_BIT_ARB: u32 = 0x00000001;
pub const WGL_CONTEXT_COMPATIBILITY_PROFILE_BIT_ARB: u32 = 0x00000002;
pub const WGL_CONTEXT_MAJOR_VERSION_ARB: u32 = 0x2091;
pub const WGL_CONTEXT_MINOR_VERSION_ARB: u32 = 0x2092;
pub const WGL_CONTEXT_FLAGS_ARB: u32 = 0x2094;
pub const WGL_CONTEXT_ROBUST_ACCESS_BIT_ARB: u32 = 0x00000004;
pub const WGL_LOSE_CONTEXT_ON_RESET_ARB: u32 = 0x8252;
pub const WGL_CONTEXT_RESET_NOTIFICATION_STRATEGY_ARB: u32 = 0x8256;
pub const WGL_NO_RESET_NOTIFICATION_ARB: u32 = 0x8261;
pub const WGL_CONTEXT_RELEASE_BEHAVIOR_ARB: u32 = 0x2097;
pub const WGL_CONTEXT_RELEASE_BEHAVIOR_NONE_ARB: u32 = 0;
pub const WGL_CONTEXT_RELEASE_BEHAVIOR_FLUSH_ARB: u32 = 0x2098;
pub const WGL_COLORSPACE_EXT: u32 = 0x309d;
pub const WGL_COLORSPACE_SRGB_EXT: u32 = 0x3089;
pub const ERROR_INVALID_VERSION_ARB: u32 = 0x2095;
pub const ERROR_INVALID_PROFILE_ARB: u32 = 0x2096;
pub const ERROR_INCOMPATIBLE_DEVICE_CONTEXTS_ARB: u32 = 0x2054;

macro_rules! gl_loader {
    (
        $(
            fn $fn:ident ( $($arg:ident : $t:ty),* ) -> $res:ty
        ),*
    ) => {
        mod __pfns {
            use super::*;

            $(
                pub static mut $fn: Option<extern "C" fn ($($arg: $t),*) -> $res> = None;
            )*
        }

        $(
            pub unsafe fn $fn($($arg: $t),*) -> $res {
                __pfns::$fn.unwrap()( $($arg),* )
            }
        )*

        pub fn load_gl_funcs<T: FnMut(&str) -> Option<unsafe extern "C" fn() -> ()>>(mut getprocaddr: T) {
            $(
                unsafe {
                    let fn_name = stringify!($fn);
                    __pfns::$fn = ::std::mem::transmute_copy(&getprocaddr(fn_name));
                }
            )*
        }
    };
}

gl_loader!(
    fn glGetStringi(name: GLenum, index: GLuint) -> *const GLubyte,
    fn glGetString(name: GLenum) -> *const GLubyte,
    fn glFramebufferTextureLayer(
        target: GLenum,
        attachment: GLenum,
        texture: GLuint,
        level: GLint,
        layer: GLint
    ) -> (),
    fn glGenFramebuffers(n: GLsizei, framebuffers: *mut GLuint) -> (),
    fn glBindFramebuffer(target: GLenum, framebuffer: GLuint) -> (),
    fn glBindRenderbuffer(target: GLenum, renderbuffer: GLuint) -> (),
    fn glClearBufferfi(buffer: GLenum, drawbuffer: GLint, depth: GLfloat, stencil: GLint) -> (),
    fn glClearBufferfv(buffer: GLenum, drawbuffer: GLint, value: *const GLfloat) -> (),
    fn glClearBufferuiv(buffer: GLenum, drawbuffer: GLint, value: *const GLuint) -> (),
    fn glDeleteRenderbuffers(n: GLsizei, renderbuffers: *const GLuint) -> (),
    fn glUniform4fv(location: GLint, count: GLsizei, value: *const GLfloat) -> (),
    fn glUniform3fv(location: GLint, count: GLsizei, value: *const GLfloat) -> (),
    fn glUniform2fv(location: GLint, count: GLsizei, value: *const GLfloat) -> (),
    fn glUniform1fv(location: GLint, count: GLsizei, value: *const GLfloat) -> (),
    fn glUniform1iv(location: GLint, count: GLsizei, value: *const GLint) -> (),
    fn glUniform2iv(location: GLint, count: GLsizei, value: *const GLint) -> (),
    fn glUniform3iv(location: GLint, count: GLsizei, value: *const GLint) -> (),
    fn glUniform4iv(location: GLint, count: GLsizei, value: *const GLint) -> (),
    fn glUniform1i(location: GLint, v0: GLint) -> (),
    fn glUniform2i(location: GLint, v0: GLint, v1: GLint) -> (),
    fn glUniform3i(location: GLint, v0: GLint, v1: GLint, v2: GLint) -> (),
    fn glUniform4i(location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint) -> (),
    fn glUniform1f(location: GLint, v0: GLfloat) -> (),
    fn glUniform2f(location: GLint, v0: GLfloat, v1: GLfloat) -> (),
    fn glUniform3f(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat) -> (),
    fn glUniform4f(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat) -> (),
    fn glUseProgram(program: GLuint) -> (),
    fn glShaderSource(
        shader: GLuint,
        count: GLsizei,
        string: *const *const GLchar,
        length: *const GLint
    ) -> (),
    fn glLinkProgram(program: GLuint) -> (),
    fn glPixelStorei(pname: GLenum, param: GLint) -> (),
    fn glGetUniformLocation(program: GLuint, name: *const GLchar) -> GLint,
    fn glGetShaderiv(shader: GLuint, pname: GLenum, params: *mut GLint) -> (),
    fn glGetProgramInfoLog(
        program: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        infoLog: *mut GLchar
    ) -> (),
    fn glGetAttribLocation(program: GLuint, name: *const GLchar) -> GLint,
    fn glDisableVertexAttribArray(index: GLuint) -> (),
    fn glDeleteShader(shader: GLuint) -> (),
    fn glDeleteProgram(program: GLuint) -> (),
    fn glCompileShader(shader: GLuint) -> (),
    fn glStencilFuncSeparate(face: GLenum, func: GLenum, ref_: GLint, mask: GLuint) -> (),
    fn glStencilOpSeparate(face: GLenum, sfail: GLenum, dpfail: GLenum, dppass: GLenum) -> (),
    fn glRenderbufferStorageMultisample(
        target: GLenum,
        samples: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei
    ) -> (),
    fn glDrawBuffers(n: GLsizei, bufs: *const GLenum) -> (),
    fn glVertexAttribDivisor(index: GLuint, divisor: GLuint) -> (),
    fn glBufferSubData(
        target: GLenum,
        offset: GLintptr,
        size: GLsizeiptr,
        data: *const ::std::os::raw::c_void
    ) -> (),
    fn glGenBuffers(n: GLsizei, buffers: *mut GLuint) -> (),
    fn glCheckFramebufferStatus(target: GLenum) -> GLenum,
    fn glFramebufferRenderbuffer(
        target: GLenum,
        attachment: GLenum,
        renderbuffertarget: GLenum,
        renderbuffer: GLuint
    ) -> (),
    fn glCompressedTexImage2D(
        target: GLenum,
        level: GLint,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
        imageSize: GLsizei,
        data: *const GLvoid
    ) -> (),
    fn glCompressedTexImage3D(
        target: GLenum,
        level: GLint,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        border: GLint,
        imageSize: GLsizei,
        data: *const GLvoid
    ) -> (),
    fn glActiveTexture(texture: GLenum) -> (),
    fn glTexSubImage3D(
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        format: GLenum,
        type_: GLenum,
        pixels: *const GLvoid
    ) -> (),
    fn glUniformMatrix2fv(
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat
    ) -> (),
    fn glUniformMatrix3fv(
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat
    ) -> (),
    fn glUniformMatrix4fv(
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat
    ) -> (),
    fn glRenderbufferStorage(
        target: GLenum,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei
    ) -> (),
    fn glPolygonOffset(factor: GLfloat, units: GLfloat) -> (),
    fn glDrawElements(mode: GLenum, count: GLsizei, type_: GLenum, indices: *const GLvoid) -> (),
    fn glDeleteFramebuffers(n: GLsizei, framebuffers: *const GLuint) -> (),
    fn glBlendEquationSeparate(modeRGB: GLenum, modeAlpha: GLenum) -> (),
    fn glDeleteTextures(n: GLsizei, textures: *const GLuint) -> (),
    fn glGetProgramiv(program: GLuint, pname: GLenum, params: *mut GLint) -> (),
    fn glBindTexture(target: GLenum, texture: GLuint) -> (),
    fn glTexImage3D(
        target: GLenum,
        level: GLint,
        internalFormat: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        border: GLint,
        format: GLenum,
        type_: GLenum,
        pixels: *const GLvoid
    ) -> (),
    fn glCreateShader(type_: GLenum) -> GLuint,
    fn glTexSubImage2D(
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        type_: GLenum,
        pixels: *const GLvoid
    ) -> (),
    fn glCopyTexImage2D(
        target: GLenum,
        level: GLint,
        internalformat: GLenum,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
        border: GLint
    ) -> (),
    fn glClearDepthf(d: GLfloat) -> (),
    fn glClearDepth(depth: GLclampd) -> (),
    fn glFramebufferTexture2D(
        target: GLenum,
        attachment: GLenum,
        textarget: GLenum,
        texture: GLuint,
        level: GLint
    ) -> (),
    fn glCreateProgram() -> GLuint,
    fn glViewport(x: GLint, y: GLint, width: GLsizei, height: GLsizei) -> (),
    fn glDeleteBuffers(n: GLsizei, buffers: *const GLuint) -> (),
    fn glDrawArrays(mode: GLenum, first: GLint, count: GLsizei) -> (),
    fn glDrawElementsInstanced(
        mode: GLenum,
        count: GLsizei,
        type_: GLenum,
        indices: *const ::std::os::raw::c_void,
        instancecount: GLsizei
    ) -> (),
    fn glVertexAttribPointer(
        index: GLuint,
        size: GLint,
        type_: GLenum,
        normalized: GLboolean,
        stride: GLsizei,
        pointer: *const ::std::os::raw::c_void
    ) -> (),
    fn glDisable(cap: GLenum) -> (),
    fn glColorMask(red: GLboolean, green: GLboolean, blue: GLboolean, alpha: GLboolean) -> (),
    fn glBindBuffer(target: GLenum, buffer: GLuint) -> (),
    fn glBindVertexArray(array: GLuint) -> (),
    fn glDeleteVertexArrays(n: GLsizei, arrays: *const GLuint) -> (),
    fn glDepthMask(flag: GLboolean) -> (),
    fn glDrawArraysInstanced(
        mode: GLenum,
        first: GLint,
        count: GLsizei,
        instancecount: GLsizei
    ) -> (),
    fn glClearStencil(s: GLint) -> (),
    fn glScissor(x: GLint, y: GLint, width: GLsizei, height: GLsizei) -> (),
    fn glGenRenderbuffers(n: GLsizei, renderbuffers: *mut GLuint) -> (),
    fn glBufferData(
        target: GLenum,
        size: GLsizeiptr,
        data: *const ::std::os::raw::c_void,
        usage: GLenum
    ) -> (),
    fn glBlendFuncSeparate(
        sfactorRGB: GLenum,
        dfactorRGB: GLenum,
        sfactorAlpha: GLenum,
        dfactorAlpha: GLenum
    ) -> (),
    fn glTexParameteri(target: GLenum, pname: GLenum, param: GLint) -> (),
    fn glGetIntegerv(pname: GLenum, params: *mut GLint) -> (),
    fn glEnable(cap: GLenum) -> (),
    fn glBlitFramebuffer(
        srcX0: GLint,
        srcY0: GLint,
        srcX1: GLint,
        srcY1: GLint,
        dstX0: GLint,
        dstY0: GLint,
        dstX1: GLint,
        dstY1: GLint,
        mask: GLbitfield,
        filter: GLenum
    ) -> (),
    fn glStencilMask(mask: GLuint) -> (),
    fn glStencilMaskSeparate(face: GLenum, mask: GLuint) -> (),
    fn glAttachShader(program: GLuint, shader: GLuint) -> (),
    fn glGetError() -> GLenum,
    fn glClearColor(red: GLclampf, green: GLclampf, blue: GLclampf, alpha: GLclampf) -> (),
    fn glBlendColor(red: GLclampf, green: GLclampf, blue: GLclampf, alpha: GLclampf) -> (),
    fn glTexParameterf(target: GLenum, pname: GLenum, param: GLfloat) -> (),
    fn glTexParameterfv(target: GLenum, pname: GLenum, params: *const GLfloat) -> (),
    fn glGetShaderInfoLog(
        shader: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        infoLog: *mut GLchar
    ) -> (),
    fn glDepthFunc(func: GLenum) -> (),
    fn glStencilOp(fail: GLenum, zfail: GLenum, zpass: GLenum) -> (),
    fn glStencilFunc(func: GLenum, ref_: GLint, mask: GLuint) -> (),
    fn glEnableVertexAttribArray(index: GLuint) -> (),
    fn glBlendFunc(sfactor: GLenum, dfactor: GLenum) -> (),
    fn glReadBuffer(mode: GLenum) -> (),
    fn glClear(mask: GLbitfield) -> (),
    fn glTexImage2D(
        target: GLenum,
        level: GLint,
        internalFormat: GLint,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
        format: GLenum,
        type_: GLenum,
        pixels: *const GLvoid
    ) -> (),
    fn glGenVertexArrays(n: GLsizei, arrays: *mut GLuint) -> (),
    fn glFrontFace(mode: GLenum) -> (),
    fn glCullFace(mode: GLenum) -> (),
    fn glGenTextures(n: GLsizei, textures: *mut GLuint) -> (),
    fn glReadPixels(
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        type_: GLenum,
        pixels: *mut GLvoid
    ) -> (),
    fn glBeginQuery(target: GLenum, id: GLuint) -> (),
    fn glDeleteQueries(n: GLsizei, ids: *const GLuint) -> (),
    fn glEndQuery(target: GLenum) -> (),
    fn glGenQueries(n: GLsizei, ids: *mut GLuint) -> (),
    fn glGetQueryObjectiv(id: GLuint, pname: GLenum, params: *mut GLint) -> (),
    fn glGetQueryObjectui64v(id: GLuint, pname: GLenum, params: *mut GLuint64) -> (),
    fn glFlush() -> (),
    fn glFinish() -> ()
);

// note that glGetString only works after first glSwapBuffer,
// not just after context creation
pub unsafe fn is_gl2() -> bool {
    let version_string = glGetString(super::gl::GL_VERSION);
    let version_string = std::ffi::CStr::from_ptr(version_string as _)
        .to_str()
        .unwrap();

    version_string.is_empty()
        || version_string.starts_with("2")
        || version_string.starts_with("OpenGL ES 2")
}
