
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
    let mut sel_time: f32 = 0.0;
    let mut selected: u32=0;
    let mut mouse_state = MouseState { pos: Vec2::ZERO};
    
    
    loop {
        sim.input();
        //input(&mut cam_pos, &mut selected, &agents);
        let selected_agent = sim.agents.get(selected);
        ui_process(&mut sim.ui_state, sim.fps, sim.dt, selected_agent, &sim.mouse_state, &mut sim.signals);
        sim.update();
        sim.draw();
        //draw(&agents, &cam_pos, selected, sel_time);
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

async fn wait(delta: f32) {
    let t = FIX_DT - delta;
    if t > 0.0 {
        sleep(Duration::from_secs_f32(t));
    }
}