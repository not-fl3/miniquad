use miniquad::*;

use glam::{vec3, Mat4, Vec3};

const MAX_PARTICLES: usize = 512 * 1024;
const NUM_PARTICLES_EMITTED_PER_FRAME: usize = 10;

struct Stage {
    ctx: Box<dyn RenderingBackend>,

    pipeline: Pipeline,
    bindings: Bindings,

    pos: Vec<Vec3>,
    vel: Vec<Vec3>,
    ry: f32,
}

impl Stage {
    pub fn new() -> Stage {
        let mut ctx: Box<dyn RenderingBackend> = window::new_rendering_backend();

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
        let geometry_vertex_buffer = ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&vertices),
        );

        #[rustfmt::skip]
        let indices: &[u16] = &[
            0, 1, 2,    0, 2, 3,    0, 3, 4,    0, 4, 1,
            5, 1, 2,    5, 2, 3,    5, 3, 4,    5, 4, 1
        ];
        let index_buffer = ctx.new_buffer(
            BufferType::IndexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&indices),
        );

        // empty, dynamic instance data vertex buffer
        let positions_vertex_buffer = ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Stream,
            BufferSource::empty::<Vec3>(MAX_PARTICLES),
        );

        let bindings = Bindings {
            vertex_buffers: vec![geometry_vertex_buffer, positions_vertex_buffer],
            index_buffer: index_buffer,
            images: vec![],
        };

        let shader = ctx
            .new_shader(
                ShaderSource {
                    glsl_vertex: Some(shader::GL_VERTEX),
                    glsl_fragment: Some(shader::GL_FRAGMENT),
                    metal_shader: Some(shader::METAL),
                },
                shader::meta(),
            )
            .unwrap();

        let pipeline = ctx.new_pipeline(
            &[
                BufferLayout::default(),
                BufferLayout {
                    step_func: VertexStep::PerInstance,
                    ..Default::default()
                },
            ],
            &[
                VertexAttribute::with_buffer("in_pos", VertexFormat::Float3, 0),
                VertexAttribute::with_buffer("in_color", VertexFormat::Float4, 0),
                VertexAttribute::with_buffer("in_inst_pos", VertexFormat::Float3, 1),
            ],
            shader,
        );

        Stage {
            ctx,
            pipeline,
            bindings,
            pos: Vec::with_capacity(MAX_PARTICLES),
            vel: Vec::with_capacity(MAX_PARTICLES),
            ry: 0.,
        }
    }
}

impl EventHandler for Stage {
    fn update(&mut self) {
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
            if self.pos[i].y < -2.0 {
                self.pos[i].y = -1.8;
                self.vel[i] *= vec3(0.8, -0.8, 0.8);
            }
        }
    }

    fn draw(&mut self) {
        // by default glam-rs can vec3 as u128 or #[reprc(C)](f32, f32, f32). need to ensure that the second option was used
        assert_eq!(std::mem::size_of::<Vec3>(), 12);

        self.ctx.buffer_update(
            self.bindings.vertex_buffers[1],
            BufferSource::slice(&self.pos[..]),
        );

        // model-view-projection matrix
        let (width, height) = window::screen_size();

        let proj = Mat4::perspective_rh_gl(60.0f32.to_radians(), width / height, 0.01, 50.0);
        let view = Mat4::look_at_rh(
            vec3(0.0, 1.5, 12.0),
            vec3(0.0, 0.0, 0.0),
            vec3(0.0, 1.0, 0.0),
        );
        let view_proj = proj * view;

        self.ry += 0.01;
        let mvp = view_proj * Mat4::from_rotation_y(self.ry);

        self.ctx.begin_default_pass(Default::default());

        self.ctx.apply_pipeline(&self.pipeline);
        self.ctx.apply_bindings(&self.bindings);
        self.ctx
            .apply_uniforms(UniformsSource::table(&shader::Uniforms { mvp }));
        self.ctx.draw(0, 24, self.pos.len() as i32);
        self.ctx.end_render_pass();

        self.ctx.commit_frame();
    }
}

fn main() {
    // std::thread::spawn(|| {
    //     let mut conf = conf::Conf::default();
    //     let metal = std::env::args().nth(1).as_deref() == Some("metal");
    //     conf.platform.apple_gfx_api = if metal {
    //         conf::AppleGfxApi::Metal
    //     } else {
    //         conf::AppleGfxApi::OpenGl
    //     };

    //     miniquad::start(conf, move || Box::new(Stage::new()));
    // });

    //let mut conf = conf::Conf::default();
    let mut conf = conf::Conf {
        window_title: "Miniquad".to_string(),
        window_width: 1024,
        window_height: 768,
        fullscreen: true,
        platform: conf::Platform {
            linux_backend: conf::LinuxBackend::WaylandOnly,
            ..Default::default()
        },
        ..Default::default()
    };

    let metal = std::env::args().nth(1).as_deref() == Some("metal");
    conf.platform.apple_gfx_api = if metal {
        conf::AppleGfxApi::Metal
    } else {
        conf::AppleGfxApi::OpenGl
    };

    miniquad::start(conf, move || Box::new(Stage::new()));
}

mod shader {
    use miniquad::*;

    pub const GL_VERTEX: &str = r#"#version 100
    attribute vec3 in_pos;
    attribute vec4 in_color;
    attribute vec3 in_inst_pos;

    varying lowp vec4 color;

    uniform mat4 mvp;

    void main() {
        vec4 pos = vec4(in_pos + in_inst_pos, 1.0);
        gl_Position = mvp * pos;
        color = in_color;
    }
    "#;

    pub const GL_FRAGMENT: &str = r#"#version 100
    varying lowp vec4 color;

    void main() {
        gl_FragColor = color;
    }
    "#;

    pub const METAL: &str = r#"
    #include <metal_stdlib>

    using namespace metal;

    struct Uniforms
    {
        float4x4 mvp;
    };

    struct Vertex
    {
        float3 in_pos      [[attribute(0)]];
        float4 in_color    [[attribute(1)]];
        float3 in_inst_pos [[attribute(2)]];
    };

    struct RasterizerData
    {
        float4 position [[position]];
        float4 color [[user(locn0)]];
    };

    vertex RasterizerData vertexShader(Vertex v [[stage_in]], constant Uniforms& uniforms [[buffer(0)]])
    {
        RasterizerData out;

        out.position = uniforms.mvp * float4(v.in_pos + v.in_inst_pos, 1.0);
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
            uniforms: UniformBlockLayout {
                uniforms: vec![UniformDesc::new("mvp", UniformType::Mat4)],
            },
        }
    }

    #[repr(C)]
    pub struct Uniforms {
        pub mvp: glam::Mat4,
    }
}
