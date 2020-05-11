use miniquad::*;

use glam::{vec3, Mat4};

struct Stage {
    post_processing_pipeline: Pipeline,
    post_processing_bind: Bindings,
    offscreen_pipeline: Pipeline,
    offscreen_bind: Bindings,
    offscreen_pass: RenderPass,
    rx: f32,
    ry: f32,
}

impl Stage {
    pub fn new(ctx: &mut Context) -> Stage {
        let (w, h) = ctx.screen_size();
        let (color_img, offscreen_pass) = Self::create_offscreen_pass(ctx, w as _, h as _);

        #[rustfmt::skip]
        let vertices: &[f32] = &[
            /* pos               color                   uvs */
            -1.0, -1.0, -1.0,    1.0, 0.5, 0.5, 1.0,     0.0, 0.0,
             1.0, -1.0, -1.0,    1.0, 0.5, 0.5, 1.0,     1.0, 0.0,
             1.0,  1.0, -1.0,    1.0, 0.5, 0.5, 1.0,     1.0, 1.0,
            -1.0,  1.0, -1.0,    1.0, 0.5, 0.5, 1.0,     0.0, 1.0,
    
            -1.0, -1.0,  1.0,    0.5, 1.0, 0.5, 1.0,     0.0, 0.0, 
             1.0, -1.0,  1.0,    0.5, 1.0, 0.5, 1.0,     1.0, 0.0,
             1.0,  1.0,  1.0,    0.5, 1.0, 0.5, 1.0,     1.0, 1.0,
            -1.0,  1.0,  1.0,    0.5, 1.0, 0.5, 1.0,     0.0, 1.0,
    
            -1.0, -1.0, -1.0,    0.5, 0.5, 1.0, 1.0,     0.0, 0.0,
            -1.0,  1.0, -1.0,    0.5, 0.5, 1.0, 1.0,     1.0, 0.0,
            -1.0,  1.0,  1.0,    0.5, 0.5, 1.0, 1.0,     1.0, 1.0,
            -1.0, -1.0,  1.0,    0.5, 0.5, 1.0, 1.0,     0.0, 1.0,
    
             1.0, -1.0, -1.0,    1.0, 0.5, 0.0, 1.0,     0.0, 0.0,
             1.0,  1.0, -1.0,    1.0, 0.5, 0.0, 1.0,     1.0, 0.0,
             1.0,  1.0,  1.0,    1.0, 0.5, 0.0, 1.0,     1.0, 1.0,
             1.0, -1.0,  1.0,    1.0, 0.5, 0.0, 1.0,     0.0, 1.0,
    
            -1.0, -1.0, -1.0,    0.0, 0.5, 1.0, 1.0,     0.0, 0.0,
            -1.0, -1.0,  1.0,    0.0, 0.5, 1.0, 1.0,     1.0, 0.0,
             1.0, -1.0,  1.0,    0.0, 0.5, 1.0, 1.0,     1.0, 1.0,
             1.0, -1.0, -1.0,    0.0, 0.5, 1.0, 1.0,     0.0, 1.0,
    
            -1.0,  1.0, -1.0,    1.0, 0.0, 0.5, 1.0,     0.0, 0.0,
            -1.0,  1.0,  1.0,    1.0, 0.0, 0.5, 1.0,     1.0, 0.0,
             1.0,  1.0,  1.0,    1.0, 0.0, 0.5, 1.0,     1.0, 1.0,
             1.0,  1.0, -1.0,    1.0, 0.0, 0.5, 1.0,     0.0, 1.0
        ];

        let vertex_buffer = Buffer::immutable(ctx, BufferType::VertexBuffer, &vertices);

        #[rustfmt::skip]
        let indices: &[u16] = &[
            0, 1, 2,  0, 2, 3,
            6, 5, 4,  7, 6, 4,
            8, 9, 10,  8, 10, 11,
            14, 13, 12,  15, 14, 12,
            16, 17, 18,  16, 18, 19,
            22, 21, 20,  23, 22, 20
        ];

        let index_buffer = Buffer::immutable(ctx, BufferType::IndexBuffer, &indices);

        let offscreen_bind = Bindings {
            vertex_buffers: vec![vertex_buffer.clone()],
            index_buffer: index_buffer.clone(),
            images: vec![],
        };

        #[rustfmt::skip]
        let vertices: &[f32] = &[
            /* pos         uvs */
            -1.0, -1.0,    0.0, 0.0,
             1.0, -1.0,    1.0, 0.0,
             1.0,  1.0,    1.0, 1.0,
            -1.0,  1.0,    0.0, 1.0,
        ];

        let vertex_buffer = Buffer::immutable(ctx, BufferType::VertexBuffer, &vertices);

        let indices: &[u16] = &[0, 1, 2,  0, 2, 3];

        let index_buffer = Buffer::immutable(ctx, BufferType::IndexBuffer, &indices);

        let post_processing_bind = Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer: index_buffer,
            images: vec![color_img],
        };

        let default_shader = Shader::new(
            ctx,
            post_processing_shader::VERTEX,
            post_processing_shader::FRAGMENT,
            post_processing_shader::META,
        );

