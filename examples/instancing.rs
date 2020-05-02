use miniquad::*;

use glam::{vec3, Mat4, Vec3};

const MAX_PARTICLES: usize = 512 * 1024;
const NUM_PARTICLES_EMITTED_PER_FRAME: usize = 10;

struct Stage {
    pipeline: Pipeline,
    bindings: Bindings,

    pos: Vec<Vec3>,
    vel: Vec<Vec3>,
    ry: f32,
}

impl Stage {
    pub fn new(ctx: &mut Context) -> Stage {
        let r = 0.05;
        #[rustfmt::skip]
        let vertices: &[f32] = &[
            // positions          colors
            0.0,   -r, 0.0,       1.0, 0.0, 0.0, 1.0,
               r, 0.0, r,         0.0, 1.0, 0.0, 1.0,
               r, 0.0, -r,        0.0, 0.0, 1.0, 1.0,
              -r, 0.0, -r,        1.0, 1.0, 0.0, 1.0,
              -r, 0.0, r,         0.0, 1.0, 1.0, 1.0,
             0.0,   r, 0.0,       1.0, 0.0, 1.0, 1.0
        ];
        // vertex buffer for static geometry
        let geometry_vertex_buffer = Buffer::immutable(ctx, BufferType::VertexBuffer, &vertices);

        #[rustfmt::skip]
        let indices: &[u16] = &[
            0, 1, 2,    0, 2, 3,    0, 3, 4,    0, 4, 1,
            5, 1, 2,    5, 2, 3,    5, 3, 4,    5, 4, 1
        ];
        let index_buffer = Buffer::immutable(ctx, BufferType::IndexBuffer, &indices);

        // empty, dynamic instance-data vertex buffer
        let positions_vertex_buffer = Buffer::stream(
            ctx,
            BufferType::VertexBuffer,
            MAX_PARTICLES * std::mem::size_of::<Vec3>(),
        );

        let bindings = Bindings {
            vertex_buffers: vec![geometry_vertex_buffer, positions_vertex_buffer],
            index_buffer: index_buffer,
            images: vec![],
        };

        let shader = Shader::new(ctx, shader::VERTEX, shader::FRAGMENT, shader::META);

        let pipeline = Pipeline::new(
            ctx,
            &[
                BufferLayout::default(),
                BufferLayout {
                    step_func: VertexStep::PerInstance,
                    ..Default::default()
                },
            ],
            &[
                VertexAttribute::with_buffer("pos", VertexFormat::Float3, 0),
                VertexAttribute::with_buffer("color0", VertexFormat::Float4, 0),
                VertexAttribute::with_buffer("inst_pos", VertexFormat::Float3, 1),
            ],
            shader,
        );

        Stage {
            pipeline,
            bindings,
            pos: Vec::with_capacity(MAX_PARTICLES),
            vel: Vec::with_capacity(MAX_PARTICLES),
            ry: 0.,
        }
    }
}

impl EventHandler for Stage {
    fn update(&mut self, ctx: &mut Context) {
        let frame_time = 1. / 60.;

        // emit new particles
        for _ in 0..NUM_PARTICLES_EMITTED_PER_FRAME {
            if self.pos.len() < MAX_PARTICLES {
                self.pos.push(vec3(0., 0., 0.));
                self.vel.push(vec3(
                    quad_rand::gen_range(-1., 1.),
                    quad_rand::gen_range(0., 2.),
                    quad_rand::gen_range(-1., 1.),
                ));
            } else {
                break;
            }
        }

        // update particle positions
        for i in 0..self.pos.len() {
            self.vel[i] -= vec3(0., frame_time, 0.);
            self.pos[i] += self.vel[i] * frame_time;
            /* bounce back from 'ground' */
            if self.pos[i].y() < -2.0 {
                self.pos[i].set_y(-1.8);
                self.vel[i] *= vec3(0.8, -0.8, 0.8);
            }
        }

        // by default glam-rs can vec3 as u128 or #[reprc(C)](f32, f32, f32). need to ensure that the second option was used
        assert_eq!(std::mem::size_of::<Vec3>(), 12);

        self.bindings.vertex_buffers[1].update(ctx, &self.pos[..]);
    }

    fn draw(&mut self, ctx: &mut Context) {
        // model-view-projection matrix
        let (width, height) = ctx.screen_size();
        let proj = Mat4::perspective_rh_gl(60.0f32.to_radians(), width / height, 0.01, 50.0);
        let view = Mat4::look_at_rh(
            vec3(0.0, 1.5, 12.0),
            vec3(0.0, 0.0, 0.0),
            vec3(0.0, 1.0, 0.0),
        );
        let view_proj = proj * view;

        self.ry += 0.01;
        let mvp = view_proj * Mat4::from_rotation_y(self.ry);

        ctx.begin_default_pass(Default::default());

        ctx.apply_pipeline(&self.pipeline);
        ctx.apply_bindings(&self.bindings);
        ctx.apply_uniforms(&shader::Uniforms { mvp });
        ctx.draw(0, 24, self.pos.len() as i32);
        ctx.end_render_pass();

        ctx.commit_frame();
    }
}

fn main() {
    miniquad::start(conf::Conf::default(), |mut ctx| {
        UserData::owning(Stage::new(&mut ctx), ctx)
    });
}

mod shader {
    use miniquad::*;

    pub const VERTEX: &str = r#"#version 100
    attribute vec3 pos;
    attribute vec4 color0;
    attribute vec3 inst_pos;

    varying lowp vec4 color;

    uniform mat4 mvp;

    void main() {
        vec4 pos = vec4(pos + inst_pos, 1.0);
        gl_Position = mvp * pos;
        color = color0;
    }
    "#;

    pub const FRAGMENT: &str = r#"#version 100
    varying lowp vec4 color;
    
    void main() {
        gl_FragColor = color;
    }
    "#;

    pub const META: ShaderMeta = ShaderMeta {
        images: &[],
        uniforms: UniformBlockLayout {
            uniforms: &[UniformDesc::new("mvp", UniformType::Mat4)],
        },
    };

    #[repr(C)]
    pub struct Uniforms {
        pub mvp: glam::Mat4,
    }
}
