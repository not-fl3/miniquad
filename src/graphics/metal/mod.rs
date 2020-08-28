mod texture;

pub use texture::*;

use metal_rs as metal;

use crate::graphics::*;
use crate::GraphicContext;
use foreign_types::ForeignType;
use metal::{
    CommandBuffer, CommandQueue, Device, Drawable, Library, MTLBlendFactor, MTLClearColor,
    MTLCompareFunction, MTLCullMode, MTLIndexType, MTLLoadAction, MTLPixelFormat, MTLPrimitiveType,
    MTLResourceOptions, MTLResourceUsage, MTLSamplerState, MTLStencilOperation, MTLStoreAction,
    MTLTexture, MTLVertexFormat, MTLVertexStepFunction, RenderCommandEncoder, RenderPassDescriptor,
    RenderPipelineDescriptor, RenderPipelineState, VertexDescriptor,
};
use std::{mem, path::Path};

// https://developer.apple.com/metal/Metal-Feature-Set-Tables.pdf
const MAX_BUFFERS_PER_STAGE: usize = 14;
const MAX_UNIFORM_BUFFER_SIZE: usize = 4 * 1024 * 1024;
const MAX_VERTEX_ATTRIBUTES: usize = 31;
const UNIFORM_BUFFER_INDEX: u64 = 0;
const NUM_INFLIGHT_FRAMES: usize = 3;

#[cfg(any(target_os = "macos", all(target_os = "ios", target_arch = "x86_64")))]
const UNIFORM_BUFFER_ALIGN: usize = 256;
#[cfg(all(target_os = "ios", not(target_arch = "x86_64")))]
const UNIFORM_BUFFER_ALIGN: usize = 16;

#[cfg(target_pointer_width = "32")]
pub type NSUInteger = u32;

#[cfg(target_pointer_width = "64")]
pub type NSUInteger = u64;

#[repr(C)]
#[derive(Copy, Clone)]
struct NSRange {
    location: NSUInteger,
    length: NSUInteger,
}

impl NSRange {
    #[inline]
    fn new(location: NSUInteger, length: NSUInteger) -> NSRange {
        NSRange { location, length }
    }
}

pub struct Context {
    device: Device,
    command_queue: CommandQueue,
    render_buffer: Option<CommandBuffer>,
    render_encoder: Option<RenderCommandEncoder>,
    pipeline_state: Option<RenderPipelineState>,
    render_pass_desc: Option<RenderPassDescriptor>,
    pipelines: Vec<PipelineInternal>,
    shaders: Vec<ShaderInternal>,
    current_frame_index: usize,
    dispatch_semaphore: dispatch::Semaphore,
    cache: Cache,
}

struct Cache {
    vertex_buffers: Vec<Option<self::Buffer>>,
    index_buffer: Option<self::Buffer>,
    uniform_buffer: Vec<metal::Buffer>,
    current_ub_offset: usize,
    cur_pipeline: Option<Pipeline>,
}

impl Cache {
    pub fn clear(&mut self) {
        self.vertex_buffers.clear();
        self.index_buffer = None;
        self.current_ub_offset = 0;
        self.cur_pipeline = None;
    }
}

impl Default for Cache {
    fn default() -> Self {
        Cache {
            vertex_buffers: Vec::with_capacity(MAX_SHADERSTAGE_BUFFERS),
            index_buffer: None,
            uniform_buffer: Vec::with_capacity(NUM_INFLIGHT_FRAMES),
            current_ub_offset: 0,
            cur_pipeline: None,
        }
    }
}

impl Context {
    pub fn new() -> Context {
        let device = get_mtl_device();
        let command_queue = device.new_command_queue();
        let mut cache = Cache::default();

        for i in 0..NUM_INFLIGHT_FRAMES {
            let raw_buffer =
                device.new_buffer(MAX_UNIFORM_BUFFER_SIZE as u64, Usage::Stream.into());
            raw_buffer.set_label(format!("MiniQuadUniformBuffer[{}]", i).as_str());
            cache.uniform_buffer.push(raw_buffer);
        }

        Context {
            device,
            command_queue,
            render_buffer: None,
            render_encoder: None,
            pipeline_state: None,
            render_pass_desc: None,
            pipelines: vec![],
            shaders: vec![],
            current_frame_index: 1,
            dispatch_semaphore: dispatch::Semaphore::new(NUM_INFLIGHT_FRAMES as u32),
            cache,
        }
    }
}

