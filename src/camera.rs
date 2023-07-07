use macroquad::prelude::*;
use crate::consts::*;

pub fn create_camera() -> Camera2D {
    let scr_ratio = SCREEN_WIDTH/SCREEN_HEIGHT;
    let zoom_rate = 1.0/800.0;
    let camera2d = Camera2D {
        //zoom: Vec2 {x: zoom_rate, y: zoom_rate*scr_ratio},
        zoom: Vec2 {x: 1.*zoom_rate, y: -1.*(scr_ratio*zoom_rate)},
        //zoom: Vec2 {x: 1.*zoom_rate, y: -1.*zoom_rate},
        target: Vec2 {x: WORLD_W/2.0, y: WORLD_H/2.0},
        ..Default::default()
    };
    return camera2d;
}

pub fn control_camera(camera: &mut Camera2D/* , screen_ratio: f32 */) {
    if is_key_pressed(KeyCode::KpAdd) {
        let h_ratio = SCREEN_HEIGHT/SCREEN_WIDTH;
        let w_ratio = SCREEN_WIDTH/SCREEN_HEIGHT;
        let ratio = 1.0;
        camera.zoom += Vec2::new(0.0001*h_ratio, -0.0001*w_ratio);
    }
    if is_key_pressed(KeyCode::KpSubtract) {
        if camera.zoom.x > 0.0001 {
            let h_ratio = SCREEN_HEIGHT/SCREEN_WIDTH;
            let w_ratio = SCREEN_WIDTH/SCREEN_HEIGHT;
            //let ratio = screen_ratio;
            let ratio = 1.0;
            camera.zoom -= Vec2::new(0.0001*h_ratio, -0.0001*w_ratio);
        }
    }
    if is_key_pressed(KeyCode::KpMultiply) {
        //let scr_ratio = 1.0;
        let scr_ratio = SCREEN_WIDTH/SCREEN_HEIGHT;
        let zoom_rate = 1.0/800.0;
        camera.zoom = Vec2::new(1.*zoom_rate, -1.*(scr_ratio*zoom_rate));
        camera.target = Vec2::new(WORLD_W/2.0, WORLD_H/2.0);
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
        camera.target.y -= 50.0;
    }
    if is_key_pressed(KeyCode::Down) {
        //println!("target <x: {} | y: {}>", camera.target.x, camera.target.y);
        camera.target.y += 50.0;
    }
}