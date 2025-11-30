//! IME Input Test Example with Text Rendering
//!
//! Demonstrates:
//! - Text input box with visible text using fontdue
//! - IME candidate window positioning using window::set_ime_position()
//! - Chinese/Japanese/Korean input support
//!
//! Click on an input box to focus it, then type with your IME.

use fontdue::{Font, FontSettings};
use miniquad::*;
use std::collections::HashMap;
use std::io::Write;

// Use Windows system font for Chinese support
#[cfg(target_os = "windows")]
const FONT_PATH: &str = "C:\\Windows\\Fonts\\msyh.ttc"; // Microsoft YaHei

#[cfg(not(target_os = "windows"))]
const FONT_PATH: &str = "/usr/share/fonts/truetype/noto/NotoSansCJK-Regular.ttc";

// ============================================================================
// Text Renderer using fontdue
// ============================================================================

struct GlyphInfo {
    uv: [f32; 4], // x, y, w, h in UV coords
    size: [f32; 2], // width, height in pixels
    offset: [f32; 2], // xmin, ymin
    advance: f32,
}

struct TextRenderer {
    font: Font,
    font_size: f32,
    cache: HashMap<char, GlyphInfo>,
    atlas: TextureId,
    atlas_data: Vec<u8>,
    atlas_size: u32,
    cursor_x: u32,
    cursor_y: u32,
    row_h: u32,
    dirty: bool,
}

impl TextRenderer {
    fn new(ctx: &mut Box<dyn RenderingBackend>, font_size: f32) -> Self {
        let font_data = std::fs::read(FONT_PATH).expect("Failed to load font file");
        let font = Font::from_bytes(font_data.as_slice(), FontSettings::default())
            .expect("Failed to parse font");
        
        let atlas_size = 1024u32;
        let atlas_data = vec![0u8; (atlas_size * atlas_size * 4) as usize];
        let atlas = ctx.new_texture_from_rgba8(atlas_size as u16, atlas_size as u16, &atlas_data);

        Self {
            font, font_size, cache: HashMap::new(), atlas, atlas_data,
            atlas_size, cursor_x: 0, cursor_y: 0, row_h: 0, dirty: false,
        }
    }

    fn cache_char(&mut self, ch: char) {
        if self.cache.contains_key(&ch) { return; }
        
        let (m, bmp) = self.font.rasterize(ch, self.font_size);
        if m.width == 0 || m.height == 0 {
            self.cache.insert(ch, GlyphInfo {
                uv: [0.0; 4], size: [0.0; 2], offset: [0.0; 2], advance: m.advance_width,
            });
            return;
        }

        if self.cursor_x + m.width as u32 > self.atlas_size {
            self.cursor_x = 0;
            self.cursor_y += self.row_h + 1;
            self.row_h = 0;
        }
        if self.cursor_y + m.height as u32 > self.atlas_size {
            eprintln!("Atlas full!");
            return;
        }

        for y in 0..m.height {
            for x in 0..m.width {
                let src = y * m.width + x;
                let dx = self.cursor_x + x as u32;
                let dy = self.cursor_y + y as u32;
                let dst = ((dy * self.atlas_size + dx) * 4) as usize;
                self.atlas_data[dst..dst+3].copy_from_slice(&[255, 255, 255]);
                self.atlas_data[dst + 3] = bmp[src];
            }
        }

        let s = self.atlas_size as f32;
        self.cache.insert(ch, GlyphInfo {
            uv: [self.cursor_x as f32 / s, self.cursor_y as f32 / s,
                 m.width as f32 / s, m.height as f32 / s],
            size: [m.width as f32, m.height as f32],
            offset: [m.xmin as f32, m.ymin as f32],
            advance: m.advance_width,
        });

        self.cursor_x += m.width as u32 + 1;
        self.row_h = self.row_h.max(m.height as u32);
        self.dirty = true;
    }

    fn flush(&mut self, ctx: &mut Box<dyn RenderingBackend>) {
        if self.dirty {
            ctx.texture_update(self.atlas, &self.atlas_data);
            self.dirty = false;
        }
    }

    fn measure(&mut self, text: &str) -> f32 {
        text.chars().map(|c| {
            self.cache_char(c);
            self.cache.get(&c).map(|g| g.advance).unwrap_or(0.0)
        }).sum()
    }
}

// ============================================================================
// Input Box
// ============================================================================

struct InputBox {
    x: f32, y: f32, w: f32, h: f32,
    text: String,
    cursor: usize,
    focused: bool,
}