impl GraphicContext for Context {
    fn apply_pipeline(&mut self, pipeline: &Pipeline) {
        assert!(!self.pipelines.is_empty(), "Create pipeline first");
        self.cache.cur_pipeline = Some(*pipeline);

        let pipeline = &self.pipelines[pipeline.0];

        if let Some(render_encoder) = &self.render_encoder {
            render_encoder.set_render_pipeline_state(self.pipeline_state.as_ref().unwrap());
            render_encoder.set_cull_mode(pipeline.params.cull_face.into());
        }
    }

    fn set_cull_face(&mut self, cull_face: CullFace) {
        if let Some(render_encoder) = &self.render_encoder {
            render_encoder.set_cull_mode(cull_face.into());
        }
    }

    fn set_color_write(&mut self, color_write: ColorMask) {
        todo!()
    }

    fn set_blend(&mut self, color_blend: Option<BlendState>, alpha_blend: Option<BlendState>) {
        todo!()
    }

    fn set_stencil(&mut self, stencil_test: Option<StencilState>) {
        todo!()
    }

    fn apply_scissor_rect(&mut self, x: i32, y: i32, w: i32, h: i32) {
        todo!()
    }

    fn apply_bindings(&mut self, bindings: &Bindings) {
        assert!(
            bindings.vertex_buffers.len() <= MAX_SHADERSTAGE_BUFFERS,
            "Only {} buffers are supported right now",
            MAX_SHADERSTAGE_BUFFERS
        );

        for vb_index in 0..bindings.vertex_buffers.len() {
            self.cache
                .vertex_buffers
                .push(Some(bindings.vertex_buffers[vb_index].clone()));
        }
        self.cache.index_buffer = Some(bindings.index_buffer.clone());

        if let Some(ref render_encoder) = self.render_encoder {
            for (index, vertex_buffer) in self.cache.vertex_buffers.iter().enumerate() {
                let vertex_buffer_raw = &vertex_buffer.as_ref().unwrap().raw;
                render_encoder.set_vertex_buffer((index + 1) as u64, Some(&vertex_buffer_raw), 0);
            }

            let img_count = bindings.images.len();
            let mut sampler_states = Vec::with_capacity(img_count);
            let mut textures = Vec::with_capacity(img_count);

            for i in 0..img_count {
                sampler_states.push(Some(bindings.images[i].sampler_state.as_ref()));
                textures.push(Some(bindings.images[i].texture.as_ref()));
            }

            render_encoder.set_fragment_sampler_states(0, sampler_states.as_slice());
            render_encoder.set_fragment_textures(0, textures.as_slice());
        }
    }

    fn apply_uniforms<U>(&mut self, uniforms: &U) {
        self.current_frame_index = (self.current_frame_index + 1) % NUM_INFLIGHT_FRAMES;

        let data = uniforms as *const _ as *const std::ffi::c_void;
        let data_lenght = mem::size_of_val(&[data]) as u64;

        assert!(data_lenght < MAX_UNIFORM_BUFFER_SIZE as u64);

        if !self.cache.uniform_buffer.is_empty() {
            assert!(self.cache.current_ub_offset < MAX_UNIFORM_BUFFER_SIZE);

            unsafe {
                let buffer = &self.cache.uniform_buffer[self.current_frame_index];
                let contents = buffer.contents();
                std::ptr::copy(
                    data,
                    contents.add(self.cache.current_ub_offset),
                    data_lenght as usize,
                );

                #[cfg(target_os = "macos")]
                buffer.did_modify_range(std::mem::transmute(NSRange::new(
                    self.cache.current_ub_offset as u64,
                    data_lenght,
                )));
            }

            if let Some(render_encoder) = &self.render_encoder {
                render_encoder.set_vertex_buffer(
                    UNIFORM_BUFFER_INDEX,
                    Some(self.cache.uniform_buffer[self.current_frame_index].as_ref()),
                    0,
                );
                render_encoder.set_vertex_buffer_offset(
                    UNIFORM_BUFFER_INDEX,
                    self.cache.current_ub_offset as u64,
                );
            }

            self.cache.current_ub_offset =
                roundup_ub_buffer(self.cache.current_ub_offset + (data_lenght as usize));
        }
    }

