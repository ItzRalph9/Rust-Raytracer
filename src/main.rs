mod render;
mod check_input;
mod sphere;
mod ray;
mod color;
mod constants;
mod scene;
mod hit_object;
mod draw;
mod fps_counter;
mod material;
mod vector3;
mod camera;

extern crate minifb;

use color::Color;
use fps_counter::FpsCounter;
use minifb::{Key, Window, WindowOptions};

use constants::{WIDTH, HEIGHT};

fn main() {
    let mut window = Window::new("Renderer", WIDTH, HEIGHT, WindowOptions::default())
    .expect("Unable to create window");

    let mut fps_counter = FpsCounter::start();
    let mut frame_index = 1;

    let mut buffer = vec![Color::new(0.0, 0.0, 0.0); WIDTH * HEIGHT];
    
    // Event loop
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let reset_accumulation;
        (buffer, reset_accumulation) = render::render(&window, buffer);

        let clamped_buffer = render::get_clamped_buffer(&buffer, &mut fps_counter, &mut frame_index, reset_accumulation);
        
        window.update_with_buffer(&clamped_buffer, WIDTH, HEIGHT).unwrap();
    }
}
