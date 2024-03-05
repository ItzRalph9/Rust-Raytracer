use nalgebra::Vector3;

use crate::{color::Color, image::Image, perlin::Perlin};


#[derive(Debug, Clone)]
pub enum Texture {
    SolidColor(Color),
    Checkered(f64, Color, Color),
    Image(Image),
    Perlin(Perlin, f64)
}

impl Texture {
    pub fn value(&self, u: f64, v: f64, p: Vector3<f64>) -> Color {
        match self {
            Texture::SolidColor(color) => {
                *color
            },
            Texture::Checkered(scale, even, odd) => {
                let inv_scale = 1.0 / scale;

                let x_integer = (inv_scale * p.x).floor();
                let y_integer = (inv_scale * p.y).floor();
                let z_integer = (inv_scale * p.z).floor();

                let is_even = (x_integer + y_integer + z_integer) % 2.0 == 0.0;

                if is_even {
                    *even
                } else {
                    *odd
                }
            }
            Texture::Image(image) => {
                let u = u.clamp(0.0, 1.0);
                let v = 1.0 - v.clamp(0.0, 1.0);  // Flip V to image coordinates

                let i = u * image.width() as f64;
                let j = v * image.height() as f64;
                let pixel = image.pixel_data(i as u32, j as u32);

                pixel
            }
            Texture::Perlin(perlin_noise, scale) => {
                let s = *scale * p;

                let sin = (s.z + 10.0 * perlin_noise.turbulance(s, None)).sin();
                Color::new(1.0, 1.0, 1.0) * 0.5 * (1.0 + sin)
            },
        }
    }
}