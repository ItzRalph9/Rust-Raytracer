use crate::{color::Color, hit_object::HitObject, ray::Ray, vector3::Vector3Extensions};

#[derive(Debug, Clone, Copy)]
pub enum Material {
    Lambertian(Color),
    Metal(Color, f64)
}

impl Material {
    pub fn scatter(&self, r_in: Ray, hit_object: &HitObject) -> Option<(Color, Ray)> {
        match self {
            Material::Lambertian(albedo) => {
                let mut scatter_direction = hit_object.normal + Ray::random_unit_vector();

                // Catch degenerate scatter direction
                if scatter_direction.near_zero() {
                    scatter_direction = hit_object.normal;
                }
                
                let scattered = Ray::new(hit_object.point, scatter_direction);
                let attenuation = *albedo;

                Some((attenuation, scattered))
            },
            Material::Metal(albedo, fuzz) => {
                let reflected = Ray::reflect(r_in.direction.normalize(), hit_object.normal);
                let scattered = Ray::new(hit_object.point, reflected + *fuzz * Ray::random_unit_vector());
                let attenuation = *albedo;
                
                if scattered.direction.dot(&hit_object.normal) > 0.0 {
                    Some((attenuation, scattered))
                } else {
                    None
                }
            },
        }
    }
}