impl InputBox {
    fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self { x, y, w, h, text: String::new(), cursor: 0, focused: false }
    }
    fn hit(&self, px: f32, py: f32) -> bool {
        px >= self.x && px <= self.x + self.w && py >= self.y && py <= self.y + self.h
    }
    fn cursor_x(&self, tr: &mut TextRenderer) -> f32 {
        let before: String = self.text.chars().take(self.cursor).collect();
        self.x + 6.0 + tr.measure(&before)
    }
    fn insert(&mut self, ch: char) {
        let pos = self.text.char_indices().nth(self.cursor).map(|(i,_)|i).unwrap_or(self.text.len());
        self.text.insert(pos, ch);
        self.cursor += 1;
    }
    fn backspace(&mut self) {
        if self.cursor > 0 {
            let start = self.text.char_indices().nth(self.cursor - 1).map(|(i,_)|i).unwrap_or(0);
            let end = self.text.char_indices().nth(self.cursor).map(|(i,_)|i).unwrap_or(self.text.len());
            self.text.replace_range(start..end, "");
            self.cursor -= 1;
        }
    }
    fn left(&mut self) { if self.cursor > 0 { self.cursor -= 1; } }
    fn right(&mut self) { if self.cursor < self.text.chars().count() { self.cursor += 1; } }
}

// ============================================================================
// Vertices
// ============================================================================

#[repr(C)]
#[derive(Clone, Copy)]
struct ColorVert { pos: [f32; 2], color: [f32; 4] }

#[repr(C)]
#[derive(Clone, Copy)]
struct TextVert { pos: [f32; 2], uv: [f32; 2], color: [f32; 4] }

// ============================================================================
// Stage
// ============================================================================

struct Stage {
    boxes: Vec<InputBox>,
    focus: Option<usize>,
    color_pl: Pipeline,
    color_bind: Bindings,
    text_pl: Pipeline,
    text_bind: Bindings,
    tr: TextRenderer,
    ctx: Box<dyn RenderingBackend>,
    dpi: f32,
}

impl Stage {
    fn new() -> Self {
        let mut ctx = window::new_rendering_backend();
        let dpi = window::dpi_scale();

        // Color pipeline
        let cs = ctx.new_shader(ShaderSource::Glsl { vertex: COLOR_VS, fragment: COLOR_FS },
            ShaderMeta { images: vec![], uniforms: UniformBlockLayout { uniforms: vec![] } }).unwrap();
        let color_pl = ctx.new_pipeline(&[BufferLayout::default()],
            &[VertexAttribute::new("in_pos", VertexFormat::Float2),
              VertexAttribute::new("in_color", VertexFormat::Float4)], cs,
            PipelineParams { color_blend: Some(BlendState::new(Equation::Add,
                BlendFactor::Value(BlendValue::SourceAlpha),
                BlendFactor::OneMinusValue(BlendValue::SourceAlpha))), ..Default::default() });
        let cvb = ctx.new_buffer(BufferType::VertexBuffer, BufferUsage::Stream, BufferSource::empty::<ColorVert>(1024));
        let cib = ctx.new_buffer(BufferType::IndexBuffer, BufferUsage::Stream, BufferSource::empty::<u16>(2048));
        let color_bind = Bindings { vertex_buffers: vec![cvb], index_buffer: cib, images: vec![] };

        // Text renderer & pipeline
        let tr = TextRenderer::new(&mut ctx, 22.0);
        let ts = ctx.new_shader(ShaderSource::Glsl { vertex: TEXT_VS, fragment: TEXT_FS },
            ShaderMeta { images: vec!["tex".into()], uniforms: UniformBlockLayout { uniforms: vec![] } }).unwrap();
        let text_pl = ctx.new_pipeline(&[BufferLayout::default()],
            &[VertexAttribute::new("in_pos", VertexFormat::Float2),
              VertexAttribute::new("in_uv", VertexFormat::Float2),
              VertexAttribute::new("in_color", VertexFormat::Float4)], ts,
            PipelineParams { color_blend: Some(BlendState::new(Equation::Add,
                BlendFactor::Value(BlendValue::SourceAlpha),
                BlendFactor::OneMinusValue(BlendValue::SourceAlpha))), ..Default::default() });
        let tvb = ctx.new_buffer(BufferType::VertexBuffer, BufferUsage::Stream, BufferSource::empty::<TextVert>(4096));
        let tib = ctx.new_buffer(BufferType::IndexBuffer, BufferUsage::Stream, BufferSource::empty::<u16>(8192));
        let text_bind = Bindings { vertex_buffers: vec![tvb], index_buffer: tib, images: vec![tr.atlas] };

        let boxes = vec![
            InputBox::new(50.0, 80.0, 500.0, 36.0),
            InputBox::new(50.0, 160.0, 500.0, 36.0),
        ];

        Self { boxes, focus: None, color_pl, color_bind, text_pl, text_bind, tr, ctx, dpi }
    }

