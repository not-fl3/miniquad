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
            window_title: "Desktop Center Test".to_string(),
            window_width: 640,
            window_height: 480,
            desktop_center: true, // Center window on desktop
            ..Default::default()
        },
        || {
            Box::new(Stage {
                ctx: GlContext::new(),
            })
        },
    );
}
