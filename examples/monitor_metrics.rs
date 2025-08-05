use miniquad::*;

struct Stage {
    ctx: GlContext,
}

impl EventHandler for Stage {
    fn update(&mut self) {}

    fn draw(&mut self) {
        self.ctx.clear(Some((0.0, 0.0, 0.0, 1.0)), None, None);

        // Print monitor information every 60 frames (approximately once per second at 60fps)
        static mut FRAME_COUNT: u32 = 0;
        unsafe {
            FRAME_COUNT += 1;
            if FRAME_COUNT % 60 == 0 {
                println!("=== Monitor Information ===");

                // Primary monitor
                let primary = window::primary_monitor();
                println!("Primary Monitor:");
                println!("  Name: {:?}", primary.name);
                println!("  Size: {}x{}", primary.width, primary.height);
                println!("  Position: {:?}", primary.position);
                println!("  DPI Scale: {}", primary.dpi_scale);
                println!("  Refresh Rate: {:?}", primary.refresh_rate);

                // Current monitor (where the window is displayed)
                let current = window::current_monitor();
                println!("\nCurrent Monitor (where window is displayed):");
                println!("  Name: {:?}", current.name);
                println!("  Size: {}x{}", current.width, current.height);
                println!("  Position: {:?}", current.position);
                println!("  DPI Scale: {}", current.dpi_scale);
                println!("  Refresh Rate: {:?}", current.refresh_rate);

                // All monitors
                let monitors = window::monitors();
                println!("\nAll Monitors ({} total):", monitors.len());
                for (i, monitor) in monitors.iter().enumerate() {
                    println!("  Monitor {}:", i + 1);
                    println!("    Name: {:?}", monitor.name);
                    println!("    Size: {}x{}", monitor.width, monitor.height);
                    println!("    Position: {:?}", monitor.position);
                    println!("    DPI Scale: {}", monitor.dpi_scale);
                    println!("    Refresh Rate: {:?}", monitor.refresh_rate);
                }

                // Compare with window metrics
                let window_metrics = window::screen_metrics();
                println!("\nWindow Metrics (for comparison):");
                println!("  Size: {}x{}", window_metrics.width, window_metrics.height);
                println!("  Position: {:?}", window_metrics.position);
                println!("  DPI Scale: {}", window_metrics.dpi_scale);
                println!("  High DPI: {}", window_metrics.high_dpi);
                println!("---");
            }
        }
    }
}

fn main() {
    miniquad::start(conf::Conf::default(), || {
        Box::new(Stage {
            ctx: GlContext::new(),
        })
    });
}
