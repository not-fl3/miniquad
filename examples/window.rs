use miniquad::*;

struct Stage;
impl EventHandler for Stage {
    fn update(&mut self, _ctx: &mut Context) {}

    fn draw(&mut self, _ctx: &mut Context) {}
}

fn main() {
    miniquad::start(conf::Conf::default(), |ctx| UserData::owning(Stage, ctx));
}
