use crate::{sapp::*, Context};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Texture {
    pub(crate) texture: GLuint,
    pub width: u32,
    pub height: u32,
    pub format: TextureFormat,
}

impl Texture {
    pub fn empty() -> Texture {
        Texture {
            texture: 0,
            width: 0,
            height: 0,
            format: TextureFormat::RGBA8,
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
}

/// List of all the possible internal texture formats for storing texture array in GPU memory
/// The list is built by intersection of texture formats supported by 3.3 core profile and webgl1.
#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TextureFormat {
    RGB8,
    RGBA8,
    Depth,
}

/// List of all the possible pixel formats of input data when uploading to texture.
/// The list is built by intersection of texture formats supported by 3.3 core profile and webgl1.
#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PixelFormat {
    RGB8,
    RGBA8,
    Depth,
    Alpha,
}

impl From<TextureFormat> for GLenum {
    fn from(format: TextureFormat) -> Self {
        match format {
            TextureFormat::RGB8 => GL_RGB,
            TextureFormat::RGBA8 => GL_RGBA,
            TextureFormat::Depth => GL_DEPTH_COMPONENT,
        }
    }
}

impl TextureFormat {
    /// Returns the size in bytes of texture with `dimensions`.
    pub fn size(self, width: u32, height: u32) -> u32 {
        let square = width * height;

        match self {
            TextureFormat::RGB8 => 3 * square,
            TextureFormat::RGBA8 => 4 * square,
            TextureFormat::Depth => 2 * square,
        }
    }
}

impl PixelFormat {
    /// Returns the size in bytes of texture with `dimensions`.
    pub fn size(self, width: u32, height: u32) -> u32 {
        let square = width * height;

        match self {
            PixelFormat::RGB8 => 3 * square,
            PixelFormat::RGBA8 => 4 * square,
            PixelFormat::Depth => 2 * square,
            PixelFormat::Alpha => 1 * square,
        }
    }
}

/// Convert from `PixelFormat` into (format, pixel_tyle, row_alignment)
impl From<PixelFormat> for (GLenum, GLenum, GLenum) {
    fn from(format: PixelFormat) -> Self {
        match format {
            PixelFormat::RGB8 => (GL_RGB, GL_UNSIGNED_BYTE, 4),
            PixelFormat::RGBA8 => (GL_RGBA, GL_UNSIGNED_BYTE, 4),
            PixelFormat::Depth => (GL_DEPTH_COMPONENT, GL_UNSIGNED_SHORT, 4),
            PixelFormat::Alpha => (GL_RED, GL_UNSIGNED_BYTE, 1),
        }
    }
}

impl From<TextureFormat> for PixelFormat {
    fn from(format: TextureFormat) -> Self {
        match format {
            TextureFormat::RGB8 => PixelFormat::RGB8,
            TextureFormat::RGBA8 => PixelFormat::RGBA8,
            TextureFormat::Depth => PixelFormat::Depth,
        }
    }
}

impl Default for TextureParams {
    fn default() -> Self {
        TextureParams {
            format: TextureFormat::RGBA8,
            wrap: TextureWrap::Clamp,
            filter: FilterMode::Linear,
            width: 0,
            height: 0,
        }
    }
}

