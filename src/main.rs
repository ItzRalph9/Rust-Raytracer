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
mod interval;
mod aabb;
mod hittable_list;
mod hittable;
mod bvh;
mod texture;
mod image;

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
        buffer = render::render(buffer);

        let clamped_buffer = render::get_clamped_buffer(&buffer, &mut fps_counter, &mut frame_index);
        
        window.update_with_buffer(&clamped_buffer, WIDTH, HEIGHT).unwrap();
    }
}
