use std::ffi::CString;

use crate::{window, ResourceManager};

mod cache;

use super::*;
use cache::*;

/// Raw OpenGL bindings
/// Highly unsafe, some of the functions could be missing due to incompatible GL version
/// or all of them might be missing alltogether if rendering context is not a GL one.
pub mod raw_gl {
    use super::*;

    #[doc(inline)]
    pub use crate::native::gl::*;

    pub fn texture_format_into_gl(format: TextureFormat) -> (GLenum, GLenum, GLenum) {
        format.into()
    }
}

#[derive(Clone, Copy, Debug)]
struct Buffer {
    gl_buf: GLuint,
    buffer_type: BufferType,
    size: usize,
    // Dimension of the indices for this buffer,
    // used only as a type argument for glDrawElements and can be
    // 1, 2 or 4
    index_type: Option<u32>,
}

#[derive(Debug)]
struct ShaderUniform {
    gl_loc: UniformLocation,
    uniform_type: UniformType,
    array_count: i32,
}

struct ShaderInternal {
    program: GLuint,
    images: Vec<ShaderImage>,
    uniforms: Vec<ShaderUniform>,
}

#[derive(Clone, Copy, Debug)]
enum TextureOrRenderbuffer {
    Texture(GLuint),
    Renderbuffer(GLuint),
}
impl TextureOrRenderbuffer {
    fn texture(&self) -> Option<GLuint> {
        match self {
            TextureOrRenderbuffer::Texture(id) => Some(*id),
            _ => None,
        }
    }
    fn renderbuffer(&self) -> Option<GLuint> {
        match self {
            TextureOrRenderbuffer::Renderbuffer(id) => Some(*id),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Texture {
    raw: TextureOrRenderbuffer,
    params: TextureParams,
}

impl TextureFormat {
    fn sized_internal_format(&self) -> GLenum {
        match self {
            TextureFormat::RGB8 => GL_RGB8,
            TextureFormat::RGBA8 => GL_RGBA8,
            TextureFormat::RGBA16F => GL_RGBA16F,
            TextureFormat::Depth => GL_DEPTH_COMPONENT16,
            TextureFormat::Depth32 => GL_DEPTH_COMPONENT32,
            #[cfg(target_arch = "wasm32")]
            TextureFormat::Alpha => GL_ALPHA,
            #[cfg(not(target_arch = "wasm32"))]
            TextureFormat::Alpha => GL_R8,
        }
    }
}

/// Converts from TextureFormat to (internal_format, format, pixel_type)
impl From<TextureFormat> for (GLenum, GLenum, GLenum) {
    fn from(format: TextureFormat) -> Self {
        match format {
            TextureFormat::RGB8 => (GL_RGB, GL_RGB, GL_UNSIGNED_BYTE),
            TextureFormat::RGBA8 => (GL_RGBA, GL_RGBA, GL_UNSIGNED_BYTE),
            TextureFormat::RGBA16F => (GL_RGBA16F, GL_RGBA, GL_FLOAT),
            TextureFormat::Depth => (GL_DEPTH_COMPONENT, GL_DEPTH_COMPONENT, GL_UNSIGNED_SHORT),
            TextureFormat::Depth32 => (GL_DEPTH_COMPONENT, GL_DEPTH_COMPONENT, GL_FLOAT),
            #[cfg(target_arch = "wasm32")]
            TextureFormat::Alpha => (GL_ALPHA, GL_ALPHA, GL_UNSIGNED_BYTE),
            #[cfg(not(target_arch = "wasm32"))]
            TextureFormat::Alpha => (GL_R8, GL_RED, GL_UNSIGNED_BYTE), // texture updates will swizzle Red -> Alpha to match WASM
        }
    }
}

impl From<TextureKind> for GLuint {
    fn from(kind: TextureKind) -> GLuint {
        match kind {
            TextureKind::Texture2D => GL_TEXTURE_2D,
            TextureKind::CubeMap => GL_TEXTURE_CUBE_MAP,
        }
    }
}
impl From<Equation> for GLenum {
    fn from(eq: Equation) -> Self {
        match eq {
            Equation::Add => GL_FUNC_ADD,
            Equation::Subtract => GL_FUNC_SUBTRACT,
            Equation::ReverseSubtract => GL_FUNC_REVERSE_SUBTRACT,
        }
    }
}

impl From<BlendFactor> for GLenum {
    fn from(factor: BlendFactor) -> GLenum {
        match factor {
            BlendFactor::Zero => GL_ZERO,
            BlendFactor::One => GL_ONE,
            BlendFactor::Value(BlendValue::SourceColor) => GL_SRC_COLOR,
            BlendFactor::Value(BlendValue::SourceAlpha) => GL_SRC_ALPHA,
            BlendFactor::Value(BlendValue::DestinationColor) => GL_DST_COLOR,
            BlendFactor::Value(BlendValue::DestinationAlpha) => GL_DST_ALPHA,
            BlendFactor::OneMinusValue(BlendValue::SourceColor) => GL_ONE_MINUS_SRC_COLOR,
            BlendFactor::OneMinusValue(BlendValue::SourceAlpha) => GL_ONE_MINUS_SRC_ALPHA,
            BlendFactor::OneMinusValue(BlendValue::DestinationColor) => GL_ONE_MINUS_DST_COLOR,
            BlendFactor::OneMinusValue(BlendValue::DestinationAlpha) => GL_ONE_MINUS_DST_ALPHA,
            BlendFactor::SourceAlphaSaturate => GL_SRC_ALPHA_SATURATE,
        }
    }
}

impl From<StencilOp> for GLenum {
    fn from(op: StencilOp) -> Self {
        match op {
            StencilOp::Keep => GL_KEEP,
            StencilOp::Zero => GL_ZERO,
            StencilOp::Replace => GL_REPLACE,
            StencilOp::IncrementClamp => GL_INCR,
            StencilOp::DecrementClamp => GL_DECR,
            StencilOp::Invert => GL_INVERT,
            StencilOp::IncrementWrap => GL_INCR_WRAP,
            StencilOp::DecrementWrap => GL_DECR_WRAP,
        }
    }
}

impl From<CompareFunc> for GLenum {
    fn from(cf: CompareFunc) -> Self {
        match cf {
            CompareFunc::Always => GL_ALWAYS,
            CompareFunc::Never => GL_NEVER,
            CompareFunc::Less => GL_LESS,
            CompareFunc::Equal => GL_EQUAL,
            CompareFunc::LessOrEqual => GL_LEQUAL,
            CompareFunc::Greater => GL_GREATER,
            CompareFunc::NotEqual => GL_NOTEQUAL,
            CompareFunc::GreaterOrEqual => GL_GEQUAL,
        }
    }
}

impl Texture {
    pub fn new(
        ctx: &mut GlContext,
        access: TextureAccess,
        source: TextureSource,
        params: TextureParams,
    ) -> Texture {
        if let TextureSource::Bytes(bytes_data) = source {
            assert_eq!(
                params.format.size(params.width, params.height) as usize,
                bytes_data.len()
            );
        }
        if access != TextureAccess::RenderTarget {
            assert!(
                params.sample_count <= 1,
                "Multisampling is only supported for render textures"
            );
        }
        let (internal_format, format, pixel_type) = params.format.into();

        if access == TextureAccess::RenderTarget && params.sample_count > 1 {
            let mut renderbuffer: u32 = 0;
            unsafe {
                glGenRenderbuffers(1, &mut renderbuffer as *mut _);
                glBindRenderbuffer(GL_RENDERBUFFER, renderbuffer as _);
                let internal_format = params.format.sized_internal_format();
                glRenderbufferStorageMultisample(
                    GL_RENDERBUFFER,
                    params.sample_count,
                    internal_format,
                    params.width as _,
                    params.height as _,
                );
            }
            return Texture {
                raw: TextureOrRenderbuffer::Renderbuffer(renderbuffer),
                params,
            };
        }

        ctx.cache.store_texture_binding(0);

        let mut texture: GLuint = 0;

        unsafe {
            glGenTextures(1, &mut texture as *mut _);
            ctx.cache.bind_texture(0, params.kind.into(), texture);
            glPixelStorei(GL_UNPACK_ALIGNMENT, 1); // miniquad always uses row alignment of 1

            if cfg!(not(target_arch = "wasm32")) {
                // if not WASM
                if params.format == TextureFormat::Alpha {
                    // if alpha miniquad texture, the value on non-WASM is stored in red channel
                    // swizzle red -> alpha
                    glTexParameteri(params.kind.into(), GL_TEXTURE_SWIZZLE_A, GL_RED as _);
                } else {
                    // keep alpha -> alpha
                    glTexParameteri(params.kind.into(), GL_TEXTURE_SWIZZLE_A, GL_ALPHA as _);
                }
            }

            match source {
                TextureSource::Empty => {
                    // not quite sure if glTexImage2D(null) is really a requirement
                    // but it was like this for quite a while and apparantly it works?
                    glTexImage2D(
                        GL_TEXTURE_2D,
                        0,
                        internal_format as i32,
                        params.width as i32,
                        params.height as i32,
                        0,
                        format,
                        pixel_type,
                        std::ptr::null() as _,
                    );
                }
                TextureSource::Bytes(source) => {
                    assert!(params.kind == TextureKind::Texture2D, "incompatible TextureKind and TextureSource. Cubemaps require TextureSource::Array of 6 textures.");
                    glTexImage2D(
                        GL_TEXTURE_2D,
                        0,
                        internal_format as i32,
                        params.width as i32,
                        params.height as i32,
                        0,
                        format,
                        pixel_type,
                        source.as_ptr() as *const _,
                    );
                }
                TextureSource::Array(array) => {
                    if params.kind == TextureKind::CubeMap {
                        assert!(
                            array.len() == 6,
                            "Cubemaps require TextureSource::Array of 6 textures."
                        );
                    }
                    for (cubemap_face, mipmaps) in array.iter().enumerate() {
                        if mipmaps.len() != 1 {
                            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_BASE_LEVEL, 0);
                            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAX_LEVEL, array.len() as _);
                        }
                        for (mipmap_level, bytes) in mipmaps.iter().enumerate() {
                            let target = match params.kind {
                                TextureKind::Texture2D => GL_TEXTURE_2D,
                                TextureKind::CubeMap => {
                                    GL_TEXTURE_CUBE_MAP_POSITIVE_X + cubemap_face as u32
                                }
                            };
                            glTexImage2D(
                                target,
                                mipmap_level as _,
                                internal_format as i32,
                                params.width as i32,
                                params.height as i32,
                                0,
                                format,
                                pixel_type,
                                bytes.as_ptr() as *const _,
                            );
                        }
                    }
                }
            }

            let wrap = match params.wrap {
                TextureWrap::Repeat => GL_REPEAT,
                TextureWrap::Mirror => GL_MIRRORED_REPEAT,
                TextureWrap::Clamp => GL_CLAMP_TO_EDGE,
            };

            let min_filter = Self::gl_filter(params.min_filter, params.mipmap_filter);
            let mag_filter = match params.mag_filter {
                FilterMode::Nearest => GL_NEAREST,
                FilterMode::Linear => GL_LINEAR,
            };

            glTexParameteri(params.kind.into(), GL_TEXTURE_WRAP_S, wrap as i32);
            glTexParameteri(params.kind.into(), GL_TEXTURE_WRAP_T, wrap as i32);
            glTexParameteri(params.kind.into(), GL_TEXTURE_MIN_FILTER, min_filter as i32);
            glTexParameteri(params.kind.into(), GL_TEXTURE_MAG_FILTER, mag_filter as i32);
        }
        ctx.cache.restore_texture_binding(0);

        Texture {
            raw: TextureOrRenderbuffer::Texture(texture),
            params,
        }
    }

    pub fn resize(&mut self, ctx: &mut GlContext, width: u32, height: u32, source: Option<&[u8]>) {
        let raw = self
            .raw
            .texture()
            .expect("Resize not yet implemented for RenderBuffer(multisampled) textures");
        ctx.cache.store_texture_binding(0);
        ctx.cache.bind_texture(0, self.params.kind.into(), raw);

        let (internal_format, format, pixel_type) = self.params.format.into();

        self.params.width = width;
        self.params.height = height;

        unsafe {
            glTexImage2D(
                GL_TEXTURE_2D,
                0,
                internal_format as i32,
                self.params.width as i32,
                self.params.height as i32,
                0,
                format,
                pixel_type,
                match source {
                    Some(source) => source.as_ptr() as *const _,
                    Option::None => std::ptr::null(),
                },
            );
        }

        ctx.cache.restore_texture_binding(0);
    }

    pub fn update_texture_part(
        &self,
        ctx: &mut GlContext,
        x_offset: i32,
        y_offset: i32,
        width: i32,
        height: i32,
        source: &[u8],
    ) {
        assert_eq!(self.size(width as _, height as _), source.len());
        assert!(x_offset + width <= self.params.width as _);
        assert!(y_offset + height <= self.params.height as _);
        let raw = self.raw.texture().expect(
            "update_texture_part not yet implemented for RenderBuffer(multisampled) textures",
        );

        ctx.cache.store_texture_binding(0);
        ctx.cache.bind_texture(0, self.params.kind.into(), raw);

        let (_, format, pixel_type) = self.params.format.into();

        unsafe {
            glPixelStorei(GL_UNPACK_ALIGNMENT, 1); // miniquad always uses row alignment of 1

            if cfg!(not(target_arch = "wasm32")) {
                // if not WASM
                if self.params.format == TextureFormat::Alpha {
                    // if alpha miniquad texture, the value on non-WASM is stored in red channel
                    // swizzle red -> alpha
                    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_SWIZZLE_A, GL_RED as _);
                } else {
                    // keep alpha -> alpha
                    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_SWIZZLE_A, GL_ALPHA as _);
                }
            }

            glTexSubImage2D(
                GL_TEXTURE_2D,
                0,
                x_offset as _,
                y_offset as _,
                width as _,
                height as _,
                format,
                pixel_type,
                source.as_ptr() as *const _,
            );
        }

