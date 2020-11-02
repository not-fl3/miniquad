mod texture;

pub use texture::*;

use metal_rs as metal;

use crate::graphics::*;
use crate::GraphicContext;
use foreign_types::ForeignType;
use metal::{
    CommandBuffer, CommandQueue, DepthStencilDescriptor, DepthStencilState, Device, MTLBlendFactor,
    MTLClearColor, MTLCompareFunction, MTLCullMode, MTLIndexType, MTLLoadAction, MTLPixelFormat,
    MTLPrimitiveType, MTLResourceOptions, MTLScissorRect, MTLStencilOperation, MTLStoreAction,
    MTLVertexFormat, MTLVertexStepFunction, MTLViewport, MTLWinding, RenderCommandEncoder,
    RenderPassDescriptor, RenderPipelineDescriptor, RenderPipelineState, StencilDescriptor,
    VertexDescriptor,
};
use std::mem;

// https://developer.apple.com/metal/Metal-Feature-Set-Tables.pdf
const MAX_BUFFERS_PER_STAGE: usize = 14;
const MAX_UNIFORM_BUFFER_SIZE: usize = 4 * 1024 * 1024;
const MAX_VERTEX_ATTRIBUTES: usize = 31;
const UNIFORM_BUFFER_INDEX: u64 = 0;
const NUM_INFLIGHT_FRAMES: usize = 3;

const DEFAULT_FRAMEBUFFER_PIXEL_FORMAT: MTLPixelFormat = MTLPixelFormat::BGRA8Unorm;
const DEFAULT_FRAMEBUFFER_DEPTH_FORMAT: MTLPixelFormat = MTLPixelFormat::Depth32Float_Stencil8;

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
    cmd_buffer: Option<CommandBuffer>,
    render_encoder: Option<RenderCommandEncoder>,
    default_render_pass_desc: RenderPassDescriptor,
    pipelines: Vec<PipelineInternal>,
    shaders: Vec<ShaderInternal>,
    passes: Vec<RenderPassInternal>,
    current_pass: Option<RenderPass>,
    current_frame_index: usize,
    cache: Cache,
    internal_pipeline: RenderPipelineState,
}

struct Cache {
    vertex_buffers: Vec<Option<self::Buffer>>,
    index_buffer: Option<self::Buffer>,
    uniform_buffer: Vec<metal::Buffer>,
    current_ub_offset: usize,
    cur_pipeline: Option<Pipeline>,
    textures: Vec<metal::Texture>,
    samplers: Vec<metal::SamplerState>,
}

impl Cache {
    pub fn clear(&mut self) {
        // TODO: clear texture cache
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
            textures: vec![],
            samplers: vec![],
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

        let rpd = get_renderpass_descriptor();
        let () = unsafe { msg_send![rpd.as_ref(), retain] };

        let internal_pipeline = Self::init_internal_pipeline(device.as_ref());

        Context {
            device,
            command_queue,
            cmd_buffer: None,
            render_encoder: None,
            default_render_pass_desc: rpd,
            pipelines: vec![],
            shaders: vec![],
            passes: vec![],
            current_pass: None,
            current_frame_index: 1,
            cache,
            internal_pipeline,
        }
    }

    fn clear_background_color(&self, r: f32, g:f32, b:f32, a: f32) {
        if let Some(render_encoder) = &self.render_encoder {
            let clear_rect: &[f32] = &[
                // x y w h
                -1.0, -1.0, 2.0, 2.0,
                r, g, b, a,
            ];

            let clear_rect_buffer = self.device.new_buffer_with_data(
                clear_rect.as_ptr() as *const _,
                mem::size_of_val(clear_rect) as u64,
                Usage::Immutable.into(),
            );

            render_encoder.set_render_pipeline_state(&self.internal_pipeline);
            render_encoder.set_vertex_buffer(0, Some(&clear_rect_buffer), 0);
            render_encoder.draw_primitives_instanced(
                metal::MTLPrimitiveType::TriangleStrip,
                0,
                4,
                1,
            );
        }
    }

