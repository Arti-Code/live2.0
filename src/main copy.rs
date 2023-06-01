
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
use egui_extras::RetainedImage;
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



#[macroquad::main(app_configuration)]
async fn main() {
    init();
    let logo = RetainedImage::from_image_bytes("life2_logo.png", include_bytes!("life2_logo.png")).unwrap();
    let cfg = SimConfig::default();
    let mut sim = Simulation::new(&"Simulation One", cfg);
    sim.init();

    //let mut cam_pos: Vec2=Vec2::ZERO;
    //let mut agents: AgentsBox=AgentsBox::new();
    //let mut ui_state = UIState::new();
    //let mut signals = Signals::new();
    let mut sel_time: f32 = 0.0;
    let mut selected: u32=0;
    let mut mouse_state = MouseState { pos: Vec2::ZERO};
//    for _ in 0..AGENTS_NUM {
//        let agent: Agent = Agent::new();
//        agents.add_agent(agent);
//    }

    loop {
        let delta = get_frame_time();
        let fps = get_fps();
        let (mouse_x, mouse_y) = mouse_position();
        mouse_state.pos = Vec2::new(mouse_x, mouse_y);
        let mut agents_num = agents.count();
        if signals.spawn_agent {
            signals.spawn_agent = false;
            let agent = Agent::new();
            agents.add_agent(agent);
        }
        input(&mut cam_pos, &mut selected, &agents);
        update(&mut agents, delta, &mut sel_time);
        if agents.count() < AGENTS_NUM_MIN {
            let agent = Agent::new();
            agents.add_agent(agent);
            selected = 0;
        }
        let selected_agent = agents.get(selected);
        ui_process(&mut ui_state, fps, delta, selected_agent, &mouse_state, &mut signals, &logo);
        draw(&agents, &cam_pos, selected, sel_time);
        ui_draw();
        next_frame().await;
    }
}

fn init() {
    set_pc_assets_folder("assets");
    window::request_new_screen_size(SCREEN_WIDTH, SCREEN_HEIGHT);
}

fn draw(agents: &AgentsBox, cam_pos: &Vec2, selected: u32, sel_time: f32) {
    clear_background(BLACK);
    for (id, a) in agents.get_iter() {
        let mut draw_field_of_view: bool=false;
        if *id == selected {
            draw_field_of_view = true;
        };
        a.draw(draw_field_of_view);
    }
    match agents.get(selected) {
        Some(selected_agent) => {
            let pos = Vec2::new(selected_agent.pos.x, selected_agent.pos.y);
            let s = selected_agent.size;
            draw_circle_lines(pos.x, pos.y, 2.0*s+(sel_time.sin()*s*0.5), 1.0, ORANGE);
        },
        None => {},
    };
}

fn check_selected(agent: &Agent, agents: &AgentsBox, selected: u32) -> bool {
    match agents.get(selected) {
        Some(selected_agent) => {
            return true;
        },
        Some(_) => {
            return false;
        },
        None => {
            return false;
        },
    }
}

fn update(agents: &mut AgentsBox, dt: f32, sel_time: &mut f32) {
    calc_selection_time(sel_time, dt);
    let mut hit_map = CollisionsMap::new();
    let mut detections_map = DetectionsMap::new();
    hit_map = map_collisions(agents);
    detections_map = map_detections(agents);
    for (id, a) in agents.get_iter_mut() {
        let uid = *id;
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
    agents.agents.retain(|_, agent| agent.alife == true);
}

fn calc_selection_time(selection_time: &mut f32, dt: f32) {
    *selection_time += dt*4.0;
    *selection_time = *selection_time%(2.0*PI as f32);
}

fn map_detections(agents: &AgentsBox) -> DetectionsMap {
    let mut detections = DetectionsMap::new();
    for (id1, agent1) in agents.get_iter() {
        for (id2, agent2) in agents.get_iter() {
            let idx1 = *id1; let idx2 = *id2;
            if idx1 != idx2 {
                let contact = contact_circles(agent1.pos, agent1.rot, agent1.vision_range, agent2.pos, agent2.rot, agent2.size);
                match contact {
                    Some(contact) => {
                        let rel_pos2 = agent2.pos - agent1.pos;
                        let dir1 = Vec2::from_angle(agent1.rot);
                        let ang = dir1.angle_between(rel_pos2);
                        let dist = agent1.pos.distance(agent2.pos);
                        let detection = Detection::new(dist, ang, agent2.pos);
                        detections.add_detection(idx1, detection);
                    },
                    None => {},
                }
            }
        }
    }
    return detections;
}

fn map_collisions(agents: &AgentsBox) -> CollisionsMap {
    let mut hits: CollisionsMap = CollisionsMap::new();
    for (id1, a1) in agents.get_iter() {
        for (id2, a2) in agents.get_iter() {
            let idx1 = *id1; let idx2 = *id2;
            if idx1 != idx2 {
                let contact = contact_circles(a1.pos, a1.rot, a1.size, a2.pos,a2.rot, a2.size);
                match contact {
                    Some(contact) => {
                        if contact.dist <= 0.0 {
                            let p = Vec2::new(contact.point1.x, contact.point1.y);
                            let norm = contact.normal1.data.0[0];
                            let n = Vec2::new(norm[0], norm[1]);
                            let penetration = contact.dist;
                            let hit: Hit=Hit{ normal: n, overlap: contact.dist };
                            hits.add_collision(idx1, hit);
                        }
                    },
                    None => {}
                }
            }
        }
    }
    return hits;
}

fn input(cam_pos: &mut Vec2, agent_idx: &mut u32, agents: &AgentsBox) {
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
//    if is_key_released(KeyCode::D) {
//        if agent_idx < max_agent_idx {
//            *agent_idx += 1;
//        }
//    }
//    if is_key_released(KeyCode::A) {
//        if agent_idx > &mut 0 {
//            *agent_idx -= 1;
//        }
//    }
    if is_mouse_button_released(MouseButton::Left) {
        let (mouse_posx, mouse_posy) = mouse_position();
        let mouse_pos = Vec2::new(mouse_posx, mouse_posy);
        let mut i: u32 = 0;
        for (id, agent) in agents.get_iter() {
            if contact_mouse(mouse_pos, agent.pos, agent.size) {
                *agent_idx = *id;
                break; 
            }
            //i += 1;
        }
    }
}

async fn wait(delta: f32) {
    let t = FIX_DT - delta;
    if t > 0.0 {
        sleep(Duration::from_secs_f32(t));
    }
}