        ctx.cache.restore_texture_binding(0);
    }

    /// Read texture data into CPU memory
    pub fn read_pixels(&self, bytes: &mut [u8]) {
        let raw = self
            .raw
            .texture()
            .expect("read_pixels not yet implemented for RenderBuffer(multisampled) textures");

        let (_, format, pixel_type) = self.params.format.into();

        let mut fbo = 0;
        unsafe {
            let mut binded_fbo: i32 = 0;
            glGetIntegerv(gl::GL_DRAW_FRAMEBUFFER_BINDING, &mut binded_fbo);
            glGenFramebuffers(1, &mut fbo);
            glBindFramebuffer(gl::GL_FRAMEBUFFER, fbo);
            glFramebufferTexture2D(
                gl::GL_FRAMEBUFFER,
                gl::GL_COLOR_ATTACHMENT0,
                gl::GL_TEXTURE_2D,
                raw,
                0,
            );

            glReadPixels(
                0,
                0,
                self.params.width as _,
                self.params.height as _,
                format,
                pixel_type,
                bytes.as_mut_ptr() as _,
            );

            glBindFramebuffer(gl::GL_FRAMEBUFFER, binded_fbo as _);
            glDeleteFramebuffers(1, &fbo);
        }
    }

    #[inline]
    fn size(&self, width: u32, height: u32) -> usize {
        self.params.format.size(width, height) as usize
    }

    fn gl_filter(filter: FilterMode, mipmap_filter: MipmapFilterMode) -> GLenum {
        match filter {
            FilterMode::Nearest => match mipmap_filter {
                MipmapFilterMode::None => GL_NEAREST,
                MipmapFilterMode::Nearest => GL_NEAREST_MIPMAP_NEAREST,
                MipmapFilterMode::Linear => GL_NEAREST_MIPMAP_LINEAR,
            },
            FilterMode::Linear => match mipmap_filter {
                MipmapFilterMode::None => GL_LINEAR,
                MipmapFilterMode::Nearest => GL_LINEAR_MIPMAP_NEAREST,
                MipmapFilterMode::Linear => GL_LINEAR_MIPMAP_LINEAR,
            },
        }
    }
}

