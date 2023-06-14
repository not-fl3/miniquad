use crate::graphics::*;

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct VertexAttributeInternal {
    pub attr_loc: GLuint,
    pub size: i32,
    pub type_: GLuint,
    pub offset: i64,
    pub stride: i32,
    pub buffer_index: usize,
    pub divisor: i32,
}

#[derive(Default, Copy, Clone)]
pub struct CachedAttribute {
    pub attribute: VertexAttributeInternal,
    pub gl_vbuf: GLuint,
}

pub struct GlCache {
    pub stored_index_buffer: GLuint,
    pub stored_index_type: Option<u32>,
    pub stored_vertex_buffer: GLuint,
    pub stored_texture: GLuint,
    pub index_buffer: GLuint,
    pub index_type: Option<u32>,
    pub vertex_buffer: GLuint,
    pub textures: [GLuint; MAX_SHADERSTAGE_IMAGES],
    pub cur_pipeline: Option<Pipeline>,
    pub color_blend: Option<BlendState>,
    pub alpha_blend: Option<BlendState>,
    pub stencil: Option<StencilState>,
    pub color_write: ColorMask,
    pub cull_face: CullFace,
    pub attributes: [Option<CachedAttribute>; MAX_VERTEX_ATTRIBUTES],
}

impl GlCache {
    pub fn bind_buffer(&mut self, target: GLenum, buffer: GLuint, index_type: Option<u32>) {
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

    pub fn store_buffer_binding(&mut self, target: GLenum) {
        if target == GL_ARRAY_BUFFER {
            self.stored_vertex_buffer = self.vertex_buffer;
        } else {
            self.stored_index_buffer = self.index_buffer;
            self.stored_index_type = self.index_type;
        }
    }

    pub fn restore_buffer_binding(&mut self, target: GLenum) {
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

    pub fn bind_texture(&mut self, slot_index: usize, texture: GLuint) {
        unsafe {
            glActiveTexture(GL_TEXTURE0 + slot_index as GLuint);
            if self.textures[slot_index] != texture {
                glBindTexture(GL_TEXTURE_2D, texture);
                self.textures[slot_index] = texture;
            }
        }
    }

    pub fn store_texture_binding(&mut self, slot_index: usize) {
        self.stored_texture = self.textures[slot_index];
    }

    pub fn restore_texture_binding(&mut self, slot_index: usize) {
        self.bind_texture(slot_index, self.stored_texture);
    }

    pub fn clear_buffer_bindings(&mut self) {
        self.bind_buffer(GL_ARRAY_BUFFER, 0, None);
        self.vertex_buffer = 0;

        self.bind_buffer(GL_ELEMENT_ARRAY_BUFFER, 0, None);
        self.index_buffer = 0;
    }

    pub fn clear_texture_bindings(&mut self) {
        for ix in 0..MAX_SHADERSTAGE_IMAGES {
            if self.textures[ix] != 0 {
                self.bind_texture(ix, 0);
                self.textures[ix] = 0;
            }
        }
    }

    pub fn clear_vertex_attributes(&mut self) {
        for attr_index in 0..MAX_VERTEX_ATTRIBUTES {
            let cached_attr = &mut self.attributes[attr_index];
            *cached_attr = None;
        }
    }
}
