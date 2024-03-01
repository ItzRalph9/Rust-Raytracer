use minifb::{Key, Window};

use crate::scene::Scene;

pub fn check_input(window: &Window, scene: &mut Scene) {
    move_camera(window, scene);
    move_sphere(window, scene);
}

fn move_camera(window: &Window, scene: &mut Scene) {
    let speed = 0.25;
    let mut camera = scene.camera;

    let lookfrom = &mut camera.defaults.lookfrom;
    let lookat = &mut camera.defaults.lookat;
    let vup = &mut camera.defaults.vup;

    let keys = window.get_keys();
    for t in keys {
        match t {
            Key::W => {
                let forward = (*lookat - *lookfrom).normalize();
                *lookfrom += forward * speed;
            },
            Key::A => {
                let right = lookfrom.cross(&vup).normalize();
                *lookfrom += -right * speed;
            },
            Key::S => {
                let forward = (*lookat - *lookfrom).normalize();
                *lookfrom += -forward * speed;
            },
            Key::D => {
                let right = lookfrom.cross(&vup).normalize();
                *lookfrom += right * speed;
            },
            Key::E => {
                *lookfrom += *vup * speed;
            },
            Key::Q => {
                *lookfrom -= *vup * speed;
            },
            _ => {}
        }
    }

    camera.update_camera(scene);
}

fn move_sphere(window: &Window, scene: &mut Scene) {
    let speed = 0.25;

    let mut sphere_center = scene.get_sphere_position();

    let keys = window.get_keys();
    for t in keys {
        match t {
            Key::Left => sphere_center.x -= speed,
            Key::Right => sphere_center.x += speed,
            Key::Up => sphere_center.y += speed,
            Key::Down => sphere_center.y -= speed,
            Key::LeftBracket => sphere_center.z -= speed,
            Key::RightBracket => sphere_center.z += speed,
            _ => {}
        }
    }

    scene.set_sphere_position(sphere_center);
}