    fn update_ime(&mut self) {
        if let Some(i) = self.focus {
            let b = &self.boxes[i];
            let x = (b.cursor_x(&mut self.tr) * self.dpi) as i32;
            let y = ((b.y + b.h) * self.dpi) as i32;
            // Use miniquad's built-in IME position API
            window::set_ime_position(x, y);
        }
    }
}

impl EventHandler for Stage {
    fn update(&mut self) {}

    fn draw(&mut self) {
        let (sw, sh) = window::screen_size();
        self.ctx.clear(Some((0.11, 0.11, 0.14, 1.0)), None, None);

        let mut cv: Vec<ColorVert> = vec![];
        let mut ci: Vec<u16> = vec![];
        let mut tv: Vec<TextVert> = vec![];
        let mut ti: Vec<u16> = vec![];

        // Draw boxes
        for (i, b) in self.boxes.iter().enumerate() {
            let f = self.focus == Some(i);
            let bg = if f { [0.2, 0.2, 0.26, 1.0] } else { [0.16, 0.16, 0.2, 1.0] };
            let br = if f { [0.3, 0.5, 1.0, 1.0] } else { [0.3, 0.3, 0.35, 1.0] };
            rect(&mut cv, &mut ci, b.x, b.y, b.w, b.h, bg, sw, sh);
            outline(&mut cv, &mut ci, b.x, b.y, b.w, b.h, br, 2.0, sw, sh);
            if f {
                let cx = b.cursor_x(&mut self.tr);
                rect(&mut cv, &mut ci, cx, b.y + 6.0, 2.0, b.h - 12.0, [1.0,1.0,1.0,0.9], sw, sh);
            }

            // Draw text
            let mut x = b.x + 6.0;
            let baseline = b.y + b.h * 0.72;
            for ch in b.text.chars() {
                self.tr.cache_char(ch);
                if let Some(g) = self.tr.cache.get(&ch) {
                    if g.size[0] > 0.0 {
                        let gx = x + g.offset[0];
                        let gy = baseline - g.offset[1] - g.size[1];
                        glyph_quad(&mut tv, &mut ti, gx, gy, g, [1.0,1.0,1.0,1.0], sw, sh);
                    }
                    x += g.advance;
                }
            }
        }

        // Draw labels
        let labels = ["Input 1 (type Chinese here):", "Input 2:"];
        for (i, b) in self.boxes.iter().enumerate() {
            let mut x = b.x;
            for ch in labels[i].chars() {
                self.tr.cache_char(ch);
                if let Some(g) = self.tr.cache.get(&ch) {
                    if g.size[0] > 0.0 {
                        glyph_quad(&mut tv, &mut ti, x + g.offset[0], b.y - 24.0 + 16.0 - g.offset[1] - g.size[1],
                            g, [0.6,0.6,0.65,1.0], sw, sh);
                    }
                    x += g.advance;
                }
            }
        }

        self.tr.flush(&mut self.ctx);
        self.ctx.buffer_update(self.color_bind.vertex_buffers[0], BufferSource::slice(&cv));
        self.ctx.buffer_update(self.color_bind.index_buffer, BufferSource::slice(&ci));
        self.ctx.buffer_update(self.text_bind.vertex_buffers[0], BufferSource::slice(&tv));
        self.ctx.buffer_update(self.text_bind.index_buffer, BufferSource::slice(&ti));

        self.ctx.begin_default_pass(Default::default());
        self.ctx.apply_pipeline(&self.color_pl);
        self.ctx.apply_bindings(&self.color_bind);
        if !ci.is_empty() { self.ctx.draw(0, ci.len() as i32, 1); }
        self.ctx.apply_pipeline(&self.text_pl);
        self.ctx.apply_bindings(&self.text_bind);
        if !ti.is_empty() { self.ctx.draw(0, ti.len() as i32, 1); }
        self.ctx.end_render_pass();
        self.ctx.commit_frame();
    }

    fn mouse_button_down_event(&mut self, _: MouseButton, x: f32, y: f32) {
        self.focus = None;
        for (i, b) in self.boxes.iter_mut().enumerate() {
            b.focused = b.hit(x, y);
            if b.focused { 
                self.focus = Some(i); 
                // Enable IME when an input box is focused
                window::set_ime_enabled(true);
            }
        }
        self.update_ime();
        if self.focus.is_none() { 
            // Disable IME when no input box is focused (for game controls)
            window::set_ime_enabled(false);
        }
    }

