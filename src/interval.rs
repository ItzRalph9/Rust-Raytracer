#[derive(Debug, Clone, Copy)]
pub struct Interval {
    pub min: f64,
    pub max: f64
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Interval { min, max }
    }

    pub fn new_from_interval(a: Interval, b: Interval) -> Self {
        Interval {
            min: a.min.min(b.min),
            max: a.max.max(b.max)
        }
    }

    pub fn contains(&self, value: f64) -> bool {
        self.min <= value && value <= self.max
    }

    pub fn _surrounds(&self, value: f64) -> bool {
        self.min < value && value < self.max
    }

    pub fn empty() -> Self {
        Interval::new(f64::INFINITY, f64::NEG_INFINITY)
    }

    pub fn _universe() -> Self {
        Interval::new(f64::NEG_INFINITY, f64::INFINITY)
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }
    
    pub fn expand(&self, delta: f64) -> Self {
        let padding = delta / 2.0;
        Interval::new(self.min - padding, self.max + padding)
    }
}

impl std::ops::Add<f64> for Interval {
    type Output = Interval;

    fn add(self, offset: f64) -> Interval {
        Self::new(self.min + offset, self.max + offset)
    }
}

impl std::ops::Add<Interval> for f64 {
    type Output = Interval;

    fn add(self, offset: Interval) -> Interval {
        offset + self
    }
}
