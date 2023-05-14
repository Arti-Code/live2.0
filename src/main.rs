#![allow(unused)]

mod consts;
mod util;
mod agent;
//mod rock;
mod num;

use std::thread::sleep;
use std::time::Duration;
use macroquad::miniquad::conf::Icon;
use macroquad::prelude::*;
use macroquad::window; 
use macroquad::file::*;
use num::*;
use parry2d::query::details::contact_ball_ball;
use crate::consts::*;
use crate::util::*;
use crate::agent::*;
use macroquad::time::*;
use std::collections::VecDeque;
use parry2d::query::*;
use parry2d::shape::*;

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

struct CollisionPair<'a> {
    object1: &'a Agent,
    object2: &'a Agent,
    overlap: f32,
}

struct TextParamsBox {
    title: TextParams,
    title2: TextParams,
    standard: TextParams,
}

async fn init_text_params() -> TextParamsBox {
    let font = load_ttf_font("fonts\\jetbrain_medium.ttf").await.unwrap();
    let txt_box = TextParamsBox {
        title: TextParams{
            font: font,
            color: WHITE,
            font_size: 22,
            ..Default::default()
        },
        title2: TextParams{
            font: font,
            color: WHITE,
            font_size: 18,
            ..Default::default()
        },
        standard: TextParams{
            font: font,
            color: GRAY,
            font_size: 12,
            ..Default::default()
        },
    };
    return txt_box
}

#[macroquad::main(app_configuration)]
async fn main() {
    init();
    let text_params = init_text_params().await;
    let mut agents: Vec<Agent> = vec![];
    let mut collisions_list: Vec<CollisionPair> = vec![];
    for _ in 0..AGENTS_NUM {
        let agent = Agent::new();
        agents.push(agent);
    }

    loop {
        let delta = get_frame_time();
        let fps = get_fps();
        let mut contacts: usize = 0;
        update(&mut agents, delta);
        contacts = collisions(&agents/* , &mut collisions_list */);
        draw(&agents, contacts, fps, &text_params);
        next_frame().await;
    }
}

fn init() {
    set_pc_assets_folder("assets");
    window::request_new_screen_size(SCREEN_WIDTH, SCREEN_HEIGHT);
}

fn draw(agents: &Vec<Agent>, contacts: usize, fps: i32, fonts: &TextParamsBox) {
    clear_background(BLACK);
    for a in agents.iter() {
        a.draw();
    }
    draw_text(fonts, contacts, fps);
}

fn draw_text(fonts: &TextParamsBox, contacts: usize, fps: i32) {
    draw_text_ex("LIVE", SCREEN_WIDTH/2.0-30.0, 25.0, fonts.title);
    draw_text_ex("2", SCREEN_WIDTH/2.0+26.0, 20.0, fonts.title2);
    draw_text_ex(&format!("FPS: {}", fps), 10.0, 15.0, fonts.standard);
    draw_text_ex(&format!("CONTACTS: {}", contacts), 15.0, 50.0, fonts.title2);
}

fn update(agents: &mut Vec<Agent>, dt: f32) {
    for a in agents.iter_mut() {
        a.update(dt);
    }
}

fn collisions(agents: &Vec<Agent>/* , collision_list: &'b Vec<CollisionPair> */) -> usize {
    let mut c: usize = 0;
    for a1 in agents.iter() {
        for a2 in agents.iter() {
            if a1.pos != a2.pos {
                let pos1 = make_isometry(a1.pos.x, a1.pos.y, a1.rot);
                let pos2 = make_isometry(a2.pos.x, a2.pos.y, a2.rot);
                let d = contact_circles(pos1, a1.size, pos2, a2.size);
                if d <= 0.0 {
                    c += 1;
                    /* let mut collision_event = CollisionPair{
                        object1: a1,
                        object2: a2,
                        overlap: d,
                    };
                    collision_list.push(collision_event); */
                }
            }
        }
    }
    c
}

async fn wait(delta: f32) {
    let t = FIX_DT - delta;
    if t > 0.0 {
        sleep(Duration::from_secs_f32(t));
    }
}