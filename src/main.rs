
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


use macroquad::miniquad::conf::Icon;
use macroquad::prelude::*;
use macroquad::window;
use crate::sim::*;
use crate::world::*;
use crate::consts::*;
use crate::util::*;
use crate::agent::*;
use crate::world::*;
use macroquad::time::*;
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

#[macroquad::main(app_configuration)]
async fn main() {
    let cfg = SimConfig::default();
    let mut sim = Simulation::new(&"Simulation One", cfg);
    sim.init();    
    
    loop {
        sim.input();
        let selected_agent = sim.agents.get(sim.selected);
        sim.ui.ui_process(sim.fps, sim.dt, selected_agent, &mut sim.signals);
        sim.update();
        sim.draw();
        sim.draw_ui();
        next_frame().await;
    }
}