use std::time::{Instant, Duration};

pub struct Fps {
    last_second: Instant,
    frame_count: usize,
    pub fps_text: String,
}

impl Fps {
    pub fn start() -> Self {
        Fps {
            last_second: Instant::now(),
            frame_count: 0,
            fps_text: String::new()
        }
    }

    pub fn update(&mut self) {
        let frame_start = Instant::now();
        self.frame_count += 1;

        if frame_start.duration_since(self.last_second) >= Duration::from_secs(1) {
            self.fps_text = format!("FPS: {}", self.frame_count);
            
            self.frame_count = 0;
            self.last_second = Instant::now();
        }
    }
}