pub(crate) struct PipelineInternal {
    layout: Vec<Option<VertexAttributeInternal>>,
    shader: ShaderId,
    params: PipelineParams,
}

type UniformLocation = Option<GLint>;

pub struct ShaderImage {
    gl_loc: UniformLocation,
}

fn get_uniform_location(program: GLuint, name: &str) -> Option<i32> {
    let cname = CString::new(name).unwrap_or_else(|e| panic!("{}", e));
    let location = unsafe { glGetUniformLocation(program, cname.as_ptr()) };

    if location == -1 {
        return None;
    }

    Some(location)
}

pub(crate) struct RenderPassInternal {
    gl_fb: GLuint,
    color_textures: Vec<TextureId>,
    resolves: Option<Vec<(u32, TextureId)>>,
    depth_texture: Option<TextureId>,
}

struct Textures(Vec<Texture>);
impl Textures {
    fn get(&self, texture: TextureId) -> Texture {
        match texture.0 {
            TextureIdInner::Raw(RawId::OpenGl(texture)) => Texture {
                raw: TextureOrRenderbuffer::Texture(texture),
                params: Default::default(),
            },
            #[cfg(target_vendor = "apple")]
            TextureIdInner::Raw(RawId::Metal(..)) => panic!("Metal texture in OpenGL context!"),
            TextureIdInner::Managed(texture) => self.0[texture],
        }
    }
}
pub struct GlContext {
    shaders: ResourceManager<ShaderInternal>,
    pipelines: ResourceManager<PipelineInternal>,
    passes: ResourceManager<RenderPassInternal>,
    buffers: ResourceManager<Buffer>,
    textures: Textures,
    default_framebuffer: GLuint,
    pub(crate) cache: GlCache,
    pub(crate) info: ContextInfo,
}

impl Default for GlContext {
    fn default() -> Self {
        Self::new()
    }
}

impl GlContext {
    pub fn new() -> GlContext {
        unsafe {
            let mut default_framebuffer: GLuint = 0;
            glGetIntegerv(
                GL_FRAMEBUFFER_BINDING,
                &mut default_framebuffer as *mut _ as *mut _,
            );
            let mut vao = 0;

            glGenVertexArrays(1, &mut vao as *mut _);
            glBindVertexArray(vao);
            let info = gl_info();
            GlContext {
                default_framebuffer,
                shaders: ResourceManager::default(),
                pipelines: ResourceManager::default(),
                passes: ResourceManager::default(),
                buffers: ResourceManager::default(),
                textures: Textures(vec![]),
                info,
                cache: GlCache {
                    stored_index_buffer: 0,
                    stored_index_type: None,
                    stored_vertex_buffer: 0,
                    index_buffer: 0,
                    index_type: None,
                    vertex_buffer: 0,
                    cur_pipeline: None,
                    cur_pass: None,
                    color_blend: None,
                    alpha_blend: None,
                    stencil: None,
                    color_write: (true, true, true, true),
                    cull_face: CullFace::Nothing,
                    stored_texture: 0,
                    stored_target: 0,
                    textures: [CachedTexture {
                        target: 0,
                        texture: 0,
                    }; MAX_SHADERSTAGE_IMAGES],
                    attributes: [None; MAX_VERTEX_ATTRIBUTES],
                },
            }
        }
    }

    pub fn features(&self) -> &Features {
        &self.info.features
    }
}

fn load_shader_internal(
    vertex_shader: &str,
    fragment_shader: &str,
    meta: ShaderMeta,
) -> Result<ShaderInternal, ShaderError> {
    unsafe {
        let vertex_shader = load_shader(GL_VERTEX_SHADER, vertex_shader)?;
        let fragment_shader = load_shader(GL_FRAGMENT_SHADER, fragment_shader)?;

        let program = glCreateProgram();
        glAttachShader(program, vertex_shader);
        glAttachShader(program, fragment_shader);
        glLinkProgram(program);

        // delete no longer used shaders
        glDetachShader(program, vertex_shader);
        glDeleteShader(vertex_shader);
        glDeleteShader(fragment_shader);

        let mut link_status = 0;
        glGetProgramiv(program, GL_LINK_STATUS, &mut link_status as *mut _);
        if link_status == 0 {
            let mut max_length: i32 = 0;
            glGetProgramiv(program, GL_INFO_LOG_LENGTH, &mut max_length as *mut _);

            let mut error_message = vec![0u8; max_length as usize + 1];
            glGetProgramInfoLog(
                program,
                max_length,
                &mut max_length as *mut _,
                error_message.as_mut_ptr() as *mut _,
            );
            assert!(max_length >= 1);
            let error_message =
                std::string::String::from_utf8_lossy(&error_message[0..max_length as usize - 1]);
            return Err(ShaderError::LinkError(error_message.to_string()));
        }

        glUseProgram(program);

        #[rustfmt::skip]
        let images = meta.images.iter().map(|name| ShaderImage {
            gl_loc: get_uniform_location(program, name),
        }).collect();

        #[rustfmt::skip]
        let uniforms = meta.uniforms.uniforms.iter().scan(0, |offset, uniform| {
            let res = ShaderUniform {
                gl_loc: get_uniform_location(program, &uniform.name),
                uniform_type: uniform.uniform_type,
                array_count: uniform.array_count as _,
            };
            *offset += uniform.uniform_type.size() * uniform.array_count;
            Some(res)
        }).collect();

        Ok(ShaderInternal {
            program,
            images,
            uniforms,
        })
    }
}

