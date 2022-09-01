use miniquad::*;

#[repr(C)]
struct Vec2 {
    x: f32,
    y: f32,
}
#[repr(C)]
struct Vertex {
    pos: Vec2,
    uv: Vec2,
}

struct Stage {
    pipeline: Pipeline,
    bindings: Bindings,
    start_time: f64,
    last_frame: f64,
    uniforms: shader::Uniforms,
    blobs_velocities: [(f32, f32); 32],
}

impl Stage {
    pub fn new(ctx: &mut Context) -> Stage {
        #[rustfmt::skip]
        let vertices: [Vertex; 4] = [
            Vertex { pos : Vec2 { x: -1.0, y: -1.0 }, uv: Vec2 { x: 0., y: 0. } },
            Vertex { pos : Vec2 { x:  1.0, y: -1.0 }, uv: Vec2 { x: 1., y: 0. } },
            Vertex { pos : Vec2 { x:  1.0, y:  1.0 }, uv: Vec2 { x: 1., y: 1. } },
            Vertex { pos : Vec2 { x: -1.0, y:  1.0 }, uv: Vec2 { x: 0., y: 1. } },
        ];
        let vertex_buffer = Buffer::immutable(ctx, BufferType::VertexBuffer, &vertices);

        let indices: [u16; 6] = [0, 1, 2, 0, 2, 3];
        let index_buffer = Buffer::immutable(ctx, BufferType::IndexBuffer, &indices);

        let bindings = Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer: index_buffer,
            images: vec![],
        };

        let shader = Shader::new(ctx, shader::VERTEX, shader::FRAGMENT, shader::meta()).unwrap();

        let pipeline = Pipeline::new(
            ctx,
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("pos", VertexFormat::Float2),
                VertexAttribute::new("uv", VertexFormat::Float2),
            ],
            shader,
        );

        let uniforms = shader::Uniforms {
            time: 0.,
            blobs_count: 1,
            blobs_positions: [(0., 0.); 32],
        };

        let time = miniquad::date::now();

        Stage {
            pipeline,
            bindings,
            start_time: time,
            uniforms,
            blobs_velocities: [(0., 0.); 32],
            last_frame: time,
        }
    }
}

impl EventHandler for Stage {
    fn update(&mut self, _ctx: &mut Context) {
        let time = miniquad::date::now();
        let delta = (time - self.last_frame) as f32;
        self.last_frame = time;

        for i in 1..self.uniforms.blobs_count as usize {
            self.uniforms.blobs_positions[i].0 += self.blobs_velocities[i].0 * delta * 0.1;
            self.uniforms.blobs_positions[i].1 += self.blobs_velocities[i].1 * delta * 0.1;

            if self.uniforms.blobs_positions[i].0 < 0. || self.uniforms.blobs_positions[i].0 > 1. {
                self.blobs_velocities[i].0 *= -1.;
            }
            if self.uniforms.blobs_positions[i].1 < 0. || self.uniforms.blobs_positions[i].1 > 1. {
                self.blobs_velocities[i].1 *= -1.;
            }
        }
    }

    fn mouse_motion_event(&mut self, ctx: &mut Context, x: f32, y: f32) {
        let (w, h) = ctx.screen_size();
        let (x, y) = (x / w, 1. - y / h);
        self.uniforms.blobs_positions[0] = (x, y);
    }

    fn mouse_button_down_event(&mut self, ctx: &mut Context, _button: MouseButton, x: f32, y: f32) {
        if self.uniforms.blobs_count >= 32 {
            return;
        }

        let (w, h) = ctx.screen_size();
        let (x, y) = (x / w, 1. - y / h);
        let (dx, dy) = (quad_rand::gen_range(-1., 1.), quad_rand::gen_range(-1., 1.));

        self.uniforms.blobs_positions[self.uniforms.blobs_count as usize] = (x, y);
        self.blobs_velocities[self.uniforms.blobs_count as usize] = (dx, dy);
        self.uniforms.blobs_count += 1;
    }

    fn draw(&mut self, ctx: &mut Context) {
        self.uniforms.time = (miniquad::date::now() - self.start_time) as f32;

        ctx.begin_default_pass(Default::default());
        ctx.apply_pipeline(&self.pipeline);
        ctx.apply_bindings(&self.bindings);
        ctx.apply_uniforms(&self.uniforms);
        ctx.draw(0, 6, 1);
        ctx.end_render_pass();

        ctx.commit_frame();
    }
}

fn main() {
    miniquad::start(conf::Conf::default(), |mut ctx| {
        Box::new(Stage::new(&mut ctx))
    });
}

// based on: https://www.shadertoy.com/view/XsS3DV
mod shader {
    use miniquad::*;

    pub const VERTEX: &str = include_str!("shaders/vertex-uv.glsl");
    pub const FRAGMENT: &str = include_str!("shaders/blobs-fragment.glsl");

    pub fn meta() -> ShaderMeta {
        ShaderMeta {
            images: vec![],
            uniforms: UniformBlockLayout {
                uniforms: vec![
                    UniformDesc::new("time", UniformType::Float1),
                    UniformDesc::new("blobs_count", UniformType::Int1),
                    UniformDesc::new("blobs_positions", UniformType::Float2).array(32),
                ],
            },
        }
    }

    #[repr(C)]
    pub struct Uniforms {
        pub time: f32,
        pub blobs_count: i32,
        pub blobs_positions: [(f32, f32); 32],
    }
}
