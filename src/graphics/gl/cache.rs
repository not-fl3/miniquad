use std::cell::Cell;

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
    pub gl_pass_as_float: bool,
}

#[derive(Default, Copy, Clone)]
pub struct CachedAttribute {
    pub attribute: VertexAttributeInternal,
    pub gl_vbuf: GLuint,
}

#[derive(Clone, Copy)]
pub struct CachedTexture {
    // GL_TEXTURE_2D or GL_TEXTURE_CUBEMAP
    pub target: GLuint,
    pub texture: GLuint,
}

pub struct GlCache {
    pub stored_target: GLuint,
    pub stored_texture: GLuint,
    /// The index buffer draws read, as the bindings last applied it.
    pub index_buffer: GLuint,
    pub index_type: Option<u32>,
    /// Another index buffer, bound since to upload into it, which the next
    /// draw binds `index_buffer` back over.
    pub index_buffer_uploading: Cell<Option<GLuint>>,
    pub vertex_buffer: GLuint,
    pub textures: [CachedTexture; MAX_SHADERSTAGE_IMAGES],
    pub cur_pipeline: Option<Pipeline>,
    pub cur_pass: Option<RenderPass>,
    pub color_blend: Option<BlendState>,
    pub alpha_blend: Option<BlendState>,
    pub stencil: Option<StencilState>,
    pub color_write: ColorMask,
    pub cull_face: CullFace,
    pub program: GLuint,
    /// `None` while the depth test is off.
    pub depth_test: Option<Comparison>,
    pub front_face: FrontFaceOrder,
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
            let bound = self
                .index_buffer_uploading
                .take()
                .unwrap_or(self.index_buffer);
            if bound != buffer {
                unsafe {
                    glBindBuffer(target, buffer);
                }
            }
            self.index_buffer = buffer;
            self.index_type = index_type;
        }
    }

    /// Binds `buffer` to upload into it, without binding back what was
    /// bound: draws don't read the vertex buffer binding, and the next draw
    /// binds back the index buffer the bindings applied.
    pub fn bind_buffer_to_upload(&mut self, target: GLenum, buffer: GLuint) {
        if target == GL_ARRAY_BUFFER {
            self.bind_buffer(target, buffer, None);
        } else {
            let bound = self
                .index_buffer_uploading
                .get()
                .unwrap_or(self.index_buffer);
            if bound != buffer {
                unsafe {
                    glBindBuffer(target, buffer);
                }
            }
            self.index_buffer_uploading
                .set((buffer != self.index_buffer).then_some(buffer));
        }
    }

    /// Binds back the index buffer the bindings applied, if an upload has
    /// bound another since.
    pub fn bind_index_buffer_to_draw(&self) {
        if self.index_buffer_uploading.take().is_some() {
            unsafe {
                glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, self.index_buffer);
            }
        }
    }

    pub fn bind_texture(&mut self, slot_index: usize, target: GLuint, texture: GLuint) {
        unsafe {
            glActiveTexture(GL_TEXTURE0 + slot_index as GLuint);
            if self.textures[slot_index].target != target
                || self.textures[slot_index].texture != texture
            {
                let target = if target == 0 { GL_TEXTURE_2D } else { target };
                glBindTexture(target, texture);
                self.textures[slot_index] = CachedTexture { target, texture };
            }
        }
    }

    pub fn store_texture_binding(&mut self, slot_index: usize) {
        self.stored_target = self.textures[slot_index].target;
        self.stored_texture = self.textures[slot_index].texture;
    }

    pub fn restore_texture_binding(&mut self, slot_index: usize) {
        self.bind_texture(slot_index, self.stored_target, self.stored_texture);
    }

    pub fn clear_buffer_bindings(&mut self) {
        self.bind_buffer(GL_ARRAY_BUFFER, 0, None);
        self.vertex_buffer = 0;

        self.bind_buffer(GL_ELEMENT_ARRAY_BUFFER, 0, None);
        self.index_buffer = 0;
    }

    pub fn clear_texture_bindings(&mut self) {
        for ix in 0..MAX_SHADERSTAGE_IMAGES {
            if self.textures[ix].texture != 0 {
                self.bind_texture(ix, self.textures[ix].target, 0);
                self.textures[ix] = CachedTexture {
                    target: 0,
                    texture: 0,
                };
            }
        }
    }

    pub fn clear_vertex_attributes(&mut self) {
        for attr_index in 0..MAX_VERTEX_ATTRIBUTES {
            let cached_attr = &mut self.attributes[attr_index];

            if cached_attr.is_some() {
                unsafe { glDisableVertexAttribArray(attr_index as GLuint) };
            }
            *cached_attr = None;
        }
    }
}
