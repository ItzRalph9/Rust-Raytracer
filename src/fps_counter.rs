use std::time::Instant;
use crate::draw;
use crate::constants::WIDTH;

pub struct FpsCounter {
    last_second: Instant,
    frame: usize,
}

impl FpsCounter {
    pub fn start() -> Self {
        FpsCounter {
            last_second: Instant::now(),
            frame: 1,
        }
    }

    pub fn update(&mut self, clamped_buffer: &mut Vec<u32>) {
        let frame_start = Instant::now();

        let frame_duration_ms = frame_start.duration_since(self.last_second).as_millis();
        let fps_text = format!("time: {} ms", frame_duration_ms);
        let frame_text = format!("frame: {}", self.frame);
        
        self.last_second = Instant::now();
        self.frame += 1;

        draw::draw_text(clamped_buffer, &fps_text, 5, 5, 0x00FF00, WIDTH);
        draw::draw_text(clamped_buffer, &frame_text, 5, 18, 0x00FF00, WIDTH);
    }
}
