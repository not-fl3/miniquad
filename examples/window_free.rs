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
    miniquad::start(conf::Conf::default(), |ctx| UserData::free(Stage { ctx }));
}