    fn key_down_event(&mut self, k: KeyCode, _: KeyMods, _: bool) {
        if let Some(i) = self.focus {
            match k {
                KeyCode::Backspace => { self.boxes[i].backspace(); self.update_ime(); }
                KeyCode::Left => { self.boxes[i].left(); self.update_ime(); }
                KeyCode::Right => { self.boxes[i].right(); self.update_ime(); }
                KeyCode::Enter => {
                    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open("ime_output.txt") {
                        let _ = writeln!(f, "Input {}: \"{}\"", i + 1, self.boxes[i].text);
                    }
                }
                KeyCode::Escape => { 
                    self.boxes[i].focused = false; 
                    self.focus = None; 
                    // Disable IME when focus is lost
                    window::set_ime_enabled(false);
                }
                _ => {}
            }
        }
    }

    fn char_event(&mut self, ch: char, _: KeyMods, _: bool) {
        if ch.is_control() { return; }
        if let Some(i) = self.focus {
            self.boxes[i].insert(ch);
            self.update_ime();
        }
    }
}

// ============================================================================
// Helpers
// ============================================================================

fn rect(v: &mut Vec<ColorVert>, i: &mut Vec<u16>, x: f32, y: f32, w: f32, h: f32, c: [f32;4], sw: f32, sh: f32) {
    let b = v.len() as u16;
    let (x0, y0) = ((x/sw)*2.0-1.0, 1.0-(y/sh)*2.0);
    let (x1, y1) = (((x+w)/sw)*2.0-1.0, 1.0-((y+h)/sh)*2.0);
    v.extend([ColorVert{pos:[x0,y0],color:c}, ColorVert{pos:[x1,y0],color:c},
              ColorVert{pos:[x1,y1],color:c}, ColorVert{pos:[x0,y1],color:c}]);
    i.extend([b, b+1, b+2, b, b+2, b+3]);
}

fn outline(v: &mut Vec<ColorVert>, i: &mut Vec<u16>, x: f32, y: f32, w: f32, h: f32, c: [f32;4], t: f32, sw: f32, sh: f32) {
    rect(v,i,x,y,w,t,c,sw,sh); rect(v,i,x,y+h-t,w,t,c,sw,sh);
    rect(v,i,x,y,t,h,c,sw,sh); rect(v,i,x+w-t,y,t,h,c,sw,sh);
}

fn glyph_quad(v: &mut Vec<TextVert>, i: &mut Vec<u16>, x: f32, y: f32, g: &GlyphInfo, c: [f32;4], sw: f32, sh: f32) {
    let b = v.len() as u16;
    let (x0, y0) = ((x/sw)*2.0-1.0, 1.0-(y/sh)*2.0);
    let (x1, y1) = (((x+g.size[0])/sw)*2.0-1.0, 1.0-((y+g.size[1])/sh)*2.0);
    let (u0, v0, u1, v1) = (g.uv[0], g.uv[1], g.uv[0]+g.uv[2], g.uv[1]+g.uv[3]);
    v.extend([TextVert{pos:[x0,y0],uv:[u0,v0],color:c}, TextVert{pos:[x1,y0],uv:[u1,v0],color:c},
              TextVert{pos:[x1,y1],uv:[u1,v1],color:c}, TextVert{pos:[x0,y1],uv:[u0,v1],color:c}]);
    i.extend([b, b+1, b+2, b, b+2, b+3]);
}

// ============================================================================
// Shaders
// ============================================================================

const COLOR_VS: &str = "#version 100\nattribute vec2 in_pos; attribute vec4 in_color; varying lowp vec4 color;\nvoid main() { gl_Position = vec4(in_pos, 0.0, 1.0); color = in_color; }";
const COLOR_FS: &str = "#version 100\nvarying lowp vec4 color; void main() { gl_FragColor = color; }";
const TEXT_VS: &str = "#version 100\nattribute vec2 in_pos; attribute vec2 in_uv; attribute vec4 in_color;\nvarying lowp vec2 uv; varying lowp vec4 color;\nvoid main() { gl_Position = vec4(in_pos, 0.0, 1.0); uv = in_uv; color = in_color; }";
const TEXT_FS: &str = "#version 100\nprecision mediump float; varying lowp vec2 uv; varying lowp vec4 color; uniform sampler2D tex;\nvoid main() { gl_FragColor = vec4(color.rgb, color.a * texture2D(tex, uv).a); }";

// ============================================================================
// Main
// ============================================================================

fn main() {
    miniquad::start(conf::Conf {
        window_title: "IME Test - Chinese Input".into(),
        window_width: 640,
        window_height: 300,
        ..Default::default()
    }, || Box::new(Stage::new()));
}
