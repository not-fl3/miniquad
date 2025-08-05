use miniquad::*;

struct Stage {
    ctx: GlContext,
}

impl EventHandler for Stage {
    fn update(&mut self) {}

    fn draw(&mut self) {
        self.ctx.clear(Some((0.2, 0.2, 0.2, 1.0)), None, None);

        let metrics = window::screen_metrics();
        println!("Screen Metrics:");
        println!("  Size: {}x{}", metrics.width, metrics.height);
        println!("  Position: {:?}", metrics.position);
        println!("  DPI Scale: {}", metrics.dpi_scale);
        println!("  High DPI: {}", metrics.high_dpi);

        let (width, height) = window::screen_size();
        println!("Screen Size (legacy): {}x{}", width, height);

        println!("DPI Scale (legacy): {}", window::dpi_scale());
        println!("High DPI (legacy): {}", window::high_dpi());
        println!("---");
    }
}

fn main() {
    miniquad::start(
        conf::Conf {
            window_title: "Screen Metrics Demo".to_string(),
            window_width: 800,
            window_height: 600,
            high_dpi: true,
            ..Default::default()
        },
        || {
            Box::new(Stage {
                ctx: GlContext::new(),
            })
        },
    );
}
