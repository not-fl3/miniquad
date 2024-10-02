//! There is no glGetProcAddr on web.
//! The only way to get gl functions - actually tell the linker to link with
//! their gl.js counterparts.

#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]

pub type GLenum = ::core::ffi::c_uint;
pub type GLboolean = ::core::ffi::c_uchar;
pub type GLbitfield = ::core::ffi::c_uint;
pub type GLvoid = ::core::ffi::c_void;
pub type GLbyte = ::core::ffi::c_schar;
pub type GLshort = ::core::ffi::c_short;
pub type GLint = ::core::ffi::c_int;
pub type GLubyte = ::core::ffi::c_uchar;
pub type GLushort = ::core::ffi::c_ushort;
pub type GLuint = ::core::ffi::c_uint;
pub type GLint64 = ::core::ffi::c_longlong;
pub type GLuint64 = ::core::ffi::c_ulonglong;
pub type GLsizei = ::core::ffi::c_int;
pub type GLchar = ::core::ffi::c_char;

pub type khronos_ssize_t = ::core::ffi::c_long;
pub type khronos_usize_t = ::core::ffi::c_ulong;
pub type khronos_intptr_t = ::core::ffi::c_long;

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
pub const GL_DEPTH_COMPONENT24: u32 = 0x81A6;
pub const GL_DEPTH_COMPONENT32: u32 = 0x81A7;
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
pub const GL_SHADING_LANGUAGE_VERSION: GLenum = 0x8B8C;
pub const GL_TEXTURE_BASE_LEVEL: GLenum = 0x813C;
pub const GL_TEXTURE_MAX_LEVEL: GLenum = 0x813D;
pub const GL_TEXTURE_CUBE_MAP_SEAMLESS: GLenum = 0x884F;

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

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __GLsync {
    _unused: [u8; 0],
}
pub type GLsync = *mut __GLsync;

