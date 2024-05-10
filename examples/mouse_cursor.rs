use miniquad_wasm_bindgen::*;

struct Stage(usize);

impl EventHandler for Stage {
	fn update(&mut self) {}
	fn draw(&mut self) {}

	fn char_event(&mut self, character: char, _: KeyMods, _: bool) {
		match character {
			'z' => window::show_mouse(false),
			'x' => window::show_mouse(true),
			_ => (),
		}
	}

	fn raw_mouse_motion(&mut self, dx: f32, dy: f32) {
		println!("Mouse moved by: {} {}", dx, dy);
	}

	fn mouse_motion_event(&mut self, _x: f32, _y: f32) {
		println!("Raw Mouse moved to: {} {}", _x, _y);
	}

	fn mouse_button_down_event(&mut self, button: MouseButton, _: f32, _: f32) {
		static CURSORS: [CursorIcon; 12] = [
			CursorIcon::Default,
			CursorIcon::Help,
			CursorIcon::Pointer,
			CursorIcon::Wait,
			CursorIcon::Crosshair,
			CursorIcon::Text,
			CursorIcon::Move,
			CursorIcon::NotAllowed,
			CursorIcon::EWResize,
			CursorIcon::NSResize,
			CursorIcon::NESWResize,
			CursorIcon::NWSEResize,
		];

		// test extra buttons
		match button {
			MouseButton::Other(4) => {
				self.0 = (self.0 + 1) % CURSORS.len();
				window::set_mouse_cursor(CURSORS[self.0]);
			}
			MouseButton::Other(5) => {
				self.0 = (self.0 + CURSORS.len() - 1) % CURSORS.len();
				window::set_mouse_cursor(CURSORS[self.0]);
			}
			MouseButton::Left => {
				self.0 = (self.0 + CURSORS.len() - 1) % CURSORS.len();
				window::set_mouse_cursor(CURSORS[self.0]);
			}
			MouseButton::Right => {
				self.0 = (self.0 + 1) % CURSORS.len();
				window::set_mouse_cursor(CURSORS[self.0]);
			}
			_ => {}
		}
	}
}

fn main() {
	miniquad_wasm_bindgen::start(conf::Conf::default(), || Box::new(Stage(0)));
}
