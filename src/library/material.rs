use nalgebra::Vector3;
use rand::prelude::*;

use crate::library::{color::Color, hit_object::HitObject, ray::Ray, texture::Texture, vector3::Vector3Extensions};

#[derive(Debug, Clone)]
pub enum Material {
    Lambertian(Texture),
    Metal(Color, f64),
    Dielectric(f64),
    DiffuseLight(Texture),
    IsoTropic(Texture),
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
                
                let scattered = Ray::new(hit_object.point, scatter_direction, r_in.time);
                let attenuation = albedo.value(hit_object.u, hit_object.v, hit_object.point);

                Some((attenuation, scattered))
            },
            Material::Metal(albedo, fuzz) => {
                let reflected = Ray::reflect(r_in.direction.normalize(), hit_object.normal);
                let scattered = Ray::new(hit_object.point, reflected + *fuzz * Ray::random_in_unit_sphere(), r_in.time);
                let attenuation = *albedo;
                
                if scattered.direction.dot(&hit_object.normal) > 0.0 {
                    Some((attenuation, scattered))
                } else {
                    None
                }
            },
            Material::Dielectric(refaction_index) => {
                let attenuation = Color::new(1.0, 1.0, 1.0);

                let mut refraction_ratio = *refaction_index;
                if hit_object.front_face {
                    refraction_ratio = 1.0 / refaction_index;
                }

                let unit_direction = r_in.direction.normalize();

                let cos_theta = -unit_direction.dot(&hit_object.normal).min(1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        
                let cannot_refract = refraction_ratio * sin_theta > 1.0;
        
                let mut direction = Ray::refract(unit_direction, hit_object.normal, refraction_ratio);
                if cannot_refract || Self::reflectance(cos_theta, refraction_ratio) > Self::random_float() {
                    direction = Ray::reflect(unit_direction, hit_object.normal);
                }

                let scattered = Ray::new(hit_object.point, direction, r_in.time);
                
                Some((attenuation, scattered))
            },
            Material::DiffuseLight(_) => {
                None
            },
            Material::IsoTropic(albedo) => {
                let scattered = Ray::new(hit_object.point, Ray::random_unit_vector(), r_in.time);
                let attenuation = albedo.value(hit_object.u, hit_object.v, hit_object.point);
                
                Some((attenuation, scattered))
            },
        }
    }

    pub fn emitted(&self, u: f64, v: f64, p: Vector3<f64>) -> Color {
        match self {
            Material::DiffuseLight(emit) => {
                emit.value(u, v, p)
            },
            _ => Color::new(0.0, 0.0, 0.0),
        }
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0 = r0 * r0;

        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }

    pub fn random_float() -> f64 {
        let mut rng = rand::thread_rng();
        rng.gen()
    }

    pub fn random_float_range(range: std::ops::Range<f64>) -> f64 {
        let mut rng = rand::thread_rng();
        rng.gen_range(range)
    }
}
