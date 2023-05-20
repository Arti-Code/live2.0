
#![allow(unused)]

mod consts;
mod util;
mod agent;
//mod particle;
mod kinetic;
mod ui;

use std::thread::sleep;
use std::time::Duration;
use macroquad::miniquad::conf::Icon;
use macroquad::prelude::*;
use macroquad::window; 
use macroquad::file::*;
use kinetic::*;
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
    let mut cam_pos: Vec2=Vec2::ZERO;
    let mut agents: Vec<Agent> = vec![];
    let mut ui_state = UIState::new();
    for _ in 0..AGENTS_NUM {
        let agent = Agent::new();
        agents.push(agent);
    }

    loop {
        let delta = get_frame_time();
        let fps = get_fps();
        input(&mut cam_pos);
        update(&mut agents, delta);
        ui_process(&mut ui_state, fps, delta);
        draw(&agents, &cam_pos);
        ui_draw();
        next_frame().await;
    }
}

fn init() {
    set_pc_assets_folder("assets");
    window::request_new_screen_size(SCREEN_WIDTH, SCREEN_HEIGHT);
}

fn draw(agents: &Vec<Agent>, cam_pos: &Vec2) {
    /* set_camera(&Camera2D { */
    /*     //target: *cam_pos, */
    /*      */
    /*     zoom: Vec2 { x: 0.05, y: 0.05 }, */
    /*     ..Default::default() */
    /* }); */
    clear_background(BLACK);
    for a in agents.iter() {
        a.draw();
    }
}

fn update(agents: &mut Vec<Agent>, dt: f32) {
    let mut hit_map = CollisionsMap::new();
    hit_map = map_collisions(agents);
    for a in agents.iter_mut() {
        let uid = a.unique;
        a.update(dt);
        match hit_map.get_collision(uid) {
            Some(hit) => {
                a.update_collision(&hit.normal, hit.overlap, dt);
            },
            None => {

            }
        }
    }
}

fn collisions(agents: &Vec<Agent>) -> Vec<(&Agent, Vec2, f32)> {
    let mut contacts_num: usize = 0;
    let mut hits: Vec<(&Agent, Vec2, f32)> = vec![];
    for a1 in agents.iter() {
        for a2 in agents.iter() {
            if a1.unique != a2.unique {
                let contact = contact_circles2(a1.pos, a1.rot, a1.size, a2.pos,a2.rot, a2.size);
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
}

fn map_collisions(agents: &Vec<Agent>) -> CollisionsMap {
    let mut hits: CollisionsMap = CollisionsMap::new();
    for a1 in agents.iter() {
        for a2 in agents.iter() {
            if a1.unique != a2.unique {
                let contact = contact_circles2(a1.pos, a1.rot, a1.size, a2.pos,a2.rot, a2.size);
                match contact {
                    Some(contact) => {
                        if contact.dist <= 0.0 {
                            let p = Vec2::new(contact.point1.x, contact.point1.y);
                            let norm = contact.normal1.data.0[0];
                            let n = Vec2::new(norm[0], norm[1]);
                            let penetration = contact.dist;
                            //hits.push((a1, norm, contact.dist));
                            let hit: Hit=Hit{ normal: n, overlap: contact.dist };
                            hits.add_collision(a1.unique, hit);
                        }
                    },
                    None => {}
                }
            }
        }
    }
    return hits;
}

fn input(cam_pos: &mut Vec2) {
    if is_key_released(KeyCode::Up) {
        cam_pos.y += 10.0;
    }
    if is_key_released(KeyCode::Down) {
        cam_pos.y -= 10.0;
    }
    if is_key_released(KeyCode::Left) {
        cam_pos.x -= 10.0;
    }
    if is_key_released(KeyCode::Right) {
        cam_pos.x += 10.0;
    }
}

async fn wait(delta: f32) {
    let t = FIX_DT - delta;
    if t > 0.0 {
        sleep(Duration::from_secs_f32(t));
    }
}