    fn clear(&self, color: Option<(f32, f32, f32, f32)>, depth: Option<f32>, stencil: Option<i32>) {
        if let Some(ref render_pass_desc) = self.render_pass_desc {
            if let Some((r, g, b, a)) = color {
                let color_attachment = render_pass_desc.color_attachments().object_at(0).unwrap();
                color_attachment.set_load_action(MTLLoadAction::Clear);
                color_attachment.set_store_action(MTLStoreAction::Store);
                color_attachment
                    .set_clear_color(MTLClearColor::new(r as f64, g as f64, b as f64, a as f64));
            }

            if let Some(v) = depth {
                let depth_attachment = render_pass_desc.depth_attachment().unwrap();
                depth_attachment.set_load_action(MTLLoadAction::Clear);
                depth_attachment.set_store_action(MTLStoreAction::Store);
                depth_attachment.set_clear_depth(v as f64);
            }

            if let Some(v) = stencil {
                let stencil_attachment = render_pass_desc.stencil_attachment().unwrap();
                stencil_attachment.set_load_action(MTLLoadAction::Clear);
                stencil_attachment.set_store_action(MTLStoreAction::Store);
                stencil_attachment.set_clear_stencil(v as u32);
            }
        }
    }

    fn begin_default_pass(&mut self, action: PassAction) {
        self.begin_pass(None, action);
    }

