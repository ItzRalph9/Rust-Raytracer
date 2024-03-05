use std::f64::consts::PI;
use rand::prelude::*;
use nalgebra::Vector3;

use crate::library::constants::{WIDTH, HEIGHT};
use crate::library::{ray::Ray, scene::Scene, material::Material, color::Color};

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub defaults: CameraDefaults,
    position: Vector3<f64>,

    pixel00_loc: Vector3<f64>,
    pixel_delta_u: Vector3<f64>,
    pixel_delta_v: Vector3<f64>,
    defocus_disk_u: Vector3<f64>,
    defocus_disk_v: Vector3<f64>,
}

impl Camera {
    pub fn init(default: CameraDefaults) -> Self {
        let position = default.lookfrom;

        // Determine viewport dimensions.
        let theta = Self::degrees_to_radians(default.vertical_fov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * default.focus_distance;
        let viewport_width: f64 = viewport_height * (WIDTH as f64 / HEIGHT as f64);

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        let w = (default.lookfrom - default.lookat).normalize();
        let u = default.vup.cross(&w).normalize();
        let v = w.cross(&u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / WIDTH as f64;
        let pixel_delta_v = viewport_v / HEIGHT as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left = position - (default.focus_distance * w) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        // Calculate the camera defocus disk basis vectors.
        let defocus_radius = default.focus_distance * Self::degrees_to_radians(default.defocus_angle / 2.0).tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Camera::new(default, position, pixel00_loc, pixel_delta_u, pixel_delta_v, defocus_disk_u, defocus_disk_v)
    }

    pub fn new(defaults: CameraDefaults, position: Vector3<f64>, pixel00_loc: Vector3<f64>,
        pixel_delta_u: Vector3<f64>, pixel_delta_v: Vector3<f64>, defocus_disk_u: Vector3<f64>, defocus_disk_v: Vector3<f64>) -> Self {
        Camera {
            position,
            defaults,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            defocus_disk_u,
            defocus_disk_v,
        }
    }

    pub fn _update_camera(&mut self, scene: &mut Scene) {
        scene.camera = Camera::init(self.defaults);
    }

    // Get a randomly-sampled camera ray for the pixel at location i,j
    // originating from the camera defocus disk.
    pub fn get_ray(&self, i: usize, j: usize) -> Ray {
        let delta_u = self.pixel_delta_u;
        let delta_v = self.pixel_delta_v;
        let pixel_center = self.pixel00_loc + (i as f64 * delta_u) + (j as f64 * delta_v);   

        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_origin = self.defocus_disk_sample();
        let ray_direction = pixel_sample - ray_origin;
        let ray_time = Material::random_float();

        Ray::new(ray_origin, ray_direction, ray_time)
    }
    
    pub fn degrees_to_radians(degrees: f64) -> f64 {
        degrees * PI / 180.0
    }

    // Returns a random point in the square surrounding a pixel at the origin.
    fn pixel_sample_square(&self) -> Vector3<f64> {
        let mut rng = rand::thread_rng();

        let px = -0.5 + rng.gen::<f64>();
        let py = -0.5 + rng.gen::<f64>();

        (px * self.pixel_delta_u) + (py * self.pixel_delta_v)
    }

    // Returns a random point in the camera defocus disk.
    fn defocus_disk_sample(&self) -> Vector3<f64> {
        let p = Ray::random_in_unit_disk();
        self.position + (p.x * self.defocus_disk_u) + (p.y * self.defocus_disk_v)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CameraDefaults {
    pub samples_per_pixel: usize,
    pub max_depth: usize,
    pub background: Color,
    pub vertical_fov: f64, 
    pub lookfrom: Vector3<f64>, 
    pub lookat: Vector3<f64>, 
    pub vup: Vector3<f64>, 
    pub defocus_angle: f64, 
    pub focus_distance: f64
}