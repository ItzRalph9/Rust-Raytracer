use std::sync::RwLock;

use nalgebra::Vector3;
use once_cell::sync::Lazy;

use crate::library::{camera::{Camera, CameraDefaults}, color::Color, image::Image, material::Material, perlin::Perlin, vector3::Vector3Extensions};
use crate::library::{hittable_list::HittableList, quad::Quad, rotate_y::RotateY, translate::Translate, sphere::Sphere, quadbox::Quadbox};
use crate::library::material::Material::*;
use crate::library::texture::Texture::*;
use crate::library::hittable::Hittable::*;

use super::{constant_medium::ConstantMedium, triangle::Triangle};

pub static SCENE: Lazy<RwLock<Scene>> = Lazy::new(|| RwLock::new(Scene::new(10)));

pub struct Scene {
    pub hittable_list: HittableList,
    pub camera: Camera,
}

impl Scene {
    pub fn new(scene: usize) -> Self {
        match scene {
            1 => Self::random_spheres(),
            2 => Self::two_spheres(),
            3 => Self::earth(),
            4 => Self::two_perlin_spheres(),
            5 => Self::quads(),
            6 => Self::simple_light(),
            7 => Self::cornell_box(),
            8 => Self::cornell_smoke(),
            9 => Self::final_scene(),
            10 => Self::triangle(),
            _ => Self::random_spheres(),
        }
    }

    fn random_spheres() -> Self {
        let mut hittable_list = HittableList::new();

        let ground_texture = Checkered(0.32, Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9));

        hittable_list.add(Sphere(Sphere::new_stationary(
            Vector3::new(0.0, -1000.0, 0.0),
            1000.0,
            Lambertian(ground_texture),
        )));
        
        for a in (-7..7).step_by(4) {
            for b in (-5..5).step_by(2) {
                let choose_material = Material::random_float();
                let center = Vector3::new(a as f64 + 0.9 *  Material::random_float(), 0.2, b as f64 + 0.9 *  Material::random_float());
    
                if (center - Vector3::new(4.0, 0.2, 0.0)).magnitude() > 0.9 {
                    let material;
    
                    if choose_material < 0.8 {
                        // diffuse
                        // let albedo = SolidColor(Color::random() * Color::random());
                        // material = Lambertian(albedo);
                        let texture = Image(Image::load_image("assets/earth_1024.jpg"));
                        let material = Lambertian(texture);
                        // let center2 = center + Vector3::new(0.0,  Material::random_float_range(0.0..0.5), 0.0);
                        // hittable_list.add(Arc::new(Sphere::new_moving(center, center2, 0.2, material)));
                        hittable_list.add(Sphere(Sphere::new_stationary(center, 0.2, material)));
                    } else if choose_material < 0.95 {
                        // metal
                        let albedo = Color::random_range(0.5..1.0);
                        let fuzz =  Material::random_float_range(0.0..0.5);
                        material = Metal(albedo, fuzz);
                        hittable_list.add(Sphere(Sphere::new_stationary(center, 0.2, material)));
                    } else {
                        // glass
                        material = Dielectric(1.5);
                        hittable_list.add(Sphere(Sphere::new_stationary(center, 0.2, material)));
                    }
                }
            }
        }

        hittable_list.add(Sphere(Sphere::new_stationary(
            Vector3::new(2.0, 1.0, 0.0),
            1.0,
            Dielectric(1.5),
        )));

        hittable_list.add(Sphere(Sphere::new_stationary(
            Vector3::new(-2.0, 1.0, 0.0),
            1.0,
            Lambertian(SolidColor(Color::new(0.4, 0.2, 0.1))),
        )));

        hittable_list.add(Sphere(Sphere::new_stationary(
            Vector3::new(6.0, 1.0, 0.0),
            1.0,
            Metal(Color::new(0.4, 0.4, 0.4), 0.025),
        )));

        let earth_texture = Image(Image::load_image("assets/earth_1024.jpg"));
        hittable_list.add(Sphere(Sphere::new_stationary(
            Vector3::new(-6.0, 1.0, 0.0),
            1.0,
            Lambertian(earth_texture),
        )));

        // let list = HittableList::new_from_list(Arc::new(BvhNode::new_from_list(hittable_list)));
        let list = hittable_list;

