//#![allow(unused)]

// main Simulation struct

use std::f32::consts::PI;
use macroquad::prelude::*;
use macroquad::camera::Camera2D;
use egui_macroquad;
use crate::particle::*;
use crate::consts::*;
use crate::kinetic::*;
use crate::ui::*;
use crate::source::*;
use crate::util::Signals;
use crate::object::*;
use crate::world::*;
use crate::camera::*;


pub struct Simulation {
    pub simulation_name: String,
    pub world_size: Vec2,
    pub world: World,
    zoom_rate: f32,
    screen_ratio: f32,
    pub camera: Camera2D,
    pub running: bool,
    pub sim_time: f64,
    config: SimConfig,
    pub ui: UISystem,
    pub sim_state: SimState,
    pub signals: Signals,
    select_phase: f32,
    pub selected: u64,
    pub mouse_state: MouseState,
    pub molecules: MoleculesBox,
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
            zoom_rate: 1.0 / 600.0,
            screen_ratio: SCREEN_WIDTH / SCREEN_HEIGHT,
            camera: create_camera(),
            running: false,
            sim_time: 0.0,    
            config: configuration,
            ui: UISystem::new(),
            sim_state: SimState::new(),
            signals: Signals::new(),
            selected: 0,
            select_phase: 0.0,
            mouse_state: MouseState { pos: Vec2::NAN},
            molecules: MoleculesBox::new(),
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
        self.molecules.molecules.clear();
        self.sim_time = 0.0;
        self.sim_state = SimState::new();
        self.sim_state.sim_name = String::from(&self.simulation_name);
        self.signals = Signals::new();
        self.selected = 0;
        self.select_phase = 0.0;
        self.mouse_state = MouseState { pos: Vec2::NAN};
        self.running = true;
    }

    pub fn init(&mut self) {
        self.world.build();
        let molecules_num = self.config.molecules_init_num;
        self.molecules.add_many_molecules(1024, &mut self.world);
        //self.sources.add_many(48);
    }

    pub fn autorun_new_sim(&mut self) {
        self.signals.new_sim = true;
        self.signals.new_sim_name = "Simulation".to_string();
    }

    fn update_molecules(&mut self) {
        for (id, molecule) in self.molecules.get_iter_mut() {
            molecule.update2(&mut self.world);
        }
        let dt = self.sim_state.dt;
    }

    pub fn update(&mut self) {
        self.signals_check();
        self.update_sim_state();
        self.check_molecules_num();
        self.calc_selection_time();
        self.update_molecules();
        self.world.step_physics();
    }

    pub fn draw(&self) {
        //set_default_camera();
        set_camera(&self.camera);
        clear_background(BLACK);
        draw_rectangle_lines(0.0, 0.0, self.world_size.x, self.world_size.y, 3.0, WHITE);
        self.draw_grid(50);
        self.draw_molecules();
    }

    fn draw_molecules(&self) {
        for (id, molecule) in self.molecules.get_iter() {
            molecule.draw();
        }
        match self.molecules.get(self.selected) {
            Some(selected_molecule) => {
                let pos = Vec2::new(selected_molecule.pos.x, selected_molecule.pos.y);
                let s = selected_molecule.size;
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
        if self.signals.spawn_molecule {
            let molecule = Molecule::new();
            self.molecules.add_molecule(molecule, &mut self.world);
            self.signals.spawn_molecule = false;
        }
        if self.signals.new_sim {
            self.signals.new_sim = false;
            //if !self.signals.new_sim_name.is_empty() {
            self.reset_sim(Some(&self.signals.new_sim_name.to_owned()));
            //}
        }
    }

    fn get_selected(&self) -> Option<&Molecule> {
        match self.molecules.get(self.selected) {
            Some(selected_molecule) => {
                return Some(selected_molecule);
            },
            None => {
                return None;
            },
        };
    }

    pub fn input(&mut self) {
        self.mouse_input();
        control_camera(&mut self.camera, self.screen_ratio);
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
                for (id, molecule) in self.molecules.get_iter() {
                    if contact_mouse(rel_coords, molecule.pos, molecule.size) {
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
        self.sim_state.molecules_num = self.molecules.count() as i32;
        self.sim_state.physics_num = self.world.get_physics_obj_num() as i32;
    }

    fn check_molecules_num(&mut self) {
        if self.sim_state.molecules_num < (self.config.molecule_min_num as i32) {
            let molecule = Molecule::new();
            self.molecules.add_molecule(molecule, &mut self.world);
        }
    }

    fn calc_selection_time(&mut self) {
        self.select_phase += self.sim_state.dt*4.0;
        self.select_phase = self.select_phase%(2.0*PI as f32);
    }

    pub fn process_ui(&mut self) {
        let marked_molecule = self.molecules.get(self.selected);
        self.ui.ui_process(&self.sim_state, marked_molecule, &mut self.signals)
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
    pub molecules_init_num: usize,
    pub molecule_min_num: usize,
    pub molecule_speed: f32,
}

impl Default for SimConfig {
    fn default() -> Self {
        Self {
            molecules_init_num: MOLECULE_NUM,
            molecule_min_num: MOLECULE_NUM_MIN,
            molecule_speed: MOLECULE_SPEED,
        }
    }
}

impl SimConfig {
    pub fn new(molecules_num: usize, molecules_min_num: usize, molecule_speed: f32, molecule_turn: f32, vision_range: f32, sources_num: usize, sources_min_num: usize) -> Self {
        Self {
            molecules_init_num: molecules_num,
            molecule_min_num: molecules_min_num,
            molecule_speed: molecule_speed,
        }
    }
}

//?         [[[SIM_STATE]]]
pub struct SimState {
    pub sim_name: String,
    pub molecules_num: i32,
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
            molecules_num: 0,
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