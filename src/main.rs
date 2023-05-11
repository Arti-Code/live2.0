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
    let font = load_ttf_font("font\\font.ttf").await.unwrap();
    window::request_new_screen_size(SCREEN_WIDTH, SCREEN_HEIGHT);
    let mut agents: Vec<Agent> = vec![];

    for _ in 0..AGENTS_NUM {
        let agent = Agent::new();

        agents.push(agent);
    }

    let txt_param = TextParams{
        font: font,
        color: GRAY,
        font_size: 12,
        ..Default::default()
    };
    let title_param = TextParams{
        font: font,
        color: WHITE,
        font_size: 22,
        ..Default::default()
    };
    let title2_param = TextParams{
        font: font,
        color: WHITE,
        font_size: 18,
        ..Default::default()
    };

    loop {
        let delta = get_frame_time();
        let fps = get_fps();
        update(&mut agents, delta);
        draw(&agents, fps, title_param, title2_param, txt_param);

        next_frame().await;
    }
}

fn draw(agents: &Vec<Agent>, fps: i32, font_param_title: TextParams, font_param_title2: TextParams, font_param: TextParams,) {
    clear_background(BLACK);
    draw_text_ex("LIVE", SCREEN_WIDTH/2.0-30.0, 20.0, font_param_title);
    draw_text_ex("2", SCREEN_WIDTH/2.0+26.0, 15.0, font_param_title2);
    draw_text_ex(&format!("FPS: {}", fps), 10.0, 15.0, font_param);
    for a in agents.iter() {
        a.draw();
    }
}

fn update(agents: &mut Vec<Agent>, dt: f32) {
    for a in agents.iter_mut() {
        a.update(dt);
    }
}

