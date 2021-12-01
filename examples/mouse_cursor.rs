use miniquad::*;

struct Stage {}

impl EventHandler for Stage {
    fn update(&mut self, _ctx: &mut Context) {}

    fn draw(&mut self, _ctx: &mut Context) {}

    fn char_event(&mut self, ctx: &mut Context, character: char, _: KeyMods, _: bool) {
        match character {
            'z' => ctx.show_mouse(false),
            'x' => ctx.show_mouse(true),
            _ => (),
        }

        let icon = match character {
            '1' => CursorIcon::Default,
            '2' => CursorIcon::Help,
            '3' => CursorIcon::Pointer,
            '4' => CursorIcon::Wait,
            '5' => CursorIcon::Crosshair,
            '6' => CursorIcon::Text,
            '7' => CursorIcon::Move,
            '8' => CursorIcon::NotAllowed,
            '9' => CursorIcon::EWResize,
            '0' => CursorIcon::NSResize,
            'q' => CursorIcon::NESWResize,
            'w' => CursorIcon::NWSEResize,
            _ => return,
        };
        ctx.set_mouse_cursor(icon);
    }
}

fn main() {
    miniquad::start(conf::Conf::default(), |_ctx| Box::new(Stage {}));
}
