use lokinit::*;

struct Stage {}
impl EventHandler for Stage {
    fn update(&mut self, _ctx: &mut Context) {}

    fn draw(&mut self, _ctx: &mut Context) {
        unsafe {
            gl::glClearColor(0.3, 0.4, 0.5, 1.);
            gl::glClear(gl::GL_COLOR_BUFFER_BIT);
        }
    }
}

fn main() {
    lokinit::start(
        conf::Conf {
            window_title: "Lokinit".to_string(),
            window_width: 1024,
            window_height: 768,
            ..Default::default()
        },
        |_ctx| Box::new(Stage {}),
    );
}
