use core::f64;

use minifb::Window;
use rayon::prelude::*;

use crate::check_input as input;
use crate::fps_counter::FpsCounter;
use crate::{color::Color, ray::Ray, scene::Scene, scene::SCENE, interval::Interval};

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

    let interval = Interval::new(0.001, f64::INFINITY);
    if let Some(hit_object) = scene.hittable_list.hit(ray, interval) {
        let color_from_emission = hit_object.material.emitted(hit_object.u, hit_object.v, hit_object.point);
        
        if let Some((attenuation, scattered)) = hit_object.material.scatter(ray, &hit_object) {
            let color_from_scatter = attenuation * ray_color(scattered, depth-1, scene);
            return color_from_emission + color_from_scatter;
        }
        
        return color_from_emission;
    }

    scene.camera.defaults.background
}

fn write_color(pixel: &mut Color, mut color: Color, samples_per_pixel: usize) {
    color *= 1.0 / samples_per_pixel as f64;

    *pixel += color;
}

pub fn get_clamped_buffer(buffer: &Vec<Color>, fps_counter: &mut FpsCounter, frame_index: &mut usize, reset_accumulation: bool) -> Vec<u32> {
    if reset_accumulation {
        *frame_index = 1;
    }
    
    let mut accumulator: Vec<u32> = vec![0; WIDTH * HEIGHT];
    accumulator.iter_mut().zip(buffer.iter()).for_each(|(acc_pixel, buffer_pixel)| {
        let mut color = *buffer_pixel * (1.0 / *frame_index as f64);
        color = color.linear_to_gamma();
        color = color.clamp();
        *acc_pixel = color.to_u32();
    });

    fps_counter.update(&mut accumulator);

    *frame_index += 1;

    accumulator
}
