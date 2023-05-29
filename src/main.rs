
#![allow(unused)]

mod sim;
mod consts;
mod util;
mod agent;
mod timer;
mod kinetic;
mod ui;
mod neuro;
mod progress_bar;
mod prelude;
mod world;

use std::f32::consts::PI;
use std::thread::sleep;
use std::time::Duration;
use macroquad::miniquad::conf::Icon;
use macroquad::prelude::*;
use macroquad::window; 
use macroquad::file::*;
use kinetic::*;
use parry2d::query::details::contact_ball_ball;
use crate::sim::*;
use crate::prelude::*;
use crate::world::*;
use crate::consts::*;
use crate::util::*;
use crate::agent::*;
use crate::world::*;
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

pub struct Signals {
    pub spawn_agent: bool,
}

impl Signals {
    pub fn new() -> Self {
        Self {
            spawn_agent: false,
        }
    }
}

#[macroquad::main(app_configuration)]
async fn main() {
    init();
    let mut cam_pos: Vec2=Vec2::ZERO;
    let mut agents: Vec<Agent> = vec![];
    let mut ui_state = UIState::new();
    let mut signals = Signals::new();
    let mut main_timer = Timer::new(60.0, true, true, true);
    let mut sel_time: f32 = 0.0;
    let mut selected: u8=0;
    let mut mouse_state = MouseState { pos: Vec2::ZERO};
    for _ in 0..AGENTS_NUM {
        let agent: Agent = Agent::new();
        agents.push(agent);
    }

    loop {
        let delta = get_frame_time();
        let fps = get_fps();
        let (mouse_x, mouse_y) = mouse_position();
        mouse_state.pos = Vec2::new(mouse_x, mouse_y);
        let mut agents_num = agents.len() as u8;
        if signals.spawn_agent {
            signals.spawn_agent = false;
            let agent = Agent::new();
            agents.push(agent);
        }
        input(&mut cam_pos, &mut selected, &mut agents_num, &agents);
        update(&mut agents, delta, &mut main_timer, &mut sel_time);
        if agents.len() < 5 {
            let agent = Agent::new();
            agents.push(agent);
            selected = 0;
        }
        let selected_agent = agents.get(selected as usize);
        ui_process(&mut ui_state, fps, delta, main_timer.time, selected_agent, &mouse_state, &mut signals);
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
        let draw_field_of_view: bool = check_selected(a, agents, selected);
        a.draw(draw_field_of_view);
    }
    match agents.get(selected as usize) {
        Some(selected_agent) => {
            let pos = Vec2::new(selected_agent.pos.x, selected_agent.pos.y);
            let s = selected_agent.size;
            draw_circle_lines(pos.x, pos.y, 2.0*s+(sel_time.sin()*s*0.5), 1.0, ORANGE);
        },
        None => {},
    };
}

fn check_selected(agent: &Agent, agents: &Vec<Agent>, selected: u8) -> bool {
    match agents.get(selected as usize) {
        Some(selected_agent) if agent.unique == selected_agent.unique => {
            return true;
        },
        Some(selected_agent) if agent.unique != selected_agent.unique => {
            return false;
        },
        Some(_) => {
            return false;
        },
        None => {
            return false;
        },
    }
}

fn update(agents: &mut Vec<Agent>, dt: f32, timer: &mut Timer, sel_time: &mut f32) {
    let mut to_kill: Vec<u32> = vec![];
    calc_selection_time(sel_time, dt);
    let mut hit_map = CollisionsMap::new();
    let mut detections_map = DetectionsMap::new();
    hit_map = map_collisions(agents);
    detections_map = map_detections(agents);
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
        a.reset_detections();
        match detections_map.get_detection(uid) {
            Some(detection) => {
                a.update_detection(detection);
            },
            None => {
                a.update_detection(&Detection::new_empty());
            }
        }
    }
    agents.retain(|a| a.alife == true);
}

fn calc_selection_time(selection_time: &mut f32, dt: f32) {
    *selection_time += dt*4.0;
    *selection_time = *selection_time%(2.0*PI as f32);
}

fn detect(agents: &mut Vec<Agent>){
    for agent1 in agents.iter() {
        for agent2 in agents.iter() {
            
        }
    }
}

fn map_detections(agents: &Vec<Agent>) -> DetectionsMap {
    let mut detections = DetectionsMap::new();
    for agent1 in agents {
        for agent2 in agents {
            if agent1.unique != agent2.unique {
                let contact = contact_circles(agent1.pos, agent1.rot, agent1.vision_range, agent2.pos, agent2.rot, agent2.vision_range);
                match contact {
                    Some(contact) => {
                        let rel_pos2 = agent2.pos - agent1.pos;
                        let dir1 = Vec2::from_angle(agent1.rot);
                        let ang = dir1.angle_between(rel_pos2);
                        let dist = agent1.pos.distance(agent2.pos);
                        let detection = Detection::new(dist, ang, agent2.pos);
                        detections.add_detection(agent1.unique, detection);
                    },
                    None => {},
                }
            }
        }
    }
    return detections;
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