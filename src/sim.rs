//#![allow(unused)]

// main Simulation struct

use std::f32::consts::PI;
use macroquad::prelude::*;
use macroquad::camera::Camera2D;
use egui_macroquad;
use crate::agent::*;
use crate::consts::*;
use crate::kinetic::*;
use crate::ui::*;
use crate::source::*;
use crate::util::Signals;
use crate::object::*;
use crate::world::*;


pub struct Simulation {
    pub simulation_name: String,
    pub world_size: Vec2,
    pub world: World,
    zoom_rate: f32,
    pub camera: Camera2D,
    pub running: bool,
    pub sim_time: f64,
    config: SimConfig,
    pub collisions_map: CollisionsMap,
    pub detections_map: DetectionsMap,
    pub ui: UISystem,
    pub sim_state: SimState,
    pub signals: Signals,
    select_phase: f32,
    pub selected: u64,
    pub mouse_state: MouseState,
    pub agents: AgentsBox,
}

struct CamConfig {
    zoom_rate: f32,
    ratio: f32,
    target: Vec2,
    zoom: Vec2,
    offset: Vec2,
}

impl Simulation {
    pub fn new(configuration: SimConfig) -> Self {
        let scr_ratio = SCREEN_WIDTH/SCREEN_HEIGHT;
        let zoom_rate = 1.0/1000.0;
        Self {
            simulation_name: String::new(),
            world_size: Vec2 { x: WORLD_W, y: WORLD_H },
            world: World::new(),
            zoom_rate: scr_ratio,
            camera: Camera2D {
                zoom: Vec2 {x: zoom_rate, y: zoom_rate*scr_ratio},
                offset: Vec2 {x: -1.0, y: -1.0},
                ..Default::default()
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
            select_phase: 0.0,
            mouse_state: MouseState { pos: Vec2::NAN},
            agents: AgentsBox::new(),
            //sources: SourcesBox::new(),
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
        self.world = World::new();
        self.agents.agents.clear();
        self.sim_time = 0.0;
        self.collisions_map = CollisionsMap::new();
        self.detections_map = DetectionsMap::new();
        self.sim_state = SimState::new();
        self.sim_state.sim_name = String::from(&self.simulation_name);
        self.signals = Signals::new();
        self.selected = 0;
        self.select_phase = 0.0;
        self.mouse_state = MouseState { pos: Vec2::NAN};
        self.running = true;
    }

    pub fn init(&mut self) {
        let agents_num = self.config.agents_init_num;
        self.agents.add_many_agents(agents_num as usize, &mut self.world);
        //self.sources.add_many(48);
    }

    pub fn autorun_new_sim(&mut self) {
        self.signals.new_sim = true;
        self.signals.new_sim_name = "Simulation".to_string();
    }

    fn update_agents(&mut self) {
        for (id, agent) in self.agents.get_iter_mut() {
            agent.update2(&mut self.world);
        }
        let dt = self.sim_state.dt;
        for (id, agent) in self.agents.get_iter_mut() {
            let uid = *id;
            if !agent.update(dt) {
                match agent.physics_handle {
                    Some(handle) => {
                        self.world.remove_physics_object(handle);
                    },
                    None => {},
                }
            };
        }
        self.agents.agents.retain(|_, agent| agent.alife == true);
    }

    pub fn update(&mut self) {
        self.signals_check();
        self.update_sim_state();
        self.check_agents_num();
        self.calc_selection_time();
        self.update_agents();
        self.world.step_physics();
    }

    pub fn draw(&self) {
        set_camera(&self.camera);
        clear_background(BLACK);
        draw_rectangle_lines(0.0, 0.0, self.world_size.x, self.world_size.y, 3.0, WHITE);
        self.draw_grid(50);
        self.draw_agents();
    }

    fn draw_agents(&self) {
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

    fn draw_grid(&self, cell_size: u32) {
        let w = self.world_size.x;
        let h = self.world_size.y;
        let col_num = ((w/cell_size as f32).floor() as u32);
        let row_num = ((h/cell_size as f32).floor() as u32);
        //draw_grid(100, 20.0, GRAY, DARKGRAY);
        for x in 0..col_num+1 {
            for y in 0..row_num+1 {
                draw_circle((x*cell_size) as f32, (y*cell_size )as f32, 1.0, GRAY);
            }
        }
    }

    pub fn signals_check(&mut self) {
        if self.signals.spawn_agent {
            let agent = Agent::new();
            self.agents.add_agent(agent, &mut self.world);
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
            //let ratio = self.cam_config.ratio;
            //self.camera.zoom += Vec2::new(0.0001, 0.0001*ratio);
        }
        if is_key_pressed(KeyCode::KpSubtract) {
            if self.camera.zoom.x > 0.0001 {
                //let ratio = self.cam_config.ratio;
                //self.camera.zoom -= Vec2::new(0.0001, 0.0001*ratio);
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
                let offset = self.camera.offset;
                let target = self.camera.target;
                let zoom = self.camera.zoom;
                let rotation = self.camera.rotation;
                let mouse_pos = Vec2::new(mouse_posx, mouse_posy);
                //println!("rel mouse: [{} | {}]", rel_x, rel_y);
                //println!("offset: [{} | {}], zoom: [{} | {}], target: [{} | {}], rotation: [{}]", offset.x, offset.y, zoom.x, zoom.y, target.x, target.y, rotation);
                let rel_coords = self.camera.screen_to_world(mouse_pos);
                //println!("SCR COORDS: [{} | {}] ==> WORLD COORDS: [{} | {}]", mouse_posx, mouse_posy, rel_coords.x, rel_coords.y);
                for (id, agent) in self.agents.get_iter() {
                    if contact_mouse(rel_coords, agent.pos, agent.size) {
                        self.selected = *id;
                        break; 
                    }
                }
            }
        }
    }

    fn update_sim_state(&mut self) {
        self.sim_state.fps = get_fps();
        self.sim_state.dt = get_frame_time();
        self.sim_state.sim_time += self.sim_state.dt as f64;
        let (mouse_x, mouse_y) = mouse_position();
        self.mouse_state.pos = Vec2::new(mouse_x, mouse_y);
        self.sim_state.agents_num = self.agents.count() as i32;
        self.sim_state.physics_num = self.world.get_physics_obj_num() as i32;
    }

    fn check_agents_num(&mut self) {
        if self.sim_state.agents_num < (self.config.agent_min_num as i32) {
            let agent = Agent::new();
            self.agents.add_agent(agent, &mut self.world);
        }
        if self.sim_state.sources_num < (self.config.sources_min_num as i32) {
            let source = Source::new();
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
                                let hit: Hit=Hit{ normal: n, overlap: contact.dist, target_type: ObjectType::Agent, target_id: idx2 };
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
    pub sources_init_num: usize,
    pub sources_min_num: usize,
}

impl Default for SimConfig {
    fn default() -> Self {
        Self {
            agents_init_num: AGENTS_NUM,
            agent_min_num: AGENTS_NUM_MIN,
            agent_speed: AGENT_SPEED,
            agent_rotation: AGENT_ROTATION,
            agent_vision_range: AGENT_VISION_RANGE,
            sources_init_num: SOURCES_NUM,
            sources_min_num: SOURCES_NUM_MIN,
        }
    }
}

impl SimConfig {
    pub fn new(agents_num: usize, agents_min_num: usize, agent_speed: f32, agent_turn: f32, vision_range: f32, sources_num: usize, sources_min_num: usize) -> Self {
        Self {
            agents_init_num: agents_num,
            agent_min_num: agents_min_num,
            agent_speed: agent_speed,
            agent_rotation: agent_turn,
            agent_vision_range: vision_range,
            sources_init_num: sources_num,
            sources_min_num: sources_min_num,
        }
    }
}

//?         [[[SIM_STATE]]]
pub struct SimState {
    pub sim_name: String,
    pub agents_num: i32,
    pub sources_num: i32,
    pub physics_num: i32,
    pub sim_time: f64,
    pub fps: i32,
    pub dt: f32,
}

impl SimState {
    pub fn new() -> Self {
        Self {
            sim_name: String::new(),
            agents_num: 0,
            sources_num: 0,
            physics_num: 0,
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