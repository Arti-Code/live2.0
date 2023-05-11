#![allow(unused)]

mod consts;
mod util;
mod agent;

use macroquad::miniquad::conf::Icon;
use macroquad::prelude::*;
use macroquad::window; 
use macroquad::file::*;
use crate::consts::*;
use crate::util::*;
use crate::agent::*;
use macroquad::time::*;


fn app_configuration() -> Conf {
    Conf{
        window_title: "LIVE 2.0".to_string(),
        window_width: SCREEN_WIDTH as i32,
        window_height: SCREEN_HEIGHT as i32,
        sample_count: 8,
        window_resizable: false,
        //icon: Some(load_file("img\\icon.png").into()),
        //icon: Icon::from::<Vec<u8>>(load_image("img\\icon.png").await.unwrap().bytes),
        ..Default::default()
    }
}

#[macroquad::main(app_configuration)]
async fn main() {
    set_pc_assets_folder("assets");
    load_ttf_font("font\\font.ttf").await;
    window::request_new_screen_size(SCREEN_WIDTH, SCREEN_HEIGHT);
    let mut agents: Vec<Agent> = vec![];

    for _ in 0..AGENTS_NUM {
        let agent = Agent::new();

        agents.push(agent);
    }

    loop {
        let delta = get_frame_time();
        let fps = get_fps();
        update(&mut agents, delta);
        draw(&agents, fps);

        next_frame().await;
    }
}

fn draw(agents: &Vec<Agent>, fps: i32) {
    clear_background(BLACK);
    draw_text("LIVE", SCREEN_WIDTH/2.0-20.0, 20.0, 24.0, WHITE);
    draw_text("2", SCREEN_WIDTH/2.0+22.0, 16.0, 24.0, WHITE);
    draw_text(&format!("FPS: {}", fps), 10.0, 15.0, 18.0, GRAY);
    for a in agents.iter() {
        a.draw();
    }
}

fn update(agents: &mut Vec<Agent>, dt: f32) {
    for a in agents.iter_mut() {
        a.update(dt);
    }
}