    fn begin_pass(&mut self, pass: impl Into<Option<RenderPass>>, action: PassAction) {
        /* block until the oldest frame in flight has finished */
        self.dispatch_semaphore.wait();
        self.cache.clear();

        if self.render_pass_desc.is_none() {
            self.render_pass_desc = Some(get_renderpass_descriptor());
        }

        if let Some(ref render_pass_desc) = self.render_pass_desc {
            let () = unsafe { msg_send![render_pass_desc.as_ref(), retain] };

            let render_buffer = self.command_queue.new_command_buffer().to_owned();
            render_buffer.set_label("MiniQuadCmdBuffer");

            let render_encoder = render_buffer
                .new_render_command_encoder(render_pass_desc)
                .to_owned();
            render_encoder.set_label("MiniQuadRenderEncoder");
            render_encoder.push_debug_group("MiniQuadRenderPass");

            self.render_buffer = Some(render_buffer);
            self.render_encoder = Some(render_encoder);
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
        if let Some(render_encoder) = &self.render_encoder {
            render_encoder.end_encoding();
            render_encoder.pop_debug_group();
        }

        self.render_encoder = None;
    }

    fn commit_frame(&mut self) {
        let sem = self.dispatch_semaphore.clone();

        if let Some(render_buffer) = &self.render_buffer {
            let current_drawable = get_drawable();
            let () = unsafe { msg_send![current_drawable.as_ref(), retain] };
            render_buffer.present_drawable(&current_drawable);

            let block = block::ConcreteBlock::new(move |_: &metal::CommandBufferRef| {
                sem.signal();
            })
            .copy();

            render_buffer.add_completed_handler(&block);

            render_buffer.commit();
        }

        self.render_pass_desc = None;
        self.render_buffer = None;
        self.cache.current_ub_offset = 0;

        if (self.current_frame_index + 1) >= NUM_INFLIGHT_FRAMES {
            self.current_frame_index = 0;
        }
    }

    fn draw(&self, base_element: i32, num_elements: i32, num_instances: i32) {
        if let Some(render_encoder) = &self.render_encoder {
            let pip = &self.pipelines[self.cache.cur_pipeline.unwrap().0];
            let primitive_type = pip.params.primitive_type.into();
            let index_buffer = self.cache.index_buffer.as_ref().unwrap();

            render_encoder.draw_indexed_primitives_instanced(
                primitive_type,
                num_elements as u64,
                MTLIndexType::UInt16, //TODO:
                &index_buffer.raw,
                0,
                num_instances as u64,
            );
        }
    }
}

impl From<Comparison> for MTLCompareFunction {
    fn from(cmp: Comparison) -> Self {
        match cmp {
            Comparison::Never => MTLCompareFunction::Never,
            Comparison::Less => MTLCompareFunction::Less,
            Comparison::LessOrEqual => MTLCompareFunction::LessEqual,
            Comparison::Greater => MTLCompareFunction::Greater,
            Comparison::GreaterOrEqual => MTLCompareFunction::GreaterEqual,
            Comparison::Equal => MTLCompareFunction::Equal,
            Comparison::NotEqual => MTLCompareFunction::NotEqual,
            Comparison::Always => MTLCompareFunction::Always,
        }
    }
}

impl From<BlendFactor> for MTLBlendFactor {
    fn from(factor: BlendFactor) -> Self {
        match factor {
            BlendFactor::Zero => MTLBlendFactor::Zero,
            BlendFactor::One => MTLBlendFactor::One,
            BlendFactor::Value(BlendValue::SourceColor) => MTLBlendFactor::SourceColor,
            BlendFactor::Value(BlendValue::SourceAlpha) => MTLBlendFactor::SourceAlpha,
            BlendFactor::Value(BlendValue::DestinationColor) => MTLBlendFactor::DestinationColor,
            BlendFactor::Value(BlendValue::DestinationAlpha) => MTLBlendFactor::DestinationAlpha,
            BlendFactor::OneMinusValue(BlendValue::SourceColor) => {
                MTLBlendFactor::OneMinusSourceColor
            }
            BlendFactor::OneMinusValue(BlendValue::SourceAlpha) => {
                MTLBlendFactor::OneMinusSourceAlpha
            }
            BlendFactor::OneMinusValue(BlendValue::DestinationColor) => {
                MTLBlendFactor::OneMinusDestinationColor
            }
            BlendFactor::OneMinusValue(BlendValue::DestinationAlpha) => {
                MTLBlendFactor::OneMinusDestinationAlpha
            }
            BlendFactor::SourceAlphaSaturate => MTLBlendFactor::SourceAlphaSaturated,
        }
    }
}

impl From<StencilOp> for MTLStencilOperation {
    fn from(op: StencilOp) -> Self {
        match op {
            StencilOp::Keep => MTLStencilOperation::Keep,
            StencilOp::Zero => MTLStencilOperation::Zero,
            StencilOp::Replace => MTLStencilOperation::Replace,
            StencilOp::IncrementClamp => MTLStencilOperation::IncrementClamp,
            StencilOp::DecrementClamp => MTLStencilOperation::DecrementClamp,
            StencilOp::Invert => MTLStencilOperation::Invert,
            StencilOp::IncrementWrap => MTLStencilOperation::IncrementWrap,
            StencilOp::DecrementWrap => MTLStencilOperation::DecrementWrap,
        }
    }
}

impl From<CompareFunc> for MTLCompareFunction {
    fn from(cf: CompareFunc) -> Self {
        match cf {
            CompareFunc::Always => MTLCompareFunction::Always,
            CompareFunc::Never => MTLCompareFunction::Never,
            CompareFunc::Less => MTLCompareFunction::Less,
            CompareFunc::Equal => MTLCompareFunction::Equal,
            CompareFunc::LessOrEqual => MTLCompareFunction::LessEqual,
            CompareFunc::Greater => MTLCompareFunction::Greater,
            CompareFunc::NotEqual => MTLCompareFunction::NotEqual,
            CompareFunc::GreaterOrEqual => MTLCompareFunction::GreaterEqual,
        }
    }
}

impl From<VertexStep> for MTLVertexStepFunction {
    fn from(vs: VertexStep) -> Self {
        match vs {
            VertexStep::PerVertex => MTLVertexStepFunction::PerVertex,
            VertexStep::PerInstance => MTLVertexStepFunction::PerInstance,
        }
    }
}

impl From<PrimitiveType> for MTLPrimitiveType {
    fn from(primitive_type: PrimitiveType) -> Self {
        match primitive_type {
            PrimitiveType::Triangles => MTLPrimitiveType::Triangle,
            PrimitiveType::Lines => MTLPrimitiveType::Line,
        }
    }
}

impl From<CullFace> for MTLCullMode {
    fn from(cull_face: CullFace) -> Self {
        match cull_face {
            CullFace::Back => MTLCullMode::Back,
            CullFace::Front => MTLCullMode::Front,
            CullFace::Nothing => MTLCullMode::None,
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
        assert!(!ctx.shaders.is_empty(), "Create shaders first");

        let shader_inernal = &ctx.shaders[shader.0];

        let attributes_len = attributes
            .iter()
            .map(|layout| match layout.format {
                VertexFormat::Mat4 => 4,
                _ => 1,
            })
            .sum();

        let mut vertex_layout: Vec<VertexAttributeInternal> = Vec::with_capacity(attributes_len);
        let mut offset = 0;

        for VertexAttribute {
            format,
            buffer_index,
            ..
        } in attributes
        {
            let mut attributes_count: usize = 1;
            let mut format = *format;

            if format == VertexFormat::Mat4 {
                format = VertexFormat::Float4;
                attributes_count = 4;
            }
            for i in 0..attributes_count {
                let size = format.size();
                let attr = VertexAttributeInternal {
                    size,
                    offset,
                    buffer_index: (*buffer_index + 1) as u64,
                    format: format.into(),
                };
                offset += size;
                vertex_layout.push(attr);
            }
        }

        let pipeline = PipelineInternal {
            layout: buffer_layout.to_vec(),
            attributes: vertex_layout,
            shader,
            params,
        };

        ctx.pipelines.push(pipeline.clone());

        let vertex_descriptor = VertexDescriptor::new();

        // Uniform buffer object
        // always with buffer index = 0
        let uniforms = &shader_inernal.uniforms;
        for (i, elem) in uniforms.iter().enumerate() {
            let mtl_attribute_desc = vertex_descriptor
                .attributes()
                .object_at((MAX_VERTEX_ATTRIBUTES - 1 - i) as u64)
                .unwrap();
            mtl_attribute_desc.set_buffer_index(UNIFORM_BUFFER_INDEX);
            mtl_attribute_desc.set_offset(elem.offset);
            mtl_attribute_desc.set_format(elem.format);
        }

        assert!(shader_inernal.stride > 0);

        let mtl_buffer_desc = vertex_descriptor.layouts().object_at(0 as u64).unwrap();
        mtl_buffer_desc.set_stride(shader_inernal.stride);

        // Vertex Buffers
        for (i, attrib) in pipeline.attributes.iter().enumerate() {
            let mtl_attribute_desc = vertex_descriptor.attributes().object_at(i as u64).unwrap();
            mtl_attribute_desc.set_buffer_index(attrib.buffer_index as u64);
            mtl_attribute_desc.set_offset(attrib.offset as u64);
            mtl_attribute_desc.set_format(attrib.format);
        }

        for (i, layout) in pipeline.layout.iter().enumerate() {
            assert!(layout.stride > 0);

            let mtl_buffer_desc = vertex_descriptor
                .layouts()
                .object_at((i + 1) as u64)
                .unwrap();
            mtl_buffer_desc.set_stride(layout.stride as u64);
            mtl_buffer_desc.set_step_function(layout.step_func.into());
            mtl_buffer_desc.set_step_rate(layout.step_rate as u64);
        }

        let vert = shader_inernal
            .library
            .get_function(shader_inernal.vs_entry.as_str(), None)
            .unwrap();
        let frag = shader_inernal
            .library
            .get_function(shader_inernal.fs_entry.as_str(), None)
            .unwrap();

        let pipeline_state_descriptor = RenderPipelineDescriptor::new();
        pipeline_state_descriptor.set_vertex_function(Some(&vert));
        pipeline_state_descriptor.set_fragment_function(Some(&frag));
        pipeline_state_descriptor.set_vertex_descriptor(Some(vertex_descriptor));
        //TODO: get from mtkview
        pipeline_state_descriptor
            .set_depth_attachment_pixel_format(MTLPixelFormat::Depth32Float_Stencil8);
        pipeline_state_descriptor
            .set_stencil_attachment_pixel_format(MTLPixelFormat::Depth32Float_Stencil8);

        let color_attachment = pipeline_state_descriptor
            .color_attachments()
            .object_at(0)
            .unwrap();
        color_attachment.set_pixel_format(MTLPixelFormat::BGRA8Unorm);

        ctx.pipeline_state = Some(
            ctx.device
                .new_render_pipeline_state(&pipeline_state_descriptor)
                .unwrap(),
        );

        Pipeline(ctx.pipelines.len() - 1)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct VertexAttributeInternal {
    size: u64,
    offset: u64,
    buffer_index: u64,
    format: MTLVertexFormat,
}

#[derive(Clone, Debug)]
struct PipelineInternal {
    layout: Vec<BufferLayout>,
    attributes: Vec<VertexAttributeInternal>,
    shader: Shader,
    params: PipelineParams,
}

#[derive(Clone, Debug)]
pub struct Buffer {
    raw: metal::Buffer,
    buffer_type: BufferType,
    size: usize,
}

impl Buffer {
    pub fn immutable<T>(ctx: &mut Context, buffer_type: BufferType, data: &[T]) -> Buffer {
        //TODO: GL Backend limitation
        if buffer_type == BufferType::IndexBuffer {
            assert!(
                mem::size_of::<T>() == 2,
                "Only u16/i16 index buffers are implemented right now"
            );
        }

        let size = mem::size_of_val(data) as u64;
        let buffer = ctx.device.new_buffer_with_data(
            data.as_ptr() as *const std::ffi::c_void,
            size,
            Usage::Immutable.into(),
        );

        Buffer {
            raw: buffer,
            buffer_type,
            size: size as usize,
        }
    }

    pub fn stream(ctx: &mut Context, buffer_type: BufferType, size: usize) -> Buffer {
        let buffer = ctx.device.new_buffer(size as u64, Usage::Stream.into());

        Buffer {
            raw: buffer,
            buffer_type,
            size: size as usize,
        }
    }

    pub fn update<T>(&self, ctx: &mut Context, data: &[T]) {
        todo!();
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn delete(&self) {
        let () = unsafe {
            msg_send![
                self.raw.as_ref(),
                setPurgeableState: metal::MTLPurgeableState::Empty
            ]
        };
        let () = unsafe { msg_send![self.raw.as_ref(), release] };
    }

    pub fn with_label(&self, label: &str) {
        self.raw.set_label(label);
    }
}

impl From<Usage> for MTLResourceOptions {
    fn from(usage: Usage) -> Self {
        match usage {
            Usage::Immutable => MTLResourceOptions::StorageModeShared,
            #[cfg(target_os = "macos")]
            Usage::Dynamic | Usage::Stream => {
                MTLResourceOptions::CPUCacheModeWriteCombined
                    | MTLResourceOptions::StorageModeManaged
            }
            #[cfg(target_os = "ios")]
            Usage::Dynamic | Usage::Stream => MTLResourceOptions::CPUCacheModeWriteCombined,
        }
    }
}

impl From<VertexFormat> for MTLVertexFormat {
    fn from(vf: VertexFormat) -> Self {
        match vf {
            VertexFormat::Float1 => MTLVertexFormat::Float,
            VertexFormat::Float2 => MTLVertexFormat::Float2,
            VertexFormat::Float3 => MTLVertexFormat::Float3,
            VertexFormat::Float4 => MTLVertexFormat::Float4,
            VertexFormat::Byte1 => MTLVertexFormat::Char,
            VertexFormat::Byte2 => MTLVertexFormat::Char2,
            VertexFormat::Byte3 => MTLVertexFormat::Char3,
            VertexFormat::Byte4 => MTLVertexFormat::Char4,
            VertexFormat::Short1 => MTLVertexFormat::Short,
            VertexFormat::Short2 => MTLVertexFormat::Short2,
            VertexFormat::Short3 => MTLVertexFormat::Short3,
            VertexFormat::Short4 => MTLVertexFormat::Short4,
            VertexFormat::Int1 => MTLVertexFormat::Int,
            VertexFormat::Int2 => MTLVertexFormat::Int2,
            VertexFormat::Int3 => MTLVertexFormat::Int3,
            VertexFormat::Int4 => MTLVertexFormat::Int4,
            _ => unreachable!(),
        }
    }
}

impl From<UniformType> for MTLVertexFormat {
    fn from(ut: UniformType) -> Self {
        match ut {
            UniformType::Float1 => MTLVertexFormat::Float,
            UniformType::Float2 => MTLVertexFormat::Float2,
            UniformType::Float3 => MTLVertexFormat::Float3,
            UniformType::Float4 => MTLVertexFormat::Float4,
            UniformType::Int1 => MTLVertexFormat::Int,
            UniformType::Int2 => MTLVertexFormat::Int2,
            UniformType::Int3 => MTLVertexFormat::Int3,
            UniformType::Int4 => MTLVertexFormat::Int4,
            _ => unreachable!(),
        }
    }
}

// https://developer.apple.com/metal/Metal-Shading-Language-Specification.pdf
impl VertexFormat {
    pub fn size(&self) -> u64 {
        match self {
            VertexFormat::Float2 => 8,
            VertexFormat::Float4 => 16,
            _ => todo!(),
        }
    }
}

struct ShaderInternal {
    library: metal::Library,
    fs_entry: String,
    vs_entry: String,
    uniforms: Vec<ShaderUniform>,
    stride: u64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct ShaderUniform {
    size: u64,
    offset: u64,
    format: MTLVertexFormat,
}

impl Shader {
    pub fn new(
        ctx: &mut Context,
        metal_lib: &[u8],
        meta: ShaderMeta,
    ) -> Result<Shader, ShaderError> {
        Self::create(ctx, metal_lib, "vertex_function", "fragment_function", meta)
    }

    pub fn with_entry_points(
        ctx: &mut Context,
        metal_lib: &[u8],
        vs_entry: &str,
        fs_entry: &str,
        meta: ShaderMeta,
    ) -> Result<Shader, ShaderError> {
        Self::create(ctx, metal_lib, vs_entry, fs_entry, meta)
    }

    fn create(
        ctx: &mut Context,
        metal_lib: &[u8],
        vs_entry: &str,
        fs_entry: &str,
        meta: ShaderMeta,
    ) -> Result<Shader, ShaderError> {
        //TODO: ShaderError
        let library = ctx.device.new_library_with_data(metal_lib).unwrap();
        let mut stride = 0;
        let mut index = 0;
        let uniforms = meta
            .uniforms
            .uniforms
            .iter()
            .scan(0, |offset, uniform| {
                let size = uniform.uniform_type.size(1) as u64;
                stride += size;
                let shader_uniform = ShaderUniform {
                    size: uniform.uniform_type.size(1) as u64,
                    offset: *offset,
                    format: uniform.uniform_type.into(),
                };
                index += 1;
                *offset += size * uniform.array_count as u64;
                Some(shader_uniform)
            })
            .collect();

        let shader = ShaderInternal {
            library,
            vs_entry: vs_entry.into(),
            fs_entry: fs_entry.into(),
            uniforms,
            stride,
        };

        ctx.shaders.push(shader);
        Ok(Shader(ctx.shaders.len() - 1))
    }
}

fn get_mtl_device() -> metal::Device {
    unsafe {
        let raw_device_ptr = sapp::sapp_metal_get_device();
        metal::Device::from_ptr(raw_device_ptr as _)
    }
}

fn get_renderpass_descriptor() -> metal::RenderPassDescriptor {
    unsafe {
        let raw_rpd_ptr = sapp::sapp_metal_get_renderpass_descriptor();
        metal::RenderPassDescriptor::from_ptr(raw_rpd_ptr as _)
    }
}

fn get_drawable() -> metal::Drawable {
    unsafe {
        let raw_drawable_ptr = sapp::sapp_metal_get_drawable();
        metal::Drawable::from_ptr(raw_drawable_ptr as _)
    }
}

#[inline]
fn roundup_ub_buffer(current_buffer: usize) -> usize {
    ((current_buffer) + ((UNIFORM_BUFFER_ALIGN) - 1)) & !((UNIFORM_BUFFER_ALIGN) - 1)
}
