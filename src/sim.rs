#![allow(unused)]


use macroquad::prelude::*;
use crate::agent::*;


pub struct Simulation {
    pub ui_state: UIState,
    pub signals: Signals,
    //pub sel_time: f32,
    pub selected: u32,
    pub mouse_state: MouseState,
    pub agents: AgentsBox,
    pub dt: f32,
    pub fps: i32,
}

impl Simulation {
    pub fn new() -> Self {
        Self {
            ui_state: UIState::new(),
            signals: Signals::new(),
            selected: 0,
            mouse_state: MouseState { pos: Vec2::NAN},
            agents: AgentsBox::new(),
            dt: f32::NAN,
            fps: 0,
        }
    }

    pub fn update(&mut self) {
        self.dt = get_frame_time();
        self.fps = get_fps();
    }

    pub fn draw(&self) {
        todo!();
    }

    pub fn input(&mut self) {
        todo!();
    }
}
//?         [[[SIGNALS]]]
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

//?         [[[UISTATE]]]
pub struct UIState {
    pub performance: bool,
    pub inspect: bool,
    pub mouse: bool,
    pub create: bool,
    pub quit: bool,
}

impl UIState {
    pub fn new() -> Self {
        Self {
            performance: false,
            inspect: false,
            mouse: false,
            create: false,
            quit: false,
        }
    }
}

//?         [[[MOUSESTATE]]]
pub struct MouseState {
    pub pos: Vec2,
}