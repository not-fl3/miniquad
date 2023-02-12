use std::ffi::CString;

mod cache;

use cache::*;

use crate::{native::gl::*, window};

use super::*;

#[derive(Clone, Copy, Debug)]
pub struct Buffer {
    gl_buf: GLuint,
    buffer_type: BufferType,
    size: usize,
    index_type: Option<IndexType>,
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

#[derive(Clone, Copy, Debug, PartialEq, Hash)]
struct Texture {
    texture: GLuint,
    width: u32,
    height: u32,
    format: TextureFormat,
}

/// Converts from TextureFormat to (internal_format, format, pixel_type)
impl From<TextureFormat> for (GLenum, GLenum, GLenum) {
    fn from(format: TextureFormat) -> Self {
        match format {
            TextureFormat::RGB8 => (GL_RGB, GL_RGB, GL_UNSIGNED_BYTE),
            TextureFormat::RGBA8 => (GL_RGBA, GL_RGBA, GL_UNSIGNED_BYTE),
            TextureFormat::Depth => (GL_DEPTH_COMPONENT, GL_DEPTH_COMPONENT, GL_UNSIGNED_SHORT),
            #[cfg(target_arch = "wasm32")]
            TextureFormat::Alpha => (GL_ALPHA, GL_ALPHA, GL_UNSIGNED_BYTE),
            #[cfg(not(target_arch = "wasm32"))]
            TextureFormat::Alpha => (GL_R8, GL_RED, GL_UNSIGNED_BYTE), // texture updates will swizzle Red -> Alpha to match WASM
        }
    }
}

impl Texture {
    pub fn new(
        ctx: &mut GlContext,
        _access: TextureAccess,
        bytes: Option<&[u8]>,
        params: TextureParams,
    ) -> Texture {
        if let Some(bytes_data) = bytes {
            assert_eq!(
                params.format.size(params.width, params.height) as usize,
                bytes_data.len()
            );
        }

        let (internal_format, format, pixel_type) = params.format.into();

        ctx.cache.store_texture_binding(0);

        let mut texture: GLuint = 0;

        unsafe {
            glGenTextures(1, &mut texture as *mut _);
            ctx.cache.bind_texture(0, texture);
            glPixelStorei(GL_UNPACK_ALIGNMENT, 1); // miniquad always uses row alignment of 1

            if cfg!(not(target_arch = "wasm32")) {
                // if not WASM
                if params.format == TextureFormat::Alpha {
                    // if alpha miniquad texture, the value on non-WASM is stored in red channel
                    // swizzle red -> alpha
                    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_SWIZZLE_A, GL_RED as _);
                } else {
                    // keep alpha -> alpha
                    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_SWIZZLE_A, GL_ALPHA as _);
                }
            }

            glTexImage2D(
                GL_TEXTURE_2D,
                0,
                internal_format as i32,
                params.width as i32,
                params.height as i32,
                0,
                format,
                pixel_type,
                match bytes {
                    Some(bytes) => bytes.as_ptr() as *const _,
                    Option::None => std::ptr::null(),
                },
            );

