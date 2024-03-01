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

use fps_counter::Fps;
use minifb::{Key, Window, WindowOptions};

use constants::{WIDTH, HEIGHT};

fn main() {
    let mut window = Window::new("Renderer", WIDTH, HEIGHT, WindowOptions::default())
    .expect("Unable to create window");

    let mut fps_counter = Fps::start();

    // Event loop
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let mut buffer = render::render(&window);
        
        fps_counter.update();
        draw::draw_text(&mut buffer, &fps_counter.fps_text, 10, 10, 0xFFFFFFFF, WIDTH);

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