pub fn load_shader(shader_type: GLenum, source: &str) -> Result<GLuint, ShaderError> {
    unsafe {
        let shader = glCreateShader(shader_type);
        assert!(shader != 0);

        let cstring = CString::new(source)?;
        let csource = [cstring];
        glShaderSource(shader, 1, csource.as_ptr() as *const _, std::ptr::null());
        glCompileShader(shader);

        let mut is_compiled = 0;
        glGetShaderiv(shader, GL_COMPILE_STATUS, &mut is_compiled as *mut _);
        if is_compiled == 0 {
            let mut max_length: i32 = 0;
            glGetShaderiv(shader, GL_INFO_LOG_LENGTH, &mut max_length as *mut _);

            let mut error_message = vec![0u8; max_length as usize + 1];
            glGetShaderInfoLog(
                shader,
                max_length,
                &mut max_length as *mut _,
                error_message.as_mut_ptr() as *mut _,
            );

            assert!(max_length >= 1);
            let mut error_message =
                std::string::String::from_utf8_lossy(&error_message[0..max_length as usize - 1])
                    .into_owned();

            // On Wasm + Chrome, for unknown reason, string with zero-terminator is returned. On Firefox there is no zero-terminators in JavaScript string.
            if error_message.ends_with('\0') {
                error_message.pop();
            }

            return Err(ShaderError::CompilationError {
                shader_type: match shader_type {
                    GL_VERTEX_SHADER => ShaderType::Vertex,
                    GL_FRAGMENT_SHADER => ShaderType::Fragment,
                    _ => unreachable!(),
                },
                error_message,
            });
        }

        Ok(shader)
    }
}

impl GlContext {
    fn set_blend(&mut self, color_blend: Option<BlendState>, alpha_blend: Option<BlendState>) {
        if color_blend.is_none() && alpha_blend.is_some() {
            panic!("AlphaBlend without ColorBlend");
        }
        if self.cache.color_blend == color_blend && self.cache.alpha_blend == alpha_blend {
            return;
        }

        unsafe {
            if let Some(color_blend) = color_blend {
                if self.cache.color_blend.is_none() {
                    glEnable(GL_BLEND);
                }

                let BlendState {
                    equation: eq_rgb,
                    sfactor: src_rgb,
                    dfactor: dst_rgb,
                } = color_blend;

                if let Some(BlendState {
                    equation: eq_alpha,
                    sfactor: src_alpha,
                    dfactor: dst_alpha,
                }) = alpha_blend
                {
                    glBlendFuncSeparate(
                        src_rgb.into(),
                        dst_rgb.into(),
                        src_alpha.into(),
                        dst_alpha.into(),
                    );
                    glBlendEquationSeparate(eq_rgb.into(), eq_alpha.into());
                } else {
                    glBlendFunc(src_rgb.into(), dst_rgb.into());
                    glBlendEquationSeparate(eq_rgb.into(), eq_rgb.into());
                }
            } else if self.cache.color_blend.is_some() {
                glDisable(GL_BLEND);
            }
        }

        self.cache.color_blend = color_blend;
        self.cache.alpha_blend = alpha_blend;
    }

    fn set_stencil(&mut self, stencil_test: Option<StencilState>) {
        if self.cache.stencil == stencil_test {
            return;
        }
        unsafe {
            if let Some(stencil) = stencil_test {
                if self.cache.stencil.is_none() {
                    glEnable(GL_STENCIL_TEST);
                }

                let front = &stencil.front;
                glStencilOpSeparate(
                    GL_FRONT,
                    front.fail_op.into(),
                    front.depth_fail_op.into(),
                    front.pass_op.into(),
                );
                glStencilFuncSeparate(
                    GL_FRONT,
                    front.test_func.into(),
                    front.test_ref,
                    front.test_mask,
                );
                glStencilMaskSeparate(GL_FRONT, front.write_mask);

                let back = &stencil.back;
                glStencilOpSeparate(
                    GL_BACK,
                    back.fail_op.into(),
                    back.depth_fail_op.into(),
                    back.pass_op.into(),
                );
                glStencilFuncSeparate(
                    GL_BACK,
                    back.test_func.into(),
                    back.test_ref,
                    back.test_mask,
                );
                glStencilMaskSeparate(GL_BACK, back.write_mask);
            } else if self.cache.stencil.is_some() {
                glDisable(GL_STENCIL_TEST);
            }
        }

        self.cache.stencil = stencil_test;
    }

    fn set_cull_face(&mut self, cull_face: CullFace) {
        if self.cache.cull_face == cull_face {
            return;
        }

        match cull_face {
            CullFace::Nothing => unsafe {
                glDisable(GL_CULL_FACE);
            },
            CullFace::Front => unsafe {
                glEnable(GL_CULL_FACE);
                glCullFace(GL_FRONT);
            },
            CullFace::Back => unsafe {
                glEnable(GL_CULL_FACE);
                glCullFace(GL_BACK);
            },
        }
        self.cache.cull_face = cull_face;
    }

    fn set_color_write(&mut self, color_write: ColorMask) {
        if self.cache.color_write == color_write {
            return;
        }
        let (r, g, b, a) = color_write;
        unsafe { glColorMask(r as _, g as _, b as _, a as _) }
        self.cache.color_write = color_write;
    }
}