extern "C" {
    pub fn glActiveTexture(texture: GLenum);
    pub fn glAttachShader(program: GLuint, shader: GLuint);
    pub fn glBindAttribLocation(program: GLuint, index: GLuint, name: *const GLchar);
    pub fn glBindBuffer(target: GLenum, buffer: GLuint);
    pub fn glBindFramebuffer(target: GLenum, framebuffer: GLuint);
    pub fn glBindRenderbuffer(target: GLenum, renderbuffer: GLuint);
    pub fn glBindTexture(target: GLenum, texture: GLuint);
    pub fn glBlendColor(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat);
    pub fn glBlendEquation(mode: GLenum);
    pub fn glBlendEquationSeparate(modeRGB: GLenum, modeAlpha: GLenum);
    pub fn glBlendFunc(sfactor: GLenum, dfactor: GLenum);
    pub fn glBlendFuncSeparate(
        sfactorRGB: GLenum,
        dfactorRGB: GLenum,
        sfactorAlpha: GLenum,
        dfactorAlpha: GLenum,
    );
    pub fn glBufferData(
        target: GLenum,
        size: GLsizeiptr,
        data: *const ::core::ffi::c_void,
        usage: GLenum,
    );
    pub fn glBufferSubData(
        target: GLenum,
        offset: GLintptr,
        size: GLsizeiptr,
        data: *const ::core::ffi::c_void,
    );
    pub fn glCheckFramebufferStatus(target: GLenum) -> GLenum;
    pub fn glClear(mask: GLbitfield);
    pub fn glClearColor(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat);
    pub fn glClearDepthf(d: GLfloat);
    pub fn glClearStencil(s: GLint);
    pub fn glColorMask(red: GLboolean, green: GLboolean, blue: GLboolean, alpha: GLboolean);
    pub fn glCompileShader(shader: GLuint);
    pub fn glCompressedTexImage2D(
        target: GLenum,
        level: GLint,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
        imageSize: GLsizei,
        data: *const ::core::ffi::c_void,
    );
    pub fn glCompressedTexSubImage2D(
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        imageSize: GLsizei,
        data: *const ::core::ffi::c_void,
    );
    pub fn glCopyTexImage2D(
        target: GLenum,
        level: GLint,
        internalformat: GLenum,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
    );
    pub fn glCopyTexSubImage2D(
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
    );
    pub fn glCreateProgram() -> GLuint;
    pub fn glCreateShader(type_: GLenum) -> GLuint;
    pub fn glCullFace(mode: GLenum);
    pub fn glDeleteBuffers(n: GLsizei, buffers: *const GLuint);
    pub fn glDeleteFramebuffers(n: GLsizei, framebuffers: *const GLuint);
    pub fn glDeleteProgram(program: GLuint);
    pub fn glDeleteRenderbuffers(n: GLsizei, renderbuffers: *const GLuint);
    pub fn glDeleteShader(shader: GLuint);
    pub fn glDeleteTextures(n: GLsizei, textures: *const GLuint);
    pub fn glDepthFunc(func: GLenum);
    pub fn glDepthMask(flag: GLboolean);
    pub fn glDepthRangef(n: GLfloat, f: GLfloat);
    pub fn glDetachShader(program: GLuint, shader: GLuint);
    pub fn glDisable(cap: GLenum);
    pub fn glDisableVertexAttribArray(index: GLuint);
    pub fn glDrawArrays(mode: GLenum, first: GLint, count: GLsizei);
    pub fn glDrawElements(
        mode: GLenum,
        count: GLsizei,
        type_: GLenum,
        indices: *const ::core::ffi::c_void,
    );
    pub fn glEnable(cap: GLenum);
    pub fn glEnableVertexAttribArray(index: GLuint);
    pub fn glFinish();
    pub fn glFlush();
    pub fn glFramebufferRenderbuffer(
        target: GLenum,
        attachment: GLenum,
        renderbuffertarget: GLenum,
        renderbuffer: GLuint,
    );
    pub fn glFramebufferTexture2D(
        target: GLenum,
        attachment: GLenum,
        textarget: GLenum,
        texture: GLuint,
        level: GLint,
    );
    pub fn glFrontFace(mode: GLenum);
    pub fn glGenBuffers(n: GLsizei, buffers: *mut GLuint);
    pub fn glGenerateMipmap(target: GLenum);
    pub fn glGenFramebuffers(n: GLsizei, framebuffers: *mut GLuint);
    pub fn glGenRenderbuffers(n: GLsizei, renderbuffers: *mut GLuint);
    pub fn glGenTextures(n: GLsizei, textures: *mut GLuint);
    pub fn glGetActiveAttrib(
        program: GLuint,
        index: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        size: *mut GLint,
        type_: *mut GLenum,
        name: *mut GLchar,
    );
    pub fn glGetActiveUniform(
        program: GLuint,
        index: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        size: *mut GLint,
        type_: *mut GLenum,
        name: *mut GLchar,
    );
    pub fn glGetAttachedShaders(
        program: GLuint,
        maxCount: GLsizei,
        count: *mut GLsizei,
        shaders: *mut GLuint,
    );
    pub fn glGetAttribLocation(program: GLuint, name: *const GLchar) -> GLint;
    pub fn glGetBooleanv(pname: GLenum, data: *mut GLboolean);
    pub fn glGetBufferParameteriv(target: GLenum, pname: GLenum, params: *mut GLint);
    pub fn glGetError() -> GLenum;
    pub fn glGetFloatv(pname: GLenum, data: *mut GLfloat);
    pub fn glGetFramebufferAttachmentParameteriv(
        target: GLenum,
        attachment: GLenum,
        pname: GLenum,
        params: *mut GLint,
    );
    pub fn glGetIntegerv(pname: GLenum, data: *mut GLint);
    pub fn glGetProgramiv(program: GLuint, pname: GLenum, params: *mut GLint);
    pub fn glGetProgramInfoLog(
        program: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        infoLog: *mut GLchar,
    );
    pub fn glGetRenderbufferParameteriv(target: GLenum, pname: GLenum, params: *mut GLint);
    pub fn glGetShaderiv(shader: GLuint, pname: GLenum, params: *mut GLint);
    pub fn glGetShaderInfoLog(
        shader: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        infoLog: *mut GLchar,
    );
    pub fn glGetShaderPrecisionFormat(
        shadertype: GLenum,
        precisiontype: GLenum,
        range: *mut GLint,
        precision: *mut GLint,
    );
    pub fn glGetShaderSource(
        shader: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        source: *mut GLchar,
    );
    pub fn glGetString(name: GLenum) -> *const GLubyte;
    pub fn glGetTexParameterfv(target: GLenum, pname: GLenum, params: *mut GLfloat);
    pub fn glGetTexParameteriv(target: GLenum, pname: GLenum, params: *mut GLint);
    pub fn glGetUniformfv(program: GLuint, location: GLint, params: *mut GLfloat);
    pub fn glGetUniformiv(program: GLuint, location: GLint, params: *mut GLint);
    pub fn glGetUniformLocation(program: GLuint, name: *const GLchar) -> GLint;
    pub fn glGetVertexAttribfv(index: GLuint, pname: GLenum, params: *mut GLfloat);
    pub fn glGetVertexAttribiv(index: GLuint, pname: GLenum, params: *mut GLint);
    pub fn glGetVertexAttribPointerv(
        index: GLuint,
        pname: GLenum,
        pointer: *mut *mut ::core::ffi::c_void,
    );
    pub fn glHint(target: GLenum, mode: GLenum);
    pub fn glIsBuffer(buffer: GLuint) -> GLboolean;
    pub fn glIsEnabled(cap: GLenum) -> GLboolean;
    pub fn glIsFramebuffer(framebuffer: GLuint) -> GLboolean;
    pub fn glIsProgram(program: GLuint) -> GLboolean;
    pub fn glIsRenderbuffer(renderbuffer: GLuint) -> GLboolean;
    pub fn glIsShader(shader: GLuint) -> GLboolean;
    pub fn glIsTexture(texture: GLuint) -> GLboolean;
    pub fn glLineWidth(width: GLfloat);
    pub fn glLinkProgram(program: GLuint);
    pub fn glPixelStorei(pname: GLenum, param: GLint);
    pub fn glPolygonOffset(factor: GLfloat, units: GLfloat);
    pub fn glReadPixels(
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        type_: GLenum,
        pixels: *mut ::core::ffi::c_void,
    );
    pub fn glReleaseShaderCompiler();
    pub fn glRenderbufferStorage(
        target: GLenum,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
    );
    pub fn glSampleCoverage(value: GLfloat, invert: GLboolean);
    pub fn glScissor(x: GLint, y: GLint, width: GLsizei, height: GLsizei);
    pub fn glShaderBinary(
        count: GLsizei,
        shaders: *const GLuint,
        binaryformat: GLenum,
        binary: *const ::core::ffi::c_void,
        length: GLsizei,
    );
    pub fn glShaderSource(
        shader: GLuint,
        count: GLsizei,
        string: *const *const GLchar,
        length: *const GLint,
    );
    pub fn glStencilFunc(func: GLenum, ref_: GLint, mask: GLuint);
    pub fn glStencilFuncSeparate(face: GLenum, func: GLenum, ref_: GLint, mask: GLuint);
    pub fn glStencilMask(mask: GLuint);
    pub fn glStencilMaskSeparate(face: GLenum, mask: GLuint);
    pub fn glStencilOp(fail: GLenum, zfail: GLenum, zpass: GLenum);
    pub fn glStencilOpSeparate(face: GLenum, sfail: GLenum, dpfail: GLenum, dppass: GLenum);
    pub fn glTexImage2D(
        target: GLenum,
        level: GLint,
        internalformat: GLint,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
        format: GLenum,
        type_: GLenum,
        pixels: *const ::core::ffi::c_void,
    );
    pub fn glTexParameterf(target: GLenum, pname: GLenum, param: GLfloat);
    pub fn glTexParameterfv(target: GLenum, pname: GLenum, params: *const GLfloat);
    pub fn glTexParameteri(target: GLenum, pname: GLenum, param: GLint);
    pub fn glTexParameteriv(target: GLenum, pname: GLenum, params: *const GLint);
    pub fn glTexSubImage2D(
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        type_: GLenum,
        pixels: *const ::core::ffi::c_void,
    );
    pub fn glUniform1f(location: GLint, v0: GLfloat);
    pub fn glUniform1fv(location: GLint, count: GLsizei, value: *const GLfloat);
    pub fn glUniform1i(location: GLint, v0: GLint);
    pub fn glUniform1iv(location: GLint, count: GLsizei, value: *const GLint);
    pub fn glUniform2f(location: GLint, v0: GLfloat, v1: GLfloat);
    pub fn glUniform2fv(location: GLint, count: GLsizei, value: *const GLfloat);
    pub fn glUniform2i(location: GLint, v0: GLint, v1: GLint);
    pub fn glUniform2iv(location: GLint, count: GLsizei, value: *const GLint);
    pub fn glUniform3f(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat);
    pub fn glUniform3fv(location: GLint, count: GLsizei, value: *const GLfloat);
    pub fn glUniform3i(location: GLint, v0: GLint, v1: GLint, v2: GLint);
    pub fn glUniform3iv(location: GLint, count: GLsizei, value: *const GLint);
    pub fn glUniform4f(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat);
    pub fn glUniform4fv(location: GLint, count: GLsizei, value: *const GLfloat);
    pub fn glUniform4i(location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint);
    pub fn glUniform4iv(location: GLint, count: GLsizei, value: *const GLint);
    pub fn glUniformMatrix2fv(
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    );
    pub fn glUniformMatrix3fv(
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    );
    pub fn glUniformMatrix4fv(
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    );
    pub fn glUseProgram(program: GLuint);
    pub fn glValidateProgram(program: GLuint);
    pub fn glVertexAttrib1f(index: GLuint, x: GLfloat);
    pub fn glVertexAttrib1fv(index: GLuint, v: *const GLfloat);
    pub fn glVertexAttrib2f(index: GLuint, x: GLfloat, y: GLfloat);
    pub fn glVertexAttrib2fv(index: GLuint, v: *const GLfloat);
    pub fn glVertexAttrib3f(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat);
    pub fn glVertexAttrib3fv(index: GLuint, v: *const GLfloat);
    pub fn glVertexAttrib4f(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);
    pub fn glVertexAttrib4fv(index: GLuint, v: *const GLfloat);
    pub fn glVertexAttribPointer(
        index: GLuint,
        size: GLint,
        type_: GLenum,
        normalized: GLboolean,
        stride: GLsizei,
        pointer: *const ::core::ffi::c_void,
    );
    pub fn glVertexAttribIPointer(
        index: GLuint,
        size: GLint,
        type_: GLenum,
        stride: GLsizei,
        pointer: *const ::core::ffi::c_void,
    );
    pub fn glViewport(x: GLint, y: GLint, width: GLsizei, height: GLsizei);
    pub fn glReadBuffer(src: GLenum);
    pub fn glDrawRangeElements(
        mode: GLenum,
        start: GLuint,
        end: GLuint,
        count: GLsizei,
        type_: GLenum,
        indices: *const ::core::ffi::c_void,
    );
    pub fn glTexImage3D(
        target: GLenum,
        level: GLint,
        internalformat: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        border: GLint,
        format: GLenum,
        type_: GLenum,
        pixels: *const ::core::ffi::c_void,
    );
    pub fn glTexSubImage3D(
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
        pixels: *const ::core::ffi::c_void,
    );
    pub fn glCopyTexSubImage3D(
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
    );
    pub fn glCompressedTexImage3D(
        target: GLenum,
        level: GLint,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        border: GLint,
        imageSize: GLsizei,
        data: *const ::core::ffi::c_void,
    );
    pub fn glCompressedTexSubImage3D(
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        format: GLenum,
        imageSize: GLsizei,
        data: *const ::core::ffi::c_void,
    );
    pub fn glGenQueries(n: GLsizei, ids: *mut GLuint);
    pub fn glDeleteQueries(n: GLsizei, ids: *const GLuint);
    pub fn glIsQuery(id: GLuint) -> GLboolean;
    pub fn glBeginQuery(target: GLenum, id: GLuint);
    pub fn glEndQuery(target: GLenum);
    pub fn glGetQueryiv(target: GLenum, pname: GLenum, params: *mut GLint);
    pub fn glQueryCounter(id: GLenum, pname: GLenum);
    pub fn glGetQueryObjectiv(id: GLuint, pname: GLenum, params: *mut GLint);
    pub fn glGetQueryObjectui64v(id: GLuint, pname: GLenum, params: *mut GLuint64);
    pub fn glUnmapBuffer(target: GLenum) -> GLboolean;
    pub fn glGetBufferPointerv(
        target: GLenum,
        pname: GLenum,
        params: *mut *mut ::core::ffi::c_void,
    );
    pub fn glDrawBuffers(n: GLsizei, bufs: *const GLenum);
    pub fn glUniformMatrix2x3fv(
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    );
    pub fn glUniformMatrix3x2fv(
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    );
    pub fn glUniformMatrix2x4fv(
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    );
    pub fn glUniformMatrix4x2fv(
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    );
    pub fn glUniformMatrix3x4fv(
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    );
    pub fn glUniformMatrix4x3fv(
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    );
    pub fn glBlitFramebuffer(
        srcX0: GLint,
        srcY0: GLint,
        srcX1: GLint,
        srcY1: GLint,
        dstX0: GLint,
        dstY0: GLint,
        dstX1: GLint,
        dstY1: GLint,
        mask: GLbitfield,
        filter: GLenum,
    );
    pub fn glRenderbufferStorageMultisample(
        target: GLenum,
        samples: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
    );
    pub fn glFramebufferTextureLayer(
        target: GLenum,
        attachment: GLenum,
        texture: GLuint,
        level: GLint,
        layer: GLint,
    );
    pub fn glMapBufferRange(
        target: GLenum,
        offset: GLintptr,
        length: GLsizeiptr,
        access: GLbitfield,
    ) -> *mut ::core::ffi::c_void;
    pub fn glFlushMappedBufferRange(target: GLenum, offset: GLintptr, length: GLsizeiptr);
    pub fn glBindVertexArray(array: GLuint);
    pub fn glDeleteVertexArrays(n: GLsizei, arrays: *const GLuint);
    pub fn glGenVertexArrays(n: GLsizei, arrays: *mut GLuint);
    pub fn glIsVertexArray(array: GLuint) -> GLboolean;
    pub fn glGetIntegeri_v(target: GLenum, index: GLuint, data: *mut GLint);
    pub fn glBeginTransformFeedback(primitiveMode: GLenum);
    pub fn glEndTransformFeedback();
    pub fn glBindBufferRange(
        target: GLenum,
        index: GLuint,
        buffer: GLuint,
        offset: GLintptr,
        size: GLsizeiptr,
    );
    pub fn glBindBufferBase(target: GLenum, index: GLuint, buffer: GLuint);
    pub fn glTransformFeedbackVaryings(
        program: GLuint,
        count: GLsizei,
        varyings: *const *const GLchar,
        bufferMode: GLenum,
    );
    pub fn glGetTransformFeedbackVarying(
        program: GLuint,
        index: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        size: *mut GLsizei,
        type_: *mut GLenum,
        name: *mut GLchar,
    );
    pub fn glGetVertexAttribIiv(index: GLuint, pname: GLenum, params: *mut GLint);
    pub fn glGetVertexAttribIuiv(index: GLuint, pname: GLenum, params: *mut GLuint);
    pub fn glVertexAttribI4i(index: GLuint, x: GLint, y: GLint, z: GLint, w: GLint);
    pub fn glVertexAttribI4ui(index: GLuint, x: GLuint, y: GLuint, z: GLuint, w: GLuint);
    pub fn glVertexAttribI4iv(index: GLuint, v: *const GLint);
    pub fn glVertexAttribI4uiv(index: GLuint, v: *const GLuint);
    pub fn glGetUniformuiv(program: GLuint, location: GLint, params: *mut GLuint);
    pub fn glGetFragDataLocation(program: GLuint, name: *const GLchar) -> GLint;
    pub fn glUniform1ui(location: GLint, v0: GLuint);
    pub fn glUniform2ui(location: GLint, v0: GLuint, v1: GLuint);
    pub fn glUniform3ui(location: GLint, v0: GLuint, v1: GLuint, v2: GLuint);
    pub fn glUniform4ui(location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint);
    pub fn glUniform1uiv(location: GLint, count: GLsizei, value: *const GLuint);
    pub fn glUniform2uiv(location: GLint, count: GLsizei, value: *const GLuint);
    pub fn glUniform3uiv(location: GLint, count: GLsizei, value: *const GLuint);
    pub fn glUniform4uiv(location: GLint, count: GLsizei, value: *const GLuint);
    pub fn glClearBufferiv(buffer: GLenum, drawbuffer: GLint, value: *const GLint);
    pub fn glClearBufferuiv(buffer: GLenum, drawbuffer: GLint, value: *const GLuint);
    pub fn glClearBufferfv(buffer: GLenum, drawbuffer: GLint, value: *const GLfloat);
    pub fn glClearBufferfi(buffer: GLenum, drawbuffer: GLint, depth: GLfloat, stencil: GLint);
    pub fn glGetStringi(name: GLenum, index: GLuint) -> *const GLubyte;
    pub fn glCopyBufferSubData(
        readTarget: GLenum,
        writeTarget: GLenum,
        readOffset: GLintptr,
        writeOffset: GLintptr,
        size: GLsizeiptr,
    );
    pub fn glGetUniformIndices(
        program: GLuint,
        uniformCount: GLsizei,
        uniformNames: *const *const GLchar,
        uniformIndices: *mut GLuint,
    );
    pub fn glGetActiveUniformsiv(
        program: GLuint,
        uniformCount: GLsizei,
        uniformIndices: *const GLuint,
        pname: GLenum,
        params: *mut GLint,
    );
    pub fn glGetUniformBlockIndex(program: GLuint, uniformBlockName: *const GLchar) -> GLuint;
    pub fn glGetActiveUniformBlockiv(
        program: GLuint,
        uniformBlockIndex: GLuint,
        pname: GLenum,
        params: *mut GLint,
    );
    pub fn glGetActiveUniformBlockName(
        program: GLuint,
        uniformBlockIndex: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        uniformBlockName: *mut GLchar,
    );
    pub fn glUniformBlockBinding(
        program: GLuint,
        uniformBlockIndex: GLuint,
        uniformBlockBinding: GLuint,
    );
    pub fn glDrawArraysInstanced(
        mode: GLenum,
        first: GLint,
        count: GLsizei,
        instancecount: GLsizei,
    );
    pub fn glDrawElementsInstanced(
        mode: GLenum,
        count: GLsizei,
        type_: GLenum,
        indices: *const ::core::ffi::c_void,
        instancecount: GLsizei,
    );
    pub fn glFenceSync(condition: GLenum, flags: GLbitfield) -> GLsync;
    pub fn glIsSync(sync: GLsync) -> GLboolean;
    pub fn glDeleteSync(sync: GLsync);
    pub fn glClientWaitSync(sync: GLsync, flags: GLbitfield, timeout: GLuint64) -> GLenum;
    pub fn glWaitSync(sync: GLsync, flags: GLbitfield, timeout: GLuint64);
    pub fn glGetInteger64v(pname: GLenum, data: *mut GLint64);
    pub fn glGetSynciv(
        sync: GLsync,
        pname: GLenum,
        bufSize: GLsizei,
        length: *mut GLsizei,
        values: *mut GLint,
    );
    pub fn glGetInteger64i_v(target: GLenum, index: GLuint, data: *mut GLint64);
    pub fn glGetBufferParameteri64v(target: GLenum, pname: GLenum, params: *mut GLint64);
    pub fn glGenSamplers(count: GLsizei, samplers: *mut GLuint);
    pub fn glDeleteSamplers(count: GLsizei, samplers: *const GLuint);
    pub fn glIsSampler(sampler: GLuint) -> GLboolean;
    pub fn glBindSampler(unit: GLuint, sampler: GLuint);
    pub fn glSamplerParameteri(sampler: GLuint, pname: GLenum, param: GLint);
    pub fn glSamplerParameteriv(sampler: GLuint, pname: GLenum, param: *const GLint);
    pub fn glSamplerParameterf(sampler: GLuint, pname: GLenum, param: GLfloat);
    pub fn glSamplerParameterfv(sampler: GLuint, pname: GLenum, param: *const GLfloat);
    pub fn glGetSamplerParameteriv(sampler: GLuint, pname: GLenum, params: *mut GLint);
    pub fn glGetSamplerParameterfv(sampler: GLuint, pname: GLenum, params: *mut GLfloat);
    pub fn glVertexAttribDivisor(index: GLuint, divisor: GLuint);
    pub fn glBindTransformFeedback(target: GLenum, id: GLuint);
    pub fn glDeleteTransformFeedbacks(n: GLsizei, ids: *const GLuint);
    pub fn glGenTransformFeedbacks(n: GLsizei, ids: *mut GLuint);
    pub fn glIsTransformFeedback(id: GLuint) -> GLboolean;
    pub fn glPauseTransformFeedback();
    pub fn glResumeTransformFeedback();
    pub fn glGetProgramBinary(
        program: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        binaryFormat: *mut GLenum,
        binary: *mut ::core::ffi::c_void,
    );
    pub fn glProgramBinary(
        program: GLuint,
        binaryFormat: GLenum,
        binary: *const ::core::ffi::c_void,
        length: GLsizei,
    );
    pub fn glProgramParameteri(program: GLuint, pname: GLenum, value: GLint);
    pub fn glInvalidateFramebuffer(
        target: GLenum,
        numAttachments: GLsizei,
        attachments: *const GLenum,
    );
    pub fn glInvalidateSubFramebuffer(
        target: GLenum,
        numAttachments: GLsizei,
        attachments: *const GLenum,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
    );
    pub fn glTexStorage2D(
        target: GLenum,
        levels: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
    );
    pub fn glTexStorage3D(
        target: GLenum,
        levels: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
    );
    pub fn glGetInternalformativ(
        target: GLenum,
        internalformat: GLenum,
        pname: GLenum,
        bufSize: GLsizei,
        params: *mut GLint,
    );
}

pub unsafe fn is_gl2() -> bool {
    false
}