        Scene {
            hittable_list: list,
            camera: Camera::init(
                CameraDefaults {
                    samples_per_pixel: 1,
                    max_depth: 50,
                    background: Color::new(0.7, 0.8, 1.0),
                    vertical_fov: 20.0,
                    lookfrom: Vector3::new(15.0, 2.5, 8.0),
                    lookat : Vector3::new(0.0, 0.0, -1.0),
                    vup: Vector3::new(0.0, 1.0, 0.0),
                    defocus_angle: 0.15,
                    focus_distance: 10.0,
                }
            ),
        }
    }

    fn two_spheres() -> Self {
        let mut hittable_list = HittableList::new();
    
        let checker = Checkered(0.32, Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9));
    
        hittable_list.add(Sphere(Sphere::new_stationary(
            Vector3::new(0.0, -10.0, 0.0),
            10.0,
            Lambertian(checker.clone())
        )));
        
        hittable_list.add(Sphere(Sphere::new_stationary(
            Vector3::new(0.0, 10.0, 0.0),
            10.0,
            Lambertian(checker.clone())
        )));
    
        Scene {
            hittable_list,
            camera: Camera::init(
                CameraDefaults {
                    samples_per_pixel: 1,
                    max_depth: 50,
                    background: Color::new(0.7, 0.8, 1.0),
                    vertical_fov: 20.0,
                    lookfrom: Vector3::new(13.0, 2.0, 3.0),
                    lookat : Vector3::new(0.0, 0.0, 0.0),
                    vup: Vector3::new(0.0, 1.0, 0.0),
                    defocus_angle: 0.0,
                    focus_distance: 10.0,
                }
            ),
        }
    }

    fn earth() -> Self {
        let mut hittable_list = HittableList::new();
    
        let texture = Image(Image::load_image("assets/earth_1024.jpg"));
        let surface = Lambertian(texture);
    
        hittable_list.add(Sphere(Sphere::new_stationary(Vector3::new(0.0, 0.0, 0.0), 2.0, surface)));
    
        Scene {
            hittable_list,
            camera: Camera::init(
                CameraDefaults {
                    samples_per_pixel: 1,
                    max_depth: 50,
                    background: Color::new(0.7, 0.8, 1.0),
                    vertical_fov: 20.0,
                    lookfrom: Vector3::new(0.0, 0.0, 12.0),
                    lookat : Vector3::new(0.0, 0.0, 0.0),
                    vup: Vector3::new(0.0, 1.0, 0.0),
                    defocus_angle: 0.0,
                    focus_distance: 10.0,
                }
            ),
        }
    }

    fn two_perlin_spheres() -> Self {
        let mut hittable_list = HittableList::new();
    
        let perlin_texture = Perlin(Perlin::new(), 4.0);

        hittable_list.add(Sphere(Sphere::new_stationary(
            Vector3::new(0.0, -1000.0, 0.0),
            1000.0,
            Lambertian(perlin_texture.clone())
        )));
        
        hittable_list.add(Sphere(Sphere::new_stationary(
            Vector3::new(0.0, 2.0, 0.0),
            2.0,
            Lambertian(perlin_texture.clone())
        )));

        Scene {
            hittable_list,
            camera: Camera::init(
                CameraDefaults {
                    samples_per_pixel: 1,
                    max_depth: 50,
                    background: Color::new(0.7, 0.8, 1.0),
                    vertical_fov: 20.0,
                    lookfrom: Vector3::new(13.0, 2.0, 3.0),
                    lookat : Vector3::new(0.0, 0.0, 0.0),
                    vup: Vector3::new(0.0, 1.0, 0.0),
                    defocus_angle: 0.0,
                    focus_distance: 10.0,
                }
            ),
        }
    }

    fn quads() -> Self {
        let mut hittable_list = HittableList::new();
    
        // Materials
        let left_red     = Lambertian(SolidColor(Color::new(1.0, 0.2, 0.2)));
        let back_green   = Lambertian(SolidColor(Color::new(0.2, 1.0, 0.2)));
        let right_blue   = Lambertian(SolidColor(Color::new(0.2, 0.2, 1.0)));
        let upper_orange = Lambertian(SolidColor(Color::new(1.0, 0.5, 0.0)));
        let lower_teal   = Lambertian(SolidColor(Color::new(0.2, 0.8, 0.8)));
    
        // Quads
        hittable_list.add(Quad(Quad::new(Vector3::new(-3.0, -2.0, 5.0), Vector3::new(0.0, 0.0,-4.0), Vector3::new(0.0, 4.0, 0.0), left_red)));
        hittable_list.add(Quad(Quad::new(Vector3::new(-2.0, -2.0, 0.0), Vector3::new(4.0, 0.0, 0.0), Vector3::new(0.0, 4.0, 0.0), back_green)));
        hittable_list.add(Quad(Quad::new(Vector3::new( 3.0, -2.0, 1.0), Vector3::new(0.0, 0.0, 4.0), Vector3::new(0.0, 4.0, 0.0), right_blue)));
        hittable_list.add(Quad(Quad::new(Vector3::new(-2.0,  3.0, 1.0), Vector3::new(4.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 4.0), upper_orange)));
        hittable_list.add(Quad(Quad::new(Vector3::new(-2.0, -3.0, 5.0), Vector3::new(4.0, 0.0, 0.0), Vector3::new(0.0, 0.0,-4.0), lower_teal)));
        
        Scene {
            hittable_list,
            camera: Camera::init(
                CameraDefaults {
                    samples_per_pixel: 1,
                    max_depth: 50,
                    background: Color::new(0.7, 0.8, 1.0),
                    vertical_fov: 80.0,
                    lookfrom: Vector3::new(0.0, 0.0, 9.0),
                    lookat : Vector3::new(0.0, 0.0, 0.0),
                    vup: Vector3::new(0.0, 1.0, 0.0),
                    defocus_angle: 0.0,
                    focus_distance: 10.0,
                }
            ),
        }
    }

    fn simple_light() -> Self {
        let mut hittable_list = HittableList::new();
    
        let pertext = Perlin(Perlin::new(), 4.0);
        hittable_list.add(Sphere(Sphere::new_stationary(Vector3::new(0.0, -1000.0, 0.0), 1000.0, Lambertian(pertext.clone()))));
        hittable_list.add(Sphere(Sphere::new_stationary(Vector3::new(0.0, 2.0, 0.0), 2.0, Lambertian(pertext.clone()))));
    
        let difflight = DiffuseLight(SolidColor(Color::new(20.0, 20.0, 20.0)));
        hittable_list.add(Quad(Quad::new(Vector3::new(3.0, 1.0, -2.0), Vector3::new(2.0, 0.0, 0.0), Vector3::new(0.0, 2.0, 0.0), difflight.clone())));
        hittable_list.add(Sphere(Sphere::new_stationary(Vector3::new(0.0, 7.0, 0.0), 2.0, difflight.clone())));

        Scene {
            hittable_list,
            camera: Camera::init(
                CameraDefaults {
                    samples_per_pixel: 1,
                    max_depth: 50,
                    background: Color::new(0.0, 0.0, 0.0),
                    vertical_fov: 20.0,
                    lookfrom: Vector3::new(26.0, 3.0, 6.0),
                    lookat : Vector3::new(0.0, 2.0, 0.0),
                    vup: Vector3::new(0.0, 1.0, 0.0),
                    defocus_angle: 0.0,
                    focus_distance: 10.0,
                }
            ),
        }
    }

    fn cornell_box() -> Self {
        let mut hittable_list = HittableList::new();
    
        let red   = Lambertian(SolidColor(Color::new(0.65, 0.05, 0.05)));
        let white = Lambertian(SolidColor(Color::new(0.73, 0.73, 0.73)));
        let green = Lambertian(SolidColor(Color::new(0.12, 0.45, 0.15)));
        let light = DiffuseLight(SolidColor(Color::new(15.0, 15.0, 15.0)));
    
        hittable_list.add(Quad(Quad::new(Vector3::new(555.0, 0.0, 0.0), Vector3::new(0.0, 555.0, 0.0), Vector3::new(0.0, 0.0, 555.0), green.clone())));
        hittable_list.add(Quad(Quad::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 555.0, 0.0), Vector3::new(0.0, 0.0, 555.0), red.clone())));
        hittable_list.add(Quad(Quad::new(Vector3::new(343.0, 554.0, 332.0), Vector3::new(-130.0, 0.0, 0.0), Vector3::new(0.0, 0.0, -105.0), light.clone())));
        hittable_list.add(Quad(Quad::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(555.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 555.0), white.clone())));
        hittable_list.add(Quad(Quad::new(Vector3::new(555.0, 555.0, 555.0), Vector3::new(-555.0, 0.0, 0.0), Vector3::new(0.0, 0.0,-555.0), white.clone())));
        hittable_list.add(Quad(Quad::new(Vector3::new(0.0, 0.0, 555.0), Vector3::new(555.0, 0.0, 0.0), Vector3::new(0.0, 555.0, 0.0), white.clone())));

        let box1 = Quadbox::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(165.0, 330.0, 165.0), white.clone());
        for side in box1.objects {
            let side = RotateY::new(side, 15.0);
            let side = RotateY(Box::new(side));
            let side = Translate::new(side, Vector3::new(265.0, 0.0, 295.0));
            let side = Translate(Box::new(side));
            hittable_list.add(side)
        }

        let box2 = Quadbox::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(165.0, 165.0, 165.0), white.clone());
        for side in box2.objects {
            let side = RotateY::new(side, -18.0);
            let side = RotateY(Box::new(side));
            let side = Translate::new(side, Vector3::new(130.0, 0.0, 65.0));
            let side = Translate(Box::new(side));
            hittable_list.add(side)
        }

        Scene {
            hittable_list,
            camera: Camera::init(
                CameraDefaults {
                    samples_per_pixel: 1,
                    max_depth: 50,
                    background: Color::new(0.0, 0.0, 0.0),
                    vertical_fov: 40.0,
                    lookfrom: Vector3::new(278.0, 278.0, -800.0),
                    lookat : Vector3::new(278.0, 278.0, 0.0),
                    vup: Vector3::new(0.0, 1.0, 0.0),
                    defocus_angle: 0.0,
                    focus_distance: 10.0,
                }
            ),
        }
    }
    
    fn cornell_smoke() -> Self {
        let mut hittable_list = HittableList::new();
    
        let red   = Lambertian(SolidColor(Color::new(0.65, 0.05, 0.05)));
        let white = Lambertian(SolidColor(Color::new(0.73, 0.73, 0.73)));
        let green = Lambertian(SolidColor(Color::new(0.12, 0.45, 0.15)));
        let light = DiffuseLight(SolidColor(Color::new(7.0, 7.0, 7.0)));
    
        hittable_list.add(Quad(Quad::new(Vector3::new(555.0, 0.0, 0.0), Vector3::new(0.0, 555.0, 0.0), Vector3::new(0.0, 0.0, 555.0), green.clone())));
        hittable_list.add(Quad(Quad::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 555.0, 0.0), Vector3::new(0.0, 0.0, 555.0), red.clone())));
        hittable_list.add(Quad(Quad::new(Vector3::new(113.0, 554.0, 127.0), Vector3::new(330.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 305.0), light.clone())));
        hittable_list.add(Quad(Quad::new(Vector3::new(0.0, 555.0, 0.0), Vector3::new(555.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 555.0), white.clone())));
        hittable_list.add(Quad(Quad::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(555.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 555.0), white.clone())));
        hittable_list.add(Quad(Quad::new(Vector3::new(0.0, 0.0, 555.0), Vector3::new(555.0, 0.0, 0.0), Vector3::new(0.0, 555.0, 0.0), white.clone())));

        let box1 = Translate(Box::new(
            Translate::new(RotateY(Box::new(RotateY::new(
                QuadBox(Quadbox::new(
                    Vector3::new(0.0, 0.0, 0.0),
                    Vector3::new(165.0, 330.0, 165.0),
                    white.clone()
                )),
                15.0
            ))), Vector3::new(265.0, 0.0, 295.0))
        ));

        let box2 = Translate(Box::new(
            Translate::new(RotateY(Box::new(RotateY::new(
                QuadBox(Quadbox::new(
                    Vector3::new(0.0, 0.0, 0.0),
                    Vector3::new(165.0, 165.0, 165.0),
                    white.clone()
                )), -18.0
            ))), Vector3::new(103.0, 0.0, 65.0))
        ));

        hittable_list.add(ConstantMedium(ConstantMedium::new_from_color(box1, 0.01, Color::new(0.0, 0.0, 0.0))));
        hittable_list.add(ConstantMedium(ConstantMedium::new_from_color(box2, 0.005, Color::new(1.0, 1.0, 1.0))));

        Scene {
            hittable_list,
            camera: Camera::init(
                CameraDefaults {
                    samples_per_pixel: 1,
                    max_depth: 50,
                    background: Color::new(0.0, 0.0, 0.0),
                    vertical_fov: 40.0,
                    lookfrom: Vector3::new(278.0, 278.0, -800.0),
                    lookat : Vector3::new(278.0, 278.0, 0.0),
                    vup: Vector3::new(0.0, 1.0, 0.0),
                    defocus_angle: 0.0,
                    focus_distance: 10.0,
                }
            ),
        }
    }

    pub fn final_scene() -> Self {
        let mut boxes1 = HittableList::new();
        let ground = Lambertian(SolidColor(Color::new(0.48, 0.83, 0.53)));

        let boxes_per_side = 20;
        for i in 0..boxes_per_side {
            for j in 0..boxes_per_side {
                let w = 100.0;
                let x0 = -1000.0 + i as f64 * w;
                let z0 = -1000.0 + j as f64 * w;
                let y0 = 0.0;
                let x1 = x0 + w;
                let y1 = Material::random_float_range(1.0..101.0);
                let z1 = z0 + w;

                boxes1.add(QuadBox(Quadbox::new(Vector3::new(x0, y0, z0), Vector3::new(x1, y1, z1), ground.clone())));
            }
        }

        let mut hittable_list = HittableList::new();

        hittable_list.add_list(boxes1);

        let light = DiffuseLight(SolidColor(Color::new(7.0, 7.0, 7.0)));
        hittable_list.add(Quad(Quad::new(Vector3::new(123.0, 554.0, 147.0), Vector3::new(300.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 265.0), light)));

        let center1 = Vector3::new(400.0, 400.0, 200.0);
        let center2 = center1 + Vector3::new(30.0, 0.0, 0.0);
        let sphere_material = Lambertian(SolidColor(Color::new(0.7, 0.3, 0.1)));
        hittable_list.add(Sphere(Sphere::new_moving(center1, center2, 50.0, sphere_material)));

        hittable_list.add(Sphere(Sphere::new_stationary(Vector3::new(260.0, 150.0, 45.0), 50.0, Dielectric(1.5))));
        hittable_list.add(Sphere(Sphere::new_stationary(
            Vector3::new(0.0, 150.0, 145.0), 50.0, Metal(Color::new(0.8, 0.8, 0.9), 1.0)
        )));

        let boundary = Sphere(Sphere::new_stationary(Vector3::new(360.0, 150.0, 145.0), 70.0, Dielectric(1.5)));
        hittable_list.add(boundary.clone());
        hittable_list.add(ConstantMedium(ConstantMedium::new_from_color(boundary.clone(), 0.2, Color::new(0.2, 0.4, 0.9))));
        let boundary = Sphere(Sphere::new_stationary(Vector3::new(0.0, 0.0, 0.0), 5000.0, Dielectric(1.5)));
        hittable_list.add(ConstantMedium(ConstantMedium::new_from_color(boundary, 0.0001, Color::new(1.0, 1.0, 1.0))));

        let emat = Lambertian(Image(Image::load_image("assets/earth_400.jpg")));
        hittable_list.add(Sphere(Sphere::new_stationary(Vector3::new(400.0, 200.0, 400.0), 100.0, emat)));
        let pertext = Perlin(Perlin::new(), 0.1);
        hittable_list.add(Sphere(Sphere::new_stationary(Vector3::new(220.0, 280.0, 300.0), 80.0, Lambertian(pertext))));

        let mut boxes2 = HittableList::new();
        let white = Lambertian(SolidColor(Color::new(0.73, 0.73, 0.73)));
        let ns = 1000;
        for _ in 0..ns {
            boxes2.add(Sphere(Sphere::new_stationary(Vector3::random_float_range(0.0..165.0), 10.0, white.clone())));
        }

        for object in boxes2.objects {
            hittable_list.add(Translate(Box::new(Translate::new(
                RotateY(Box::new(RotateY::new(object, 15.0))), Vector3::new(-100.0, 270.0, 395.0)
            ))));
        }

        Scene {
            hittable_list,
            camera: Camera::init(
                CameraDefaults {
                    samples_per_pixel: 1,
                    max_depth: 10,
                    background: Color::new(0.0, 0.0, 0.0),
                    vertical_fov: 40.0,
                    lookfrom: Vector3::new(478.0, 278.0, -600.0),
                    lookat : Vector3::new(278.0, 278.0, 0.0),
                    vup: Vector3::new(0.0, 1.0, 0.0),
                    defocus_angle: 0.0,
                    focus_distance: 10.0,
                }
            ),
        }
    }

    pub fn triangle() -> Self {
        let mut hittable_list = HittableList::new();
    
        // Materials
        let white = Lambertian(SolidColor(Color::new(1.0, 1.0, 1.0)));
        let teal =  Lambertian(SolidColor(Color::new(0.0, 0.9, 1.0)));

        // Box
        let light = DiffuseLight(SolidColor(Color::new(2.5, 2.5, 2.5)));
        let left_light = DiffuseLight(SolidColor(Color::new(1.0, 1.0, 1.0)));
        hittable_list.add(Quad(Quad::new(Vector3::new(77.5, 554.0, 77.5), Vector3::new(400.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 400.0), light.clone())));
        hittable_list.add(Quad(Quad::new(Vector3::new(800.0, 0.0, 0.0), Vector3::new(0.0, 555.0, 0.0), Vector3::new(0.0, 0.0, 555.0), left_light.clone())));
        // hittable_list.add(Quad(Quad::new(Vector3::new(77.5, -554.0, 77.5), Vector3::new(400.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 400.0), light.clone())));
        // hittable_list.add(Quad(Quad::new(Vector3::new(555.0, 0.0, 0.0), Vector3::new(0.0, 555.0, 0.0), Vector3::new(0.0, 0.0, 555.0), metal.clone())));
        // hittable_list.add(Quad(Quad::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 555.0, 0.0), Vector3::new(0.0, 0.0, 555.0), white.clone())));
        hittable_list.add(Quad(Quad::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(555.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 555.0), white.clone())));
        hittable_list.add(Quad(Quad::new(Vector3::new(555.0, 555.0, 555.0), Vector3::new(-555.0, 0.0, 0.0), Vector3::new(0.0, 0.0,-555.0), white.clone())));
        hittable_list.add(Quad(Quad::new(Vector3::new(0.0, 0.0, 555.0), Vector3::new(555.0, 0.0, 0.0), Vector3::new(0.0, 555.0, 0.0), white.clone())));

        // Pyramid
        hittable_list.add(Triangle(Triangle::new(Vector3::new(220.0, 220.0, 220.0), Vector3::new(335.0, 220.0, 220.0), Vector3::new(277.5, 300.0, 277.5), teal.clone())));
        hittable_list.add(Triangle(Triangle::new(Vector3::new(220.0, 220.0, 220.0), Vector3::new(220.0, 220.0, 335.0), Vector3::new(277.5, 300.0, 277.5), teal.clone())));
        hittable_list.add(Triangle(Triangle::new(Vector3::new(335.0, 220.0, 335.0), Vector3::new(220.0, 220.0, 335.0), Vector3::new(277.5, 300.0, 277.5), teal.clone())));
        hittable_list.add(Triangle(Triangle::new(Vector3::new(335.0, 220.0, 335.0), Vector3::new(335.0, 220.0, 220.0), Vector3::new(277.5, 300.0, 277.5), teal.clone())));
        hittable_list.add(Quad(Quad::new(Vector3::new(220.0, 220.0, 220.0), Vector3::new(115.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 115.0), teal.clone())));

        let fog_box = QuadBox(Quadbox::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(-50.0, 555.0, 555.0), teal.clone()));
        let fog = ConstantMedium(ConstantMedium::new(fog_box, 0.005, SolidColor(Color::new(0.2, 0.2, 0.2))));
        hittable_list.add(fog);


        Scene {
            hittable_list,
            camera: Camera::init(
                CameraDefaults {
                    samples_per_pixel: 1,
                    max_depth: 50,
                    // background: Color::new(0.7, 0.8, 1.0),
                    background: Color::new(0.0, 0.0, 0.0),
                    vertical_fov: 40.0,
                    // lookfrom: Vector3::new(550.0, 300.0, 150.0),
                    // lookat : Vector3::new(0.0, 250.0, 345.0),
                    lookfrom: Vector3::new(278.0, 278.0, -800.0),
                    lookat : Vector3::new(278.0, 278.0, 0.0),
                    vup: Vector3::new(0.0, 1.0, 0.0),
                    defocus_angle: 0.0,
                    focus_distance: 10.0,
                }
            ),
        }
    }
}
