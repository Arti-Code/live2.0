#![allow(unused)]

mod consts;
mod util;
mod agent;

use macroquad::prelude::*;
use macroquad::window; 
use crate::consts::*;
use crate::util::*;
use crate::agent::*;
use macroquad::time::*;


#[macroquad::main("Macro Physics")]
async fn main() {
    let conf = Conf{
        window_title: "LIVE2.0".to_string(),
        window_width: SCREEN_WIDTH as i32,
        window_height: SCREEN_HEIGHT as i32,
        sample_count: 8,
        window_resizable: false,
        ..Default::default()
        //icon: Some(())
    };
    window::request_new_screen_size(SCREEN_WIDTH, SCREEN_HEIGHT);
    let mut agents: Vec<Agent> = vec![];

    for _ in 0..AGENTS_NUM {
        let agent = Agent::new();

        agents.push(agent);
    }

    loop {
        let delta =get_frame_time();
        update(&mut agents, delta);
        draw(&agents);

        next_frame().await;
    }
}

fn draw(agents: &Vec<Agent>) {
    clear_background(BLACK);

    for a in agents.iter() {
        a.draw();
    }
}

fn update(agents: &mut Vec<Agent>, dt: f32) {
    for a in agents.iter_mut() {
        a.update(dt);
    }
}

