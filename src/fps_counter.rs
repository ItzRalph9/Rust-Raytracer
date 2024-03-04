use std::time::{Instant, Duration};
use crate::draw;
use crate::constants::WIDTH;

pub struct FpsCounter {
    last_second: Instant,
    frame_count: usize,
    pub fps_text: String,
}

impl FpsCounter {
    pub fn start() -> Self {
        FpsCounter {
            last_second: Instant::now(),
            frame_count: 0,
            fps_text: String::new()
        }
    }

    pub fn update(&mut self, clamped_buffer: &mut Vec<u32>) {
        let frame_start = Instant::now();
        self.frame_count += 1;

        if frame_start.duration_since(self.last_second) >= Duration::from_secs(1) {
            let frame_duration_ms = frame_start.duration_since(self.last_second).as_millis();
            self.fps_text = format!("frame time: {} ms", frame_duration_ms);
            
            self.frame_count = 0;
            self.last_second = Instant::now();
        }

        draw::draw_text(clamped_buffer, &self.fps_text, 10, 10, 0x000000, WIDTH);
    }
}
