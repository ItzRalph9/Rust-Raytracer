use nalgebra::Vector3;

use crate::{color::Color, image::Image};

#[derive(Debug, Clone)]
pub enum Texture {
    SolidColor(Color),
    Checkered(f64, Color, Color),
    Image(Image),
}

impl Texture {
    pub fn value(&self, u: f64, v: f64, point: Vector3<f64>) -> Color {
        match self {
            Texture::SolidColor(color) => {
                *color
            },
            Texture::Checkered(scale, even, odd) => {
                let inv_scale = 1.0 / scale;

                let x_integer = (inv_scale * point.x).floor();
                let y_integer = (inv_scale * point.y).floor();
                let z_integer = (inv_scale * point.z).floor();

                let is_even = (x_integer + y_integer + z_integer) % 2.0 == 0.0;

                if is_even {
                    *even
                } else {
                    *odd
                }
            }
            Texture::Image(image) => {
                // If we have no texture data, then return solid cyan as a debugging aid.
                if image.height() <= 0 {
                    return Color::new(0.0, 1.0, 1.0);
                }

                let u = u.clamp(0.0, 1.0);
                let v = 1.0 - v.clamp(0.0, 1.0);  // Flip V to image coordinates

                let i = u * image.width() as f64;
                let j = v * image.height() as f64;
                let pixel = image.pixel_data(i as u32, j as u32);

                pixel
            }
        }
    }
}