    fn init_internal_pipeline(device: &metal::DeviceRef) -> RenderPipelineState {
        // TODO: metallib
        let shader_source = include_str!("shaders/shaders.metal");
        let library = device
            .new_library_with_source(shader_source, &metal::CompileOptions::new())
            .unwrap();
        let vert = library.get_function("clear_rect_vertex", None).unwrap();
        let frag = library.get_function("clear_rect_fragment", None).unwrap();

        let pipeline_state_descriptor = RenderPipelineDescriptor::new();
        pipeline_state_descriptor.set_vertex_function(Some(&vert));
        pipeline_state_descriptor.set_fragment_function(Some(&frag));
        let attachment = pipeline_state_descriptor
            .color_attachments()
            .object_at(0)
            .unwrap();
        attachment.set_pixel_format(DEFAULT_FRAMEBUFFER_PIXEL_FORMAT);
        attachment.set_blending_enabled(true);
        attachment.set_rgb_blend_operation(metal::MTLBlendOperation::Add);
        attachment.set_alpha_blend_operation(metal::MTLBlendOperation::Add);
        attachment.set_source_rgb_blend_factor(metal::MTLBlendFactor::SourceAlpha);
        attachment.set_source_alpha_blend_factor(metal::MTLBlendFactor::SourceAlpha);
        attachment.set_destination_rgb_blend_factor(metal::MTLBlendFactor::OneMinusSourceAlpha);
        attachment.set_destination_alpha_blend_factor(metal::MTLBlendFactor::OneMinusSourceAlpha);

        device
            .new_render_pipeline_state(&pipeline_state_descriptor)
            .unwrap()
    }
}

struct RenderPassInternal {
    render_pass_desc: RenderPassDescriptor,
    texture: Texture,
    depth_texture: Option<Texture>,
}

impl RenderPass {
    pub fn new(
        context: &mut Context,
        color_img: Texture,
        depth_img: impl Into<Option<Texture>>,
    ) -> RenderPass {
        let render_pass_desc = metal::RenderPassDescriptor::new();

        let color_texture = &context.cache.textures[color_img.texture];

        let color_attach = render_pass_desc.color_attachments().object_at(0).unwrap();
        color_attach.set_texture(Some(color_texture));
        color_attach.set_load_action(MTLLoadAction::Load);
        color_attach.set_store_action(MTLStoreAction::Store);

        let depth_img = depth_img.into().unwrap();
        let depth_texture = &context.cache.textures[depth_img.texture];

        let depth_attach = render_pass_desc.depth_attachment().unwrap();
        depth_attach.set_texture(Some(depth_texture));

        let stencil_attach = render_pass_desc.stencil_attachment().unwrap();
        stencil_attach.set_texture(Some(depth_texture));

        let pass = RenderPassInternal {
            render_pass_desc: render_pass_desc.to_owned(),
            texture: color_img,
            depth_texture: depth_img.into(),
        };

        context.passes.push(pass);

        RenderPass(context.passes.len() - 1)
    }

    pub fn texture(&self, ctx: &mut Context) -> Texture {
        let render_pass = &mut ctx.passes[self.0];

        render_pass.texture
    }

    pub fn delete(&self, ctx: &mut Context) {
        ctx.passes.remove(self.0);
    }
}

