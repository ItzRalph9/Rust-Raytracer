use minifb::{Key, Window};

use crate::library::scene::Scene;

pub fn check_input(window: &Window, scene: &mut Scene) -> bool {
    let pressed_keys = window.get_keys_pressed(minifb::KeyRepeat::Yes);
    let speed = 10.0;
    turn_camera(&pressed_keys, scene, speed) || move_camera(&pressed_keys, scene, speed)
}

fn turn_camera(pressed_keys: &Vec<Key>, scene: &mut Scene, speed: f64) -> bool {
    let mut camera = scene.camera;

    let lookfrom = &mut camera.defaults.lookfrom;
    let lookat = &mut camera.defaults.lookat;
    let vup = &mut camera.defaults.vup;

    let mut is_key_pressed = false;
    for key in pressed_keys {
        
        let forward = (*lookat - *lookfrom).normalize();
        let right = lookfrom.cross(&vup).normalize();
        
        is_key_pressed = true;
        match key {
            Key::W => *lookfrom += forward * speed,
            Key::A => *lookfrom += right * speed,
            Key::S => *lookfrom -= forward * speed,
            Key::D => *lookfrom -= right * speed,
            Key::E => *lookfrom += *vup * speed,
            Key::Q => *lookfrom -= *vup * speed,
            _ => is_key_pressed = false
        }
    }

    camera._update_camera(scene);

    is_key_pressed
}

fn move_camera(pressed_keys: &Vec<Key>, scene: &mut Scene, speed: f64) -> bool {
    let mut camera = scene.camera;

    let lookfrom = &mut camera.defaults.lookfrom;
    let lookat = &mut camera.defaults.lookat;
    let vup = &mut camera.defaults.vup;

    let mut is_key_pressed = false;
    for key in pressed_keys {
        
        let forward = (*lookat - *lookfrom).normalize();
        let right = lookfrom.cross(&vup).normalize();
        
        is_key_pressed = true;
        match key {
            Key::Up => {*lookfrom += forward * speed; *lookat += forward * speed;},
            Key::Left => {*lookfrom += right * speed; *lookat += right * speed;},
            Key::Down => {*lookfrom -= forward * speed; *lookat -= forward * speed;},
            Key::Right => {*lookfrom -= right * speed; *lookat -= right * speed;},
            Key::Semicolon => {*lookfrom += *vup * speed; *lookat += *vup * speed;},
            Key::Apostrophe => {*lookfrom -= *vup * speed; *lookat -= *vup * speed;},
            _ => is_key_pressed = false
        }
    }

    camera._update_camera(scene);

    is_key_pressed
}
