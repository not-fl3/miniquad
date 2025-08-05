use miniquad::*;

struct Stage {
    ctx: GlContext,
}
impl EventHandler for Stage {
    fn update(&mut self) {}

    fn draw(&mut self) {
        self.ctx.clear(Some((0.2, 0.7, 0.3, 1.0)), None, None);
    }
}

fn main() {
    miniquad::start(
        conf::Conf {
            window_title: "Window Position Test".to_string(),
            window_width: 640,
            window_height: 480,
            window_position: Some((200, 150)), // Position window at (200, 150)
            ..Default::default()
        },
        || {
            Box::new(Stage {
                ctx: GlContext::new(),
            })
        },
    );
}
