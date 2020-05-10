use std::{ffi::CString, mem};

mod texture;

use crate::sapp::*;

// workaround sapp::* also contains None on Android
use std::option::Option::None;

pub use texture::{FilterMode, Texture, TextureAccess, TextureFormat, TextureParams, TextureWrap};

fn get_uniform_location(program: GLuint, name: &str) -> i32 {
    let cname = CString::new(name).unwrap_or_else(|e| panic!(e));
    let location = unsafe { glGetUniformLocation(program, cname.as_ptr()) };

    assert!(
        location != -1,
        format!("Cant get \"{}\" uniform location", name)
    );

    location
}

#[derive(Clone, Copy, Debug)]
pub enum UniformType {
    Float1,
    Float2,
    Float3,
    Float4,
    Int1,
    Int2,
    Int3,
    Int4,
    Mat4,
}

impl UniformType {
    fn size(&self, count: usize) -> usize {
        match self {
            UniformType::Float1 => 4 * count,
            UniformType::Float2 => 8 * count,
            UniformType::Float3 => 12 * count,
            UniformType::Float4 => 16 * count,
            UniformType::Int1 => 4 * count,
            UniformType::Int2 => 8 * count,
            UniformType::Int3 => 12 * count,
            UniformType::Int4 => 16 * count,
            UniformType::Mat4 => 64 * count,
        }
    }
}

pub struct UniformDesc {
    name: &'static str,
    uniform_type: UniformType,
    array_count: usize,
}

pub struct UniformBlockLayout {
    pub uniforms: &'static [UniformDesc],
}

impl UniformDesc {
    pub const fn new(name: &'static str, uniform_type: UniformType) -> UniformDesc {
        UniformDesc {
            name,
            uniform_type,
            array_count: 1,
        }
    }
    pub const fn with_array(
        name: &'static str,
        uniform_type: UniformType,
        array_count: usize,
    ) -> UniformDesc {
        UniformDesc {
            name,
            uniform_type,
            array_count,
        }
    }
}

pub struct ShaderMeta {
    pub uniforms: UniformBlockLayout,
    pub images: &'static [&'static str],
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum VertexFormat {
    Float1,
    Float2,
    Float3,
    Float4,
    Byte1,
    Byte2,
    Byte3,
    Byte4,
    Short1,
    Short2,
    Short3,
    Short4,
    Int1,
    Int2,
    Int3,
    Int4,
    Mat4,
}

impl VertexFormat {
    pub fn size(&self) -> i32 {
        match self {
            VertexFormat::Float1 => 1,
            VertexFormat::Float2 => 2,
            VertexFormat::Float3 => 3,
            VertexFormat::Float4 => 4,
            VertexFormat::Byte1 => 1,
            VertexFormat::Byte2 => 2,
            VertexFormat::Byte3 => 3,
            VertexFormat::Byte4 => 4,
            VertexFormat::Short1 => 1,
            VertexFormat::Short2 => 2,
            VertexFormat::Short3 => 3,
            VertexFormat::Short4 => 4,
            VertexFormat::Int1 => 1,
            VertexFormat::Int2 => 2,
            VertexFormat::Int3 => 3,
            VertexFormat::Int4 => 4,
            VertexFormat::Mat4 => 16,
        }
    }

    pub fn byte_len(&self) -> i32 {
        match self {
            VertexFormat::Float1 => 1 * 4,
            VertexFormat::Float2 => 2 * 4,
            VertexFormat::Float3 => 3 * 4,
            VertexFormat::Float4 => 4 * 4,
            VertexFormat::Byte1 => 1,
            VertexFormat::Byte2 => 2,
            VertexFormat::Byte3 => 3,
            VertexFormat::Byte4 => 4,
            VertexFormat::Short1 => 1 * 2,
            VertexFormat::Short2 => 2 * 2,
            VertexFormat::Short3 => 3 * 2,
            VertexFormat::Short4 => 4 * 2,
            VertexFormat::Int1 => 1 * 4,
            VertexFormat::Int2 => 2 * 4,
            VertexFormat::Int3 => 3 * 4,
            VertexFormat::Int4 => 4 * 4,
            VertexFormat::Mat4 => 16 * 4,
        }
    }

