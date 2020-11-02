use crate::graphics::metal::DEFAULT_FRAMEBUFFER_PIXEL_FORMAT;
use crate::{
    Context, FilterMode, GraphicTexture, TextureAccess, TextureFormat, TextureParams, TextureWrap,
};
use metal::{
    MTLCPUCacheMode, MTLOrigin, MTLPixelFormat, MTLRegion, MTLResourceOptions,
    MTLSamplerMinMagFilter, MTLSize, MTLStorageMode, MTLTextureUsage, SamplerDescriptor,
    TextureDescriptor,
};
use metal_rs as metal;

// https://developer.apple.com/metal/Metal-Feature-Set-Tables.pdf
const MAX_TEXTURE_SIZE: u32 = 8192;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Texture {
    pub(crate) texture: usize,
    pub(crate) sampler: usize,
    pub width: u32,
    pub height: u32,
    pub format: TextureFormat,
}

impl GraphicTexture for Texture {
    fn new_render_texture(ctx: &mut Context, params: TextureParams) -> Texture {
        Self::new(ctx, TextureAccess::RenderTarget, None, params)
    }

    fn new(
        ctx: &mut Context,
        access: TextureAccess,
        bytes: Option<&[u8]>,
        params: TextureParams,
    ) -> Texture {
        assert!(params.height < MAX_TEXTURE_SIZE);
        assert!(params.width < MAX_TEXTURE_SIZE);

        if let Some(bytes_data) = bytes {
            assert_eq!(
                params.format.size(params.width, params.height) as usize,
                bytes_data.len()
            );
        }

        let texture_dsc = TextureDescriptor::new();
        texture_dsc.set_width(params.width as u64);
        texture_dsc.set_height(params.height as u64);
        texture_dsc.set_pixel_format(params.format.into());

        if access == TextureAccess::RenderTarget {
            if params.format != TextureFormat::Depth {
                texture_dsc.set_pixel_format(DEFAULT_FRAMEBUFFER_PIXEL_FORMAT);
            }
            texture_dsc.set_cpu_cache_mode(MTLCPUCacheMode::DefaultCache);
            texture_dsc.set_resource_options(MTLResourceOptions::StorageModePrivate);
            texture_dsc.set_storage_mode(MTLStorageMode::Private);
            texture_dsc.set_usage(
                MTLTextureUsage::RenderTarget
                    | MTLTextureUsage::ShaderRead
                    | MTLTextureUsage::ShaderWrite,
            );
        }

        let sampler_dsc = SamplerDescriptor::new();
        sampler_dsc.set_min_filter(params.filter.into());
        sampler_dsc.set_mag_filter(params.filter.into());

        let sampler_state = ctx.device.new_sampler(&sampler_dsc);

        let texture_id = ctx.cache.textures.len();

        let raw_texture = ctx.device.new_texture(&texture_dsc);

        ctx.cache.textures.push(raw_texture);
        ctx.cache.samplers.push(sampler_state);

        let texture = Texture {
            texture: texture_id,
            sampler: texture_id,
            width: params.width,
            height: params.height,
            format: params.format,
        };

        if let Some(bytes_data) = bytes {
            texture.update_texture_part(
                ctx,
                0,
                0,
                params.width as i32,
                params.height as i32,
                &bytes_data,
            );
        }

        texture
    }

    fn from_data_and_format(ctx: &mut Context, bytes: &[u8], params: TextureParams) -> Texture {
        Self::new(ctx, TextureAccess::Static, Some(bytes), params)
    }

    fn from_rgba8(ctx: &mut Context, width: u16, height: u16, bytes: &[u8]) -> Texture {
        assert_eq!(width as usize * height as usize * 4, bytes.len());

        Self::from_data_and_format(
            ctx,
            bytes,
            TextureParams {
                width: width as _,
                height: height as _,
                format: TextureFormat::RGBA8,
                wrap: TextureWrap::Clamp,
                filter: FilterMode::Linear,
            },
        )
    }

    fn set_filter(&self, ctx: &mut Context, filter: FilterMode) {
        todo!()
    }

    fn resize(&mut self, ctx: &mut Context, width: u32, height: u32, bytes: Option<&[u8]>) {
        todo!()
    }

    fn update(&self, ctx: &mut Context, bytes: &[u8]) {
        todo!()
    }

    fn update_texture_part(
        &self,
        ctx: &mut Context,
        x_offset: i32,
        y_offset: i32,
        width: i32,
        height: i32,
        bytes: &[u8],
    ) {
        let raw_texture = &ctx.cache.textures[self.texture];
        raw_texture.replace_region(
            MTLRegion {
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
            },
            0,
            bytes.as_ptr() as *const std::ffi::c_void,
            (width * 4) as u64,
        );
    }

    fn read_pixels(&self, bytes: &mut [u8]) {
        todo!();
    }

    #[inline]
    fn size(&self, width: u32, height: u32) -> usize {
        self.format.size(width, height) as usize
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

impl From<FilterMode> for MTLSamplerMinMagFilter {
    fn from(filter: FilterMode) -> Self {
        match filter {
            FilterMode::Linear => MTLSamplerMinMagFilter::Linear,
            FilterMode::Nearest => MTLSamplerMinMagFilter::Nearest,
        }
    }
}
