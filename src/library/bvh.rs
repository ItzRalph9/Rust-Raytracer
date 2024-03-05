// use std::{cmp::Ordering, sync::Arc};
// use rand::prelude::*;

// use crate::{aabb::Aabb, hit_object::HitObject, hittable::{Hittable, HittableTrait}, hittable_list::HittableList, interval::Interval, ray::Ray};

// #[derive(Debug, Clone)]
// pub struct BvhNode {
//     left: Hittable,
//     right: Hittable,
//     bounding_box: Aabb,
// }

// impl BvhNode {
//     pub fn new(src_objects: &Vec<Hittable>, start: usize, end: usize) -> Self {
//         let mut objects: Vec<Hittable> = src_objects.iter().cloned().collect();

//         let axis = Self::random_int(0..3);

//         let comparator = match axis {
//             0 => Self::box_x_compare,
//             1 => Self::box_y_compare,
//             _ => Self::box_z_compare,
//         };

//         let object_span = end - start;

//         let left;
//         let right;

//         if object_span == 1 {
//             left = &objects[start];
//             right = &objects[start];
//         } else if object_span == 2 {
//             if comparator(&objects[start], &objects[start + 1]) == Ordering::Less {
//                 left = &objects[start];
//                 right = &objects[start + 1];
//             } else {
//                 left = &objects[start + 1];
//                 right = &objects[start];
//             }
//         } else {
//             objects[start..end].sort_by(|a, b| comparator(a, b));

//             let mid = start + object_span / 2;
//             left = &Hittable::Bvh(Self::new(&objects, start, mid));
//             right = &Hittable::Bvh(Self::new(&objects, mid, end));
//         }

//         let bounding_box = Aabb::new_from_box(left.get_bounding_box(), right.get_bounding_box());

//         BvhNode { left, right, bounding_box }
//     }

//     pub fn new_from_list(list: HittableList) -> Self {
//         let length = list.objects.len();
//         Self::new(&list.objects, 0, length)
//     }

//     fn random_int(range: std::ops::Range<i32>) -> i32 {
//         let mut rng = rand::thread_rng();
//         rng.gen_range(range)
//     }

//     fn box_compare(a: &Hittable, b: &Hittable, axis_index: i32) -> std::cmp::Ordering {
//         let a_min = a.get_bounding_box().axis(axis_index).min;
//         let b_min = b.get_bounding_box().axis(axis_index).min;

//         if a_min < b_min {
//             Ordering::Less
//         } else if a_min > b_min {
//             Ordering::Greater
//         } else {
//             Ordering::Equal
//         }
//     }

//     fn box_x_compare(a: &Hittable, b: &Hittable) -> std::cmp::Ordering {
//         Self::box_compare(a, b, 0)
//     }

//     fn box_y_compare(a: &Hittable, b: &Hittable) -> std::cmp::Ordering {
//         Self::box_compare(a, b, 1)
//     }

//     fn box_z_compare(a: &Hittable, b: &Hittable) -> std::cmp::Ordering {
//         Self::box_compare(a, b, 2)
//     }
// }

// impl HittableTrait for BvhNode {
//     fn hit(&self, ray: Ray, ray_t: Interval) -> Option<HitObject> {
//         if self.bounding_box.hit(ray, ray_t).is_none() {
//             return None;
//         }

//         let mut hit_object = self.left.hit(ray, ray_t);

//         let mut max = ray_t.max;
//         if let Some(object) = hit_object {
//             max = object.t;
//         }

//         hit_object = self.right.hit(ray, Interval::new(ray_t.min, max));

//         hit_object
//     }

//     fn get_bounding_box(&self) -> Aabb {
//         self.bounding_box
//     }
// }
