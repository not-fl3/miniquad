//! `Conf::headless`: the triangle example with no window.
//!
//! Draws one frame into the offscreen default framebuffer, reads a pixel
//! from the middle of the triangle and one from a corner, checks both, and
//! quits. Run it from a terminal: nothing should appear on screen or in the
//! Dock, and it should exit 0 within a second.

use miniquad::*;

#[repr(C)]
struct Vertex {
    pos: [f32; 2],
    color: [f32; 4],
}

struct Stage {
    pipeline: Pipeline,
    bindings: Bindings,
    ctx: Box<dyn RenderingBackend>,
    frames: u32,
}

impl Stage {
    pub fn new() -> Stage {
        let mut ctx: Box<dyn RenderingBackend> = window::new_rendering_backend();

        #[rustfmt::skip]
        let vertices: [Vertex; 3] = [
            Vertex { pos : [ -0.5, -0.5 ], color: [1., 0., 0., 1.] },
            Vertex { pos : [  0.5, -0.5 ], color: [0., 1., 0., 1.] },
            Vertex { pos : [  0.0,  0.5 ], color: [0., 0., 1., 1.] },
        ];
        let vertex_buffer = ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&vertices),
        );

        let indices: [u16; 3] = [0, 1, 2];
        let index_buffer = ctx.new_buffer(
            BufferType::IndexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&indices),
        );

        let bindings = Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer,
            images: vec![],
        };

        let shader = ctx
            .new_shader(
                match ctx.info().backend {
                    Backend::OpenGl => ShaderSource::Glsl {
                        vertex: shader::VERTEX,
                        fragment: shader::FRAGMENT,
                    },
                    Backend::Metal => ShaderSource::Msl {
                        program: shader::METAL,
                    },
                },
                shader::meta(),
            )
            .unwrap();

        let pipeline = ctx.new_pipeline(
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("in_pos", VertexFormat::Float2),
                VertexAttribute::new("in_color", VertexFormat::Float4),
            ],
            shader,
            PipelineParams::default(),
        );

        Stage {
            pipeline,
            bindings,
            ctx,
            frames: 0,
        }
    }
}

impl EventHandler for Stage {
    fn update(&mut self) {}

    fn draw(&mut self) {
        self.ctx
            .begin_default_pass(PassAction::clear_color(0., 0., 0., 1.));
        self.ctx.apply_pipeline(&self.pipeline);
        self.ctx.apply_bindings(&self.bindings);
        self.ctx.draw(0, 3, 1);
        self.ctx.end_render_pass();
        self.ctx.commit_frame();

        // Give it a couple of frames, as a real app would take, then look.
        self.frames += 1;
        if self.frames < 3 {
            return;
        }

        let (w, h) = window::screen_size();
        let (w, h) = (w as i32, h as i32);
        let middle = read_pixel(w / 2, h / 2);
        let corner = read_pixel(1, 1);
        println!(
            "{}x{} framebuffer: middle {:?}, corner {:?}",
            w, h, middle, corner
        );
        assert_eq!(
            corner,
            [0, 0, 0, 255],
            "the corner should be the clear color"
        );
        assert!(
            middle[0] > 0 || middle[1] > 0 || middle[2] > 0,
            "the middle of the triangle should have been drawn"
        );
        println!("ok");
        window::order_quit();
    }
}

/// One RGBA8 pixel of the current framebuffer.
fn read_pixel(x: i32, y: i32) -> [u8; 4] {
    let mut px = [0u8; 4];
    unsafe {
        gl::glReadPixels(
            x,
            y,
            1,
            1,
            gl::GL_RGBA,
            gl::GL_UNSIGNED_BYTE,
            px.as_mut_ptr() as *mut _,
        );
    }
    px
}

fn main() {
    let conf = conf::Conf {
        window_title: "headless".to_string(),
        window_width: 320,
        window_height: 240,
        headless: true,
        ..Default::default()
    };
    miniquad::start(conf, || Box::new(Stage::new()));
}

mod shader {
    use miniquad::*;

    pub const VERTEX: &str = r#"#version 100
    attribute vec2 in_pos;
    attribute vec4 in_color;

    varying lowp vec4 color;

    void main() {
        gl_Position = vec4(in_pos, 0, 1);
        color = in_color;
    }"#;

    pub const FRAGMENT: &str = r#"#version 100
    varying lowp vec4 color;

    void main() {
        gl_FragColor = color;
    }"#;

    pub const METAL: &str = r#"
    #include <metal_stdlib>

    using namespace metal;

    struct Vertex
    {
        float2 in_pos   [[attribute(0)]];
        float4 in_color [[attribute(1)]];
    };

    struct RasterizerData
    {
        float4 position [[position]];
        float4 color [[user(locn0)]];
    };

    vertex RasterizerData vertexShader(Vertex v [[stage_in]])
    {
        RasterizerData out;

        out.position = float4(v.in_pos.xy, 0.0, 1.0);
        out.color = v.in_color;

        return out;
    }

    fragment float4 fragmentShader(RasterizerData in [[stage_in]])
    {
        return in.color;
    }"#;

    pub fn meta() -> ShaderMeta {
        ShaderMeta {
            images: vec![],
            uniforms: UniformBlockLayout { uniforms: vec![] },
        }
    }
}
