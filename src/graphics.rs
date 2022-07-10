use std::{ffi::CString, mem};

mod texture;

use crate::{native::gl::*, Context};

use std::{error::Error, fmt::Display};

pub use texture::{FilterMode, Texture, TextureAccess, TextureFormat, TextureParams, TextureWrap};

fn get_uniform_location(program: GLuint, name: &str) -> Option<i32> {
    let cname = CString::new(name).unwrap_or_else(|e| panic!("{}", e));
    let location = unsafe { glGetUniformLocation(program, cname.as_ptr()) };

    if location == -1 {
        return None;
    }

    Some(location)
}

#[derive(Clone, Copy, Debug)]
pub enum UniformType {
    /// One 32-bit wide float (equivalent to `f32`)
    Float1,
    /// Two 32-bit wide floats (equivalent to `[f32; 2]`)
    Float2,
    /// Three 32-bit wide floats (equivalent to `[f32; 3]`)
    Float3,
    /// Four 32-bit wide floats (equivalent to `[f32; 4]`)
    Float4,
    /// One unsigned 32-bit integers (equivalent to `[u32; 1]`)
    Int1,
    /// Two unsigned 32-bit integers (equivalent to `[u32; 2]`)
    Int2,
    /// Three unsigned 32-bit integers (equivalent to `[u32; 3]`)
    Int3,
    /// Four unsigned 32-bit integers (equivalent to `[u32; 4]`)
    Int4,
    /// Four by four matrix of 32-bit floats
    Mat4,
}

impl UniformType {
    /// Byte size for a given UniformType
    pub fn size(&self) -> usize {
        match self {
            UniformType::Float1 => 4,
            UniformType::Float2 => 8,
            UniformType::Float3 => 12,
            UniformType::Float4 => 16,
            UniformType::Int1 => 4,
            UniformType::Int2 => 8,
            UniformType::Int3 => 12,
            UniformType::Int4 => 16,
            UniformType::Mat4 => 64,
        }
    }
}

#[derive(Clone)]
pub struct UniformDesc {
    name: String,
    uniform_type: UniformType,
    array_count: usize,
}

#[derive(Clone)]
pub struct UniformBlockLayout {
    pub uniforms: Vec<UniformDesc>,
}

impl UniformDesc {
    pub fn new(name: &str, uniform_type: UniformType) -> UniformDesc {
        UniformDesc {
            name: name.to_string(),
            uniform_type,
            array_count: 1,
        }
    }

    pub fn array(self, array_count: usize) -> UniformDesc {
        UniformDesc {
            array_count,
            ..self
        }
    }
}

#[derive(Clone)]
pub struct ShaderMeta {
    pub uniforms: UniformBlockLayout,
    pub images: Vec<String>,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum VertexFormat {
    /// One 32-bit wide float (equivalent to `f32`)
    Float1,
    /// Two 32-bit wide floats (equivalent to `[f32; 2]`)
    Float2,
    /// Three 32-bit wide floats (equivalent to `[f32; 3]`)
    Float3,
    /// Four 32-bit wide floats (equivalent to `[f32; 4]`)
    Float4,
    /// One unsigned 8-bit integer (equivalent to `u8`)
    Byte1,
    /// Two unsigned 8-bit integers (equivalent to `[u8; 2]`)
    Byte2,
    /// Three unsigned 8-bit integers (equivalent to `[u8; 3]`)
    Byte3,
    /// Four unsigned 8-bit integers (equivalent to `[u8; 4]`)
    Byte4,
    /// One unsigned 16-bit integer (equivalent to `u16`)
    Short1,
    /// Two unsigned 16-bit integers (equivalent to `[u16; 2]`)
    Short2,
    /// Tree unsigned 16-bit integers (equivalent to `[u16; 3]`)
    Short3,
    /// Four unsigned 16-bit integers (equivalent to `[u16; 4]`)
    Short4,
    /// One unsigned 32-bit integers (equivalent to `[u32; 1]`)
    Int1,
    /// Two unsigned 32-bit integers (equivalent to `[u32; 2]`)
    Int2,
    /// Three unsigned 32-bit integers (equivalent to `[u32; 3]`)
    Int3,
    /// Four unsigned 32-bit integers (equivalent to `[u32; 4]`)
    Int4,
    /// Four by four matrix of 32-bit floats
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

#[derive(Clone, Debug, Copy)]
pub enum ShaderType {
    Vertex,
    Fragment,
}

#[derive(Clone, Debug)]
pub enum ShaderError {
    CompilationError {
        shader_type: ShaderType,
        error_message: String,
    },
    LinkError(String),
    /// Shader strings should never contains \00 in the middle
    FFINulError(std::ffi::NulError),
}

impl From<std::ffi::NulError> for ShaderError {
    fn from(e: std::ffi::NulError) -> ShaderError {
        ShaderError::FFINulError(e)
    }
}

impl Display for ShaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self) // Display the same way as Debug
    }
}

