pub struct Interval {
    pub min: f64,
    pub max: f64
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Interval { min, max }
    }

    pub fn _contains(&self, value: f64) -> bool {
        self.min <= value && value <= self.max
    }

    pub fn _surrounds(&self, value: f64) -> bool {
        self.min < value && value < self.max
    }

    pub fn _get_empty() -> Self {
        Interval::new(f64::INFINITY, f64::NEG_INFINITY)
    }

    pub fn _get_universe() -> Self {
        Interval::new(f64::NEG_INFINITY, f64::INFINITY)
    }
}