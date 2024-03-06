use nalgebra::Vector3;

use crate::library::basic_lib::*;
use crate::library::{hittable::{Hittable, HittableTrait}, material::Material, quad::Quad};

#[derive(Debug, Clone)]
pub struct Quadbox {
    pub objects: Vec<Hittable>,
}

impl Quadbox {
    // Returns the 3D box (six sides) that contains the two opposite vertices a & b.
    pub fn new(a: Vector3<f64>, b: Vector3<f64>, material: Material) -> Self {
        let mut objects = vec![];

        // Construct the two opposite vertices with the minimum and maximum coordinates.
        let min = Vector3::new(
            a.x.min(b.x),
            a.y.min(b.y),
            a.z.min(b.z),
        );

        let max = Vector3::new(
            a.x.max(b.x),
            a.y.max(b.y),
            a.z.max(b.z),
        );

        let dx = Vector3::new(max.x - min.x, 0., 0.);
        let dy = Vector3::new(0., max.y - min.y, 0.);
        let dz = Vector3::new(0., 0., max.z - min.z);

        let front =  Quad::new(Vector3::new(min.x, min.y, max.z), dx, dy, material.clone());
        let right =  Quad::new(Vector3::new(max.x, min.y, max.z),-dz, dy, material.clone());
        let back =   Quad::new(Vector3::new(max.x, min.y, min.z),-dx, dy, material.clone());
        let left =   Quad::new(Vector3::new(min.x, min.y, min.z), dz, dy, material.clone());
        let top =    Quad::new(Vector3::new(min.x, max.y, max.z), dx,-dz, material.clone());
        let bottom = Quad::new(Vector3::new(min.x, min.y, min.z), dx, dz, material.clone());

        objects.push(Hittable::Quad(front));
        objects.push(Hittable::Quad(right));
        objects.push(Hittable::Quad(back));
        objects.push(Hittable::Quad(left));
        objects.push(Hittable::Quad(top));
        objects.push(Hittable::Quad(bottom));

        Self { objects }
    }
}

impl HittableTrait for Quadbox {
    fn hit(&self, ray: Ray, ray_t: Interval) -> Option<HitObject> {
        let (_closest, hit_record) = self.objects.iter().fold(
            (ray_t.max, None),
            |acc, item| {
                if let Some(temp_rec) =
                    item.hit(ray, Interval::new(ray_t.min, acc.0))
                {
                    (temp_rec.t, Some(temp_rec))
                } else {
                    acc
                }
            },
        );

        hit_record
    }

    fn get_bounding_box(&self) -> Aabb {
        Aabb::new(Interval::universe(), Interval::universe(), Interval::universe())
    }
}
