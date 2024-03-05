use nalgebra::Vector3;

use crate::library::{interval::Interval, ray::Ray};

#[derive(Debug, Clone, Copy)]
pub struct Aabb {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl Aabb {
    pub fn new(ix: Interval, iy: Interval, iz: Interval) -> Self {
        Aabb { x: ix, y: iy, z: iz }
    }

    pub fn new_from_point(a: Vector3<f64>, b: Vector3<f64>) -> Self {
        Aabb {
            x: Interval::new(a.x.min(b.x), a.x.max(b.x)),
            y: Interval::new(a.y.min(b.y), a.y.max(b.y)),
            z: Interval::new(a.z.min(b.z), a.z.max(b.z)),
        }
    }

    pub fn new_from_box(box0: Aabb, box1: Aabb) -> Self {
        Aabb {
            x: Interval::new_from_interval(box0.x, box1.x),
            y: Interval::new_from_interval(box0.y, box1.y),
            z: Interval::new_from_interval(box0.z, box1.z),
        }
    }

    pub fn hit(&self, ray: Ray, ray_t: Interval) -> Option<Interval> {
        let mut t_result: Option<Interval> = None;

        for a in 0..3 {
            let index = a as usize;

            if let (Some(origin), Some(direction)) = (ray.origin.get(index), ray.direction.get(index)) {
                let inverse_direction = 1.0 / direction;

                let mut t0 = (self.axis(a).min - origin) * inverse_direction;
                let mut t1 = (self.axis(a).max - origin) * inverse_direction;

                if inverse_direction < 0.0 {
                    (t0, t1) = (t1, t0);
                }

                let mut min = ray_t.min;
                let mut max = ray_t.max;

                if t0 > min {
                    min = t0;
                }

                if t1 < max {
                    max = t1;
                }
                
                if max <= min {
                    return None;
                }

                t_result = Some(Interval::new(min, max));
            }
        }

        t_result
    }

    pub fn axis(&self, n: i32) -> Interval {
        match n {
            1 => self.y,
            2 => self.z,
            _ => self.x,
        }
    }

    // Return an AABB that has no side narrower than some delta, add padding if necessary.
    pub fn pad(&self) -> Self {
        let delta = 0.0001;
        let new_x = if self.x.size() >= delta { self.x } else { self.x.expand(delta) };
        let new_y = if self.y.size() >= delta { self.y } else { self.y.expand(delta) };
        let new_z = if self.z.size() >= delta { self.z } else { self.z.expand(delta) };

        Aabb::new(new_x, new_y, new_z)
    }
}

impl std::ops::Add<Vector3<f64>> for Aabb {
    type Output = Aabb;

    fn add(self, offset: Vector3<f64>) -> Aabb {
        Self::new(self.x + offset.x, self.y + offset.y, self.z + offset.z)
    }
}
impl std::ops::Add<Aabb> for Vector3<f64> {
    type Output = Aabb;

    fn add(self, offset: Aabb) -> Aabb {
        offset + self
    }
}