impl Error for ShaderError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

#[derive(Clone, Debug, Copy)]
pub struct Shader(usize);

impl Shader {
    pub fn new(
        ctx: &mut Context,
        vertex_shader: &str,
        fragment_shader: &str,
        meta: ShaderMeta,
    ) -> Result<Shader, ShaderError> {
        let shader = load_shader_internal(vertex_shader, fragment_shader, meta)?;
        ctx.shaders.push(shader);
        Ok(Shader(ctx.shaders.len() - 1))
    }
}

type UniformLocation = Option<GLint>;

pub struct ShaderImage {
    gl_loc: UniformLocation,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct ShaderUniform {
    gl_loc: UniformLocation,
    _offset: usize,
    _size: usize,
    uniform_type: UniformType,
    array_count: i32,
}

struct ShaderInternal {
    program: GLuint,
    images: Vec<ShaderImage>,
    uniforms: Vec<ShaderUniform>,
}

/// Pixel arithmetic description for blending operations.
/// Will be used in an equation:
/// `equation(sfactor * source_color, dfactor * destination_color)`
/// Where source_color is the new pixel color and destination color is color from the destination buffer.
///
/// Example:
///```
///# use miniquad::{BlendState, BlendFactor, BlendValue, Equation};
///BlendState::new(
///    Equation::Add,
///    BlendFactor::Value(BlendValue::SourceAlpha),
///    BlendFactor::OneMinusValue(BlendValue::SourceAlpha)
///);
///```
/// This will be `source_color * source_color.a + destination_color * (1 - source_color.a)`
/// Wich is quite common set up for alpha blending.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct BlendState {
    equation: Equation,
    sfactor: BlendFactor,
    dfactor: BlendFactor,
}

impl BlendState {
    pub fn new(equation: Equation, sfactor: BlendFactor, dfactor: BlendFactor) -> BlendState {
        BlendState {
            equation,
            sfactor,
            dfactor,
        }
    }
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
    stored_index_type: Option<IndexType>,
    stored_vertex_buffer: GLuint,
    stored_texture: GLuint,
    index_buffer: GLuint,
    index_type: Option<IndexType>,
    vertex_buffer: GLuint,
    textures: [GLuint; MAX_SHADERSTAGE_IMAGES],
    cur_pipeline: Option<Pipeline>,
    color_blend: Option<BlendState>,
    alpha_blend: Option<BlendState>,
    stencil: Option<StencilState>,
    color_write: ColorMask,
    cull_face: CullFace,
    attributes: [Option<CachedAttribute>; MAX_VERTEX_ATTRIBUTES],
}

impl GlCache {
    fn bind_buffer(&mut self, target: GLenum, buffer: GLuint, index_type: Option<IndexType>) {
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
            self.index_type = index_type;
        }
    }

    fn store_buffer_binding(&mut self, target: GLenum) {
        if target == GL_ARRAY_BUFFER {
            self.stored_vertex_buffer = self.vertex_buffer;
        } else {
            self.stored_index_buffer = self.index_buffer;
            self.stored_index_type = self.index_type;
        }
    }

    fn restore_buffer_binding(&mut self, target: GLenum) {
        if target == GL_ARRAY_BUFFER {
            if self.stored_vertex_buffer != 0 {
                self.bind_buffer(target, self.stored_vertex_buffer, None);
                self.stored_vertex_buffer = 0;
            }
        } else {
            if self.stored_index_buffer != 0 {
                self.bind_buffer(target, self.stored_index_buffer, self.stored_index_type);
                self.stored_index_buffer = 0;
            }
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
        self.bind_buffer(GL_ARRAY_BUFFER, 0, None);
        self.vertex_buffer = 0;

        self.bind_buffer(GL_ELEMENT_ARRAY_BUFFER, 0, None);
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
        ctx: &mut Context,
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
            glBindFramebuffer(GL_FRAMEBUFFER, ctx.default_framebuffer);
        }
        let pass = RenderPassInternal {
            gl_fb,
            texture: color_img,
            depth_texture: depth_img,
        };

        ctx.passes.push(pass);

        RenderPass(ctx.passes.len() - 1)
    }

    pub fn texture(&self, ctx: &mut Context) -> Texture {
        let render_pass = &mut ctx.passes[self.0];

        render_pass.texture
    }

    pub fn delete(&self, ctx: &mut Context) {
        let render_pass = &mut ctx.passes[self.0];

        unsafe { glDeleteFramebuffers(1, &mut render_pass.gl_fb as *mut _) }

        render_pass.texture.delete();
        if let Some(depth_texture) = render_pass.depth_texture {
            depth_texture.delete();
        }
    }
}