        let post_processing_pipeline = Pipeline::new(
            ctx,
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("pos", VertexFormat::Float2),
                VertexAttribute::new("uv", VertexFormat::Float2),
            ],
            default_shader,
        );

        let offscreen_shader = Shader::new(
            ctx,
            offscreen_shader::VERTEX,
            offscreen_shader::FRAGMENT,
            offscreen_shader::META,
        );

        let offscreen_pipeline = Pipeline::with_params(
            ctx,
            &[BufferLayout {
                stride: 36,
                ..Default::default()
            }],
            &[
                VertexAttribute::new("pos", VertexFormat::Float3),
                VertexAttribute::new("color0", VertexFormat::Float4),
            ],
            offscreen_shader,
            PipelineParams {
                depth_test: Comparison::LessOrEqual,
                depth_write: true,
                ..Default::default()
            },
        );

        Stage {
            post_processing_pipeline,
            post_processing_bind,
            offscreen_pipeline,
            offscreen_bind,
            offscreen_pass,
            rx: 0.,
            ry: 0.,
        }
    }

    fn create_offscreen_pass(ctx: &mut Context, width: u32, height: u32) -> (Texture, RenderPass) {
        let color_img = Texture::new_render_texture(
            ctx,
            TextureParams {
                width,
                height,
                format: TextureFormat::RGBA8,
                ..Default::default()
            },
        );
        let depth_img = Texture::new_render_texture(
            ctx,
            TextureParams {
                width,
                height,
                format: TextureFormat::Depth,
                ..Default::default()
            },
        );

        (color_img, RenderPass::new(ctx, color_img, depth_img))
    }

}

impl EventHandler for Stage {
    fn update(&mut self, _ctx: &mut Context) {}

    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        let (color_img, offscreen_pass) = Stage::create_offscreen_pass(ctx, width as _, height as _);

        self.offscreen_pass.delete(ctx);
        self.offscreen_pass = offscreen_pass;
        self.post_processing_bind.images[0] = color_img;
    }

    fn draw(&mut self, ctx: &mut Context) {
        let (width, height) = ctx.screen_size();
        let proj = Mat4::perspective_rh_gl(60.0f32.to_radians(), width / height, 0.01, 10.0);
        let view = Mat4::look_at_rh(
            vec3(0.0, 1.5, 3.0),
            vec3(0.0, 0.0, 0.0),
            vec3(0.0, 1.0, 0.0),
        );
        let view_proj = proj * view;

        self.rx += 0.01;
        self.ry += 0.03;
        let model = Mat4::from_rotation_ypr(self.rx, self.ry, 0.);

        let vs_params = post_processing_shader::Uniforms {
            mvp: view_proj * model,
        };

        let (w, h) = ctx.screen_size();
        
        // the offscreen pass, rendering an rotating, untextured cube into a render target image
        ctx.begin_pass(
            self.offscreen_pass,
            PassAction::clear_color(1.0, 1.0, 1.0, 1.0),
        );
        ctx.apply_pipeline(&self.offscreen_pipeline);
        ctx.apply_bindings(&self.offscreen_bind);
        ctx.apply_uniforms(&vs_params);
        ctx.draw(0, 36, 1);
        ctx.end_render_pass();

        // and the post-processing-pass, rendering a rotating, textured cube, using the
        // previously rendered offscreen render-target as texture
        ctx.begin_default_pass(PassAction::Nothing);
        ctx.apply_pipeline(&self.post_processing_pipeline);
        ctx.apply_bindings(&self.post_processing_bind);
        ctx.draw(0, 36, 1);
        ctx.end_render_pass();
        ctx.commit_frame();
    }
}

fn main() {
    miniquad::start(conf::Conf::default(), |mut ctx| {
        UserData::owning(Stage::new(&mut ctx), ctx)
    });
}

mod post_processing_shader {
    use miniquad::*;

    pub const VERTEX: &str = r#"#version 100
    attribute vec2 pos;
    attribute vec2 uv;

    varying lowp vec2 texcoord;

    void main() {
        gl_Position = vec4(pos, 0, 1);
        texcoord = uv;
    }
    "#;

    pub const FRAGMENT: &str = r#"#version 100
    varying lowp vec2 texcoord;

    uniform sampler2D tex;

    precision lowp float;

    // Source: https://github.com/Jam3/glsl-fast-gaussian-blur/blob/master/5.glsl
    vec4 blur5(sampler2D image, vec2 uv, vec2 resolution, vec2 direction) {
        vec4 color = vec4(0.0);
        vec2 off1 = vec2(1.3333333333333333) * direction;
        color += texture2D(image, uv) * 0.29411764705882354;
        color += texture2D(image, uv + (off1 / resolution)) * 0.35294117647058826;
        color += texture2D(image, uv - (off1 / resolution)) * 0.35294117647058826;
        return color;
    }

    void main() {
        gl_FragColor = blur5(tex, texcoord, vec2(256.0), vec2(3.0));
    }
    "#;

    pub const META: ShaderMeta = ShaderMeta {
        images: &["tex"],
        uniforms: UniformBlockLayout {
            uniforms: &[],
        },
    };

    #[repr(C)]
    pub struct Uniforms {
        pub mvp: glam::Mat4,
    }
}

mod offscreen_shader {
    use miniquad::*;

    pub const VERTEX: &str = r#"#version 100
    attribute vec4 pos;
    attribute vec4 color0;

    varying lowp vec4 color;

    uniform mat4 mvp;

    void main() {
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
}
