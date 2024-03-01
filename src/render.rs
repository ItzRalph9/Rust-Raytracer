use core::f64;

use minifb::Window;
use rayon::prelude::*;

use crate::check_input as input;
use crate::fps_counter::FpsCounter;
use crate::{color::Color, ray::Ray, scene::Scene, scene::SCENE};

use crate::constants::{WIDTH, HEIGHT};

pub fn render(window: &Window, mut buffer: Vec<Color>) -> (Vec<Color>, bool) {
    let reset_accumulation = input::check_input(&window, &mut SCENE.write().unwrap());

    if reset_accumulation {
        buffer = vec![Color::new(0.0, 0.0, 0.0); WIDTH * HEIGHT];
    }

    let scene = SCENE.read().unwrap();
    
    // Parallelize rendering using Rayon
    buffer
        .par_chunks_mut(WIDTH)
        .enumerate()
        .for_each(|(j, row)| {
            for i in 0..WIDTH {
                render_pixel(row, i, j, &scene);
            }
        });

    (buffer, reset_accumulation)
}

fn render_pixel(row: &mut[Color], i: usize, j: usize, scene: &Scene) {
    let mut color = Color::new(0.0, 0.0, 0.0);
    
    for _ in 0..scene.camera.defaults.samples_per_pixel {
        let ray = scene.camera.get_ray(i, j);
        color += ray_color(ray, scene.camera.defaults.max_depth, &scene);
    }

    write_color(&mut row[i], color, scene.camera.defaults.samples_per_pixel);
}

fn ray_color(ray: Ray, depth: usize, scene: &Scene) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(hit_object) = scene.hit(ray) {
        if let Some((attenuation, scattered)) = hit_object.material.scatter(ray, &hit_object) {
            return attenuation * ray_color(scattered, depth-1, scene);
        }

        return Color::new(0.0, 0.0, 0.0);
    }

    // background
    let unit_direction = ray.direction.normalize();
    let a = 0.5 * (unit_direction.y + 1.0);
    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}

fn write_color(pixel: &mut Color, mut color: Color, samples_per_pixel: usize) {
    color *= 1.0 / samples_per_pixel as f64;

    color = color.linear_to_gamma();
    color = color.clamp();

    *pixel += color;
}

pub fn get_clamped_buffer(buffer: &Vec<Color>, fps_counter: &mut FpsCounter, frame_index: &mut usize, reset_accumulation: bool) -> Vec<u32> {
    if reset_accumulation {
        *frame_index = 1;
    }

    let mut accumulator: Vec<u32> = vec![0; WIDTH * HEIGHT];
    accumulator.iter_mut().zip(buffer.iter()).for_each(|(acc_pixel, buffer_pixel)| {
        let color = *buffer_pixel * (1.0 / *frame_index as f64);
        *acc_pixel = color.to_u32();
    });

    fps_counter.update(&mut accumulator);

    *frame_index += 1;

    accumulator
}