#[allow(clippy::field_reassign_with_default)]
fn gl_info() -> ContextInfo {
    let version_string = unsafe { glGetString(super::gl::GL_VERSION) };
    let gl_version_string = unsafe { std::ffi::CStr::from_ptr(version_string as _) }
        .to_str()
        .unwrap()
        .to_string();
    //let gles2 = !gles3 && gl_version_string.contains("OpenGL ES");

    let gl2 = gl_version_string.is_empty()
        || gl_version_string.starts_with("2")
        || gl_version_string.starts_with("OpenGL ES 2");
    let webgl1 = gl_version_string == "WebGL 1.0";

    let features = Features {
        instancing: !gl2,
        resolve_attachments: !webgl1 && !gl2,
    };

    let mut glsl_support = GlslSupport::default();

    // this is not quite documented,
    // but somehow even GL2.1 usually have all the compatibility extensions to support glsl100
    // It was tested on really old windows machines, virtual machines etc. glsl100 always works!
    glsl_support.v100 = true;

    // on wasm miniquad always creates webgl1 context, with the only glsl available being version 100
    #[cfg(target_arch = "wasm32")]
    {
        // on web, miniquad always loads EXT_shader_texture_lod and OES_standard_derivatives
        glsl_support.v100_ext = true;

        let webgl2 = gl_version_string.contains("WebGL 2.0");
        if webgl2 {
            glsl_support.v300es = true;
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let gles3 = gl_version_string.contains("OpenGL ES 3");

        if gles3 {
            glsl_support.v300es = true;
        }
    }

    // there is no gl3.4, so 4+ and 3.3 covers all modern OpenGL
    if gl_version_string.starts_with("3.2") {
        glsl_support.v150 = true; // MacOS is defaulting to 3.2 and GLSL 150
    } else if gl_version_string.starts_with("4") || gl_version_string.starts_with("3.3") {
        glsl_support.v330 = true;
    // gl 3.0, 3.1, 3.2 maps to 1.30, 1.40, 1.50 glsl versions
    } else if gl_version_string.starts_with("3") {
        glsl_support.v130 = true;
    }

    ContextInfo {
        backend: Backend::OpenGl,
        gl_version_string,
        glsl_support,
        features,
    }
}

impl RenderingBackend for GlContext {
    fn info(&self) -> ContextInfo {
        self.info.clone()
    }

    fn new_shader(
        &mut self,
        shader: ShaderSource,
        meta: ShaderMeta,
    ) -> Result<ShaderId, ShaderError> {
        let (fragment, vertex) = match shader {
            ShaderSource::Glsl { fragment, vertex } => (fragment, vertex),
            _ => panic!("Metal source on OpenGl context"),
        };
        let shader = load_shader_internal(vertex, fragment, meta)?;
        Ok(ShaderId(self.shaders.add(shader)))
    }

    fn new_texture(
        &mut self,
        access: TextureAccess,
        source: TextureSource,
        params: TextureParams,
    ) -> TextureId {
        let texture = Texture::new(self, access, source, params);
        self.textures.0.push(texture);
        TextureId(TextureIdInner::Managed(self.textures.0.len() - 1))
    }

    fn delete_texture(&mut self, texture: TextureId) {
        //self.cache.clear_texture_bindings();

        let t = self.textures.get(texture);
        match &t.raw {
            TextureOrRenderbuffer::Texture(raw) => unsafe {
                glDeleteTextures(1, raw as *const _);
            },
            TextureOrRenderbuffer::Renderbuffer(raw) => unsafe {
                glDeleteRenderbuffers(1, raw as *const _);
            },
        }
    }

    fn delete_shader(&mut self, program: ShaderId) {
        unsafe { glDeleteProgram(self.shaders[program.0].program) };
        self.shaders.remove(program.0);
        self.cache.cur_pipeline = None;
    }

    fn delete_pipeline(&mut self, pipeline: Pipeline) {
        self.pipelines.remove(pipeline.0);
    }

    fn texture_set_wrap(&mut self, texture: TextureId, wrap_x: TextureWrap, wrap_y: TextureWrap) {
        let t = self.textures.get(texture);
        let raw = t
            .raw
            .texture()
            .expect("texture_set_wrap not yet implemented for RenderBuffer(multisampled) textures");

        self.cache.store_texture_binding(0);
        self.cache.bind_texture(0, t.params.kind.into(), raw);
        let wrap_x = match wrap_x {
            TextureWrap::Repeat => GL_REPEAT,
            TextureWrap::Mirror => GL_MIRRORED_REPEAT,
            TextureWrap::Clamp => GL_CLAMP_TO_EDGE,
        };

        let wrap_y = match wrap_y {
            TextureWrap::Repeat => GL_REPEAT,
            TextureWrap::Mirror => GL_MIRRORED_REPEAT,
            TextureWrap::Clamp => GL_CLAMP_TO_EDGE,
        };

        unsafe {
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, wrap_x as i32);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, wrap_y as i32);
        }
        self.cache.restore_texture_binding(0);
    }

    fn texture_set_min_filter(
        &mut self,
        texture: TextureId,
        filter: FilterMode,
        mipmap_filter: MipmapFilterMode,
    ) {
        let t = self.textures.get(texture);
        let raw = t.raw.texture().expect(
            "texture_set_min_filter not yet implemented for RenderBuffer(multisampled) textures",
        );

        self.cache.store_texture_binding(0);
        self.cache.bind_texture(0, t.params.kind.into(), raw);

        let filter = Texture::gl_filter(filter, mipmap_filter);
        unsafe {
            glTexParameteri(t.params.kind.into(), GL_TEXTURE_MIN_FILTER, filter as i32);
        }
        self.cache.restore_texture_binding(0);
    }
    fn texture_set_mag_filter(&mut self, texture: TextureId, filter: FilterMode) {
        let t = self.textures.get(texture);
        let raw = t
            .raw
            .texture()
            .expect("texture_set_wrap not yet implemented for RenderBuffer(multisampled) textures");

        self.cache.store_texture_binding(0);
        self.cache.bind_texture(0, t.params.kind.into(), raw);

        let filter = match filter {
            FilterMode::Nearest => GL_NEAREST,
            FilterMode::Linear => GL_LINEAR,
        };
        unsafe {
            glTexParameteri(t.params.kind.into(), GL_TEXTURE_MAG_FILTER, filter as i32);
        }
        self.cache.restore_texture_binding(0);
    }
    fn texture_resize(
        &mut self,
        texture: TextureId,
        width: u32,
        height: u32,
        source: Option<&[u8]>,
    ) {
        let mut t = self.textures.get(texture);
        t.resize(self, width, height, source);
        if let TextureIdInner::Managed(tex_id) = texture.0 {
            self.textures.0[tex_id].params = t.params;
        };
    }
    fn texture_read_pixels(&mut self, texture: TextureId, source: &mut [u8]) {
        let t = self.textures.get(texture);
        t.read_pixels(source);
    }
    fn texture_generate_mipmaps(&mut self, texture: TextureId) {
        let t = self.textures.get(texture);
        let raw = t.raw.texture().expect(
            "texture_generate_mipmaps not yet implemented for RenderBuffer(multisampled) textures",
        );

        self.cache.store_texture_binding(0);
        self.cache.bind_texture(0, t.params.kind.into(), raw);
        unsafe {
            glGenerateMipmap(t.params.kind.into());
        }
        self.cache.restore_texture_binding(0);
    }
    fn texture_update_part(
        &mut self,
        texture: TextureId,
        x_offset: i32,
        y_offset: i32,
        width: i32,
        height: i32,
        source: &[u8],
    ) {
        let t = self.textures.get(texture);
        t.update_texture_part(self, x_offset, y_offset, width, height, source);
    }
    fn texture_params(&self, texture: TextureId) -> TextureParams {
        let texture = self.textures.get(texture);
        texture.params
    }
    unsafe fn texture_raw_id(&self, texture: TextureId) -> RawId {
        let texture = self.textures.get(texture);
        let raw = texture
            .raw
            .texture()
            .expect("For multisampled texture raw_id is not supported");

        RawId::OpenGl(raw)
    }

    fn new_render_pass_mrt(
        &mut self,
        color_img: &[TextureId],
        resolve_img: Option<&[TextureId]>,
        depth_img: Option<TextureId>,
    ) -> RenderPass {
        if color_img.is_empty() && depth_img.is_none() {
            panic!("Render pass should have at least one non-none target");
        }
        let mut gl_fb = 0;

        let mut resolves = None;
        unsafe {
            glGenFramebuffers(1, &mut gl_fb as *mut _);
            glBindFramebuffer(GL_FRAMEBUFFER, gl_fb);
            for (i, color_img) in color_img.iter().enumerate() {
                let texture = self.textures.get(*color_img);
                if texture.params.sample_count > 1 {
                    let raw = texture.raw.renderbuffer().unwrap();
                    glFramebufferRenderbuffer(
                        GL_FRAMEBUFFER,
                        GL_COLOR_ATTACHMENT0 + i as u32,
                        GL_RENDERBUFFER,
                        raw,
                    );
                } else {
                    let raw = texture.raw.texture().unwrap();
                    glFramebufferTexture2D(
                        GL_FRAMEBUFFER,
                        GL_COLOR_ATTACHMENT0 + i as u32,
                        GL_TEXTURE_2D,
                        raw,
                        0,
                    );
                }
            }
            if let Some(depth_img) = depth_img {
                let texture = self.textures.get(depth_img);
                if texture.params.sample_count > 1 {
                    let raw = texture.raw.texture().unwrap();
                    glFramebufferRenderbuffer(
                        GL_FRAMEBUFFER,
                        GL_DEPTH_ATTACHMENT,
                        GL_RENDERBUFFER,
                        raw,
                    );
                } else {
                    let raw = texture.raw.texture().unwrap();
                    glFramebufferTexture2D(
                        GL_FRAMEBUFFER,
                        GL_DEPTH_ATTACHMENT,
                        GL_TEXTURE_2D,
                        raw,
                        0,
                    );
                }
            }
            let mut attachments = vec![];
            for i in 0..color_img.len() {
                attachments.push(GL_COLOR_ATTACHMENT0 + i as u32);
            }

            if color_img.len() > 1 {
                glDrawBuffers(color_img.len() as _, attachments.as_ptr() as _);
            }

            if let Some(resolve_img) = resolve_img {
                resolves = Some(vec![]);
                let resolves = resolves.as_mut().unwrap();
                for (i, resolve_img) in resolve_img.iter().enumerate() {
                    let mut resolve_fb = 0;
                    glGenFramebuffers(1, &mut resolve_fb as *mut _);
                    glBindFramebuffer(GL_FRAMEBUFFER, resolve_fb);
                    resolves.push((resolve_fb, *resolve_img));
                    let texture = self.textures.get(*resolve_img);
                    let raw = texture.raw.texture().unwrap();
                    glFramebufferTexture2D(
                        GL_FRAMEBUFFER,
                        GL_COLOR_ATTACHMENT0 + i as u32,
                        GL_TEXTURE_2D,
                        raw,
                        0,
                    );
                    let fb_status = glCheckFramebufferStatus(GL_FRAMEBUFFER);
                    assert!(fb_status != 0);
                    glDrawBuffers(1, attachments.as_ptr() as _);
                }
            }
            glBindFramebuffer(GL_FRAMEBUFFER, self.default_framebuffer);
        }
        let pass = RenderPassInternal {
            gl_fb,
            color_textures: color_img.to_vec(),
            resolves,
            depth_texture: depth_img,
        };

        RenderPass(self.passes.add(pass))
    }
    fn render_pass_color_attachments(&self, render_pass: RenderPass) -> &[TextureId] {
        &self.passes[render_pass.0].color_textures
    }
    fn delete_render_pass(&mut self, render_pass: RenderPass) {
        let pass_id = render_pass.0;

        let render_pass = self.passes.remove(pass_id);

        unsafe { glDeleteFramebuffers(1, &render_pass.gl_fb as *const _) }

        for color_texture in &render_pass.color_textures {
            self.delete_texture(*color_texture);
        }
        if let Some(depth_texture) = render_pass.depth_texture {
            self.delete_texture(depth_texture);
        }
    }

    fn new_pipeline(
        &mut self,
        buffer_layout: &[BufferLayout],
        attributes: &[VertexAttribute],
        shader: ShaderId,
        params: PipelineParams,
    ) -> Pipeline {
        #[derive(Clone, Copy, Default)]
        struct BufferCacheData {
            stride: i32,
            offset: i64,
        }

        let mut buffer_cache: Vec<BufferCacheData> =
            vec![BufferCacheData::default(); buffer_layout.len()];

        for VertexAttribute {
            format,
            buffer_index,
            ..
        } in attributes
        {
            let layout = buffer_layout.get(*buffer_index).unwrap_or_else(|| panic!());
            let cache = buffer_cache
                .get_mut(*buffer_index)
                .unwrap_or_else(|| panic!());

            if layout.stride == 0 {
                cache.stride += format.size_bytes();
            } else {
                cache.stride = layout.stride;
            }
            // WebGL 1 limitation
            assert!(cache.stride <= 255);
        }

        let program = self.shaders[shader.0].program;

        let attributes_len = attributes
            .iter()
            .map(|layout| match layout.format {
                VertexFormat::Mat4 => 4,
                _ => 1,
            })
            .sum();

        let mut vertex_layout: Vec<Option<VertexAttributeInternal>> = vec![None; attributes_len];

        for VertexAttribute {
            name,
            format,
            buffer_index,
            gl_pass_as_float,
        } in attributes
        {
            let buffer_data = &mut buffer_cache
                .get_mut(*buffer_index)
                .unwrap_or_else(|| panic!());
            let layout = buffer_layout.get(*buffer_index).unwrap_or_else(|| panic!());

            let cname = CString::new(*name).unwrap_or_else(|e| panic!("{}", e));
            let attr_loc = unsafe { glGetAttribLocation(program, cname.as_ptr() as *const _) };
            let attr_loc = if attr_loc == -1 { None } else { Some(attr_loc) };
            let divisor = if layout.step_func == VertexStep::PerVertex {
                0
            } else {
                layout.step_rate
            };

            let mut attributes_count: usize = 1;
            let mut format = *format;

            if format == VertexFormat::Mat4 {
                format = VertexFormat::Float4;
                attributes_count = 4;
            }
            for i in 0..attributes_count {
                if let Some(attr_loc) = attr_loc {
                    let attr_loc = attr_loc as GLuint + i as GLuint;

                    let attr = VertexAttributeInternal {
                        attr_loc,
                        size: format.components(),
                        type_: format.type_(),
                        offset: buffer_data.offset,
                        stride: buffer_data.stride,
                        buffer_index: *buffer_index,
                        divisor,
                        gl_pass_as_float: *gl_pass_as_float,
                    };

                    assert!(
                        attr_loc < vertex_layout.len() as u32,
                        "attribute: {} outside of allocated attributes array len: {}",
                        name,
                        vertex_layout.len()
                    );
                    vertex_layout[attr_loc as usize] = Some(attr);
                }
                buffer_data.offset += format.size_bytes() as i64
            }
        }

        let pipeline = PipelineInternal {
            layout: vertex_layout,
            shader,
            params,
        };

        Pipeline(self.pipelines.add(pipeline))
    }

    fn apply_pipeline(&mut self, pipeline: &Pipeline) {
        self.cache.cur_pipeline = Some(*pipeline);

        {
            let pipeline = &self.pipelines[pipeline.0];
            let shader = &self.shaders[pipeline.shader.0];
            unsafe {
                glUseProgram(shader.program);
            }

            unsafe {
                glEnable(GL_SCISSOR_TEST);
            }

            if pipeline.params.depth_write {
                unsafe {
                    glEnable(GL_DEPTH_TEST);
                    glDepthFunc(pipeline.params.depth_test.into())
                }
            } else {
                unsafe {
                    glDisable(GL_DEPTH_TEST);
                }
            }

            match pipeline.params.front_face_order {
                FrontFaceOrder::Clockwise => unsafe {
                    glFrontFace(GL_CW);
                },
                FrontFaceOrder::CounterClockwise => unsafe {
                    glFrontFace(GL_CCW);
                },
            }
        }

        self.set_cull_face(self.pipelines[pipeline.0].params.cull_face);
        self.set_blend(
            self.pipelines[pipeline.0].params.color_blend,
            self.pipelines[pipeline.0].params.alpha_blend,
        );

        self.set_stencil(self.pipelines[pipeline.0].params.stencil_test);
        self.set_color_write(self.pipelines[pipeline.0].params.color_write);
    }

    fn new_buffer(
        &mut self,
        type_: BufferType,
        usage: BufferUsage,
        data: BufferSource,
    ) -> BufferId {
        let gl_target = gl_buffer_target(&type_);
        let gl_usage = gl_usage(&usage);
        let (size, element_size) = match &data {
            BufferSource::Slice(data) => (data.size, data.element_size),
            BufferSource::Empty { size, element_size } => (*size, *element_size),
        };
        let index_type = match type_ {
            BufferType::IndexBuffer
                if element_size == 1 || element_size == 2 || element_size == 4 =>
            {
                Some(element_size as u32)
            }
            BufferType::IndexBuffer => panic!("unsupported index buffer dimension"),
            BufferType::VertexBuffer => None,
        };
        let mut gl_buf: u32 = 0;

        unsafe {
            glGenBuffers(1, &mut gl_buf as *mut _);
            self.cache.store_buffer_binding(gl_target);
            self.cache.bind_buffer(gl_target, gl_buf, index_type);

            glBufferData(gl_target, size as _, std::ptr::null() as *const _, gl_usage);
            if let BufferSource::Slice(data) = data {
                debug_assert!(data.is_slice);
                glBufferSubData(gl_target, 0, size as _, data.ptr as _);
            }
            self.cache.restore_buffer_binding(gl_target);
        }

        let buffer = Buffer {
            gl_buf,
            buffer_type: type_,
            size,
            index_type,
        };

        BufferId(self.buffers.add(buffer))
    }

    fn buffer_update(&mut self, buffer: BufferId, data: BufferSource) {
        let data = match data {
            BufferSource::Slice(data) => data,
            _ => panic!("buffer_update expects BufferSource::slice"),
        };
        debug_assert!(data.is_slice);
        let buffer = &self.buffers[buffer.0];

        if matches!(buffer.buffer_type, BufferType::IndexBuffer) {
            assert!(buffer.index_type.is_some());
            assert!(data.element_size as u32 == buffer.index_type.unwrap());
        };

        let size = data.size;

        assert!(size <= buffer.size);

        let gl_target = gl_buffer_target(&buffer.buffer_type);
        self.cache.store_buffer_binding(gl_target);
        self.cache
            .bind_buffer(gl_target, buffer.gl_buf, buffer.index_type);
        unsafe { glBufferSubData(gl_target, 0, size as _, data.ptr as _) };
        self.cache.restore_buffer_binding(gl_target);
    }

    /// Size of buffer in bytes
    fn buffer_size(&mut self, buffer: BufferId) -> usize {
        self.buffers[buffer.0].size
    }

    /// Delete GPU buffer, leaving handle unmodified.
    ///
    /// More high-level code on top of miniquad probably is going to call this in Drop implementation of some
    /// more RAII buffer object.
    ///
    /// There is no protection against using deleted textures later. However its not an UB in OpenGl and thats why
    /// this function is not marked as unsafe
    fn delete_buffer(&mut self, buffer: BufferId) {
        unsafe { glDeleteBuffers(1, &self.buffers[buffer.0].gl_buf as *const _) }
        self.cache.clear_buffer_bindings();
        self.cache.clear_vertex_attributes();
        self.buffers.remove(buffer.0);
    }

    /// Set a new viewport rectangle.
    /// Should be applied after begin_pass.
    fn apply_viewport(&mut self, x: i32, y: i32, w: i32, h: i32) {
        unsafe {
            glViewport(x, y, w, h);
        }
    }

    /// Set a new scissor rectangle.
    /// Should be applied after begin_pass.
    fn apply_scissor_rect(&mut self, x: i32, y: i32, w: i32, h: i32) {
        unsafe {
            glScissor(x, y, w, h);
        }
    }

    fn apply_bindings_from_slice(
        &mut self,
        vertex_buffers: &[BufferId],
        index_buffer: BufferId,
        textures: &[TextureId],
    ) {
        let pip = &self.pipelines[self.cache.cur_pipeline.unwrap().0];
        let shader = &self.shaders[pip.shader.0];

        for (n, shader_image) in shader.images.iter().enumerate() {
            let bindings_image = textures
                .get(n)
                .unwrap_or_else(|| panic!("Image count in bindings and shader did not match!"));
            if let Some(gl_loc) = shader_image.gl_loc {
                let texture = self.textures.get(*bindings_image);
                let raw = match texture.raw {
                    TextureOrRenderbuffer::Texture(id) => id,
                    TextureOrRenderbuffer::Renderbuffer(id) => id,
                };
                unsafe {
                    self.cache.bind_texture(n, texture.params.kind.into(), raw);
                    glUniform1i(gl_loc, n as i32);
                }
            }
        }

        self.cache.bind_buffer(
            GL_ELEMENT_ARRAY_BUFFER,
            self.buffers[index_buffer.0].gl_buf,
            self.buffers[index_buffer.0].index_type,
        );

        let pip = &self.pipelines[self.cache.cur_pipeline.unwrap().0];

        for attr_index in 0..MAX_VERTEX_ATTRIBUTES {
            let cached_attr = &mut self.cache.attributes[attr_index];

            let pip_attribute = pip.layout.get(attr_index).copied();

            if let Some(Some(attribute)) = pip_attribute {
                assert!(
                    attribute.buffer_index < vertex_buffers.len(),
                    "Attribute index outside of vertex_buffers length"
                );
                let vb = vertex_buffers[attribute.buffer_index];
                let vb = self.buffers[vb.0];

                if cached_attr.map_or(true, |cached_attr| {
                    attribute != cached_attr.attribute || cached_attr.gl_vbuf != vb.gl_buf
                }) {
                    self.cache
                        .bind_buffer(GL_ARRAY_BUFFER, vb.gl_buf, vb.index_type);

                    unsafe {
                        match attribute.type_ {
                            GL_INT | GL_UNSIGNED_INT | GL_SHORT | GL_UNSIGNED_SHORT
                            | GL_UNSIGNED_BYTE | GL_BYTE
                                if !attribute.gl_pass_as_float =>
                            {
                                glVertexAttribIPointer(
                                    attr_index as GLuint,
                                    attribute.size,
                                    attribute.type_,
                                    attribute.stride,
                                    attribute.offset as *mut _,
                                )
                            }
                            _ => glVertexAttribPointer(
                                attr_index as GLuint,
                                attribute.size,
                                attribute.type_,
                                GL_FALSE as u8,
                                attribute.stride,
                                attribute.offset as *mut _,
                            ),
                        }
                        if self.info.features.instancing {
                            glVertexAttribDivisor(attr_index as GLuint, attribute.divisor as u32);
                        }
                        glEnableVertexAttribArray(attr_index as GLuint);
                    };

                    let cached_attr = &mut self.cache.attributes[attr_index];
                    *cached_attr = Some(CachedAttribute {
                        attribute,
                        gl_vbuf: vb.gl_buf,
                    });
                }
            } else if cached_attr.is_some() {
                unsafe {
                    glDisableVertexAttribArray(attr_index as GLuint);
                }
                *cached_attr = None;
            }
        }
    }

    fn apply_uniforms_from_bytes(&mut self, uniform_ptr: *const u8, size: usize) {
        let pip = &self.pipelines[self.cache.cur_pipeline.unwrap().0];
        let shader = &self.shaders[pip.shader.0];

        let mut offset = 0;

        for uniform in shader.uniforms.iter() {
            use UniformType::*;

            assert!(
                offset as i32 <= size as i32 - uniform.uniform_type.size() as i32 / 4,
                "Uniforms struct does not match shader uniforms layout"
            );

            unsafe {
                let data = (uniform_ptr as *const f32).add(offset);
                let data_int = (uniform_ptr as *const i32).add(offset);

                if let Some(gl_loc) = uniform.gl_loc {
                    match uniform.uniform_type {
                        Float1 => {
                            glUniform1fv(gl_loc, uniform.array_count, data);
                        }
                        Float2 => {
                            glUniform2fv(gl_loc, uniform.array_count, data);
                        }
                        Float3 => {
                            glUniform3fv(gl_loc, uniform.array_count, data);
                        }
                        Float4 => {
                            glUniform4fv(gl_loc, uniform.array_count, data);
                        }
                        Int1 => {
                            glUniform1iv(gl_loc, uniform.array_count, data_int);
                        }
                        Int2 => {
                            glUniform2iv(gl_loc, uniform.array_count, data_int);
                        }
                        Int3 => {
                            glUniform3iv(gl_loc, uniform.array_count, data_int);
                        }
                        Int4 => {
                            glUniform4iv(gl_loc, uniform.array_count, data_int);
                        }
                        Mat4 => {
                            glUniformMatrix4fv(gl_loc, uniform.array_count, 0, data);
                        }
                    }
                }
            }
            offset += uniform.uniform_type.size() / 4 * uniform.array_count as usize;
        }
    }

    fn clear(
        &mut self,
        color: Option<(f32, f32, f32, f32)>,
        depth: Option<f32>,
        stencil: Option<i32>,
    ) {
        let mut bits = 0;
        if let Some((r, g, b, a)) = color {
            bits |= GL_COLOR_BUFFER_BIT;
            unsafe {
                glClearColor(r, g, b, a);
            }
        }

        if let Some(v) = depth {
            bits |= GL_DEPTH_BUFFER_BIT;
            unsafe {
                glClearDepthf(v);
            }
        }

        if let Some(v) = stencil {
            bits |= GL_STENCIL_BUFFER_BIT;
            unsafe {
                glClearStencil(v);
            }
        }

        if bits != 0 {
            unsafe {
                glClear(bits);
            }
        }
    }

    fn begin_default_pass(&mut self, action: PassAction) {
        self.begin_pass(None, action);
    }

    fn begin_pass(&mut self, pass: Option<RenderPass>, action: PassAction) {
        self.cache.cur_pass = pass;
        let (framebuffer, w, h) = match pass {
            None => {
                let (screen_width, screen_height) = window::screen_size();

                (
                    self.default_framebuffer,
                    screen_width as i32,
                    screen_height as i32,
                )
            }
            Some(pass) => {
                let pass = &self.passes[pass.0];
                // new_render_pass will panic with both color and depth components none
                // so unwrap is safe here
                let texture = pass
                    .color_textures
                    .first()
                    .copied()
                    .or(pass.depth_texture)
                    .unwrap();
                (
                    pass.gl_fb,
                    self.textures.get(texture).params.width as i32,
                    self.textures.get(texture).params.height as i32,
                )
            }
        };
        unsafe {
            glBindFramebuffer(GL_FRAMEBUFFER, framebuffer);
            glViewport(0, 0, w, h);
            glScissor(0, 0, w, h);
        }
        match action {
            PassAction::Nothing => {}
            PassAction::Clear {
                color,
                depth,
                stencil,
            } => {
                self.clear(color, depth, stencil);
            }
        }
    }

    fn end_render_pass(&mut self) {
        unsafe {
            if let Some(pass) = self.cache.cur_pass.take() {
                let pass = &self.passes[pass.0];
                if let Some(resolves) = &pass.resolves {
                    glBindFramebuffer(GL_READ_FRAMEBUFFER, pass.gl_fb);
                    for (i, (resolve_fb, resolve_img)) in resolves.iter().enumerate() {
                        let texture = self.textures.get(*resolve_img);
                        let w = texture.params.width;
                        let h = texture.params.height;
                        glBindFramebuffer(GL_DRAW_FRAMEBUFFER, *resolve_fb);
                        glReadBuffer(GL_COLOR_ATTACHMENT0 + i as u32);
                        glBlitFramebuffer(
                            0,
                            0,
                            w as _,
                            h as _,
                            0,
                            0,
                            w as _,
                            h as _,
                            GL_COLOR_BUFFER_BIT,
                            GL_NEAREST,
                        );
                    }
                }
            }
            glBindFramebuffer(GL_FRAMEBUFFER, self.default_framebuffer);
            self.cache.bind_buffer(GL_ARRAY_BUFFER, 0, None);
            self.cache.bind_buffer(GL_ELEMENT_ARRAY_BUFFER, 0, None);
        }
    }

    fn commit_frame(&mut self) {
        self.cache.clear_buffer_bindings();
        self.cache.clear_texture_bindings();
    }

    fn draw(&self, base_element: i32, num_elements: i32, num_instances: i32) {
        assert!(
            self.cache.cur_pipeline.is_some(),
            "Drawing without any binded pipeline"
        );

        if !self.info.features.instancing && num_instances != 1 {
            eprintln!("Instanced rendering is not supported by the GPU");
            eprintln!("Ignoring this draw call");
            return;
        }

        let pip = &self.pipelines[self.cache.cur_pipeline.unwrap().0];
        let primitive_type = pip.params.primitive_type.into();
        let index_type = self.cache.index_type.expect("Unset index buffer type");

        unsafe {
            glDrawElementsInstanced(
                primitive_type,
                num_elements,
                match index_type {
                    1 => GL_UNSIGNED_BYTE,
                    2 => GL_UNSIGNED_SHORT,
                    4 => GL_UNSIGNED_INT,
                    _ => panic!("Unsupported index buffer type!"),
                },
                (index_type as i32 * base_element) as *mut _,
                num_instances,
            );
        }
    }
}