pub const MAX_VERTEX_ATTRIBUTES: usize = 16;
pub const MAX_SHADERSTAGE_IMAGES: usize = 12;

pub struct GraphicsContext {
    shaders: Vec<ShaderInternal>,
    pipelines: Vec<PipelineInternal>,
    passes: Vec<RenderPassInternal>,
    default_framebuffer: GLuint,
    cache: GlCache,

    pub(crate) display: Option<*mut dyn crate::NativeDisplay>,
}

impl GraphicsContext {
    pub fn new() -> GraphicsContext {
        unsafe {
            let mut default_framebuffer: GLuint = 0;
            glGetIntegerv(
                GL_FRAMEBUFFER_BINDING,
                &mut default_framebuffer as *mut _ as *mut _,
            );
            let mut vao = 0;

            glGenVertexArrays(1, &mut vao as *mut _);
            glBindVertexArray(vao);
            GraphicsContext {
                default_framebuffer,
                shaders: vec![],
                pipelines: vec![],
                passes: vec![],
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
                display: None,
            }
        }
    }
}

impl Context {
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

    pub fn set_cull_face(&mut self, cull_face: CullFace) {
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

    pub fn set_color_write(&mut self, color_write: ColorMask) {
        if self.cache.color_write == color_write {
            return;
        }
        let (r, g, b, a) = color_write;
        unsafe { glColorMask(r as _, g as _, b as _, a as _) }
        self.cache.color_write = color_write;
    }

    pub fn set_blend(&mut self, color_blend: Option<BlendState>, alpha_blend: Option<BlendState>) {
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
            } else if self.cache.stencil.is_some() {
                glDisable(GL_STENCIL_TEST);
            }
        }

