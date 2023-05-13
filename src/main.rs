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

#[macroquad::main(app_configuration)]
async fn main() {
    //let mut fps30: VecDeque<i32, 30> = VecDeque::from([60; 30]);
    set_pc_assets_folder("assets");
    let font = load_ttf_font("fonts\\jetbrain_medium.ttf").await.unwrap();
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
        //fps30.pop_back();
        //fps30.push_front(fps);
        //let avg_fps = fps30.
        let mut contacts: usize = 0;
        update(&mut agents, delta);
        contacts = collisions(&agents);
        draw(&agents, contacts, fps, title_param, title2_param, txt_param);
        //wait(delta).await;
        next_frame().await;
    }
}

fn draw(agents: &Vec<Agent>, contacts: usize, fps: i32, font_param_title: TextParams, font_param_title2: TextParams, font_param: TextParams,) {
    clear_background(BLACK);
    draw_text_ex("LIVE", SCREEN_WIDTH/2.0-30.0, 25.0, font_param_title);
    draw_text_ex("2", SCREEN_WIDTH/2.0+26.0, 20.0, font_param_title2);
    draw_text_ex(&format!("FPS: {}", fps), 10.0, 15.0, font_param);
    draw_text_ex(&format!("CONTACTS: {}", contacts), 15.0, 50.0, font_param_title2);
    for a in agents.iter() {
        a.draw();
    }
}

fn update(agents: &mut Vec<Agent>, dt: f32) {
    for a in agents.iter_mut() {
        a.update(dt);
    }
}

fn collisions(agents: &Vec<Agent>) -> usize {
    let mut c: usize = 0;
    for a1 in agents.iter() {
        for a2 in agents.iter() {
            if a1.pos != a2.pos {
                let pos1 = make_isometry(a1.pos.x, a1.pos.y, a1.rot);
                let pos2 = make_isometry(a2.pos.x, a2.pos.y, a2.rot);
                let d = contact_circles(pos1, a1.size, pos2, a2.size);
                if d <= 0.0 {
                    c += 1;
                }                
                /* match contact_circles(pos1, a1.size, pos2, a2.size) {
                    Some(c) => {
                        if c.dist <= 0.0 {
                            println!("contact!");
                        }
                    },
                    //Some(_) => {},
                    None => {}
                } */
            }
        }
    }
    c
}

/* fn collisions(agents: &mut Vec<Agent>) {
    for a in agents.iter_mut() {
        contact_ball_ball(pos12, b1, b2, prediction)
        a.update(dt);
    }
} */

async fn wait(delta: f32) {
    let t = FIX_DT - delta;
    if t > 0.0 {
        sleep(Duration::from_secs_f32(t));
    }
}