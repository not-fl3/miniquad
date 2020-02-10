use miniquad::*;

struct Stage;
impl EventHandler for Stage {
    fn update(&mut self, _ctx: &mut Context) {}

    fn draw(&mut self, ctx: &mut Context) {
        ctx.clear(Some((0., 1., 0., 1.)), None, None);
    }
}

fn main() {
    miniquad::start(conf::Conf::default(), |ctx| UserData::owning(Stage, ctx));
}
