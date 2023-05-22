
#![allow(unused)]

mod world;
mod consts;
mod util;
mod agent;
mod timer;
mod kinetic;
mod ui;
mod neuro;
mod progress_bar;
mod prelude;

use std::f32::consts::PI;
use std::thread::sleep;
use std::time::Duration;
use macroquad::miniquad::conf::Icon;
use macroquad::prelude::*;
use macroquad::window; 
use macroquad::file::*;
use kinetic::*;
use parry2d::query::details::contact_ball_ball;
use crate::prelude::*;
use crate::world::*;
use crate::consts::*;
use crate::util::*;
use crate::agent::*;
use macroquad::time::*;
use std::collections::VecDeque;
use parry2d::query::*;
use parry2d::shape::*;
use crate::ui::*;
use crate::timer::*;

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



#[macroquad::main(app_configuration)]
async fn main() {
    init();
    let mut cam_pos: Vec2=Vec2::ZERO;
    let mut agents: Vec<Agent> = vec![];
    let mut ui_state = UIState::new();
    let mut main_timer = Timer::new(60.0, true, true, true);
    let mut sel_time: f32 = 0.0;
    let mut selected: u8=0;
    let mut mouse_state = MouseState { pos: Vec2::ZERO};
    for _ in 0..AGENTS_NUM {
        let agent = Agent::new();
        agents.push(agent);
    }

    loop {
        let delta = get_frame_time();
        let fps = get_fps();
        let (mouse_x, mouse_y) = mouse_position();
        mouse_state.pos = Vec2::new(mouse_x, mouse_y);
        let mut agents_num = agents.len() as u8;
        input(&mut cam_pos, &mut selected, &mut agents_num, &agents);
        update(&mut agents, delta, &mut main_timer, &mut sel_time);
        ui_process(&mut ui_state, fps, delta, main_timer.time, Some(&agents[selected as usize]), &mouse_state);
        draw(&agents, &cam_pos, selected, sel_time);
        ui_draw();
        next_frame().await;
    }
}

fn init() {
    set_pc_assets_folder("assets");
    window::request_new_screen_size(SCREEN_WIDTH, SCREEN_HEIGHT);
}

fn draw(agents: &Vec<Agent>, cam_pos: &Vec2, selected: u8, sel_time: f32) {
    clear_background(BLACK);
    for a in agents.iter() {
        a.draw();
    }
    let selected_agent = agents.get(selected as usize).unwrap();
    let pos = Vec2::new(selected_agent.pos.x, selected_agent.pos.y);
    let s = selected_agent.size;
    draw_circle_lines(pos.x, pos.y, 3.0*s+(sel_time.sin()*s*1.0), 1.0, SKYBLUE);
}

fn update(agents: &mut Vec<Agent>, dt: f32, timer: &mut Timer, sel_time: &mut f32) {
    if timer.update(dt) {
        println!("TIMER!");
    }
    *sel_time += dt*4.0;
    *sel_time = *sel_time%(2.0*PI as f32);
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

fn map_collisions(agents: &Vec<Agent>) -> CollisionsMap {
    let mut hits: CollisionsMap = CollisionsMap::new();
    for a1 in agents.iter() {
        for a2 in agents.iter() {
            if a1.unique != a2.unique {
                let contact = contact_circles(a1.pos, a1.rot, a1.size, a2.pos,a2.rot, a2.size);
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

fn input(cam_pos: &mut Vec2, agent_idx: &mut u8, max_agent_idx: &mut u8, agents: &Vec<Agent>) {
    if is_key_released(KeyCode::Up) {
        cam_pos.y += 10.0;
        println!("UP");
    }
    if is_key_released(KeyCode::Down) {
        cam_pos.y -= 10.0;
        println!("DOWN");
    }
    if is_key_released(KeyCode::Left) {
        cam_pos.x -= 10.0;
        println!("LEFT");
    }
    if is_key_released(KeyCode::Right) {
        cam_pos.x += 10.0;
        println!("RIGHT");
    }
    if is_key_released(KeyCode::D) {
        if agent_idx < max_agent_idx {
            *agent_idx += 1;
        }
    }
    if is_key_released(KeyCode::A) {
        if agent_idx > &mut 0 {
            *agent_idx -= 1;
        }
    }
    if is_mouse_button_released(MouseButton::Left) {
        let (mouse_posx, mouse_posy) = mouse_position();
        let mouse_pos = Vec2::new(mouse_posx, mouse_posy);
        let mut i: u8 = 0;
        for agent in agents.iter() {
            if contact_mouse(mouse_pos, agent.pos, agent.size) {
                *agent_idx = i;
                break; 
            }
            i += 1;
        }
    }
}

async fn wait(delta: f32) {
    let t = FIX_DT - delta;
    if t > 0.0 {
        sleep(Duration::from_secs_f32(t));
    }
}