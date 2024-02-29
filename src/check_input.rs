use minifb::{Key, Window};

use crate::scene::Scene;

pub fn check_input(window: &Window, scene: &mut Scene) {
    move_camera(window, scene);
    move_sphere(window, scene);
}

fn move_camera(window: &Window, scene: &mut Scene) {
    let speed = 0.25;

    let mut camera_position = scene.camera.position;

    let keys = window.get_keys();
    for t in keys {
        match t {
            Key::W => camera_position.z -= speed,
            Key::A => camera_position.x -= speed,
            Key::S => camera_position.z += speed,
            Key::D => camera_position.x += speed,
            Key::E => camera_position.y += speed,
            Key::Q => camera_position.y -= speed,
            _ => {}
        }
    }

    scene.camera.position = camera_position;
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
