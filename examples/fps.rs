use miniquad::*;

use glam::{vec2, vec3, Mat4, Vec2, Vec3};

#[derive(Default)]
struct Cam {
    pitch_deg: f32,
    yaw_deg: f32,
    turn_vel: Vec2,
}

impl Cam {
    fn facing(&self) -> Vec3 {
        let (pitch, yaw) = (self.pitch_deg.to_radians(), self.yaw_deg.to_radians());
        vec3(
            yaw.sin() * pitch.cos(),
            pitch.sin(),
            yaw.cos() * pitch.cos(),
        )
    }

    fn turn(&mut self, yaw_delta_deg: f32, pitch_delta_deg: f32) {
        self.pitch_deg = (self.pitch_deg + pitch_delta_deg).max(-89.0).min(89.0);
        self.yaw_deg = (self.yaw_deg + yaw_delta_deg) % 360.0;
    }

    fn update(&mut self) {
        self.turn_vel *= 0.9;
        self.turn(self.turn_vel.x(), self.turn_vel.y());
    }
}

const MAX_ENEMIES: usize = 512 * 1024;
struct Stage {
    pipeline: Pipeline,
    bindings: Bindings,

    /// Contains indexes for enemies that the player would "shoot" if they clicked right now.
    targeted_enemies: Vec<usize>,
    enemies: Vec<Vec3>,

    escaped: bool,

    keys_down: [bool; 256],
    cam: Cam,
    pos: Vec3,
    vel: Vec3,
}

impl Stage {
    pub fn new(ctx: &mut Context) -> Stage {
        let r = 0.3;
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
            MAX_ENEMIES * std::mem::size_of::<Vec3>(),
        );

        let bindings = Bindings {
            vertex_buffers: vec![geometry_vertex_buffer, positions_vertex_buffer],
            index_buffer: index_buffer,
            images: vec![],
        };

        let shader = Shader::new(ctx, shader::VERTEX, shader::FRAGMENT, shader::meta()).unwrap();

        let pipeline = Pipeline::with_params(
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
            PipelineParams {
                depth_test: Comparison::LessOrEqual,
                depth_write: true,
                ..Default::default()
            },
        );

        let mut enemies = vec![];

        for x in 0..10 {
            for y in 0..10 {
                let o = (x % 2 + y % 2) as f32 / 4.0;
                let p = vec3(x as f32 - 5.0 + o, 0.0, y as f32 - 5.0 + o);
                if p.length() > 0.0 {
                    enemies.push(p);
                }
            }
        }

        Stage {
            pipeline,
            bindings,
            targeted_enemies: Vec::with_capacity(enemies.len()),
            enemies,
            escaped: true,
            keys_down: [false; 256],
            pos: vec3(0.0, 1.0, 0.0),
            vel: Vec3::zero(),
            cam: Default::default(),
        }
    }
}

impl Stage {
    fn view_proj(&self, ctx: &mut Context) -> Mat4 {
        // model-view-projection matrix
        let (width, height) = ctx.screen_size();
        let proj = Mat4::perspective_rh_gl(45.0f32.to_radians(), width / height, 0.01, 50.0);
        let view = Mat4::look_at_rh(self.pos, self.pos + self.cam.facing(), vec3(0.0, 1.0, 0.0));

        proj * view
    }
}

impl EventHandler for Stage {
    fn update(&mut self, ctx: &mut Context) {
        ctx.set_cursor_grab(!self.escaped);
        ctx.show_mouse(self.escaped);

        let mut move_dir = Vec3::zero();
        let facing = self.cam.facing();
        let side = facing.cross(Vec3::unit_y());
        if self.keys_down[KeyCode::W as usize] {
            move_dir += facing;
        }
        if self.keys_down[KeyCode::S as usize] {
            move_dir -= facing;
        }
        if self.keys_down[KeyCode::A as usize] {
            move_dir -= side;
        }
        if self.keys_down[KeyCode::D as usize] {
            move_dir += side;
        }
        *move_dir.y_mut() = 0.0;
        let len = move_dir.length();
        if len > 0.0 {
            let norm = move_dir / len;
            self.vel += norm * 0.002;
        }

        self.vel *= 0.95;
        self.pos += self.vel;

        self.cam.update();

        self.targeted_enemies.clear();
        for i in 0..self.enemies.len() {
            let enemy = &mut self.enemies[i];
            *enemy.y_mut() = 0.0;

            let mouse_project =
                line_plane_intersect(self.pos, self.cam.facing(), *enemy, *enemy - self.pos);

            if (mouse_project - *enemy).length() < 0.3 {
                self.targeted_enemies.push(i);
                *enemy.y_mut() = (date::now() * 10.0).sin() as f32 * 0.05;
            }
        }

        self.bindings.vertex_buffers[1].update(ctx, &self.enemies[..]);
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        key: KeyCode,
        _key_mods: KeyMods,
        repeat: bool,
    ) {
        if !repeat {
            self.keys_down[key as usize] = true;
        }

        if key == KeyCode::Escape {
            self.escaped = true;
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, key: KeyCode, _key_mods: KeyMods) {
        self.keys_down[key as usize] = false;
    }

    fn raw_mouse_motion(&mut self, _ctx: &mut Context, x: f32, y: f32) {
        self.cam.turn_vel += vec2(x, y) * -0.025;
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        if MouseButton::Left == button {
            if self.escaped {
                self.escaped = false;
            } else {
                for &targeted_enemy in self.targeted_enemies.iter().rev() {
                    self.enemies.remove(targeted_enemy);
                }
                *self.cam.turn_vel.y_mut() += 1.2;
            }
        }
    }

    fn draw(&mut self, ctx: &mut Context) {
        let view_proj = self.view_proj(ctx);

        ctx.begin_default_pass(Default::default());

        ctx.apply_pipeline(&self.pipeline);
        ctx.apply_bindings(&self.bindings);
        ctx.apply_uniforms(&shader::Uniforms { view_proj });
        ctx.draw(0, 24, self.enemies.len() as i32);
        ctx.end_render_pass();

        ctx.commit_frame();
    }
}

fn line_plane_intersect(
    line_origin: Vec3,
    line: Vec3,
    plane_origin: Vec3,
    plane_normal: Vec3,
) -> Vec3 {
    let d = (plane_origin - line_origin).dot(plane_normal) / line.dot(plane_normal);
    line_origin + line * d
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

    uniform mat4 view_proj;

    void main() {
        vec4 pos = vec4(pos + inst_pos, 1.0);
        gl_Position = view_proj * pos;
        color = color0;
    }
    "#;

    pub const FRAGMENT: &str = r#"#version 100
    varying lowp vec4 color;
    
    void main() {
        gl_FragColor = color;
    }
    "#;

    pub fn meta() -> ShaderMeta {
        ShaderMeta {
            images: vec![],
            uniforms: UniformBlockLayout {
                uniforms: vec![UniformDesc::new("view_proj", UniformType::Mat4)],
            },
        }
    }

    #[repr(C)]
    pub struct Uniforms {
        pub view_proj: glam::Mat4,
    }
}
