
#![allow(unused)]

mod consts;
mod util;
mod agent;
//mod particle;
mod num;
mod ui;

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
//use crate::particle::*;
use macroquad::time::*;
use std::collections::VecDeque;
use parry2d::query::*;
use parry2d::shape::*;
use crate::ui::*;

fn app_configuration() -> Conf {
    Conf{
        window_title: "LIVE 2.0".to_string(),
        window_width: SCREEN_WIDTH as i32,
        window_height: SCREEN_HEIGHT as i32,
        sample_count: 8,
        window_resizable: false,
        ..Default::default()
    }
}

struct CollisionPair<'a> {
    object1: &'a Agent,
    object2: &'a Agent,
    overlap: f32,
}

#[macroquad::main(app_configuration)]
async fn main() {
    init();
    //let text_params = init_text_params().await;
    let mut agents: Vec<Agent> = vec![];
    let mut contacts: Vec<(Vec2, Vec2)> = vec![];
    let mut collisions_list: Vec<CollisionPair> = vec![];
    let mut ui_state = UIState::new();
    for _ in 0..AGENTS_NUM {
        let agent = Agent::new();
        agents.push(agent);
    }

    loop {
        let delta = get_frame_time();
        let fps = get_fps();
        //let mut contacts_num: usize = 0;
        update(&mut agents, delta);
        let hits = collisions(&agents);
        ui_process(&mut ui_state, fps, delta);
        draw(&agents, &contacts);
        ui_draw();
        next_frame().await;
    }
}

fn init() {
    set_pc_assets_folder("assets");
    window::request_new_screen_size(SCREEN_WIDTH, SCREEN_HEIGHT);
}

fn draw(agents: &Vec<Agent>, contacts: &Vec<(Vec2, Vec2)>) {
    clear_background(BLACK);
    for a in agents.iter() {
        a.draw();
    }
    for contact in contacts.iter() {
        let p = contact.0;
        let n = contact.1*10.0;
        draw_line(p.x, p.y, p.x+n.x, p.y+n.y, 1.0, WHITE);
    }
}

fn update(agents: &mut Vec<Agent>, dt: f32) {
    for a in agents.iter_mut() {
        a.update(dt);
    }
}

fn collisions(agents: &Vec<Agent>) -> Vec<(&Agent, Vec2, f32)> {
    let mut contacts_num: usize = 0;
    let mut hits: Vec<(&Agent, Vec2, f32)> = vec![];
    for a1 in agents.iter() {
        for a2 in agents.iter() {
            if a1.pos != a2.pos {
                let pos1 = make_isometry(a1.pos.x, a1.pos.y, a1.rot);
                let pos2 = make_isometry(a2.pos.x, a2.pos.y, a2.rot);
                let contact = contact_circles(pos1, a1.size, pos2, a2.size);
                match contact {
                    Some(contact) => {
                        if contact.dist <= 0.0 {
                            contacts_num += 1;
                            let p = Vec2::new(contact.point1.x, contact.point1.y);
                            let n = contact.normal1.data.0[0];
                            let norm = Vec2::new(n[0], n[1]);
                            let penetration = contact.dist;
                            hits.push((a1, norm, contact.dist))
                        }
                    },
                    None => {}
                }
            }
        }
    }
    return hits;
    //return contacts_num;
}

async fn wait(delta: f32) {
    let t = FIX_DT - delta;
    if t > 0.0 {
        sleep(Duration::from_secs_f32(t));
    }
}