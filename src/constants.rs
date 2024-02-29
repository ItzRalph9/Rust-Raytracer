use nalgebra::Vector3;

pub const ASPECT_RATIO: f64 = 16.0 / 9.0;
pub const WIDTH: usize = 720;
pub const HEIGHT: usize = (WIDTH as f64 / ASPECT_RATIO) as usize;
pub const SAMPLES_PER_PIXEL: usize = 100;
pub const MAX_DEPTH: usize = 50;

// local
const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT * (WIDTH as f64 / HEIGHT as f64);

// camera
pub const CAMERA_POSITION: Vector3<f64> = Vector3::new(0.0, 0.0, 0.0);
pub const CAMERA_FOCAL_LENGTH: f64 = 1.0;

pub const VIEWPORT_U: Vector3<f64> = Vector3::new(VIEWPORT_WIDTH, 0.0, 0.0);
pub const VIEWPORT_V: Vector3<f64> = Vector3::new(0.0, -VIEWPORT_HEIGHT, 0.0);

pub struct Viewport {}

impl Viewport {
    pub fn pixel_delta_u() -> Vector3<f64> {
        VIEWPORT_U / WIDTH as f64
    }

    pub fn pixel_delta_v() -> Vector3<f64> {
        VIEWPORT_V / HEIGHT as f64
    }

    pub fn pixel00_loc(camera_position: Vector3<f64>) -> Vector3<f64> {
        let viewport_upper_left = camera_position - Vector3::new(0.0, 0.0, CAMERA_FOCAL_LENGTH)
        - VIEWPORT_U / 2.0 - VIEWPORT_V / 2.0;

        viewport_upper_left + 0.5 * (Self::pixel_delta_u() + Self::pixel_delta_v())
    }
}