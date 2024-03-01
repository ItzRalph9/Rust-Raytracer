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
use fps_counter::Fps;
use minifb::{Key, Window, WindowOptions};

use constants::{WIDTH, HEIGHT};

fn main() {
    let mut window = Window::new("Renderer", WIDTH, HEIGHT, WindowOptions::default())
    .expect("Unable to create window");

    let mut fps_counter = Fps::start();
    let mut frame_index = 0;

    let mut buffer = vec![Color::new(0.0, 0.0, 0.0); WIDTH * HEIGHT];

    // Event loop
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let reset_accumulation;
        (buffer, reset_accumulation) = render::render(&window, buffer);
        
        if reset_accumulation {
            frame_index = 0;
        }

        frame_index += 1;

        let mut accumulator: Vec<u32> = vec![0; WIDTH * HEIGHT];
        accumulator.iter_mut().zip(buffer.iter()).for_each(|(acc_pixel, buffer_pixel)| {
            let color = *buffer_pixel * (1.0 / frame_index as f64);
            *acc_pixel = color.to_u32();
        });

        fps_counter.update();
        draw::draw_text(&mut accumulator, &fps_counter.fps_text, 10, 10, 0x000000, WIDTH);
        
        window.update_with_buffer(&accumulator, WIDTH, HEIGHT).unwrap();
    }
}
