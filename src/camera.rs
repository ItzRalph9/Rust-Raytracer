use std::f64::consts::PI;
use rand::prelude::*;
use nalgebra::Vector3;

use crate::constants::{WIDTH, HEIGHT};
use crate::ray::Ray;

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub position: Vector3<f64>,
    pub focal_length: f64,
    
    pub vertical_fov: f64,
    pub lookfrom: Vector3<f64>,
    pub lookat : Vector3<f64>,
    pub vup: Vector3<f64>,

    pub samples_per_pixel: usize,
    pub max_depth: usize,

    pixel00_loc: Vector3<f64>,
    pixel_delta_u: Vector3<f64>,
    pixel_delta_v: Vector3<f64>,
    u: Vector3<f64>, v: Vector3<f64>, w: Vector3<f64>,
}

impl Camera {
    pub fn init() -> Self {
        // predefined values
        let vertical_fov = 20.0;
        let lookfrom = Vector3::new(-2.0, 2.0, 1.0);
        let lookat  = Vector3::new(0.0, 0.0, -1.0);
        let vup = Vector3::new(0.0, 1.0, 0.0);
        let position = lookfrom;

        // Determine viewport dimensions.
        let focal_length = (lookfrom - lookat).magnitude();
        let theta = (vertical_fov * PI) / 180.0;
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focal_length;
        let viewport_width: f64 = viewport_height * (WIDTH as f64 / HEIGHT as f64);

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        let w = (lookfrom - lookat).normalize();
        let u = vup.cross(&w).normalize();
        let v = w.cross(&u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / WIDTH as f64;
        let pixel_delta_v = viewport_v / HEIGHT as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left = position - (focal_length * w) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Camera {
            position, focal_length, vertical_fov, lookfrom, lookat, vup,
            samples_per_pixel: 100,
            max_depth: 50,
            pixel00_loc, pixel_delta_u, pixel_delta_v, u, v, w
        }
    }

    pub fn get_ray(&self, i: usize, j: usize) -> Ray {
        let delta_u = self.pixel_delta_u;
        let delta_v = self.pixel_delta_v;
        let pixel_center = self.pixel00_loc + (i as f64 * delta_u) + (j as f64 * delta_v);   

        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_origin = self.position;
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    // Returns a random point in the square surrounding a pixel at the origin.
    fn pixel_sample_square(&self) -> Vector3<f64> {
        let mut rng = rand::thread_rng();

        let px = -0.5 + rng.gen::<f64>();
        let py = -0.5 + rng.gen::<f64>();

        (px * self.pixel_delta_u) + (py * self.pixel_delta_v)
    }
}