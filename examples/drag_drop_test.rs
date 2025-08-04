use miniquad::*;

struct Stage {
    ctx: GlContext,
    dropped_files: Vec<String>,
}

impl EventHandler for Stage {
    fn update(&mut self) {}

    fn draw(&mut self) {
        // Clear the screen with a dark blue color
        self.ctx.clear(Some((0.1, 0.1, 0.3, 1.0)), None, None);

        // Note: miniquad doesn't have built-in text rendering
        // In a real application, you would use a text rendering library
        // For now, this just clears the screen and responds to file drops
    }

    fn files_dropped_event(&mut self) {
        println!("Files dropped!");
        self.dropped_files.clear();

        let count = window::dropped_file_count();
        println!("Number of dropped files: {count}");

        for i in 0..count {
            if let Some(path) = window::dropped_file_path(i) {
                let path_str = path.to_string_lossy().to_string();
                println!("Dropped file {i}: {path_str}");
                self.dropped_files.push(path_str);

                if let Some(bytes) = window::dropped_file_bytes(i) {
                    println!("  Size: {} bytes", bytes.len());
                }
            }
        }

        println!("Check the console output above to see the dropped files!");
    }
}

fn main() {
    miniquad::start(
        conf::Conf {
            window_title: "Drag and Drop Test".to_string(),
            window_width: 800,
            window_height: 600,
            ..Default::default()
        },
        || {
            Box::new(Stage {
                ctx: GlContext::new(),
                dropped_files: Vec::new(),
            })
        },
    );
}
