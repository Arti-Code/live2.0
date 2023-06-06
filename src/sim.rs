#![allow(unused)]


use std::f32::consts::PI;
use macroquad::prelude::*;
use egui_macroquad;
use crate::agent::*;
use crate::consts::*;
use crate::kinetic::*;
use crate::ui::*;
use crate::util::*;


pub struct Simulation {
    pub simulation_name: String,
    pub camera: Camera2D,
    cam_config: CamConfig,
    pub running: bool,
    pub sim_time: f64,
    config: SimConfig,
    pub collisions_map: CollisionsMap,
    pub detections_map: DetectionsMap,
    pub ui: UISystem,
    pub sim_state: SimState,
    pub signals: Signals,
    select_phase: f32,
    pub selected: u32,
    pub mouse_state: MouseState,
    pub agents: AgentsBox,
    pub dt: f32,
    pub old_dt: f32,
    pub fps: i32,
}

struct CamConfig {
    target: Vec2,
    zoom: f32,
    offset: Vec2,
    ratio: f32,
}

impl Simulation {
    pub fn new(configuration: SimConfig) -> Self {
        let screen_ratio: f32 = SCREEN_WIDTH/SCREEN_HEIGHT;
        let ratio = SCREEN_WIDTH/SCREEN_HEIGHT;
        let zoom = 1.0/1000.0;
        Self {
            simulation_name: String::new(),
            cam_config: CamConfig { 
                zoom: zoom, 
                ratio:  ratio,
                target: Vec2 { x: 0.0, y: 0.0 },
                offset: Vec2 { x: 0.0, y: 0.0 }, 
            },
            camera: Camera2D {
                //zoom: Vec2 {x: zoom*ratio.0, y: zoom*ratio.0},
                zoom: Vec2 {x: zoom, y: zoom*ratio},
                target: Vec2 {x: -0.5, y: -0.5},
                offset: Vec2 {x: -0.5, y: -0.5},
                rotation: 0.0,
                render_target: None,
                viewport: None,
            },
            running: false,
            sim_time: 0.0,    
            config: configuration,
            collisions_map: CollisionsMap::new(),
            detections_map: DetectionsMap::new(),
            ui: UISystem::new(),
            sim_state: SimState::new(),
            signals: Signals::new(),
            selected: 0,
            old_dt: 0.0,
            select_phase: 0.0,
            mouse_state: MouseState { pos: Vec2::NAN},
            agents: AgentsBox::new(),
            dt: f32::NAN,
            fps: 0,
        }
    }

    fn reset_sim(&mut self, sim_name: Option<&str>) {
        self.simulation_name = match sim_name {
            Some(name) => {
                name.to_string()
            },
            None => {
                String::new()
            },
        };
        self.agents.agents.clear();
        self.sim_time = 0.0;
        self.collisions_map = CollisionsMap::new();
        self.detections_map = DetectionsMap::new();
        self.sim_state = SimState::new();
        self.sim_state.sim_name = String::from(&self.simulation_name);
        self.signals = Signals::new();
        self.selected = 0;
        self.old_dt = 0.0;
        self.select_phase = 0.0;
        self.mouse_state = MouseState { pos: Vec2::NAN};
        self.dt = f32::NAN;
        self.fps = 0;
        self.running = true;
    }

    pub fn init(&mut self) {
        let agents_num = self.config.agents_init_num;
        self.agents.add_many_agents(agents_num as usize);
    }

    pub fn update(&mut self) {
        self.signals_check();
        self.update_sim_state();
        self.check_agents_num();
        self.calc_selection_time();
        self.collisions_map = self.map_collisions();
        self.detections_map = self.map_detections();
        let dt = self.sim_state.dt;
        for (id, a) in self.agents.get_iter_mut() {
            let uid = *id;
            a.update(dt);
            match self.collisions_map.get_collision(uid) {
                Some(hit) => {
                    a.update_collision(&hit.normal, hit.overlap, dt);
                },
                None => {
    
                }
            }
            a.reset_detections();
            match self.detections_map.get_detection(uid) {
                Some(detection) => {
                    a.update_detection(detection);
                },
                None => {
                    a.update_detection(&Detection::new_empty());
                }
            }
        }
        self.agents.agents.retain(|_, agent| agent.alife == true);
    }

    pub fn draw(&self) {
        set_camera(&self.camera);
        //set_default_camera();
        clear_background(BLACK);
        for (id, agent) in self.agents.get_iter() {
            let mut draw_field_of_view: bool=false;
            if *id == self.selected {
                draw_field_of_view = true;
            };
            agent.draw(draw_field_of_view);
        }
        match self.agents.get(self.selected) {
            Some(selected_agent) => {
                let pos = Vec2::new(selected_agent.pos.x, selected_agent.pos.y);
                let s = selected_agent.size;
                draw_circle_lines(pos.x, pos.y, 2.0*s+(self.select_phase.sin()*s*0.5), 1.0, ORANGE);
            },
            None => {},
        };
    }

    pub fn signals_check(&mut self) {
        if self.signals.spawn_agent {
            let agent = Agent::new();
            self.agents.add_agent(agent);
            self.signals.spawn_agent = false;
        }
        if self.signals.new_sim {
            self.signals.new_sim = false;
            //if !self.signals.new_sim_name.is_empty() {
            self.reset_sim(Some(&self.signals.new_sim_name.to_owned()));
            //}
        }
    }

