use miniquad::*;

struct Stage {
    is_fullscreen: bool,
}
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
        _repeat: bool,
    ) {
        if keycode == KeyCode::A {
            self.is_fullscreen = true;
            ctx.set_fullscreen(self.is_fullscreen);
        } else if keycode == KeyCode::J {
            self.is_fullscreen = false;
            ctx.set_fullscreen(self.is_fullscreen);
        } else if keycode == KeyCode::W {
            ctx.set_window_size(480, 320);
        } else if keycode == KeyCode::E {
            ctx.set_window_size(800, 500);
        } else if keycode == KeyCode::R {
            ctx.set_window_size(1600, 900);
        }
    }
}

fn main() {
    miniquad::start(conf::Conf::default(), |ctx| {
        UserData::owning(
            Stage {
                is_fullscreen: false,
            },
            ctx,
        )
    });
}
