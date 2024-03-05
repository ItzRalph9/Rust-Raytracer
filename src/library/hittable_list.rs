use crate::library::basic_lib::*;
use crate::library::hittable::Hittable;

#[derive(Debug, Clone)]
pub struct HittableList {
    pub objects: Vec<Hittable>,
    bounding_box: Aabb,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
            bounding_box: Aabb::new(Interval::empty(), Interval::empty(), Interval::empty()),
        }
    }

    pub fn _new_from_list(object: Hittable) -> Self {
        let mut hittable_list = Self::new();
        hittable_list.add(object);

        hittable_list
    }

    pub fn add(&mut self, object: Hittable) {
        self.bounding_box = Aabb::new_from_box(self.bounding_box, object.get_bounding_box());
        self.objects.push(object);
    }

    fn _get_bounding_box(&self) -> Aabb {
        self.bounding_box
    }

    pub fn hit(&self, ray: Ray, ray_t: Interval) -> Option<HitObject> {
        let mut closest_so_far = ray_t.max;
        let mut temp_hit_object = None;

        for object in self.objects.iter() {
            if let Some(hit_object) = object.hit(ray, Interval::new(ray_t.min, closest_so_far)) {
                closest_so_far = hit_object.t;
                temp_hit_object = Some(hit_object);
            }
        }

        temp_hit_object
    }
}