/// Sets the wrap parameter for texture.
#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TextureWrap {
    /// Samples at coord x + 1 map to coord x.
    Repeat,
    /// Samples at coord x + 1 map to coord 1 - x.
    Mirror,
    /// Samples at coord x + 1 map to coord 1.
    Clamp,
    /// Same as Mirror, but only for one repetition.
    MirrorClamp,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FilterMode {
    Linear = GL_LINEAR as isize,
    Nearest = GL_NEAREST as isize,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TextureAccess {
    /// Used as read-only from GPU
    Static,
    /// Can be written to from GPU
    RenderTarget,
}

#[derive(Debug, Copy, Clone)]
pub struct TextureParams {
    pub format: TextureFormat,
    pub wrap: TextureWrap,
    pub filter: FilterMode,
    pub width: u32,
    pub height: u32,
}

impl Texture {
    /// Shorthand for `new(ctx, TextureAccess::RenderTarget, params)`
    pub fn new_render_texture(ctx: &mut Context, params: TextureParams) -> Texture {
        Self::new(
            ctx,
            TextureAccess::RenderTarget,
            None,
            params.format.into(),
            params,
        )
    }

    pub fn new(
        ctx: &mut Context,
        _access: TextureAccess,
        bytes: Option<&[u8]>,
        pixel_format: PixelFormat,
        params: TextureParams,
    ) -> Texture {
        if let Some(bytes_data) = bytes {
            assert_eq!(
                params.format.size(params.width, params.height),
                bytes_data.len() as u32
            );
        }

        let internal_format: GLuint = params.format.into();
        let (format, pixel_type, row_alignment) = pixel_format.into();

        ctx.cache.store_texture_binding(0);

        let mut texture: GLuint = 0;

        unsafe {
            glGenTextures(1, &mut texture as *mut _);
            ctx.cache.bind_texture(0, texture);
            if row_alignment != 4 {
                glPixelStorei(GL_UNPACK_ALIGNMENT, row_alignment as _);
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

            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_CLAMP_TO_EDGE as i32);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_CLAMP_TO_EDGE as i32);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR as i32);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR as i32);
            if row_alignment != 4 {
                glPixelStorei(GL_UNPACK_ALIGNMENT, 4);
            }
        }
        ctx.cache.restore_texture_binding(0);

        Texture {
            texture,
            width: params.width,
            height: params.height,
            format: params.format,
        }
    }

    /// Upload texture to GPU with given TextureParams
    pub fn from_data_and_format(
        ctx: &mut Context,
        bytes: &[u8],
        pixel_format: PixelFormat,
        params: TextureParams,
    ) -> Texture {
        Self::new(
            ctx,
            TextureAccess::Static,
            Some(bytes),
            pixel_format,
            params,
        )
    }

    /// Upload RGBA8 texture to GPU
    pub fn from_rgba8(ctx: &mut Context, width: u16, height: u16, bytes: &[u8]) -> Texture {
        assert_eq!(width as usize * height as usize * 4, bytes.len());

        Self::from_data_and_format(
            ctx,
            bytes,
            PixelFormat::RGBA8,
            TextureParams {
                width: width as _,
                height: height as _,
                format: TextureFormat::RGBA8,
                wrap: TextureWrap::Clamp,
                filter: FilterMode::Linear,
            },
        )
    }

    pub fn set_filter(&self, ctx: &mut Context, filter: FilterMode) {
        ctx.cache.store_texture_binding(0);
        ctx.cache.bind_texture(0, self.texture);
        unsafe {
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, filter as i32);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, filter as i32);
        }
        ctx.cache.restore_texture_binding(0);
    }

    /// Update whole texture content
    /// Note: non rgba8 textures on the GPU are not yet supported. However, you can upload alpha data to RGBA texture by setting
    /// pixel_format to PixelFormat::Alpha
    /// To upload alpha data to RGBA textures, use update_texture_part() instead
    pub fn update(&self, ctx: &mut Context, pixels: &[u8], pixel_format: PixelFormat) {
        assert_eq!(
            pixel_format.size(self.width, self.height),
            pixels.len() as u32
        );

        self.update_texture_part(
            ctx,
            0 as _,
            0 as _,
            self.width as _,
            self.height as _,
            pixels,
            pixel_format,
        )
    }

    pub fn update_texture_part(
        &self,
        ctx: &mut Context,
        x_offset: i32,
        y_offset: i32,
        width: i32,
        height: i32,
        pixels: &[u8],
        pixel_format: PixelFormat,
    ) {
        assert_eq!(
            pixel_format.size(width as _, height as _),
            pixels.len() as u32
        );
        assert!(x_offset + width <= self.width as _);
        assert!(y_offset + height <= self.height as _);

        ctx.cache.store_texture_binding(0);
        ctx.cache.bind_texture(0, self.texture);

        let (format, pixel_type, row_alignment) = pixel_format.into();

        unsafe {
            if row_alignment != 4 {
                glPixelStorei(GL_UNPACK_ALIGNMENT, row_alignment as _);
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
                pixels.as_ptr() as *const _,
            );

            if row_alignment != 4 {
                glPixelStorei(GL_UNPACK_ALIGNMENT, 4);
            }
        }

        ctx.cache.restore_texture_binding(0);
    }
}