            // TODO: filter as i32 doesn't work anymore
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, params.wrap as i32);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, params.wrap as i32);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR as i32);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR as i32);
        }
        ctx.cache.restore_texture_binding(0);

        Texture {
            texture,
            width: params.width,
            height: params.height,
            format: params.format,
        }
    }

    /// Delete GPU texture, leaving handle unmodified.
    ///
    /// More high-level code on top of miniquad probably is going to call this in Drop implementation of some
    /// more RAII buffer object.
    ///
    /// There is no protection against using deleted textures later. However its not an UB in OpenGl and thats why
    /// this function is not marked as unsafe
    pub fn delete(&self) {
        unsafe {
            glDeleteTextures(1, &self.texture as *const _);
        }
    }

    pub fn set_filter(&self, ctx: &mut GlContext, filter: FilterMode) {
        ctx.cache.store_texture_binding(0);
        ctx.cache.bind_texture(0, self.texture);
        unsafe {
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, filter as i32);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, filter as i32);
        }
        ctx.cache.restore_texture_binding(0);
    }

    pub fn set_wrap(&self, ctx: &mut GlContext, wrap: TextureWrap) {
        ctx.cache.store_texture_binding(0);
        ctx.cache.bind_texture(0, self.texture);
        unsafe {
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, wrap as i32);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, wrap as i32);
        }
        ctx.cache.restore_texture_binding(0);
    }

    pub fn resize(&mut self, ctx: &mut GlContext, width: u32, height: u32, bytes: Option<&[u8]>) {
        ctx.cache.store_texture_binding(0);

        let (internal_format, format, pixel_type) = self.format.into();

        self.width = width;
        self.height = height;

        unsafe {
            glTexImage2D(
                GL_TEXTURE_2D,
                0,
                internal_format as i32,
                self.width as i32,
                self.height as i32,
                0,
                format,
                pixel_type,
                match bytes {
                    Some(bytes) => bytes.as_ptr() as *const _,
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
        bytes: &[u8],
    ) {
        assert_eq!(self.size(width as _, height as _), bytes.len());
        assert!(x_offset + width <= self.width as _);
        assert!(y_offset + height <= self.height as _);

        ctx.cache.store_texture_binding(0);
        ctx.cache.bind_texture(0, self.texture);

        let (_, format, pixel_type) = self.format.into();

        unsafe {
            glPixelStorei(GL_UNPACK_ALIGNMENT, 1); // miniquad always uses row alignment of 1

            if cfg!(not(target_arch = "wasm32")) {
                // if not WASM
                if self.format == TextureFormat::Alpha {
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
                bytes.as_ptr() as *const _,
            );
        }

        ctx.cache.restore_texture_binding(0);
    }

    /// Read texture data into CPU memory
    pub fn read_pixels(&self, bytes: &mut [u8]) {
        let (_, format, pixel_type) = self.format.into();

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
                self.texture,
                0,
            );

            glReadPixels(
                0,
                0,
                self.width as _,
                self.height as _,
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
        self.format.size(width, height) as usize
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
    texture: TextureId,
    depth_texture: Option<TextureId>,
}

pub struct GlContext {
    shaders: Vec<ShaderInternal>,
    pipelines: Vec<PipelineInternal>,
    passes: Vec<RenderPassInternal>,
    buffers: Vec<Buffer>,
    textures: Vec<Texture>,
    default_framebuffer: GLuint,
    pub(crate) cache: GlCache,

    pub(crate) features: Features,
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
            GlContext {
                default_framebuffer,
                shaders: vec![],
                pipelines: vec![],
                passes: vec![],
                buffers: vec![],
                textures: vec![],
                features: Features {
                    instancing: !crate::native::gl::is_gl2(),
                    ..Default::default()
                },
                cache: GlCache {
                    stored_index_buffer: 0,
                    stored_index_type: None,
                    stored_vertex_buffer: 0,
                    index_buffer: 0,
                    index_type: None,
                    vertex_buffer: 0,
                    cur_pipeline: None,
                    color_blend: None,
                    alpha_blend: None,
                    stencil: None,
                    color_write: (true, true, true, true),
                    cull_face: CullFace::Nothing,
                    stored_texture: 0,
                    textures: [0; MAX_SHADERSTAGE_IMAGES],
                    attributes: [None; MAX_VERTEX_ATTRIBUTES],
                },
            }
        }
    }

    pub fn features(&self) -> &Features {
        &self.features
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

impl RenderingBackend for GlContext {
    fn new_shader(
        &mut self,
        shader: ShaderSource,
        meta: ShaderMeta,
    ) -> Result<ShaderId, ShaderError> {
        let shader = load_shader_internal(
            shader.glsl_vertex.unwrap(),
            shader.glsl_fragment.unwrap(),
            meta,
        )?;
        self.shaders.push(shader);
        Ok(ShaderId(self.shaders.len() - 1))
    }

    fn new_texture(
        &mut self,
        access: TextureAccess,
        bytes: Option<&[u8]>,
        params: TextureParams,
    ) -> TextureId {
        let texture = Texture::new(self, access, bytes, params);
        self.textures.push(texture);
        TextureId(self.textures.len() - 1)
    }
    fn update_texture_part(
        &mut self,
        texture: TextureId,
        x_offset: i32,
        y_offset: i32,
        width: i32,
        height: i32,
        bytes: &[u8],
    ) {
        let t = self.textures[texture.0];
        t.update_texture_part(self, x_offset, y_offset, width, height, bytes);
    }
    fn texture_size(&self, texture: TextureId) -> (u32, u32) {
        let texture = self.textures[texture.0];
        (texture.width as _, texture.height as _)
    }
    fn new_render_pass(
        &mut self,
        color_img: TextureId,
        depth_img: Option<TextureId>,
    ) -> RenderPass {
        let mut gl_fb = 0;

        unsafe {
            glGenFramebuffers(1, &mut gl_fb as *mut _);
            glBindFramebuffer(GL_FRAMEBUFFER, gl_fb);
            glFramebufferTexture2D(
                GL_FRAMEBUFFER,
                GL_COLOR_ATTACHMENT0,
                GL_TEXTURE_2D,
                self.textures[color_img.0].texture,
                0,
            );
            if let Some(depth_img) = depth_img {
                glFramebufferTexture2D(
                    GL_FRAMEBUFFER,
                    GL_DEPTH_ATTACHMENT,
                    GL_TEXTURE_2D,
                    self.textures[depth_img.0].texture,
                    0,
                );
            }
            glBindFramebuffer(GL_FRAMEBUFFER, self.default_framebuffer);
        }
        let pass = RenderPassInternal {
            gl_fb,
            texture: color_img,
            depth_texture: depth_img,
        };

        self.passes.push(pass);

        RenderPass(self.passes.len() - 1)
    }
    fn render_pass_texture(&self, render_pass: RenderPass) -> TextureId {
        self.passes[render_pass.0].texture
    }
    fn delete_render_pass(&mut self, render_pass: RenderPass) {
        let render_pass = &mut self.passes[render_pass.0];

        unsafe { glDeleteFramebuffers(1, &mut render_pass.gl_fb as *mut _) }

        self.textures[render_pass.texture.0].delete();
        if let Some(depth_texture) = render_pass.depth_texture {
            self.textures[depth_texture.0].delete();
        }
    }

    fn new_pipeline(
        &mut self,
        buffer_layout: &[BufferLayout],
        attributes: &[VertexAttribute],
        shader: ShaderId,
    ) -> Pipeline {
        self.new_pipeline_with_params(buffer_layout, attributes, shader, Default::default())
    }

    fn new_pipeline_with_params(
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
            let mut cache = buffer_cache
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
        } in attributes
        {
            let mut buffer_data = &mut buffer_cache
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

        self.pipelines.push(pipeline);
        Pipeline(self.pipelines.len() - 1)
    }

    fn pipeline_set_blend(&mut self, pipeline: &Pipeline, color_blend: Option<BlendState>) {
        let mut pipeline = &mut self.pipelines[pipeline.0];
        pipeline.params.color_blend = color_blend;
    }

    fn apply_pipeline(&mut self, pipeline: &Pipeline) {
        self.cache.cur_pipeline = Some(*pipeline);

        {
            let pipeline = &self.pipelines[pipeline.0];
            let shader = &mut self.shaders[pipeline.shader.0];
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

    /// Create an immutable buffer resource object.
    /// ```ignore
    /// #[repr(C)]
    /// struct Vertex {
    ///     pos: Vec2,
    ///     uv: Vec2,
    /// }
    /// let vertices: [Vertex; 4] = [
    ///     Vertex { pos : Vec2 { x: -0.5, y: -0.5 }, uv: Vec2 { x: 0., y: 0. } },
    ///     Vertex { pos : Vec2 { x:  0.5, y: -0.5 }, uv: Vec2 { x: 1., y: 0. } },
    ///     Vertex { pos : Vec2 { x:  0.5, y:  0.5 }, uv: Vec2 { x: 1., y: 1. } },
    ///     Vertex { pos : Vec2 { x: -0.5, y:  0.5 }, uv: Vec2 { x: 0., y: 1. } },
    /// ];
    /// let buffer = ctx.new_buffer_immutable(BufferType::VertexBuffer, &vertices);
    /// ```
    fn new_buffer_immutable(&mut self, buffer_type: BufferType, data: BufferSource) -> BufferId {
        debug_assert!(data.0.is_slice);
        let index_type = if buffer_type == BufferType::IndexBuffer {
            Some(IndexType::for_type_size(data.0.element_size))
        } else {
            None
        };

        let gl_target = gl_buffer_target(&buffer_type);
        let gl_usage = gl_usage(&Usage::Immutable);
        let size = data.0.size;
        let mut gl_buf: u32 = 0;

        unsafe {
            glGenBuffers(1, &mut gl_buf as *mut _);
            self.cache.store_buffer_binding(gl_target);
            self.cache.bind_buffer(gl_target, gl_buf, index_type);
            glBufferData(gl_target, size as _, std::ptr::null() as *const _, gl_usage);
            glBufferSubData(gl_target, 0, size as _, data.0.ptr as _);
            self.cache.restore_buffer_binding(gl_target);
        }

        let buffer = Buffer {
            gl_buf,
            buffer_type,
            size,
            index_type,
        };
        self.buffers.push(buffer);
        BufferId(self.buffers.len() - 1)
    }

    fn new_buffer_stream(&mut self, buffer_type: BufferType, size: usize) -> BufferId {
        let index_type = if buffer_type == BufferType::IndexBuffer {
            Some(IndexType::Short)
        } else {
            None
        };

        let gl_target = gl_buffer_target(&buffer_type);
        let gl_usage = gl_usage(&Usage::Stream);
        let mut gl_buf: u32 = 0;

        unsafe {
            glGenBuffers(1, &mut gl_buf as *mut _);
            self.cache.store_buffer_binding(gl_target);
            self.cache.bind_buffer(gl_target, gl_buf, None);
            glBufferData(gl_target, size as _, std::ptr::null() as *const _, gl_usage);
            self.cache.restore_buffer_binding(gl_target);
        }

        let buffer = Buffer {
            gl_buf,
            buffer_type,
            size,
            index_type,
        };
        self.buffers.push(buffer);
        BufferId(self.buffers.len() - 1)
    }

    fn new_buffer_index_stream(&mut self, index_type: IndexType, size: usize) -> BufferId {
        let gl_target = gl_buffer_target(&BufferType::IndexBuffer);
        let gl_usage = gl_usage(&Usage::Stream);
        let mut gl_buf: u32 = 0;

        unsafe {
            glGenBuffers(1, &mut gl_buf as *mut _);
            self.cache.store_buffer_binding(gl_target);
            self.cache.bind_buffer(gl_target, gl_buf, None);
            glBufferData(gl_target, size as _, std::ptr::null() as *const _, gl_usage);
            self.cache.restore_buffer_binding(gl_target);
        }

        let buffer = Buffer {
            gl_buf,
            buffer_type: BufferType::IndexBuffer,
            size,
            index_type: Some(index_type),
        };
        self.buffers.push(buffer);
        BufferId(self.buffers.len() - 1)
    }

    fn buffer_update(&mut self, buffer: BufferId, data: BufferSource) {
        debug_assert!(data.0.is_slice);
        let buffer = &self.buffers[buffer.0];
        if buffer.buffer_type == BufferType::IndexBuffer {
            assert!(buffer.index_type.is_some());
            assert!(buffer.index_type.unwrap() == IndexType::for_type_size(data.0.element_size));
        };

        let size = data.0.size;

        assert!(size <= buffer.size);

        let gl_target = gl_buffer_target(&buffer.buffer_type);
        self.cache.store_buffer_binding(gl_target);
        self.cache
            .bind_buffer(gl_target, buffer.gl_buf, buffer.index_type);
        unsafe { glBufferSubData(gl_target, 0, size as _, data.0.ptr as _) };
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
    fn buffer_delete(&mut self, buffer: BufferId) {
        unsafe { glDeleteBuffers(1, &self.buffers[buffer.0].gl_buf as *const _) }
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
                    back.test_ref.into(),
                    back.test_mask,
                );
                glStencilMaskSeparate(GL_BACK, back.write_mask);
            } else if self.cache.stencil.is_some() {
                glDisable(GL_STENCIL_TEST);
            }
        }

        self.cache.stencil = stencil_test;
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

    fn apply_bindings(&mut self, bindings: &Bindings) {
        let pip = &self.pipelines[self.cache.cur_pipeline.unwrap().0];
        let shader = &self.shaders[pip.shader.0];

        for (n, shader_image) in shader.images.iter().enumerate() {
            let bindings_image = bindings
                .images
                .get(n)
                .unwrap_or_else(|| panic!("Image count in bindings and shader did not match!"));
            if let Some(gl_loc) = shader_image.gl_loc {
                unsafe {
                    self.cache
                        .bind_texture(n, self.textures[bindings_image.0].texture);
                    glUniform1i(gl_loc, n as i32);
                }
            }
        }

        self.cache.bind_buffer(
            GL_ELEMENT_ARRAY_BUFFER,
            self.buffers[bindings.index_buffer.0].gl_buf,
            self.buffers[bindings.index_buffer.0].index_type,
        );

        let pip = &self.pipelines[self.cache.cur_pipeline.unwrap().0];

        for attr_index in 0..MAX_VERTEX_ATTRIBUTES {
            let cached_attr = &mut self.cache.attributes[attr_index];

            let pip_attribute = pip.layout.get(attr_index).copied();

            if let Some(Some(attribute)) = pip_attribute {
                let vb = bindings.vertex_buffers[attribute.buffer_index];
                let vb = self.buffers[vb.0];

                if cached_attr.map_or(true, |cached_attr| {
                    attribute != cached_attr.attribute || cached_attr.gl_vbuf != vb.gl_buf
                }) {
                    self.cache
                        .bind_buffer(GL_ARRAY_BUFFER, vb.gl_buf, vb.index_type);

                    unsafe {
                        glVertexAttribPointer(
                            attr_index as GLuint,
                            attribute.size,
                            attribute.type_,
                            GL_FALSE as u8,
                            attribute.stride,
                            attribute.offset as *mut _,
                        );
                        if self.features.instancing {
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
            } else {
                if cached_attr.is_some() {
                    unsafe {
                        glDisableVertexAttribArray(attr_index as GLuint);
                    }
                    *cached_attr = None;
                }
            }
        }
    }

    fn apply_uniforms_from_bytes(&mut self, uniform_ptr: *const u8, size: usize) {
        let pip = &self.pipelines[self.cache.cur_pipeline.unwrap().0];
        let shader = &self.shaders[pip.shader.0];

        let mut offset = 0;

        for (_, uniform) in shader.uniforms.iter().enumerate() {
            use UniformType::*;

            assert!(
                offset <= size - uniform.uniform_type.size() / 4,
                "Uniforms struct does not match shader uniforms layout"
            );

            unsafe {
                let data = (uniform_ptr as *const f32).offset(offset as isize);
                let data_int = (uniform_ptr as *const i32).offset(offset as isize);

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

    fn clear(&self, color: Option<(f32, f32, f32, f32)>, depth: Option<f32>, stencil: Option<i32>) {
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
                (
                    pass.gl_fb,
                    self.textures[pass.texture.0].width as i32,
                    self.textures[pass.texture.0].height as i32,
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

        if !self.features.instancing && num_instances != 1 {
            println!("Instanced rendering is not supported by the GPU");
            println!("Ignoring this draw call");
            return;
        }

        let pip = &self.pipelines[self.cache.cur_pipeline.unwrap().0];
        let primitive_type = pip.params.primitive_type.into();
        let index_type = self.cache.index_type.expect("Unset index buffer type");

        unsafe {
            glDrawElementsInstanced(
                primitive_type,
                num_elements,
                index_type.into(),
                (index_type.size() as i32 * base_element) as *mut _,
                num_instances,
            );
        }
    }
}