impl GraphicContext for Context {
    fn apply_pipeline(&mut self, pipeline: &Pipeline) {
        assert!(!self.pipelines.is_empty(), "Create pipeline first");
        self.cache.cur_pipeline = Some(*pipeline);

        let pipeline = &self.pipelines[pipeline.0];

        if let Some(render_encoder) = &self.render_encoder {
            render_encoder.set_render_pipeline_state(pipeline.render_pipeline_state.as_ref());
            render_encoder.set_depth_stencil_state(pipeline.depth_stencil_state.as_ref());
            render_encoder.set_front_facing_winding(pipeline.params.front_face_order.into());
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

    fn apply_viewport(&mut self, x: i32, y: i32, w: i32, h: i32) {
        if let Some(render_encoder) = &self.render_encoder {
            render_encoder.set_viewport(MTLViewport {
                originX: 0.0,
                originY: 0.0,
                width: w as f64,
                height: h as f64,
                znear: 0.0,
                zfar: 1.0,
            });
        }
    }

    fn apply_scissor_rect(&mut self, x: i32, y: i32, w: i32, h: i32) {
        if let Some(render_encoder) = &self.render_encoder {
            //TODO: validate
            render_encoder.set_scissor_rect(MTLScissorRect {
                x: x as u64,
                y: y as u64,
                width: w as u64,
                height: h as u64,
            });
        }
    }

    fn apply_bindings(&mut self, bindings: &Bindings) {
        assert!(
            bindings.vertex_buffers.len() <= MAX_SHADERSTAGE_BUFFERS,
            "Only {} buffers are supported right now",
            MAX_SHADERSTAGE_BUFFERS
        );

        assert!(
            !self.cache.samplers.is_empty() || !self.cache.textures.is_empty(),
            "Create texture first"
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
                sampler_states.push(Some(self.cache.samplers[i].as_ref()));
                textures.push(Some(self.cache.textures[i].as_ref()));
            }

            if img_count > 0 {
                render_encoder.set_fragment_sampler_states(0, sampler_states.as_slice());
                render_encoder.set_fragment_textures(0, textures.as_slice());
            }
        }
    }

    fn apply_uniforms<U>(&mut self, uniforms: &U) {
        self.current_frame_index = (self.current_frame_index + 1) % NUM_INFLIGHT_FRAMES;

        let pip = &self.pipelines[self.cache.cur_pipeline.unwrap().0];
        let shader_internal = &self.shaders[pip.shader.0];

        let data = uniforms as *const _ as *const std::ffi::c_void;
        let data_lenght = shader_internal.stride;

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
        let render_pass_desc = &(match self.current_pass {
            None => self.default_render_pass_desc.as_ref(),
            Some(pass) => &self.passes[self.current_pass.unwrap().0].render_pass_desc,
        });

        if let Some((r, g, b, a)) = color {
            let color_attachment = render_pass_desc.color_attachments().object_at(0).unwrap();
            color_attachment.set_load_action(MTLLoadAction::Clear);
            color_attachment.set_store_action(MTLStoreAction::Store);
            color_attachment
                .set_clear_color(MTLClearColor::new(r as f64, g as f64, b as f64, a as f64));

            self.clear_background_color(r, g, b, a);
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

    fn begin_default_pass(&mut self, action: PassAction) {
        self.default_render_pass_desc = get_renderpass_descriptor();
        let () = unsafe { msg_send![self.default_render_pass_desc.as_ref(), retain] };

        self.begin_pass(None, action);
    }

    fn begin_pass(&mut self, pass: impl Into<Option<RenderPass>>, action: PassAction) {
        let (ref render_pass_desc, w, h) = match pass.into() {
            None => (
                &self.default_render_pass_desc,
                unsafe { sapp_width() } as f64,
                unsafe { sapp_height() } as f64,
            ),
            Some(pass) => {
                self.current_pass = Some(pass);

                let pass_internal = &self.passes[pass.0];
                (
                    &pass_internal.render_pass_desc,
                    pass_internal.texture.width as f64,
                    pass_internal.texture.height as f64,
                )
            }
        };

        if self.cmd_buffer.is_none() {
            // first pass
            let cmd_buffer = self.command_queue.new_command_buffer().to_owned();
            cmd_buffer.set_label("MiniQuadCmdBuffer");
            self.cmd_buffer = Some(cmd_buffer);
        }

        let render_encoder = self
            .cmd_buffer
            .as_ref()
            .unwrap()
            .new_render_command_encoder(render_pass_desc)
            .to_owned();
        render_encoder.set_label(format!("MiniQuadRenderEncoder[{}]", self.passes.len()).as_str());
        render_encoder
            .push_debug_group(format!("MiniQuadRenderPass[{}]", self.passes.len()).as_str());
        render_encoder.set_viewport(MTLViewport {
            originX: 0.0,
            originY: 0.0,
            width: w,
            height: h,
            znear: 0.0,
            zfar: 1.0,
        });
        render_encoder.set_scissor_rect(MTLScissorRect {
            x: 0,
            y: 0,
            width: w as u64,
            height: h as u64,
        });

        self.render_encoder = Some(render_encoder);

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
        self.current_pass = None;
    }

    fn commit_frame(&mut self) {
        if let Some(cmd_buffer) = &self.cmd_buffer {
            let current_drawable = get_drawable();
            let () = unsafe { msg_send![current_drawable.as_ref(), retain] };
            cmd_buffer.present_drawable(&current_drawable);
            cmd_buffer.commit();
            cmd_buffer.wait_until_completed();
        }

        self.cmd_buffer = None;
        self.cache.current_ub_offset = 0;
        self.cache.clear();

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

impl From<FrontFaceOrder> for MTLWinding {
    fn from(order: FrontFaceOrder) -> Self {
        match order {
            FrontFaceOrder::Clockwise => MTLWinding::Clockwise,
            FrontFaceOrder::CounterClockwise => MTLWinding::CounterClockwise,
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

        let shader_internal = &ctx.shaders[shader.0];

        let attributes_len = attributes
            .iter()
            .map(|layout| match layout.format {
                VertexFormat::Mat4 => 4,
                _ => 1,
            })
            .sum();

        let mut vertex_layout: Vec<VertexAttributeInternal> = Vec::with_capacity(attributes_len);
        let mut vertex_attrib_offset = 0;

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
                    offset: vertex_attrib_offset * attributes_count as u64,
                    buffer_index: (*buffer_index + 1) as u64,
                    format: format.into(),
                };
                vertex_attrib_offset += size;
                vertex_layout.push(attr);
            }
        }

        let vertex_descriptor = VertexDescriptor::new();

        // Uniform buffer object
        // always with buffer index = 0
        let uniforms = &shader_internal.uniforms;
        for (i, elem) in uniforms.iter().enumerate() {
            let mtl_attribute_desc = vertex_descriptor
                .attributes()
                .object_at((MAX_VERTEX_ATTRIBUTES - 1 - i) as u64)
                .unwrap();
            mtl_attribute_desc.set_buffer_index(UNIFORM_BUFFER_INDEX);
            mtl_attribute_desc.set_offset(elem.offset);
            mtl_attribute_desc.set_format(elem.format);
        }

        assert!(shader_internal.stride > 0);

        let mtl_buffer_desc = vertex_descriptor.layouts().object_at(0 as u64).unwrap();
        mtl_buffer_desc.set_stride(shader_internal.stride);

        // Vertex Buffers
        for (i, attrib) in vertex_layout.iter().enumerate() {
            let mtl_attribute_desc = vertex_descriptor.attributes().object_at(i as u64).unwrap();
            mtl_attribute_desc.set_buffer_index(attrib.buffer_index as u64);
            mtl_attribute_desc.set_offset(attrib.offset as u64);
            mtl_attribute_desc.set_format(attrib.format);
        }

        for (i, layout) in buffer_layout.iter().enumerate() {
            let stride = if layout.stride > 0 {
                layout.stride
            } else {
                vertex_attrib_offset as i32
            };

            assert!(stride > 0);

            let mtl_buffer_desc = vertex_descriptor
                .layouts()
                .object_at((i + 1) as u64)
                .unwrap();
            mtl_buffer_desc.set_stride(stride as u64);
            mtl_buffer_desc.set_step_function(layout.step_func.into());
            mtl_buffer_desc.set_step_rate(layout.step_rate as u64);
        }

        let pipeline_state_descriptor = RenderPipelineDescriptor::new();
        pipeline_state_descriptor.set_vertex_function(Some(&shader_internal.vs_function));
        pipeline_state_descriptor.set_fragment_function(Some(&shader_internal.fs_function));
        pipeline_state_descriptor.set_vertex_descriptor(Some(vertex_descriptor));
        let color_attachment = pipeline_state_descriptor
            .color_attachments()
            .object_at(0)
            .unwrap();

        color_attachment.set_pixel_format(DEFAULT_FRAMEBUFFER_PIXEL_FORMAT);
        color_attachment.set_blending_enabled(true);

        pipeline_state_descriptor
            .set_depth_attachment_pixel_format(DEFAULT_FRAMEBUFFER_DEPTH_FORMAT);
        pipeline_state_descriptor
            .set_stencil_attachment_pixel_format(DEFAULT_FRAMEBUFFER_DEPTH_FORMAT);

        let pipeline_state = ctx
            .device
            .new_render_pipeline_state(&pipeline_state_descriptor)
            .unwrap();

        let depth_stencil_desc = DepthStencilDescriptor::new();
        depth_stencil_desc.set_depth_write_enabled(params.depth_write);
        depth_stencil_desc.set_depth_compare_function(params.depth_test.into());

        if let Some(stencil_test) = params.stencil_test {
            let back_face_stencil_desc = StencilDescriptor::new();
            back_face_stencil_desc.set_stencil_compare_function(stencil_test.back.test_func.into());
            back_face_stencil_desc.set_stencil_failure_operation(stencil_test.back.fail_op.into());
            back_face_stencil_desc
                .set_depth_failure_operation(stencil_test.back.depth_fail_op.into());
            back_face_stencil_desc.set_read_mask(stencil_test.back.test_mask);
            back_face_stencil_desc.set_write_mask(stencil_test.back.write_mask);

            depth_stencil_desc.set_back_face_stencil(Some(back_face_stencil_desc.as_ref()));

            let front_face_stencil_desc = StencilDescriptor::new();
            front_face_stencil_desc
                .set_stencil_compare_function(stencil_test.front.test_func.into());
            front_face_stencil_desc
                .set_stencil_failure_operation(stencil_test.front.fail_op.into());
            front_face_stencil_desc
                .set_depth_failure_operation(stencil_test.front.depth_fail_op.into());
            front_face_stencil_desc.set_read_mask(stencil_test.front.test_mask);
            front_face_stencil_desc.set_write_mask(stencil_test.front.write_mask);

            depth_stencil_desc.set_front_face_stencil(Some(front_face_stencil_desc.as_ref()))
        }

        let depth_stencil_state = ctx.device.new_depth_stencil_state(&depth_stencil_desc);

        let pipeline = PipelineInternal {
            render_pipeline_state: pipeline_state,
            depth_stencil_state,
            layout: buffer_layout.to_vec(),
            attributes: vertex_layout,
            shader,
            params,
        };

        ctx.pipelines.push(pipeline);

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
    render_pipeline_state: RenderPipelineState,
    depth_stencil_state: DepthStencilState,
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
        let size = mem::size_of_val(data);

        assert!(size <= self.size);

        let content = data as *const _ as *const std::ffi::c_void;

        unsafe {
            self.raw.contents().copy_from_nonoverlapping(content, size);

            #[cfg(target_os = "macos")]
            self.raw
                .did_modify_range(std::mem::transmute(NSRange::new(0, size as u64)));
        }
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
            VertexFormat::Mat4 => MTLVertexFormat::Float4,
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
            UniformType::Mat4 => MTLVertexFormat::Float4,
            _ => unreachable!(),
        }
    }
}

// https://developer.apple.com/metal/Metal-Shading-Language-Specification.pdf
impl VertexFormat {
    pub fn size(&self) -> u64 {
        match self {
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
            _ => unreachable!(),
        }
    }
}

/// An error type for creating shaders.
#[derive(Clone, Debug, PartialEq)]
pub enum ShaderError {
    CompilationFailed(String),
}

impl Display for ShaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ShaderError::CompilationFailed(ref string) => {
                write!(f, "The shader failed to compile: {}", string)
            }
        }
    }
}

