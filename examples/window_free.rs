use miniquad::*;

struct Stage {
    ctx: Context,
}
impl EventHandlerFree for Stage {
    fn update(&mut self) {}

    fn draw(&mut self) {
        self.ctx.clear(Some((0., 1., 0., 1.)), None, None);
    }
}

fn main() {
    miniquad::start(
        conf::Conf {
            window_title: "Miniquad".to_string(),
            window_width: 1024,
            window_height: 768,
            fullscreen: true,
            ..Default::default()
        },
        |ctx| UserData::free(Stage { ctx }),
    );
}
