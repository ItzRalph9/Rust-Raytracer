#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color { r, g, b }
    }

    pub fn to_u32(&self) -> u32 {
        let r = (256.0 * self.r) as u32;
        let g = (256.0 * self.g) as u32;
        let b = (256.0 * self.b) as u32;

        (r << 16) | (g << 8 ) | b
    }

    pub fn linear_to_gamma(&self) -> Color {
        Color::new(
            self.r.sqrt(),
            self.g.sqrt(),
            self.b.sqrt(),
        )
    }

    pub fn clamp(&self) -> Self {
        Color::new(
            self.r.clamp(0.000, 0.999),
            self.g.clamp(0.000, 0.999),
            self.b.clamp(0.000, 0.999),
        )
    }
}

impl std::ops::Add<Color> for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        let r = self.r + other.r;
        let g = self.g + other.g;
        let b = self.b + other.b;

        Color { r, g, b}
    }
}

impl std::ops::AddAssign for Color {
    fn add_assign(&mut self, other: Color) {
        self.r += other.r;
        self.g += other.g;
        self.b += other.b;
    }
}


impl std::ops::Div<f64> for Color {
    type Output = Color;

    fn div(self, scalar: f64) -> Color {
        let r = self.r / scalar;
        let g = self.g / scalar;
        let b = self.b / scalar;

        Color { r, g, b}
    }
}

impl std::ops::DivAssign<f64> for Color {
    fn div_assign(&mut self, scalar: f64) {
        self.r /= scalar;
        self.g /= scalar;
        self.b /= scalar;
    }
}

impl std::ops::Mul<f64> for Color {
    type Output = Color;

    fn mul(self, scalar: f64) -> Color {
        let r = self.r * scalar;
        let g = self.g * scalar;
        let b = self.b * scalar;

        Color { r, g, b}
    }
}

impl std::ops::Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, color: Color) -> Color {
        color * self
    }
}

impl std::ops::Mul<Color> for Color {
    type Output = Color;

    fn mul(self, color: Color) -> Color {
        Color {
            r: self.r * color.r,
            g: self.g * color.g,
            b: self.b * color.b,
        }
    }
}