    fn type_(&self) -> GLuint {
        match self {
            VertexFormat::Float1 => GL_FLOAT,
            VertexFormat::Float2 => GL_FLOAT,
            VertexFormat::Float3 => GL_FLOAT,
            VertexFormat::Float4 => GL_FLOAT,
            VertexFormat::Byte1 => GL_UNSIGNED_BYTE,
            VertexFormat::Byte2 => GL_UNSIGNED_BYTE,
            VertexFormat::Byte3 => GL_UNSIGNED_BYTE,
            VertexFormat::Byte4 => GL_UNSIGNED_BYTE,
            VertexFormat::Short1 => GL_UNSIGNED_SHORT,
            VertexFormat::Short2 => GL_UNSIGNED_SHORT,
            VertexFormat::Short3 => GL_UNSIGNED_SHORT,
            VertexFormat::Short4 => GL_UNSIGNED_SHORT,
            VertexFormat::Int1 => GL_UNSIGNED_INT,
            VertexFormat::Int2 => GL_UNSIGNED_INT,
            VertexFormat::Int3 => GL_UNSIGNED_INT,
            VertexFormat::Int4 => GL_UNSIGNED_INT,
            VertexFormat::Mat4 => GL_FLOAT,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum VertexStep {
    PerVertex,
    PerInstance,
}

impl Default for VertexStep {
    fn default() -> VertexStep {
        VertexStep::PerVertex
    }
}

#[derive(Clone, Debug)]
pub struct BufferLayout {
    pub stride: i32,
    pub step_func: VertexStep,
    pub step_rate: i32,
}

impl Default for BufferLayout {
    fn default() -> BufferLayout {
        BufferLayout {
            stride: 0,
            step_func: VertexStep::PerVertex,
            step_rate: 1,
        }
    }
}

#[derive(Clone, Debug)]
pub struct VertexAttribute {
    pub name: &'static str,
    pub format: VertexFormat,
    pub buffer_index: usize,
}

impl VertexAttribute {
    pub const fn new(name: &'static str, format: VertexFormat) -> VertexAttribute {
        Self::with_buffer(name, format, 0)
    }

    pub const fn with_buffer(
        name: &'static str,
        format: VertexFormat,
        buffer_index: usize,
    ) -> VertexAttribute {
        VertexAttribute {
            name,
            format,
            buffer_index,
        }
    }
}

#[derive(Clone, Debug)]
pub struct PipelineLayout {
    pub buffers: &'static [BufferLayout],
    pub attributes: &'static [VertexAttribute],
}

pub struct Shader(usize);

impl Shader {
    pub fn new(
        ctx: &mut Context,
        vertex_shader: &str,
        fragment_shader: &str,
        meta: ShaderMeta,
    ) -> Shader {
        let shader = load_shader_internal(vertex_shader, fragment_shader, meta);
        ctx.shaders.push(shader);
        Shader(ctx.shaders.len() - 1)
    }
}

pub struct ShaderImage {
    gl_loc: GLint,
}

#[derive(Debug)]
pub struct ShaderUniform {
    gl_loc: GLint,
    offset: usize,
    size: usize,
    uniform_type: UniformType,
    array_count: i32,
}

struct ShaderInternal {
    program: GLuint,
    images: Vec<ShaderImage>,
    uniforms: Vec<ShaderUniform>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct BlendState {
    pub eq_rgb: Equation,
    pub eq_alpha: Equation,
    pub src_rgb: BlendFactor,
    pub dst_rgb: BlendFactor,
    pub src_alpha: BlendFactor,
    pub dst_alpha: BlendFactor,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct StencilState {
    pub front: StencilFaceState,
    pub back: StencilFaceState,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct StencilFaceState {
    /// Operation to use when stencil test fails
    pub fail_op: StencilOp,

    /// Operation to use when stencil test passes, but depth test fails
    pub depth_fail_op: StencilOp,

    /// Operation to use when both stencil and depth test pass,
    /// or when stencil pass and no depth or depth disabled
    pub pass_op: StencilOp,

    /// Used for stencil testing with test_ref and test_mask: if (test_ref & test_mask) *test_func* (*stencil* && test_mask)
    /// Default is Always, which means "always pass"
    pub test_func: CompareFunc,

    /// Default value: 0
    pub test_ref: i32,

    /// Default value: all 1s
    pub test_mask: u32,

    /// Specifies a bit mask to enable or disable writing of individual bits in the stencil planes
    /// Default value: all 1s
    pub write_mask: u32,
}

/// Operations performed on current stencil value when comparison test passes or fails.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum StencilOp {
    /// Default value
    Keep,
    Zero,
    Replace,
    IncrementClamp,
    DecrementClamp,
    Invert,
    IncrementWrap,
    DecrementWrap,
}

/// Depth and stencil compare function
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CompareFunc {
    /// Default value
    Always,
    Never,
    Less,
    Equal,
    LessOrEqual,
    Greater,
    NotEqual,
    GreaterOrEqual,
}

type ColorMask = (bool, bool, bool, bool);

#[derive(Default, Copy, Clone)]
struct CachedAttribute {
    attribute: VertexAttributeInternal,
    gl_vbuf: GLuint,
}

struct GlCache {
    stored_index_buffer: GLuint,
    stored_vertex_buffer: GLuint,
    stored_texture: GLuint,
    index_buffer: GLuint,
    vertex_buffer: GLuint,
    textures: [GLuint; MAX_SHADERSTAGE_IMAGES],
    cur_pipeline: Option<Pipeline>,
    blend: Option<BlendState>,
    stencil: Option<StencilState>,
    color_write: ColorMask,
    attributes: [Option<CachedAttribute>; MAX_VERTEX_ATTRIBUTES],
}

impl GlCache {
    fn bind_buffer(&mut self, target: GLenum, buffer: GLuint) {
        if target == GL_ARRAY_BUFFER {
            if self.vertex_buffer != buffer {
                self.vertex_buffer = buffer;
                unsafe {
                    glBindBuffer(target, buffer);
                }
            }
        } else {
            if self.index_buffer != buffer {
                self.index_buffer = buffer;
                unsafe {
                    glBindBuffer(target, buffer);
                }
            }
        }
    }

    fn store_buffer_binding(&mut self, target: GLenum) {
        if target == GL_ARRAY_BUFFER {
            self.stored_vertex_buffer = self.vertex_buffer;
        } else {
            self.stored_index_buffer = self.index_buffer;
        }
    }

    fn restore_buffer_binding(&mut self, target: GLenum) {
        if target == GL_ARRAY_BUFFER {
            self.bind_buffer(target, self.stored_vertex_buffer);
        } else {
            self.bind_buffer(target, self.stored_index_buffer);
        }
    }

    fn bind_texture(&mut self, slot_index: usize, texture: GLuint) {
        unsafe {
            glActiveTexture(GL_TEXTURE0 + slot_index as GLuint);
            if self.textures[slot_index] != texture {
                glBindTexture(GL_TEXTURE_2D, texture);
                self.textures[slot_index] = texture;
            }
        }
    }

    fn store_texture_binding(&mut self, slot_index: usize) {
        self.stored_texture = self.textures[slot_index];
    }

    fn restore_texture_binding(&mut self, slot_index: usize) {
        self.bind_texture(slot_index, self.stored_texture);
    }

    fn clear_buffer_bindings(&mut self) {
        self.bind_buffer(GL_ARRAY_BUFFER, 0);
        self.vertex_buffer = 0;

        self.bind_buffer(GL_ELEMENT_ARRAY_BUFFER, 0);
        self.index_buffer = 0;
    }

    fn clear_texture_bindings(&mut self) {
        for ix in 0..MAX_SHADERSTAGE_IMAGES {
            if self.textures[ix] != 0 {
                self.bind_texture(ix, 0);
                self.textures[ix] = 0;
            }
        }
    }
}

pub enum PassAction {
    Nothing,
    Clear {
        color: Option<(f32, f32, f32, f32)>,
        depth: Option<f32>,
        stencil: Option<i32>,
    },
}

impl PassAction {
    pub fn clear_color(r: f32, g: f32, b: f32, a: f32) -> PassAction {
        PassAction::Clear {
            color: Some((r, g, b, a)),
            depth: Some(1.),
            stencil: None,
        }
    }
}

impl Default for PassAction {
    fn default() -> PassAction {
        PassAction::Clear {
            color: Some((0.0, 0.0, 0.0, 0.0)),
            depth: Some(1.),
            stencil: None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RenderPass(usize);

struct RenderPassInternal {
    gl_fb: GLuint,
    texture: Texture,
    depth_texture: Option<Texture>,
}

impl RenderPass {
    pub fn new(
        context: &mut Context,
        color_img: Texture,
        depth_img: impl Into<Option<Texture>>,
    ) -> RenderPass {
        let mut gl_fb = 0;

        let depth_img = depth_img.into();

        unsafe {
            glGenFramebuffers(1, &mut gl_fb as *mut _);
            glBindFramebuffer(GL_FRAMEBUFFER, gl_fb);
            glFramebufferTexture2D(
                GL_FRAMEBUFFER,
                GL_COLOR_ATTACHMENT0,
                GL_TEXTURE_2D,
                color_img.texture,
                0,
            );
            if let Some(depth_img) = depth_img {
                glFramebufferTexture2D(
                    GL_FRAMEBUFFER,
                    GL_DEPTH_ATTACHMENT,
                    GL_TEXTURE_2D,
                    depth_img.texture,
                    0,
                );
            }
            glBindFramebuffer(GL_FRAMEBUFFER, context.default_framebuffer);
        }
        let pass = RenderPassInternal {
            gl_fb,
            texture: color_img,
            depth_texture: depth_img
        };

        context.passes.push(pass);

        RenderPass(context.passes.len() - 1)
    }

    pub fn delete(&self, ctx: &mut Context) {
        let render_pass = &mut ctx.passes[self.0];

        unsafe {
            glDeleteFramebuffers(1, &mut render_pass.gl_fb as *mut _)
        }

        render_pass.texture.delete();
        if let Some(depth_texture) = render_pass.depth_texture {
            depth_texture.delete();
        }
    }
}

pub const MAX_VERTEX_ATTRIBUTES: usize = 16;
pub const MAX_SHADERSTAGE_IMAGES: usize = 12;

pub struct Context {
    shaders: Vec<ShaderInternal>,
    pipelines: Vec<PipelineInternal>,
    passes: Vec<RenderPassInternal>,
    default_framebuffer: GLuint,
    cache: GlCache,
}

impl Context {
    pub fn new() -> Context {
        unsafe {
            let mut default_framebuffer: GLuint = 0;
            glGetIntegerv(
                GL_FRAMEBUFFER_BINDING,
                &mut default_framebuffer as *mut _ as *mut _,
            );
            let mut vao = 0;

            glGenVertexArrays(1, &mut vao as *mut _);
            glBindVertexArray(vao);
            Context {
                default_framebuffer,
                shaders: vec![],
                pipelines: vec![],
                passes: vec![],
                cache: GlCache {
                    stored_index_buffer: 0,
                    stored_vertex_buffer: 0,
                    index_buffer: 0,
                    vertex_buffer: 0,
                    cur_pipeline: None,
                    blend: None,
                    stencil: None,
                    color_write: (true, true, true, true),
                    stored_texture: 0,
                    textures: [0; MAX_SHADERSTAGE_IMAGES],
                    attributes: [None; MAX_VERTEX_ATTRIBUTES],
                },
                //attributes: [None; 16],
            }
        }
    }

    /// The current framebuffer size in pixels
    /// NOTE: [High DPI Rendering](../conf/index.html#high-dpi-rendering)
    pub fn screen_size(&self) -> (f32, f32) {
        unsafe { (sapp_width() as f32, sapp_height() as f32) }
    }

    /// The dpi scaling factor (window pixels to framebuffer pixels)
    /// NOTE: [High DPI Rendering](../conf/index.html#high-dpi-rendering)
    pub fn dpi_scale(&self) -> f32 {
        unsafe { sapp_dpi_scale() }
    }

    /// True when high_dpi was requested and actually running in a high-dpi scenario
    /// NOTE: [High DPI Rendering](../conf/index.html#high-dpi-rendering)
    pub fn high_dpi(&self) -> bool {
        unsafe { sapp_high_dpi() }
    }

    pub fn apply_pipeline(&mut self, pipeline: &Pipeline) {
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

            match pipeline.params.cull_face {
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

            match pipeline.params.front_face_order {
                FrontFaceOrder::Clockwise => unsafe {
                    glFrontFace(GL_CW);
                },
                FrontFaceOrder::CounterClockwise => unsafe {
                    glFrontFace(GL_CCW);
                },
            }
        }

        if self.cache.blend != self.pipelines[pipeline.0].params.color_blend {
            self.set_blend(self.pipelines[pipeline.0].params.color_blend);
        }

        if self.cache.stencil != self.pipelines[pipeline.0].params.stencil_test {
            self.set_stencil(self.pipelines[pipeline.0].params.stencil_test);
        }

        let pipeline = &self.pipelines[pipeline.0];
        if self.cache.color_write != pipeline.params.color_write {
            let (r, g, b, a) = pipeline.params.color_write;
            unsafe { glColorMask(r as _, g as _, b as _, a as _) }
            self.cache.color_write = pipeline.params.color_write;
        }
    }

    pub fn set_blend(&mut self, color_blend: Option<BlendState>) {
        if self.cache.blend == color_blend {
            return;
        }
        unsafe {
            if let Some(blend) = color_blend {
                if self.cache.blend.is_none() {
                    glEnable(GL_BLEND);
                }

                glBlendFuncSeparate(
                    blend.src_rgb.into(),
                    blend.dst_rgb.into(),
                    blend.src_alpha.into(),
                    blend.dst_alpha.into(),
                );
                glBlendEquationSeparate(blend.eq_rgb.into(), blend.eq_alpha.into());
            } else if self.cache.blend.is_some() {
                glDisable(GL_BLEND);
            }
        }

        self.cache.blend = color_blend;
    }

    pub fn set_stencil(&mut self, stencil_test: Option<StencilState>) {
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
            } else if self.cache.blend.is_some() {
                glDisable(GL_STENCIL_TEST);
            }
        }

        self.cache.stencil = stencil_test;
    }

    pub fn apply_scissor_rect(&mut self, x: i32, y: i32, w: i32, h: i32) {
        unsafe {
            glScissor(x, y, w, h);
        }
    }

    pub fn apply_bindings(&mut self, bindings: &Bindings) {
        let pip = &self.pipelines[self.cache.cur_pipeline.unwrap().0];
        let shader = &self.shaders[pip.shader.0];

        for (n, shader_image) in shader.images.iter().enumerate() {
            let bindings_image = bindings
                .images
                .get(n)
                .unwrap_or_else(|| panic!("Image count in bindings and shader did not match!"));
            unsafe {
                self.cache.bind_texture(n, bindings_image.texture);
                glUniform1i(shader_image.gl_loc, n as i32);
            }
        }

        self.cache
            .bind_buffer(GL_ELEMENT_ARRAY_BUFFER, bindings.index_buffer.gl_buf);

        let pip = &self.pipelines[self.cache.cur_pipeline.unwrap().0];

        for attr_index in 0..MAX_VERTEX_ATTRIBUTES {
            let cached_attr = &mut self.cache.attributes[attr_index];

            let pip_attribute = pip.layout.get(attr_index).copied();

            if let Some(attribute) = pip_attribute {
                let vb = bindings.vertex_buffers[attribute.buffer_index];

                if cached_attr.map_or(true, |cached_attr| {
                    attribute != cached_attr.attribute || cached_attr.gl_vbuf != vb.gl_buf
                }) {
                    self.cache.bind_buffer(GL_ARRAY_BUFFER, vb.gl_buf);

                    unsafe {
                        glVertexAttribPointer(
                            attr_index as GLuint,
                            attribute.size,
                            attribute.type_,
                            GL_FALSE as u8,
                            attribute.stride,
                            attribute.offset as *mut _,
                        );
                        glVertexAttribDivisor(attr_index as GLuint, attribute.divisor as u32);
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

    pub fn apply_uniforms<U>(&mut self, uniforms: &U) {
        let pip = &self.pipelines[self.cache.cur_pipeline.unwrap().0];
        let shader = &self.shaders[pip.shader.0];

        let mut offset = 0;

        for (_, uniform) in shader.uniforms.iter().enumerate() {
            use UniformType::*;

            assert!(
                offset <= std::mem::size_of::<U>() - uniform.uniform_type.size(1) / 4,
                "Uniforms struct does not match shader uniforms layout"
            );

            unsafe {
                let data = (uniforms as *const _ as *const f32).offset(offset as isize);
                let data_int = (uniforms as *const _ as *const i32).offset(offset as isize);

                match uniform.uniform_type {
                    Float1 => {
                        glUniform1fv(uniform.gl_loc, uniform.array_count, data);
                    }
                    Float2 => {
                        glUniform2fv(uniform.gl_loc, uniform.array_count, data);
                    }
                    Float3 => {
                        glUniform3fv(uniform.gl_loc, uniform.array_count, data);
                    }
                    Float4 => {
                        glUniform4fv(uniform.gl_loc, uniform.array_count, data);
                    }
                    Int1 => {
                        glUniform1iv(uniform.gl_loc, uniform.array_count, data_int);
                    }
                    Int2 => {
                        glUniform2iv(uniform.gl_loc, uniform.array_count, data_int);
                    }
                    Int3 => {
                        glUniform3iv(uniform.gl_loc, uniform.array_count, data_int);
                    }
                    Int4 => {
                        glUniform4iv(uniform.gl_loc, uniform.array_count, data_int);
                    }
                    Mat4 => {
                        glUniformMatrix4fv(uniform.gl_loc, uniform.array_count, 0, data);
                    }
                }
            }
            offset += uniform.uniform_type.size(1) / 4 * uniform.array_count as usize;
        }
    }

    pub fn clear(
        &self,
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

    /// start rendering to the default frame buffer
    pub fn begin_default_pass(&mut self, action: PassAction) {
        self.begin_pass(None, action);
    }

    /// start rendering to an offscreen framebuffer
    pub fn begin_pass(&mut self, pass: impl Into<Option<RenderPass>>, action: PassAction) {
        let (framebuffer, w, h) = match pass.into() {
            None => (
                self.default_framebuffer,
                unsafe { sapp_width() } as i32,
                unsafe { sapp_height() } as i32,
            ),
            Some(pass) => {
                let pass = &self.passes[pass.0];
                (
                    pass.gl_fb,
                    pass.texture.width as i32,
                    pass.texture.height as i32,
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

    pub fn end_render_pass(&mut self) {
        unsafe {
            glBindFramebuffer(GL_FRAMEBUFFER, self.default_framebuffer);
            self.cache.bind_buffer(GL_ARRAY_BUFFER, 0);
            self.cache.bind_buffer(GL_ELEMENT_ARRAY_BUFFER, 0);
        }
    }

    pub fn commit_frame(&mut self) {
        self.cache.clear_buffer_bindings();
        self.cache.clear_texture_bindings();
    }

    pub fn draw(&self, base_element: i32, num_elements: i32, num_instances: i32) {
        unsafe {
            glDrawElementsInstanced(
                GL_TRIANGLES,
                num_elements,
                GL_UNSIGNED_SHORT,
                (2 * base_element) as *mut _,
                num_instances,
            );
        }
    }
}

fn load_shader_internal(
    vertex_shader: &str,
    fragment_shader: &str,
    meta: ShaderMeta,
) -> ShaderInternal {
    unsafe {
        let vertex_shader = load_shader(GL_VERTEX_SHADER, vertex_shader);
        let fragment_shader = load_shader(GL_FRAGMENT_SHADER, fragment_shader);

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
            panic!("can't link shader {}", error_message);
        }

        glUseProgram(program);

        #[rustfmt::skip]
        let images = meta.images.iter().map(|name| ShaderImage {
                gl_loc: get_uniform_location(program, name),
            }).collect();
        #[rustfmt::skip]
        let uniforms = meta.uniforms.uniforms.iter().scan(0, |offset, uniform| {
            let res = ShaderUniform {
                gl_loc: get_uniform_location(program, uniform.name),
                offset: *offset,
                size: uniform.uniform_type.size(1),
                uniform_type: uniform.uniform_type,
                array_count: uniform.array_count as _,
            };
            *offset += uniform.uniform_type.size(1) * uniform.array_count;
            Some(res)
        }).collect();
        ShaderInternal {
            program,
            images,
            uniforms,
        }
    }
}

pub fn load_shader(shader_type: GLenum, source: &str) -> GLuint {
    unsafe {
        let shader = glCreateShader(shader_type);

        assert!(shader != 0);

        let cstring = CString::new(source).unwrap_or_else(|e| panic!(e));
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
            let error_message =
                std::string::String::from_utf8_lossy(&error_message[0..max_length as usize - 1]);
            panic!("cant compile shader {}!", error_message);
        }

        shader
    }
}

/// Specify whether front- or back-facing polygons can be culled.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CullFace {
    Nothing,
    Front,
    Back,
}

/// Define front- and back-facing polygons.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FrontFaceOrder {
    Clockwise,
    CounterClockwise,
}

/// A pixel-wise comparison function.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Comparison {
    Never,
    Less,
    LessOrEqual,
    Greater,
    GreaterOrEqual,
    Equal,
    NotEqual,
    Always,
}

impl From<Comparison> for GLenum {
    fn from(cmp: Comparison) -> Self {
        match cmp {
            Comparison::Never => GL_NEVER,
            Comparison::Less => GL_LESS,
            Comparison::LessOrEqual => GL_LEQUAL,
            Comparison::Greater => GL_GREATER,
            Comparison::GreaterOrEqual => GL_GEQUAL,
            Comparison::Equal => GL_EQUAL,
            Comparison::NotEqual => GL_NOTEQUAL,
            Comparison::Always => GL_ALWAYS,
        }
    }
}

/// Specifies how incoming RGBA values (source) and the RGBA in framebuffer (destination)
/// are combined.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Equation {
    /// Adds source and destination. Source and destination are multiplied
    /// by blending parameters before addition.
    Add,
    /// Subtracts destination from source. Source and destination are
    /// multiplied by blending parameters before subtraction.
    Subtract,
    /// Subtracts source from destination. Source and destination are
    /// multiplied by blending parameters before subtraction.
    ReverseSubtract,
}

/// Blend values.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BlendValue {
    SourceColor,
    SourceAlpha,
    DestinationColor,
    DestinationAlpha,
}

/// Blend factors.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BlendFactor {
    Zero,
    One,
    Value(BlendValue),
    OneMinusValue(BlendValue),
    SourceAlphaSaturate,
}

impl Default for Equation {
    fn default() -> Equation {
        Equation::Add
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

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct PipelineParams {
    pub cull_face: CullFace,
    pub front_face_order: FrontFaceOrder,
    pub depth_test: Comparison,
    pub depth_write: bool,
    pub depth_write_offset: Option<(f32, f32)>,
    pub color_blend: Option<BlendState>,
    pub stencil_test: Option<StencilState>,
    pub color_write: ColorMask,
}

#[derive(Copy, Clone, Debug)]
pub struct Pipeline(usize);

impl Default for PipelineParams {
    fn default() -> PipelineParams {
        PipelineParams {
            cull_face: CullFace::Nothing,
            front_face_order: FrontFaceOrder::CounterClockwise,
            depth_test: Comparison::Always, // no depth test,
            depth_write: false,             // no depth write,
            depth_write_offset: None,
            color_blend: None,
            stencil_test: None,
            color_write: (true, true, true, true),
        }
    }
}

impl Pipeline {
    pub fn new(
        ctx: &mut Context,
        buffer_layout: &[BufferLayout],
        attributes: &[VertexAttribute],
        shader: Shader,
    ) -> Pipeline {
        Self::with_params(ctx, buffer_layout, attributes, shader, Default::default())
    }

    pub fn with_params(
        ctx: &mut Context,
        buffer_layout: &[BufferLayout],
        attributes: &[VertexAttribute],
        shader: Shader,
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
                cache.stride += format.byte_len();
            } else {
                cache.stride = layout.stride;
            }
            // WebGL 1 limitation
            assert!(cache.stride <= 255);
        }

        let program = ctx.shaders[shader.0].program;

        let attributes_len = attributes
            .iter()
            .map(|layout| match layout.format {
                VertexFormat::Mat4 => 4,
                _ => 1,
            })
            .sum();

        let mut vertex_layout: Vec<VertexAttributeInternal> =
            vec![VertexAttributeInternal::default(); attributes_len];

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

            let cname = CString::new(*name).unwrap_or_else(|e| panic!(e));
            let attr_loc = unsafe { glGetAttribLocation(program, cname.as_ptr() as *const _) };
            if attr_loc == -1 {
                panic!("failed to obtain location of attribute {}", name);
            }
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
                let attr_loc = attr_loc as GLuint + i as GLuint;

                let attr = VertexAttributeInternal {
                    attr_loc,
                    size: format.size(),
                    type_: format.type_(),
                    offset: buffer_data.offset,
                    stride: buffer_data.stride,
                    buffer_index: *buffer_index,
                    divisor,
                };
                //println!("{}: {:?}", name, attr);

                assert!(
                    attr_loc < vertex_layout.len() as u32,
                    format!(
                        "attribute: {} outside of allocated attributes array len: {}",
                        name,
                        vertex_layout.len()
                    )
                );
                vertex_layout[attr_loc as usize] = attr;

                buffer_data.offset += format.byte_len() as i64
            }
        }

        // TODO: it should be possible to express a "holes" in the attribute layout in the api
        // so empty attributes will be fine. But right now empty attribute is always a bug
        assert!(vertex_layout.iter().any(|attr| attr.size == 0) == false);

        let pipeline = PipelineInternal {
            layout: vertex_layout,
            shader,
            params,
        };

        ctx.pipelines.push(pipeline);
        Pipeline(ctx.pipelines.len() - 1)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
struct VertexAttributeInternal {
    attr_loc: GLuint,
    size: i32,
    type_: GLuint,
    offset: i64,
    stride: i32,
    buffer_index: usize,
    divisor: i32,
}

struct PipelineInternal {
    layout: Vec<VertexAttributeInternal>,
    shader: Shader,
    params: PipelineParams,
}

#[derive(Clone, Debug)]
pub struct Bindings {
    pub vertex_buffers: Vec<Buffer>,
    pub index_buffer: Buffer,
    pub images: Vec<Texture>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BufferType {
    VertexBuffer,
    IndexBuffer,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Usage {
    Immutable,
    Dynamic,
    Stream,
}

fn gl_buffer_target(buffer_type: &BufferType) -> GLenum {
    match buffer_type {
        BufferType::VertexBuffer => GL_ARRAY_BUFFER,
        BufferType::IndexBuffer => GL_ELEMENT_ARRAY_BUFFER,
    }
}

fn gl_usage(usage: &Usage) -> GLenum {
    match usage {
        Usage::Immutable => GL_STATIC_DRAW,
        Usage::Dynamic => GL_DYNAMIC_DRAW,
        Usage::Stream => GL_STREAM_DRAW,
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Buffer {
    gl_buf: GLuint,
    buffer_type: BufferType,
    size: usize,
}

impl Buffer {
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
    /// let buffer = Buffer::immutable(ctx, BufferType::VertexBuffer, &vertices);
    /// ```
    pub fn immutable<T>(ctx: &mut Context, buffer_type: BufferType, data: &[T]) -> Buffer {
        if buffer_type == BufferType::IndexBuffer {
            assert!(
                mem::size_of::<T>() == 2,
                "Only u16/i16 index buffers are implemented right now"
            );
        }

        //println!("{} {}", mem::size_of::<T>(), mem::size_of_val(data));
        let gl_target = gl_buffer_target(&buffer_type);
        let gl_usage = gl_usage(&Usage::Immutable);
        let size = mem::size_of_val(data) as i64;
        let mut gl_buf: u32 = 0;

        unsafe {
            glGenBuffers(1, &mut gl_buf as *mut _);
            ctx.cache.store_buffer_binding(gl_target);
            ctx.cache.bind_buffer(gl_target, gl_buf);
            glBufferData(gl_target, size as _, std::ptr::null() as *const _, gl_usage);
            glBufferSubData(gl_target, 0, size as _, data.as_ptr() as *const _);
            ctx.cache.restore_buffer_binding(gl_target);
        }

        Buffer {
            gl_buf,
            buffer_type,
            size: size as usize,
        }
    }

    pub fn stream(ctx: &mut Context, buffer_type: BufferType, size: usize) -> Buffer {
        let gl_target = gl_buffer_target(&buffer_type);
        let gl_usage = gl_usage(&Usage::Stream);
        let mut gl_buf: u32 = 0;

        unsafe {
            glGenBuffers(1, &mut gl_buf as *mut _);
            ctx.cache.store_buffer_binding(gl_target);
            ctx.cache.bind_buffer(gl_target, gl_buf);
            glBufferData(gl_target, size as _, std::ptr::null() as *const _, gl_usage);
            ctx.cache.restore_buffer_binding(gl_target);
        }

        Buffer {
            gl_buf,
            buffer_type,
            size,
        }
    }

    pub fn update<T>(&self, ctx: &mut Context, data: &[T]) {
        //println!("{} {}", mem::size_of::<T>(), mem::size_of_val(data));

        let size = mem::size_of_val(data);

        assert!(size <= self.size);

        let gl_target = gl_buffer_target(&self.buffer_type);

        ctx.cache.bind_buffer(gl_target, self.gl_buf);
        unsafe { glBufferSubData(gl_target, 0, size as _, data.as_ptr() as *const _) };
        ctx.cache.restore_buffer_binding(gl_target);
    }

    /// Size of buffer in bytes
    pub fn size(&self) -> usize {
        self.size
    }

    /// Delete GPU buffer, leaving handle unmodified.
    ///
    /// More high-level code on top of miniquad probably is going to call this in Drop implementation of some
    /// more RAII buffer object.
    ///
    /// There is no protection against using deleted textures later. However its not an UB in OpenGl and thats why
    /// this function is not marked as unsafe
    pub fn delete(&self) {
        unsafe { glDeleteBuffers(1, &self.gl_buf as *const _) }
    }
}
