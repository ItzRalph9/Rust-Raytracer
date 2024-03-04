use image::{DynamicImage, GenericImageView};

use crate::color::Color;

#[derive(Debug, Clone)]
pub struct Image {
    pub image: DynamicImage,
}

impl Image {
    pub fn load_image(file_path: &str) -> Self {
        let image = image::open(file_path).expect("Failed to open image");
        
        Image { image }
    }

    pub fn pixel_data(&self, x: u32, y: u32) -> Color {
        let pixel = self.image.get_pixel(x, y);

        let r = pixel[0];
        let g = pixel[1];
        let b = pixel[2];

        Color::from_u8(r, g, b)
    }

    pub fn width(&self) -> u32 {
        self.image.width()
    }

    pub fn height(&self) -> u32 {
        self.image.height()
    }
}