impl From<String> for ShaderError {
    fn from(str: String) -> Self {
        ShaderError::CompilationFailed(str)
    }
}

#[derive(Debug)]
struct ShaderInternal {
    vs_function: metal::Function,
    fs_function: metal::Function,
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
        vertex_shader: &str,
        fragment_shader: &str,
        meta: ShaderMeta,
    ) -> Result<Shader, ShaderError> {
        let vs_library = ctx
            .device
            .new_library_with_source(vertex_shader, &metal::CompileOptions::new())?;

        let fs_library = ctx
            .device
            .new_library_with_source(fragment_shader, &metal::CompileOptions::new())?;

        let vs_function = vs_library.get_function("vertex_function", None)?;
        let fs_function = fs_library.get_function("fragment_function", None)?;

        Self::create(ctx, vs_function, fs_function, meta)
    }

    pub fn with_entry_points(
        ctx: &mut Context,
        metal_lib: &[u8],
        vs_entry: &str,
        fs_entry: &str,
        meta: ShaderMeta,
    ) -> Result<Shader, ShaderError> {
        let library = ctx.device.new_library_with_data(metal_lib)?;
        let vs_function = library.get_function(vs_entry, None)?;
        let fs_function = library.get_function(fs_entry, None)?;

        Self::create(ctx, vs_function, fs_function, meta)
    }

    fn create(
        ctx: &mut Context,
        vs_function: metal::Function,
        fs_function: metal::Function,
        meta: ShaderMeta,
    ) -> Result<Shader, ShaderError> {
        let mut stride = 0;
        let mut index = 0;
        let uniforms = meta
            .uniforms
            .uniforms
            .iter()
            .scan(0, |offset, uniform| {
                let size = uniform.uniform_type.size() as u64;
                stride += size;
                let shader_uniform = ShaderUniform {
                    size: uniform.uniform_type.size() as u64,
                    offset: *offset,
                    format: uniform.uniform_type.into(),
                };
                index += 1;
                *offset += size as u64;
                Some(shader_uniform)
            })
            .collect();

        let shader = ShaderInternal {
            vs_function,
            fs_function,
            uniforms,
            stride,
        };

        ctx.shaders.push(shader);
        Ok(Shader(ctx.shaders.len() - 1))
    }
}

fn get_mtl_device() -> metal::Device {
    unsafe { metal::Device::from_ptr(sapp::sapp_metal_get_device() as _) }
}

fn get_renderpass_descriptor() -> metal::RenderPassDescriptor {
    unsafe {
        metal::RenderPassDescriptor::from_ptr(sapp::sapp_metal_get_renderpass_descriptor() as _)
    }
}

fn get_drawable() -> metal::Drawable {
    unsafe { metal::Drawable::from_ptr(sapp::sapp_metal_get_drawable() as _) }
}

#[inline]
fn roundup_ub_buffer(current_buffer: usize) -> usize {
    ((current_buffer) + ((UNIFORM_BUFFER_ALIGN) - 1)) & !((UNIFORM_BUFFER_ALIGN) - 1)
}
