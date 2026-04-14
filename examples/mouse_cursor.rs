use miniquad::*;

struct Stage {}

impl EventHandler for Stage {
    fn update(&mut self) {}

    fn draw(&mut self) {}

    fn char_event(&mut self, _character: char, _: KeyMods, _: bool) {
        #[cfg(target_os = "windows")]
        match _character {
            'z' => window::show_mouse(false),
            'x' => window::show_mouse(true),
            _ => (),
        }

        let _icon = match _character {
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
        #[cfg(target_os = "windows")]
        window::set_mouse_cursor(_icon);
    }
}

fn main() {
    miniquad::start(conf::Conf::default(), || Box::new(Stage {}));
}
