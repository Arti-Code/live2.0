use macroquad::prelude::*;
use crate::consts::*;

pub fn create_camera() -> Camera2D {
    let scr_ratio = SCREEN_WIDTH/SCREEN_HEIGHT;
    let zoom_rate = 1.0/600.0;
    let camera2d = Camera2D {
        zoom: Vec2 {x: zoom_rate, y: zoom_rate*scr_ratio},
        target: Vec2 {x: WORLD_W/2.0, y: WORLD_H/2.0},
        ..Default::default()
    };
    return camera2d;
}

pub fn control_camera(camera: &mut Camera2D, screen_ratio: f32) {
    if is_key_pressed(KeyCode::KpAdd) {
        let ratio = screen_ratio;
        camera.zoom += Vec2::new(0.0001, 0.0001*ratio);
    }
    if is_key_pressed(KeyCode::KpSubtract) {
        if camera.zoom.x > 0.0001 {
            let ratio = screen_ratio;
            camera.zoom -= Vec2::new(0.0001, 0.0001*ratio);
        }
    }
    if is_key_pressed(KeyCode::Left) {
        //println!("target <x: {} | y: {}>", camera.target.x, camera.target.y);
        camera.target.x -= 50.0;
    }
    if is_key_pressed(KeyCode::Right) {
        //println!("target <x: {} | y: {}>", camera.target.x, camera.target.y);
        camera.target.x += 50.0;
    }
    if is_key_pressed(KeyCode::Up) {
        //println!("target <x: {} | y: {}>", camera.target.x, camera.target.y);
        camera.target.y += 50.0;
    }
    if is_key_pressed(KeyCode::Down) {
        //println!("target <x: {} | y: {}>", camera.target.x, camera.target.y);
        camera.target.y -= 50.0;
    }
}