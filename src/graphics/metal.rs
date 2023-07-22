#![allow(non_snake_case)]

use crate::native::apple::{
    apple_util::{self, msg_send_},
    frameworks::*,
};

use super::*;

// https://developer.apple.com/metal/Metal-Feature-Set-Tables.pdf
const MAX_UNIFORM_BUFFER_SIZE: u64 = 4 * 1024 * 1024;
const NUM_INFLIGHT_FRAMES: usize = 3;
#[cfg(any(target_os = "macos", all(target_os = "ios", target_arch = "x86_64")))]
const UNIFORM_BUFFER_ALIGN: u64 = 256;
#[cfg(all(target_os = "ios", not(target_arch = "x86_64")))]
const UNIFORM_BUFFER_ALIGN: u64 = 16;

impl From<VertexFormat> for MTLVertexFormat {
    fn from(vf: VertexFormat) -> Self {
        match vf {
            VertexFormat::Float1 => MTLVertexFormat::Float,
            VertexFormat::Float2 => MTLVertexFormat::Float2,
            VertexFormat::Float3 => MTLVertexFormat::Float3,
            VertexFormat::Float4 => MTLVertexFormat::Float4,
            VertexFormat::Byte1 => MTLVertexFormat::UChar,
            VertexFormat::Byte2 => MTLVertexFormat::UChar2,
            VertexFormat::Byte3 => MTLVertexFormat::UChar3,
            VertexFormat::Byte4 => MTLVertexFormat::UChar4,
            VertexFormat::Short1 => MTLVertexFormat::Short,
            VertexFormat::Short2 => MTLVertexFormat::Short2,
            VertexFormat::Short3 => MTLVertexFormat::Short3,
            VertexFormat::Short4 => MTLVertexFormat::Short4,
            VertexFormat::Int1 => MTLVertexFormat::Int,
            VertexFormat::Int2 => MTLVertexFormat::Int2,
            VertexFormat::Int3 => MTLVertexFormat::Int3,
            VertexFormat::Int4 => MTLVertexFormat::Int4,
            VertexFormat::Mat4 => MTLVertexFormat::Float4,
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

// impl From<StencilOp> for MTLStencilOperation {
//     fn from(op: StencilOp) -> Self {
//         match op {
//             StencilOp::Keep => MTLStencilOperation::Keep,
//             StencilOp::Zero => MTLStencilOperation::Zero,
//             StencilOp::Replace => MTLStencilOperation::Replace,
//             StencilOp::IncrementClamp => MTLStencilOperation::IncrementClamp,
//             StencilOp::DecrementClamp => MTLStencilOperation::DecrementClamp,
//             StencilOp::Invert => MTLStencilOperation::Invert,
//             StencilOp::IncrementWrap => MTLStencilOperation::IncrementWrap,
//             StencilOp::DecrementWrap => MTLStencilOperation::DecrementWrap,
//         }
//     }
// }

impl From<Equation> for MTLBlendOperation {
    fn from(cf: Equation) -> Self {
        match cf {
            Equation::Add => MTLBlendOperation::Add,
            Equation::Subtract => MTLBlendOperation::Subtract,
            Equation::ReverseSubtract => MTLBlendOperation::ReverseSubtract,
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

impl From<TextureFormat> for MTLPixelFormat {
    fn from(format: TextureFormat) -> Self {
        match format {
            TextureFormat::RGBA8 => MTLPixelFormat::RGBA8Unorm,
            //TODO: Depth16Unorm ?
            TextureFormat::Depth => MTLPixelFormat::Depth32Float_Stencil8,
            _ => todo!(),
        }
    }
}

// impl From<CullFace> for MTLCullMode {
//     fn from(cull_face: CullFace) -> Self {
//         match cull_face {
//             CullFace::Back => MTLCullMode::Back,
//             CullFace::Front => MTLCullMode::Front,
//             CullFace::Nothing => MTLCullMode::None,
//         }
//     }
// }

// impl From<FrontFaceOrder> for MTLWinding {
//     fn from(order: FrontFaceOrder) -> Self {
//         match order {
//             FrontFaceOrder::Clockwise => MTLWinding::Clockwise,
//             FrontFaceOrder::CounterClockwise => MTLWinding::CounterClockwise,
//         }
//     }
// }

#[inline]
fn roundup_ub_buffer(current_buffer: u64) -> u64 {
    ((current_buffer) + ((UNIFORM_BUFFER_ALIGN) - 1)) & !((UNIFORM_BUFFER_ALIGN) - 1)
}

// this scenario:
// buffer.update(); draw(buffer); buffer.update(); draw(buffer);
// is very problematic with metal's ownership model.
// buffer.update() half-update buffer - it doesn't really send data to the GPU claiming ownership,
// it postpone reading the CPU memory until drawcall will actually happen.
// There is no way to flush the buffer and makes metal's "update" function to update GPU memory right away.
// Thus miniquad keeps a lot of buffer's copies...
const BUFFERS_IN_ROTATION: usize = 30;

#[derive(Clone, Copy, Debug)]
pub struct Buffer {
    raw: [ObjcId; BUFFERS_IN_ROTATION],
    //buffer_type: BufferType,
    size: usize,
    //index_type: Option<IndexType>,
    value: usize,
    next_value: usize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct ShaderUniform {
    size: u64,
    offset: u64,
    format: MTLVertexFormat,
}

#[derive(Debug)]
struct ShaderInternal {
    vertex_function: ObjcId,
    fragment_function: ObjcId,
    //uniforms: Vec<ShaderUniform>,
    // the distance, in bytes, between two uniforms in uniforms buffer
    //stride: u64,
}

struct RenderPassInternal {
    render_pass_desc: ObjcId,
    texture: TextureId,
    _depth_texture: Option<TextureId>,
}

#[derive(Clone, Debug)]
struct PipelineInternal {
    pipeline_state: ObjcId,
    depth_stencil_state: ObjcId,
    //layout: Vec<BufferLayout>,
    //attributes: Vec<VertexAttributeInternal>,
    _shader: ShaderId,
    //params: PipelineParams,
}

struct Texture {
    texture: ObjcId,
    sampler: ObjcId,
    params: TextureParams,
}

pub struct MetalContext {
    buffers: Vec<Buffer>,
    shaders: Vec<ShaderInternal>,
    pipelines: Vec<PipelineInternal>,
    textures: Vec<Texture>,
    passes: Vec<RenderPassInternal>,
    command_queue: ObjcId,
    command_buffer: Option<ObjcId>,
    render_encoder: Option<ObjcId>,
    view: ObjcId,
    device: ObjcId,
    current_frame_index: usize,
    uniform_buffers: [ObjcId; 3],
    // cached index_buffer from apply_bindings
    index_buffer: Option<ObjcId>,
    // cached pipeline from apply_pipeline
    current_pipeline: Option<Pipeline>,
    current_ub_offset: u64,
}

impl MetalContext {
    pub fn new() -> MetalContext {
        unsafe {
            let view = crate::window::apple_view();
            assert!(!view.is_null());
            let device: ObjcId = msg_send![view, device];
            assert!(!device.is_null());
            let command_queue: ObjcId = msg_send![device, newCommandQueue];

            if false {
                let capture_manager = msg_send_![class![MTLCaptureManager], sharedCaptureManager];
                assert!(!capture_manager.is_null());

                let MTLCaptureDestinationGPUTraceDocument = 2u64;
                if !msg_send![
                    capture_manager,
                    supportsDestination: MTLCaptureDestinationGPUTraceDocument
                ] {
                    panic!("capture failed");
                }

                let capture_descriptor =
                    msg_send_![msg_send_![class![MTLCaptureDescriptor], alloc], init];
                msg_send_![capture_descriptor, setCaptureObject: device];
                msg_send_![
                    capture_descriptor,
                    setDestination: MTLCaptureDestinationGPUTraceDocument
                ];
                let path = apple_util::str_to_nsstring("/Users/fedor/wtf1.gputrace");
                let url = msg_send_![class!(NSURL), fileURLWithPath: path];
                msg_send_![capture_descriptor, setOutputURL: url];

                let mut error: ObjcId = nil;
                if !msg_send![capture_manager, startCaptureWithDescriptor:capture_descriptor
                              error:&mut error]
                {
                    let description: ObjcId = msg_send![error, localizedDescription];
                    let string = apple_util::nsstring_to_string(description);
                    panic!("Capture error: {}", string);
                }
            }

            #[cfg(target_os = "macos")]
            let options = {
                MTLResourceOptions::CPUCacheModeWriteCombined
                    | MTLResourceOptions::StorageModeManaged
            };
            #[cfg(target_os = "ios")]
            let options = { MTLResourceOptions::CPUCacheModeWriteCombined };

            let uniform_buffers = [
                msg_send![device, newBufferWithLength:MAX_UNIFORM_BUFFER_SIZE
                          options:options],
                msg_send![device, newBufferWithLength:MAX_UNIFORM_BUFFER_SIZE
                          options:options],
                msg_send![device, newBufferWithLength:MAX_UNIFORM_BUFFER_SIZE
                          options:options],
            ];

            MetalContext {
                command_queue,
                command_buffer: None,
                render_encoder: None,
                view,
                device,
                buffers: vec![],
                shaders: vec![],
                pipelines: vec![],
                textures: vec![],
                passes: vec![],
                index_buffer: None,
                current_pipeline: None,
                uniform_buffers,
                current_frame_index: 1,
                current_ub_offset: 0,
            }
        }
    }
}

impl RenderingBackend for MetalContext {
    fn buffer_size(&mut self, buffer: BufferId) -> usize {
        let buffer = &self.buffers[buffer.0];
        buffer.size
    }
    fn delete_buffer(&mut self, buffer: BufferId) {
        let buffer = &self.buffers[buffer.0];
        unsafe {
            for buffer in &buffer.raw {
                msg_send_![*buffer, release];
            }
        }
    }
    fn delete_texture(&mut self, texture: TextureId) {
        let texture = &self.textures[texture.0];
        unsafe {
            msg_send_![texture.texture, release];
        }
    }
    fn apply_viewport(&mut self, _x: i32, _y: i32, _w: i32, _h: i32) {}
    fn apply_scissor_rect(&mut self, x: i32, y: i32, w: i32, h: i32) {
        assert!(self.render_encoder.is_some());

        let (_, screen_height) = crate::window::screen_size();
        let r = MTLScissorRect {
            x: x as _,
            y: (screen_height as i32 - (y + h)) as u64,
            width: w as _,
            height: h as _,
        };
        unsafe { msg_send_![self.render_encoder.unwrap(), setScissorRect: r] };
    }
    fn texture_set_filter(&mut self, texture: TextureId, filter: FilterMode) {
        let mut texture = &mut self.textures[texture.0];

        let filter = match filter {
            FilterMode::Nearest => MTLSamplerMinMagFilter::Nearest,
            FilterMode::Linear => MTLSamplerMinMagFilter::Linear,
        };

        texture.sampler = unsafe {
            let sampler_dsc = msg_send_![class!(MTLSamplerDescriptor), new];

            msg_send_![sampler_dsc, setMinFilter: filter];
            msg_send_![sampler_dsc, setMagFilter: filter];

            msg_send_![self.device, newSamplerStateWithDescriptor: sampler_dsc]
        };
    }
    fn texture_set_wrap(&mut self, _texture: TextureId, _wrap: TextureWrap) {
        unimplemented!()
    }
    fn texture_resize(
        &mut self,
        _texture: TextureId,
        _width: u32,
        _height: u32,
        _bytes: Option<&[u8]>,
    ) {
        unimplemented!()
    }
    fn texture_read_pixels(&mut self, _texture: TextureId, _bytes: &mut [u8]) {
        unimplemented!()
    }
    fn texture_params(&self, texture: TextureId) -> TextureParams {
        let texture = &self.textures[texture.0];
        texture.params
    }
    unsafe fn texture_raw_id(&self, texture: TextureId) -> RawId {
        let texture = &self.textures[texture.0];
        RawId::Metal(texture.texture)
    }

    fn clear(
        &mut self,
        color: Option<(f32, f32, f32, f32)>,
        depth: Option<f32>,
        stencil: Option<i32>,
    ) {
        // TODO: begin_pass/end_pass works, but is far from optimal
        self.begin_default_pass(PassAction::Clear {
            color,
            depth,
            stencil,
        });
        self.end_render_pass();
    }

    fn new_render_pass(
        &mut self,
        color_img: TextureId,
        depth_img: Option<TextureId>,
    ) -> RenderPass {
        unsafe {
            let render_pass_desc =
                msg_send_![class!(MTLRenderPassDescriptor), renderPassDescriptor];
            msg_send_![render_pass_desc, retain];
            assert!(!render_pass_desc.is_null());
            let color_texture = self.textures[color_img.0].texture;
            let color_attachment = msg_send_![msg_send_![render_pass_desc, colorAttachments], objectAtIndexedSubscript:0];
            msg_send_![color_attachment, setTexture: color_texture];
            msg_send_![color_attachment, setLoadAction: MTLLoadAction::Clear];
            msg_send_![color_attachment, setStoreAction: MTLStoreAction::Store];

            if let Some(depth_img) = depth_img {
                let depth_texture = self.textures[depth_img.0].texture;

                let depth_attachment = msg_send_![render_pass_desc, depthAttachment];
                msg_send_![depth_attachment, setTexture: depth_texture];
                msg_send_![depth_attachment, setLoadAction: MTLLoadAction::Clear];
                msg_send_![depth_attachment, setStoreAction: MTLStoreAction::Store];
                msg_send_![depth_attachment, setClearDepth:1.];

                let stencil_attachment = msg_send_![render_pass_desc, stencilAttachment];
                msg_send_![stencil_attachment, setTexture: depth_texture];
            }
            let pass = RenderPassInternal {
                render_pass_desc,
                texture: color_img,
                _depth_texture: depth_img,
            };

            self.passes.push(pass);

            RenderPass(self.passes.len() - 1)
        }
    }

    fn delete_render_pass(&mut self, render_pass: RenderPass) {
        let render_pass = &self.passes[render_pass.0];
        unsafe {
            msg_send_![render_pass.render_pass_desc, release];
        }
    }

    fn render_pass_texture(&self, render_pass: RenderPass) -> TextureId {
        self.passes[render_pass.0].texture
    }

    fn new_buffer(&mut self, _: BufferType, _usage: BufferUsage, data: BufferSource) -> BufferId {
        let mut raw = [nil; BUFFERS_IN_ROTATION];
        let size = match &data {
            BufferSource::Slice(data) => data.size,
            BufferSource::Empty { size, .. } => *size,
        };
        for i in 0..BUFFERS_IN_ROTATION {
            let buffer: ObjcId = if let BufferSource::Slice(data) = &data {
                debug_assert!(data.is_slice);
                let size = data.size as u64;
                unsafe {
                    msg_send![self.device,
                              newBufferWithBytes:data.ptr
                              length:size
                              options:MTLResourceOptions::StorageModeShared]
                }
            } else {
                #[cfg(target_os = "macos")]
                let options = MTLResourceOptions::CPUCacheModeWriteCombined
                    | MTLResourceOptions::StorageModeManaged;
                #[cfg(target_os = "ios")]
                let options = MTLResourceOptions::CPUCacheModeWriteCombined;
                unsafe {
                    msg_send![self.device,
                              newBufferWithLength:size
                              options:options]
                }
            };

            unsafe {
                msg_send_![buffer, retain];
            }
            raw[i] = buffer;
        }
        let buffer = Buffer {
            raw,
            size: size as usize,
            value: 0,
            next_value: 0,
        };
        self.buffers.push(buffer);
        BufferId(self.buffers.len() - 1)
    }

    fn buffer_update(&mut self, buffer: BufferId, data: BufferSource) {
        let data = match data {
            BufferSource::Slice(data) => data,
            _ => panic!("buffer_update expects BufferSource::slice"),
        };
        let mut buffer = &mut self.buffers[buffer.0];
        assert!(data.size <= buffer.size);

        unsafe {
            let dest: *mut std::ffi::c_void = msg_send![buffer.raw[buffer.next_value], contents];
            std::ptr::copy(data.ptr, dest, data.size);

            #[cfg(target_os = "macos")]
            msg_send_![buffer.raw[buffer.next_value], didModifyRange:NSRange::new(0, data.size as u64)];
        }
        buffer.value = buffer.next_value;
    }

    fn new_shader(
        &mut self,
        shader: ShaderSource,
        _meta: ShaderMeta,
    ) -> Result<ShaderId, ShaderError> {
        unsafe {
            let shader = apple_util::str_to_nsstring(shader.metal_shader.unwrap());
            let mut error: ObjcId = nil;
            let library: ObjcId = msg_send![
                self.device,
                newLibraryWithSource: shader
                options:nil
                error: &mut error
            ];
            if library.is_null() {
                let description: ObjcId = msg_send![error, localizedDescription];
                let string = apple_util::nsstring_to_string(description);
                panic!("Shader {}", string);
            }

            let vertex_function: ObjcId = msg_send![library, newFunctionWithName: apple_util::str_to_nsstring("vertexShader")];
            assert!(!vertex_function.is_null());
            let fragment_function: ObjcId = msg_send![library, newFunctionWithName: apple_util::str_to_nsstring("fragmentShader")];
            assert!(!fragment_function.is_null());
            let shader = ShaderInternal {
                vertex_function,
                fragment_function,
            };
            self.shaders.push(shader);
            Ok(ShaderId(self.shaders.len() - 1))
        }
    }

    fn new_texture(
        &mut self,
        access: TextureAccess,
        bytes: Option<&[u8]>,
        params: TextureParams,
    ) -> TextureId {
        let descriptor = unsafe { msg_send_![class!(MTLTextureDescriptor), new] };
        // unsafe {
        //     msg_send_![descriptor, retain];
        // }
        unsafe {
            msg_send_![descriptor, setWidth:params.width as u64];
            msg_send_![descriptor, setHeight:params.height as u64];
            msg_send_![descriptor, setCpuCacheMode: MTLCPUCacheMode::DefaultCache];
            msg_send_![descriptor, setPixelFormat: MTLPixelFormat::from(params.format)];

            if access == TextureAccess::RenderTarget {
                if params.format != TextureFormat::Depth {
                    msg_send_![descriptor, setPixelFormat: MTLPixelFormat::RGBA8Unorm];
                }
                msg_send_![descriptor, setStorageMode: MTLStorageMode::Private];
                msg_send_![
                    descriptor,
                    setUsage: MTLTextureUsage::RenderTarget as u64
                        | MTLTextureUsage::ShaderRead as u64
                        | MTLTextureUsage::ShaderWrite as u64
                ];
            } else {
                #[cfg(target_os = "macos")]
                {
                    msg_send_![descriptor, setUsage: MTLTextureUsage::ShaderRead];
                    msg_send_![descriptor, setStorageMode: MTLStorageMode::Managed];
                    msg_send_![
                        descriptor,
                        setResourceOptions: MTLResourceOptions::StorageModeManaged
                    ];
                }
                #[cfg(target_os = "ios")]
                {
                    msg_send_![descriptor, setStorageMode: MTLStorageMode::Shared];
                    msg_send_![
                        descriptor,
                        setResourceOptions: MTLResourceOptions::StorageModeShared
                    ];
                }
            }
        };

        let texture = unsafe {
            let sampler_dsc = msg_send_![class!(MTLSamplerDescriptor), new];
            msg_send_![sampler_dsc, setMinFilter: MTLSamplerMinMagFilter::Linear];
            msg_send_![sampler_dsc, setMagFilter: MTLSamplerMinMagFilter::Linear];

            let sampler_state = msg_send_![self.device, newSamplerStateWithDescriptor: sampler_dsc];
            let raw_texture = msg_send_![self.device, newTextureWithDescriptor: descriptor];
            msg_send_![raw_texture, retain];
            self.textures.push(Texture {
                sampler: sampler_state,
                texture: raw_texture,
                params,
            });
            TextureId(self.textures.len() - 1)
        };

        if let Some(bytes) = bytes {
            assert_eq!(
                params.format.size(params.width, params.height) as usize,
                bytes.len()
            );

            self.texture_update_part(texture, 0, 0, params.width as _, params.height as _, bytes);
        }
        texture
    }

    fn texture_update_part(
        &mut self,
        texture: TextureId,
        x_offset: i32,
        y_offset: i32,
        width: i32,
        height: i32,
        bytes: &[u8],
    ) {
        let raw_texture = self.textures[texture.0].texture;
        let region = MTLRegion {
            origin: MTLOrigin {
                x: x_offset as u64,
                y: y_offset as u64,
                z: 0,
            },
            size: MTLSize {
                width: width as u64,
                height: height as u64,
                depth: 1,
            },
        };
        unsafe {
            msg_send_![raw_texture, replaceRegion:region
                       mipmapLevel:0
                       withBytes:bytes.as_ptr()
                       bytesPerRow:(width * 4) as u64];
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
        unsafe {
            let shader_internal = &self.shaders[shader.0];

            let vertex_descriptor: ObjcId =
                msg_send![class!(MTLVertexDescriptor), vertexDescriptor];

            let attribute = |i, buffer_index, offset, format: MTLVertexFormat| {
                let mtl_attribute_desc = msg_send_![
                    msg_send_![vertex_descriptor, attributes],
                    objectAtIndexedSubscript: i
                ];
                msg_send_![mtl_attribute_desc, setBufferIndex: buffer_index];
                msg_send_![mtl_attribute_desc, setOffset: offset];
                msg_send_![mtl_attribute_desc, setFormat: format];
            };
            let layout = |i, step_func: VertexStep, stride, step_rate| {
                let mtl_buffer_desc = msg_send_![
                    msg_send_![vertex_descriptor, layouts],
                    objectAtIndexedSubscript: i
                ];
                let step_func: MTLVertexStepFunction = step_func.into();
                msg_send_![mtl_buffer_desc, setStride: stride];
                msg_send_![mtl_buffer_desc, setStepFunction: step_func];
                msg_send_![mtl_buffer_desc, setStepRate: step_rate];
            };

            let mut offsets = [0u64; 50];
            for (i, a) in attributes.iter().enumerate() {
                let offset = &mut offsets[a.buffer_index];
                attribute(
                    i as u64,
                    a.buffer_index as u64 + 1,
                    *offset,
                    a.format.into(),
                );
                *offset += a.format.size_bytes() as u64;
            }
            for (i, buffer) in buffer_layout.iter().enumerate() {
                layout(
                    i as u64 + 1,
                    buffer.step_func,
                    if buffer.stride == 0 {
                        offsets[i]
                    } else {
                        buffer.stride as u64
                    },
                    buffer.step_rate as u64,
                );
            }

            let descriptor = msg_send_![class!(MTLRenderPipelineDescriptor), new];
            msg_send_![descriptor, setVertexFunction:shader_internal.vertex_function];
            msg_send_![descriptor, setFragmentFunction:shader_internal.fragment_function];
            msg_send_![descriptor, setVertexDescriptor: vertex_descriptor];
            let color_attachments = msg_send_![descriptor, colorAttachments];
            let color_attachment = msg_send_![color_attachments, objectAtIndexedSubscript: 0];

            let view_pixel_format: MTLPixelFormat = msg_send![self.view, colorPixelFormat];
            msg_send_![color_attachment, setPixelFormat: view_pixel_format];
            if let Some(color_blend) = params.color_blend {
                msg_send_![color_attachment, setBlendingEnabled: true];

                let BlendState {
                    equation: eq_rgb,
                    sfactor: src_rgb,
                    dfactor: dst_rgb,
                } = color_blend;
                let BlendState {
                    equation: eq_alpha,
                    sfactor: src_alpha,
                    dfactor: dst_alpha,
                } = color_blend;
                msg_send_![
                    color_attachment,
                    setRgbBlendOperation: MTLBlendOperation::from(eq_rgb)
                ];
                msg_send_![
                    color_attachment,
                    setAlphaBlendOperation: MTLBlendOperation::from(eq_alpha)
                ];
                msg_send_![
                    color_attachment,
                    setSourceRGBBlendFactor: MTLBlendFactor::from(src_rgb)
                ];
                msg_send_![
                    color_attachment,
                    setSourceRGBBlendFactor: MTLBlendFactor::from(src_rgb)
                ];
                msg_send_![
                    color_attachment,
                    setSourceAlphaBlendFactor: MTLBlendFactor::from(src_alpha)
                ];
                msg_send_![
                    color_attachment,
                    setDestinationRGBBlendFactor: MTLBlendFactor::from(dst_rgb)
                ];
                msg_send_![
                    color_attachment,
                    setDestinationAlphaBlendFactor: MTLBlendFactor::from(dst_alpha)
                ];
            }

            msg_send_![
                descriptor,
                setDepthAttachmentPixelFormat: MTLPixelFormat::Depth32Float_Stencil8
            ];
            msg_send_![
                descriptor,
                setStencilAttachmentPixelFormat: MTLPixelFormat::Depth32Float_Stencil8
            ];

            let mut error: ObjcId = nil;
            let pipeline_state: ObjcId = msg_send![
                self.device,
                newRenderPipelineStateWithDescriptor: descriptor
                error: &mut error
            ];
            if pipeline_state.is_null() {
                let description: ObjcId = msg_send![error, localizedDescription];
                let string = apple_util::nsstring_to_string(description);
                panic!("newRenderPipelineStateWithDescriptor error: {}", string);
            }

            let depth_stencil_desc = msg_send_![class!(MTLDepthStencilDescriptor), new];
            msg_send_![depth_stencil_desc, setDepthWriteEnabled: BOOL::from(params.depth_write)];
            msg_send_![depth_stencil_desc, setDepthCompareFunction: MTLCompareFunction::from(params.depth_test)];

            // if let Some(stencil_test) = params.stencil_test {
            //     let back_face_stencil_desc = StencilDescriptor::new();
            //     back_face_stencil_desc.set_stencil_compare_function(stencil_test.back.test_func.into());
            //     back_face_stencil_desc.set_stencil_failure_operation(stencil_test.back.fail_op.into());
            //     back_face_stencil_desc
            //         .set_depth_failure_operation(stencil_test.back.depth_fail_op.into());
            //     back_face_stencil_desc.set_read_mask(stencil_test.back.test_mask);
            //     back_face_stencil_desc.set_write_mask(stencil_test.back.write_mask);

            //     depth_stencil_desc.set_back_face_stencil(Some(back_face_stencil_desc.as_ref()));

            //     let front_face_stencil_desc = StencilDescriptor::new();
            //     front_face_stencil_desc
            //         .set_stencil_compare_function(stencil_test.front.test_func.into());
            //     front_face_stencil_desc
            //         .set_stencil_failure_operation(stencil_test.front.fail_op.into());
            //     front_face_stencil_desc
            //         .set_depth_failure_operation(stencil_test.front.depth_fail_op.into());
            //     front_face_stencil_desc.set_read_mask(stencil_test.front.test_mask);
            //     front_face_stencil_desc.set_write_mask(stencil_test.front.write_mask);

            //     depth_stencil_desc.set_front_face_stencil(Some(front_face_stencil_desc.as_ref()))
            // }

            let depth_stencil_state = msg_send_![
                self.device,
                newDepthStencilStateWithDescriptor: depth_stencil_desc
            ];

            let pipeline = PipelineInternal {
                pipeline_state,
                depth_stencil_state,
                //layout: buffer_layout.to_vec(),
                //attributes: vertex_layout,
                _shader: shader,
                //params,
            };

            self.pipelines.push(pipeline);

            Pipeline(self.pipelines.len() - 1)
        }
    }

    fn apply_pipeline(&mut self, pipeline: &Pipeline) {
        assert!(
            self.render_encoder.is_some(),
            "apply_pipeline before begin_pass"
        );
        let render_encoder = self.render_encoder.unwrap();

        unsafe {
            self.current_pipeline = Some(*pipeline);
            let pipeline = &self.pipelines[pipeline.0];

            msg_send_![render_encoder, setRenderPipelineState: pipeline.pipeline_state];
            msg_send_![render_encoder, setDepthStencilState:pipeline.depth_stencil_state];
            // render_encoder.set_front_facing_winding(pipeline.params.front_face_order.into());
            // render_encoder.set_cull_mode(pipeline.params.cull_face.into());
        }
    }

    fn apply_bindings(&mut self, bindings: &Bindings) {
        assert!(
            self.render_encoder.is_some(),
            "apply_bindings before begin_pass"
        );

        unsafe {
            let render_encoder = self.render_encoder.unwrap();
            for (index, vertex_buffer) in bindings.vertex_buffers.iter().enumerate() {
                let buffer = &mut self.buffers[vertex_buffer.0];
                let () = msg_send![render_encoder,
                                   setVertexBuffer:buffer.raw[buffer.value]
                                   offset:0
                                   atIndex:(index + 1) as u64];
                buffer.next_value = buffer.value + 1;
            }
            let mut index_buffer = &mut self.buffers[bindings.index_buffer.0];
            self.index_buffer = Some(index_buffer.raw[index_buffer.value]);
            index_buffer.next_value = index_buffer.value + 1;

            let img_count = bindings.images.len();
            if img_count > 0 {
                for (n, img) in bindings.images.iter().enumerate() {
                    let Texture {
                        sampler, texture, ..
                    } = self.textures[img.0];
                    msg_send_![render_encoder, setFragmentSamplerState:sampler
                               atIndex:n
                    ];
                    msg_send_![render_encoder, setFragmentTexture:texture
                               atIndex:n
                    ];
                }
            }
        }
    }

    fn apply_uniforms_from_bytes(&mut self, uniform_ptr: *const u8, size: usize) {
        assert!(
            self.current_pipeline.is_some(),
            "apply_uniforms before apply_pipeline"
        );
        assert!(
            self.render_encoder.is_some(),
            "apply_uniforms before begin_pass"
        );

        let render_encoder = self.render_encoder.unwrap();

        self.current_frame_index = (self.current_frame_index + 1) % NUM_INFLIGHT_FRAMES;

        assert!(size < MAX_UNIFORM_BUFFER_SIZE as usize);

        assert!(self.current_ub_offset < MAX_UNIFORM_BUFFER_SIZE);

        let buffer = self.uniform_buffers[self.current_frame_index];
        unsafe {
            let dest: *mut std::ffi::c_void = msg_send![buffer, contents];
            std::ptr::copy(
                uniform_ptr as _,
                dest.add(self.current_ub_offset as usize),
                size,
            );

            #[cfg(target_os = "macos")]
            msg_send_![buffer, didModifyRange:NSRange::new(0, size as u64)];

            msg_send_![render_encoder,
                       setVertexBuffer:buffer
                       offset:self.current_ub_offset
                       atIndex:0];
            msg_send_![render_encoder,
                       setFragmentBuffer:buffer
                       offset:self.current_ub_offset
                       atIndex:0];
        }
        self.current_ub_offset = roundup_ub_buffer(self.current_ub_offset + size as u64);
    }

    fn begin_default_pass(&mut self, action: PassAction) {
        self.begin_pass(None, action)
    }

    fn begin_pass(&mut self, pass: Option<RenderPass>, action: PassAction) {
        unsafe {
            if self.command_buffer.is_none() {
                self.command_buffer = Some(msg_send![self.command_queue, commandBuffer]);
            }

            let (descriptor, _, _) = match pass {
                None => {
                    let (screen_width, screen_height) = crate::window::screen_size();
                    (
                        {
                            let a = msg_send_![self.view, currentRenderPassDescriptor];
                            //msg_send_![a, retain];
                            a
                        },
                        screen_width as f64,
                        screen_height as f64,
                    )
                }
                Some(pass) => {
                    let pass_internal = &self.passes[pass.0];
                    (
                        pass_internal.render_pass_desc,
                        self.textures[pass_internal.texture.0].params.width as f64,
                        self.textures[pass_internal.texture.0].params.height as f64,
                    )
                }
            };
            assert!(!descriptor.is_null());

            let color_attachments = msg_send_![descriptor, colorAttachments];
            let color_attachment = msg_send_![color_attachments, objectAtIndexedSubscript: 0];

            msg_send_![color_attachment, setStoreAction: MTLStoreAction::Store];

            match action {
                PassAction::Clear { color, .. } => {
                    msg_send_![color_attachment, setLoadAction: MTLLoadAction::Clear];

                    if let Some(color) = color {
                        msg_send_![color_attachment, setClearColor:MTLClearColor::new(color.0 as _, color.1 as _, color.2 as _, color.3 as _)];
                    }
                }
                PassAction::Nothing => {
                    msg_send_![color_attachment, setLoadAction: MTLLoadAction::Load];
                }
            }

            let render_encoder = msg_send_![
                self.command_buffer.unwrap(),
                renderCommandEncoderWithDescriptor: descriptor
            ];

            // render_encoder.set_viewport(MTLViewport {
            //     originX: 0.0,
            //     originY: 0.0,
            //     width: w,
            //     height: h,
            //     znear: 0.0,
            //     zfar: 1.0,
            // });
            // render_encoder.set_scissor_rect(MTLScissorRect {
            //     x: 0,
            //     y: 0,
            //     width: w as u64,
            //     height: h as u64,
            // });

            self.render_encoder = Some(render_encoder);
        }
    }

    fn end_render_pass(&mut self) {
        assert!(
            self.render_encoder.is_some(),
            "end_render_pass unpaired with begin_render_pass!"
        );

        let render_encoder = self.render_encoder.unwrap();
        unsafe { msg_send_!(render_encoder, endEncoding) };

        self.render_encoder = None;
        self.index_buffer = None;
    }

    fn draw(&self, base_element: i32, num_elements: i32, num_instances: i32) {
        assert!(self.render_encoder.is_some(), "draw before begin_pass!");
        let render_encoder = self.render_encoder.unwrap();
        assert!(self.index_buffer.is_some());
        let index_buffer = self.index_buffer.unwrap();

        assert!(base_element == 0); // TODO: figure indexBufferOffset/baseVertex
        unsafe {
            msg_send_![render_encoder, drawIndexedPrimitives:MTLPrimitiveType::Triangle
                       indexCount:num_elements as u64
                       indexType:MTLIndexType::UInt16
                       indexBuffer:index_buffer
                       indexBufferOffset:0
                       instanceCount:num_instances as u64
                       baseVertex:0
                       baseInstance:0
            ];
        }
    }

    fn commit_frame(&mut self) {
        unsafe {
            assert!(!self.command_queue.is_null());
            let drawable: ObjcId = msg_send!(self.view, currentDrawable);
            //msg_send_![drawable, retain];
            msg_send_![self.command_buffer.unwrap(), presentDrawable: drawable];
            msg_send_![self.command_buffer.unwrap(), commit];
            msg_send_![self.command_buffer.unwrap(), waitUntilCompleted];
        }
        for buffer in &mut self.buffers {
            buffer.next_value = 0;
        }
        self.current_ub_offset = 0;
        self.current_pipeline = None;
        self.command_buffer = None;
        if (self.current_frame_index + 1) >= 3 {
            self.current_frame_index = 0;
        }
    }
}
