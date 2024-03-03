use minifb::{Key, Window};

use crate::scene::Scene;

pub fn _check_input(window: &Window, scene: &mut Scene) -> bool {
    let pressed_keys = window.get_keys();
    _move_camera(&pressed_keys, scene, 0.25) || _move_sphere(&pressed_keys, scene, 0.25)
}

fn _move_camera(pressed_keys: &Vec<Key>, scene: &mut Scene, speed: f64) -> bool {
    let mut camera = scene.camera;

    let lookfrom = &mut camera.defaults.lookfrom;
    let lookat = &mut camera.defaults.lookat;
    let vup = &mut camera.defaults.vup;

    let mut is_key_pressed = false;
    for t in pressed_keys {
        
        let forward = (*lookat - *lookfrom).normalize();
        let right = lookfrom.cross(&vup).normalize();
        
        is_key_pressed = true;
        match t {
            Key::W => *lookfrom += forward * speed,
            Key::A => *lookfrom += -right * speed,
            Key::S => *lookfrom += -forward * speed,
            Key::D => *lookfrom += right * speed,
            Key::E => *lookfrom += *vup * speed,
            Key::Q => *lookfrom -= *vup * speed,
            _ => is_key_pressed = false
        }
    }

    camera._update_camera(scene);

    is_key_pressed
}

fn _move_sphere(pressed_keys: &Vec<Key>, scene: &mut Scene, speed: f64) -> bool {
    let mut sphere_center = scene._get_sphere_position(None);

    let mut is_key_pressed = false;
    for t in pressed_keys {

        is_key_pressed = true;
        match t {
            Key::Left => sphere_center.x -= speed,
            Key::Right => sphere_center.x += speed,
            Key::Up => sphere_center.y += speed,
            Key::Down => sphere_center.y -= speed,
            Key::LeftBracket => sphere_center.z -= speed,
            Key::RightBracket => sphere_center.z += speed,
            _ => is_key_pressed = false
        }
    }

    scene._set_sphere_position(sphere_center, None);

    is_key_pressed
}