        self.cache.stencil = stencil_test;
    }

    /// Set a new viewport rectangle.
    /// Should be applied after begin_pass.
    pub fn apply_viewport(&mut self, x: i32, y: i32, w: i32, h: i32) {
        unsafe {
            glViewport(x, y, w, h);
        }
    }

    /// Set a new scissor rectangle.
    /// Should be applied after begin_pass.
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
            if let Some(gl_loc) = shader_image.gl_loc {
                unsafe {
                    self.cache.bind_texture(n, bindings_image.texture);
                    glUniform1i(gl_loc, n as i32);
                }
            }
        }

        self.cache.bind_buffer(
            GL_ELEMENT_ARRAY_BUFFER,
            bindings.index_buffer.gl_buf,
            bindings.index_buffer.index_type,
        );

        let pip = &self.pipelines[self.cache.cur_pipeline.unwrap().0];

        for attr_index in 0..MAX_VERTEX_ATTRIBUTES {
            let cached_attr = &mut self.cache.attributes[attr_index];

            let pip_attribute = pip.layout.get(attr_index).copied();

            if let Some(Some(attribute)) = pip_attribute {
                let vb = bindings.vertex_buffers[attribute.buffer_index];

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
        self.apply_uniforms_from_bytes(uniforms as *const _ as *const u8, std::mem::size_of::<U>())
    }

    #[doc(hidden)]
    /// Apply uniforms data from array of bytes with very special layout.
    /// Hidden because `apply_uniforms` is the recommended and safer way to work with uniforms.
    pub fn apply_uniforms_from_bytes(&mut self, uniform_ptr: *const u8, size: usize) {
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
            None => {
                let (screen_width, screen_height) = self.screen_size();
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
            self.cache.bind_buffer(GL_ARRAY_BUFFER, 0, None);
            self.cache.bind_buffer(GL_ELEMENT_ARRAY_BUFFER, 0, None);
        }
    }

    pub fn commit_frame(&mut self) {
        self.cache.clear_buffer_bindings();
        self.cache.clear_texture_bindings();
    }

    /// Draw elements using currently applied bindings and pipeline.
    ///
    /// + `base_element` specifies starting offset in `index_buffer`.
    /// + `num_elements` specifies length of the slice of `index_buffer` to draw.
    /// + `num_instances` specifies how many instances should be rendered.
    pub fn draw(&self, base_element: i32, num_elements: i32, num_instances: i32) {
        assert!(
            self.cache.cur_pipeline.is_some(),
            "Drawing without any binded pipeline"
        );

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
                _offset: *offset,
                _size: uniform.uniform_type.size(),
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
pub enum PrimitiveType {
    Triangles,
    Lines,
}

impl From<PrimitiveType> for GLenum {
    fn from(primitive_type: PrimitiveType) -> Self {
        match primitive_type {
            PrimitiveType::Triangles => GL_TRIANGLES,
            PrimitiveType::Lines => GL_LINES,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum IndexType {
    Byte,
    Short,
    Int,
}

impl From<IndexType> for GLenum {
    fn from(index_type: IndexType) -> Self {
        match index_type {
            IndexType::Byte => GL_UNSIGNED_BYTE,
            IndexType::Short => GL_UNSIGNED_SHORT,
            IndexType::Int => GL_UNSIGNED_INT,
        }
    }
}

impl IndexType {
    pub fn for_type<T>() -> IndexType {
        match std::mem::size_of::<T>() {
            1 => IndexType::Byte,
            2 => IndexType::Short,
            4 => IndexType::Int,
            _ => panic!("Unsupported index buffer index type"),
        }
    }

    pub fn size(self) -> u8 {
        match self {
            IndexType::Byte => 1,
            IndexType::Short => 2,
            IndexType::Int => 4,
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
    /// Color (RGB) blend function. If None - blending will be disabled for this pipeline.
    /// Usual use case to get alpha-blending:
    ///```
    ///# use miniquad::{PipelineParams, BlendState, BlendValue, BlendFactor, Equation};
    ///PipelineParams {
    ///    color_blend: Some(BlendState::new(
    ///        Equation::Add,
    ///        BlendFactor::Value(BlendValue::SourceAlpha),
    ///        BlendFactor::OneMinusValue(BlendValue::SourceAlpha))
    ///    ),
    ///    ..Default::default()
    ///};
    ///```
    pub color_blend: Option<BlendState>,
    /// Alpha blend function. If None - alpha will be blended with same equation than RGB colors.
    /// One of possible separate alpha channel blend settings is to avoid blending with WebGl background.
    /// On webgl canvas's resulting alpha channel will be used to blend the whole canvas background.
    /// To avoid modifying only alpha channel, but keep usual transparency:
    ///```
    ///# use miniquad::{PipelineParams, BlendState, BlendValue, BlendFactor, Equation};
    ///PipelineParams {
    ///    color_blend: Some(BlendState::new(
    ///        Equation::Add,
    ///        BlendFactor::Value(BlendValue::SourceAlpha),
    ///        BlendFactor::OneMinusValue(BlendValue::SourceAlpha))
    ///    ),
    ///    alpha_blend: Some(BlendState::new(
    ///        Equation::Add,
    ///        BlendFactor::Zero,
    ///        BlendFactor::One)
    ///    ),
    ///    ..Default::default()
    ///};
    ///```
    /// The same results may be achieved with ColorMask(true, true, true, false)
    pub alpha_blend: Option<BlendState>,
    pub stencil_test: Option<StencilState>,
    pub color_write: ColorMask,
    pub primitive_type: PrimitiveType,
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
            alpha_blend: None,
            stencil_test: None,
            color_write: (true, true, true, true),
            primitive_type: PrimitiveType::Triangles,
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
                        size: format.size(),
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
                buffer_data.offset += format.byte_len() as i64
            }
        }

        let pipeline = PipelineInternal {
            layout: vertex_layout,
            shader,
            params,
        };

        ctx.pipelines.push(pipeline);
        Pipeline(ctx.pipelines.len() - 1)
    }

    pub fn set_blend(&self, ctx: &mut Context, color_blend: Option<BlendState>) {
        let mut pipeline = &mut ctx.pipelines[self.0];
        pipeline.params.color_blend = color_blend;
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
    layout: Vec<Option<VertexAttributeInternal>>,
    shader: Shader,
    params: PipelineParams,
}

/// Geometry bindings
#[derive(Clone, Debug)]
pub struct Bindings {
    /// Vertex buffers. Data contained in the buffer must match layout
    /// specified in the `Pipeline`.
    ///
    /// Most commonly vertex buffer will contain `(x,y,z,w)` coordinates of the
    /// vertex in 3d space, as well as `(u,v)` coordinates that map the vertex
    /// to some position in the corresponding `Texture`.
    pub vertex_buffers: Vec<Buffer>,
    /// Index buffer which instructs the GPU in which order to draw vertices
    /// from a vertex buffer, with each subsequent 3 indices forming a
    /// triangle.
    pub index_buffer: Buffer,
    /// Textures to be used with when drawing the geometry in the fragment
    /// shader.
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
    index_type: Option<IndexType>,
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
        let index_type = if buffer_type == BufferType::IndexBuffer {
            Some(IndexType::for_type::<T>())
        } else {
            None
        };

        let gl_target = gl_buffer_target(&buffer_type);
        let gl_usage = gl_usage(&Usage::Immutable);
        let size = mem::size_of_val(data);
        let mut gl_buf: u32 = 0;

        unsafe {
            glGenBuffers(1, &mut gl_buf as *mut _);
            ctx.cache.store_buffer_binding(gl_target);
            ctx.cache.bind_buffer(gl_target, gl_buf, index_type);
            glBufferData(gl_target, size as _, std::ptr::null() as *const _, gl_usage);
            glBufferSubData(gl_target, 0, size as _, data.as_ptr() as *const _);
            ctx.cache.restore_buffer_binding(gl_target);
        }

        Buffer {
            gl_buf,
            buffer_type,
            size,
            index_type,
        }
    }

    pub fn stream(ctx: &mut Context, buffer_type: BufferType, size: usize) -> Buffer {
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
            ctx.cache.store_buffer_binding(gl_target);
            ctx.cache.bind_buffer(gl_target, gl_buf, None);
            glBufferData(gl_target, size as _, std::ptr::null() as *const _, gl_usage);
            ctx.cache.restore_buffer_binding(gl_target);
        }

        Buffer {
            gl_buf,
            buffer_type,
            size,
            index_type,
        }
    }

    pub fn index_stream(ctx: &mut Context, index_type: IndexType, size: usize) -> Buffer {
        let gl_target = gl_buffer_target(&BufferType::IndexBuffer);
        let gl_usage = gl_usage(&Usage::Stream);
        let mut gl_buf: u32 = 0;

        unsafe {
            glGenBuffers(1, &mut gl_buf as *mut _);
            ctx.cache.store_buffer_binding(gl_target);
            ctx.cache.bind_buffer(gl_target, gl_buf, None);
            glBufferData(gl_target, size as _, std::ptr::null() as *const _, gl_usage);
            ctx.cache.restore_buffer_binding(gl_target);
        }

        Buffer {
            gl_buf,
            buffer_type: BufferType::IndexBuffer,
            size,
            index_type: Some(index_type),
        }
    }
    pub fn update<T>(&self, ctx: &mut Context, data: &[T]) {
        if self.buffer_type == BufferType::IndexBuffer {
            assert!(self.index_type.is_some());
            assert!(self.index_type.unwrap() == IndexType::for_type::<T>());
        };

        let size = mem::size_of_val(data);

        assert!(size <= self.size);

        let gl_target = gl_buffer_target(&self.buffer_type);
        ctx.cache.store_buffer_binding(gl_target);
        ctx.cache
            .bind_buffer(gl_target, self.gl_buf, self.index_type);
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

/// `ElapsedQuery` is used to measure duration of GPU operations.
///
/// Usual timing/profiling methods are difficult apply to GPU workloads as draw calls are submitted
/// asynchronously effectively hiding execution time of individual operations from the user.
/// `ElapsedQuery` allows to measure duration of individual rendering operations, as though the time
/// was measured on GPU rather than CPU side.
///
/// The query is created using [`ElapsedQuery::new()`] function.
/// ```
/// use miniquad::graphics::ElapsedQuery;
/// // initialization
/// let mut query = ElapsedQuery::new();
/// ```
/// Measurement is performed by calling [`ElapsedQuery::begin_query()`] and
/// [`ElapsedQuery::end_query()`]
///
/// ```
/// # use miniquad::graphics::ElapsedQuery;
/// # let mut query = ElapsedQuery::new();
///
/// query.begin_query();
/// // one or multiple calls to miniquad::Context::draw()
/// query.end_query();
/// ```
///
/// Retreival of measured duration is only possible at a later point in time. Often a frame or
/// couple frames later. Measurement latency can especially be high on WASM/WebGL target.
///
/// ```
/// // couple frames later:
/// # use miniquad::graphics::ElapsedQuery;
/// # let mut query = ElapsedQuery::new();
/// # query.begin_query();
/// # query.end_query();
/// if query.is_available() {
///   let duration_nanoseconds = query.get_result();
///   // use/display duration_nanoseconds
/// }
/// ```
///
/// And during finalization:
/// ```
/// // clean-up
/// # use miniquad::graphics::ElapsedQuery;
/// # let mut query = ElapsedQuery::new();
/// # query.begin_query();
/// # query.end_query();
/// # if query.is_available() {
/// #   let duration_nanoseconds = query.get_result();
/// #   // use/display duration_nanoseconds
/// # }
/// query.delete();
/// ```
///
/// It is only possible to measure single query at once.
///
/// On OpenGL/WebGL platforms implementation relies on [`EXT_disjoint_timer_query`] extension.
///
/// [`EXT_disjoint_timer_query`]: https://www.khronos.org/registry/OpenGL/extensions/EXT/EXT_disjoint_timer_query.txt
///
#[derive(Clone, Copy)]
pub struct ElapsedQuery {
    gl_query: GLuint,
}

impl ElapsedQuery {
    pub fn new() -> ElapsedQuery {
        ElapsedQuery { gl_query: 0 }
    }

    /// Submit a beginning of elapsed-time query.
    ///
    /// Only a single query can be measured at any moment in time.
    ///
    /// Use [`ElapsedQuery::end_query()`] to finish the query and
    /// [`ElapsedQuery::get_result()`] to read the result when rendering is complete.
    ///
    /// The query can be used again after retriving the result.
    ///
    /// Implemented as `glBeginQuery(GL_TIME_ELAPSED, ...)` on OpenGL/WebGL platforms.
    ///
    /// Use [`ElapsedQuery::is_supported()`] to check if functionality is available and the method can be called.
    pub fn begin_query(&mut self) {
        if self.gl_query == 0 {
            unsafe { glGenQueries(1, &mut self.gl_query) };
        }
        unsafe { glBeginQuery(GL_TIME_ELAPSED, self.gl_query) };
    }

    /// Submit an end of elapsed-time query that can be read later when rendering is complete.
    ///
    /// This function is usd in conjunction with [`ElapsedQuery::begin_query()`] and
    /// [`ElapsedQuery::get_result()`].
    ///
    /// Implemented as `glEndQuery(GL_TIME_ELAPSED)` on OpenGL/WebGL platforms.
    pub fn end_query(&mut self) {
        unsafe { glEndQuery(GL_TIME_ELAPSED) };
    }

    /// Retreieve measured duration in nanonseconds.
    ///
    /// Note that the result may be ready only couple frames later due to asynchronous nature of GPU
    /// command submission. Use [`ElapsedQuery::is_available()`] to check if the result is
    /// available for retrieval.
    ///
    /// Use [`ElapsedQuery::is_supported()`] to check if functionality is available and the method can be called.
    pub fn get_result(&self) -> u64 {
        let mut time: GLuint64 = 0;
        assert!(self.gl_query != 0);
        unsafe { glGetQueryObjectui64v(self.gl_query, GL_QUERY_RESULT, &mut time) };
        time
    }

    /// Reports whenever elapsed timer is supported and other methods can be invoked.
    pub fn is_supported() -> bool {
        unimplemented!();
        //unsafe { sapp_is_elapsed_timer_supported() }
    }

    /// Reports whenever result of submitted query is available for retrieval with
    /// [`ElapsedQuery::get_result()`].
    ///
    /// Note that the result may be ready only couple frames later due to asynchrnous nature of GPU
    /// command submission.
    ///
    /// Use [`ElapsedQuery::is_supported()`] to check if functionality is available and the method can be called.
    pub fn is_available(&self) -> bool {
        let mut available: GLint = 0;

        // begin_query was not called yet
        if self.gl_query == 0 {
            return false;
        }

        unsafe { glGetQueryObjectiv(self.gl_query, GL_QUERY_RESULT_AVAILABLE, &mut available) };
        available != 0
    }

    /// Delete query.
    ///
    /// Note that the query is not deleted automatically when dropped.
    ///
    /// Implemented as `glDeleteQueries(...)` on OpenGL/WebGL platforms.
    pub fn delete(&mut self) {
        unsafe { glDeleteQueries(1, &mut self.gl_query) }
        self.gl_query = 0;
    }
}