    fn get_selected(&self) -> Option<&Agent> {
        match self.agents.get(self.selected) {
            Some(selected_agent) => {
                return Some(selected_agent);
            },
            None => {
                return None;
            },
        };
    }

    pub fn input(&mut self) {
        self.mouse_input();
        self.keys_input();
    }

    fn keys_input(&mut self) {
        if is_key_pressed(KeyCode::KpAdd) {
            let ratio = self.cam_config.ratio;
            self.camera.zoom += Vec2::new(0.0001, 0.0001*ratio);
        }
        if is_key_pressed(KeyCode::KpSubtract) {
            if self.camera.zoom.x > 0.0001 {
                let ratio = self.cam_config.ratio;
                self.camera.zoom -= Vec2::new(0.0001, 0.0001*ratio);
            }
        }
        if is_key_pressed(KeyCode::Kp4) {
            self.camera.offset.x += 0.1;
        }
        if is_key_pressed(KeyCode::Kp6) {
            self.camera.offset.x -= 0.1;
        }
        if is_key_pressed(KeyCode::Kp8) {
            self.camera.offset.y -= 0.1;
        }
        if is_key_pressed(KeyCode::Kp2) {
            self.camera.offset.y += 0.1;
        }
        if is_key_pressed(KeyCode::Left) {
            println!("target");
            self.camera.target.x += 0.1;
        }
        if is_key_pressed(KeyCode::Right) {
            println!("target");
            self.camera.target.x -= 0.1;
        }
        if is_key_pressed(KeyCode::Up) {
            println!("target");
            self.camera.target.y -= 0.1;
        }
        if is_key_pressed(KeyCode::Down) {
            println!("target");
            self.camera.target.y += 0.1;
        }
    }

    fn mouse_input(&mut self) {
        if is_mouse_button_released(MouseButton::Left) {
            if !self.ui.pointer_over {
                self.selected = 0;
                let (mouse_posx, mouse_posy) = mouse_position();
                let mouse_pos = Vec2::new(mouse_posx, mouse_posy);
                for (id, agent) in self.agents.get_iter() {
                    if contact_mouse(mouse_pos, agent.pos, agent.size) {
                        self.selected = *id;
                        break; 
                    }
                }
            }
        }
    }

    fn update_sim_state(&mut self) {
        //self.old_dt = self.dt;
        //self.dt = get_frame_time();
        //self.fps = get_fps();
        self.sim_state.fps = get_fps();
        self.sim_state.dt = get_frame_time();
        self.sim_state.sim_time += self.sim_state.dt as f64;
        let (mouse_x, mouse_y) = mouse_position();
        self.mouse_state.pos = Vec2::new(mouse_x, mouse_y);
        self.sim_state.agents_num = self.agents.count() as i32;
    }

    fn check_agents_num(&mut self) {
        if self.sim_state.agents_num < (self.config.agent_min_num as i32) {
            let agent = Agent::new();
            self.agents.add_agent(agent);
        }
    }

    fn calc_selection_time(&mut self) {
        self.select_phase += self.sim_state.dt*4.0;
        self.select_phase = self.select_phase%(2.0*PI as f32);
    }

    fn map_detections(&self) -> DetectionsMap {
        let mut detections = DetectionsMap::new();
        for (id1, agent1) in self.agents.get_iter() {
            for (id2, agent2) in self.agents.get_iter() {
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
    
    fn map_collisions(&self) -> CollisionsMap {
        let mut hits: CollisionsMap = CollisionsMap::new();
        for (id1, a1) in self.agents.get_iter() {
            for (id2, a2) in self.agents.get_iter() {
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

    pub fn process_ui(&mut self) {
        let marked_agent = self.agents.get(self.selected);
        self.ui.ui_process(&self.sim_state, marked_agent, &mut self.signals)
    }

    pub fn draw_ui(&self) {
        self.ui.ui_draw();
    }

    pub fn is_running(&self) -> bool {
        return self.running;
    }
}


//?         [[[SIM_CONFIG]]]
#[derive(Clone, Copy)]
pub struct SimConfig {
    pub agents_init_num: usize,
    pub agent_min_num: usize,
    pub agent_speed: f32,
    pub agent_vision_range: f32,
    pub agent_rotation: f32,
}

impl Default for SimConfig {
    fn default() -> Self {
        Self {
            agents_init_num: AGENTS_NUM,
            agent_min_num: AGENTS_NUM_MIN,
            agent_speed: AGENT_SPEED,
            agent_rotation: AGENT_ROTATION,
            agent_vision_range: AGENT_VISION_RANGE,
        }
    }
}

impl SimConfig {
    pub fn new(agents_num: usize, agents_min_num: usize, agent_speed: f32, agent_turn: f32, vision_range: f32) -> Self {
        Self {
            agents_init_num: agents_num,
            agent_min_num: agents_min_num,
            agent_speed: agent_speed,
            agent_rotation: agent_turn,
            agent_vision_range: vision_range,
        }
    }
}

//?         [[[SIM_STATE]]]
pub struct SimState {
    pub sim_name: String,
    pub agents_num: i32,
    pub sim_time: f64,
    pub fps: i32,
    pub dt: f32,
}

impl SimState {
    pub fn new() -> Self {
        Self {
            sim_name: String::new(),
            agents_num: 0,
            sim_time: 0.0,
            fps: 0,
            dt: 0.0,
        }
    }
}

//?         [[[MOUSESTATE]]]
pub struct MouseState {
    pub pos: Vec2,
}