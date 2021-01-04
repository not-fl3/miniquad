use miniquad::*;

struct Stage { is_fullscreen: bool }
impl EventHandler for Stage {
    fn update(&mut self, _ctx: &mut Context) {}

    fn draw(&mut self, ctx: &mut Context) {
        ctx.clear(Some((0., 1., 0., 1.)), None, None);
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool) {

            if keycode == KeyCode::A {
                self.is_fullscreen = true;
            } else if keycode == KeyCode::J {
                self.is_fullscreen = false;
            }


            ctx.set_fullscreen(self.is_fullscreen);
        }
}

fn main() {
    miniquad::start(conf::Conf::default(), |ctx| UserData::owning(Stage { is_fullscreen: false, }, ctx));
}
