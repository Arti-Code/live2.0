
use std::path::Path;

use egui_macroquad;
use macroquad::prelude::*;
use egui::{self, Context, Style};
use egui::{RichText, Color32};
use egui_extras::image::RetainedImage;
use image::open;
use macroquad::ui::StyleBuilder;

use crate::particle::Molecule;
use crate::consts::{SCREEN_WIDTH, SCREEN_HEIGHT};
use crate::{progress_bar::*, Signals};
use crate::sim::*;


static V: Vec2 = Vec2::ZERO;

pub struct UISystem {
    pub state: UIState,
    pub pointer_over: bool,
    temp_sim_name: String,
}

impl UISystem {
    pub fn new() -> Self {
        Self {
            state: UIState::new(),
            pointer_over: false,
            temp_sim_name: String::new(),
        }
    }
    
    pub fn ui_process(&mut self, sim_state: &SimState, agent: Option<&Molecule>, signals: &mut Signals) {
        egui_macroquad::ui(|egui_ctx| {
            self.pointer_over = egui_ctx.is_pointer_over_area();
            self.build_top_menu(egui_ctx, &sim_state.sim_name);
            self.build_quit_window(egui_ctx);
            self.build_monit_window(egui_ctx, sim_state.fps, sim_state.dt, sim_state.sim_time, sim_state.molecules_num, sim_state.physics_num);
            self.build_mouse_window(egui_ctx);
            match agent {
                Some(agent) => {
                    self.build_inspect_window(egui_ctx, agent)
                },
                None => {}
            }
            self.build_create_window(egui_ctx, signals);
            self.build_new_sim_window(egui_ctx, signals);
        });
    }

    fn build_top_menu(&mut self, egui_ctx: &Context, sim_name: &str) {
        egui::TopBottomPanel::top("top_panel").show(egui_ctx, |ui| {
            if !self.pointer_over {
                self.pointer_over = ui.ui_contains_pointer();
            }
            egui::menu::bar(ui, |ui| {
                ui.heading(RichText::new( sim_name).strong().color(Color32::GREEN));
                ui.add_space(5.0);
                ui.separator();
                ui.add_space(5.0);
                egui::menu::menu_button(ui, RichText::new("Simulation").strong(), |ui| {
                    if ui.button(RichText::new("New Simulation").strong().color(Color32::BLUE)).clicked() {
                        self.state.new_sim = true;
                    }
                    if ui.button(RichText::new("Load Simulation").weak().color(Color32::from_gray(100))).clicked() {
                    }
                    if ui.button(RichText::new("Save Simulation").weak().color(Color32::from_gray(100))).clicked() {
                    }
                    if ui.button(RichText::new("Quit").color(Color32::RED).strong()).clicked() {
                        self.state.quit = true;
                    }
                });
                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);
                egui::menu::menu_button(ui, RichText::new("Tools").strong(), |ui| {
                    if ui.button(RichText::new("Monitor").strong().color(Color32::from_gray(200))).clicked() {
                        self.state.performance = !self.state.performance;
                    }
                    if ui.button(RichText::new("Inspector").strong().color(Color32::from_gray(200))).clicked() {
                        self.state.inspect = !self.state.inspect;
                    }
                    if ui.button(RichText::new("Mouse").strong().color(Color32::from_gray(200))).clicked() {
                        self.state.mouse = !self.state.mouse;
                    }
                    if ui.button(RichText::new("Creator").strong().color(Color32::from_gray(200))).clicked() {
                        self.state.create = !self.state.create;
                    }
                });
                ui.add_space(10.0);
                ui.separator();
            });
        });
    }

    fn build_monit_window(&self, egui_ctx: &Context, fps: i32, delta: f32, time: f64, molecules_num: i32, physics_num: i32) {
        if self.state.performance {
            egui::Window::new("Monitor").default_pos((5.0, 100.0))
            .default_width(125.0)
            .show(egui_ctx, |ui| {
                ui.label(format!("DELTA: {}ms", (delta*1000.0).round()));
                ui.separator();
                ui.label(format!("FPS: {}", fps));
                ui.separator();
                ui.label(format!("TIME: {}", time.round()));
                ui.separator();
                ui.label(format!("AGENTS: {}", molecules_num));
                ui.separator();
                ui.label(format!("PHYSICS OBJECTS: {}", physics_num));
            });
        }    
    }

    fn build_inspect_window(&self, egui_ctx: &Context, agent: &Molecule) {
        if self.state.inspect {
            let rot = agent.rot;
            let size = agent.size;
            egui::Window::new("Inspector").default_pos((5.0, 200.0))
            .default_width(125.0)
            .show(egui_ctx, |ui| {
                ui.label(format!("ROTATION: {}", ((rot*10.0).round())/10.0));
                ui.label(format!("SIZE: {}", size));
            });
        }    
    }

    fn build_mouse_window(&self, egui_ctx: &Context) {
        if self.state.mouse {
            let (mouse_x, mouse_y) = mouse_position();
            egui::Window::new("Mouse").default_pos((5.0, 325.0))
            .default_width(125.0)
            .show(egui_ctx, |ui| {
                ui.label(format!("X: {} | Y: {}", mouse_x.round(), mouse_y.round()));
            });
        }    
    }

    fn build_quit_window(&mut self, egui_ctx: &Context) {
        if self.state.quit {
            egui::Window::new("Quit").default_pos((SCREEN_WIDTH/2.0-65.0, SCREEN_HEIGHT/4.0))
            .default_width(125.0)
            .show(egui_ctx, |ui| {
                ui.horizontal(|head| {
                    head.heading("Are you sure?");
                });
                ui.horizontal(|mid| {
                    mid.columns(2, |columns| {
                        if columns[0].button(RichText::new("No").color(Color32::WHITE)).clicked() {
                            self.state.quit = false;
                        }
                        if columns[1].button(RichText::new("Yes").color(Color32::RED)).clicked() {
                            std::process::exit(0);
                        }
                    });
                });
            });
        }    
    }

    fn build_new_sim_window(&mut self, egui_ctx: &Context, signals: &mut Signals) {
        if self.state.new_sim {
            let mut sim_name: String = String::new();
            egui::Window::new("New Simulation").default_pos((SCREEN_WIDTH/2.0-65.0, SCREEN_HEIGHT/4.0))
            .default_width(125.0)
            .show(egui_ctx, |ui| {
                ui.horizontal(|head| {
                    head.heading("Start new simulation?");
                });
                ui.horizontal(|txt| {
                    let response = txt.add(egui::widgets::TextEdit::singleline(&mut self.temp_sim_name));
                    if response.gained_focus() {
                        self.temp_sim_name=String::new();
                    }
                    if response.changed() {
                        //self.temp_sim_name = String::from(&sim_name);
                        //println!("{:?}", sim_name);
                        //println!("{:?}", self.temp_sim_name);
                    }
                    if response.lost_focus() && txt.input(|i| i.key_pressed(egui::Key::Enter)) {
                        
                        self.state.new_sim = false;
                        signals.new_sim = true;
                        signals.new_sim_name = String::from(&self.temp_sim_name);
                        self.temp_sim_name=String::new();
                    }
                    //let response = txt.text_edit_singleline(&mut sim_name);
                });
                ui.horizontal(|mid| {
                    mid.columns(2, |columns| {
                        if columns[0].button(RichText::new("No").color(Color32::WHITE)).clicked() {
                            self.state.new_sim = false;
                            self.temp_sim_name=String::new();
                        }
                        if columns[1].button(RichText::new("Yes").color(Color32::RED)).clicked() {
                            self.state.new_sim = false;
                            signals.new_sim = true;
                            signals.new_sim_name = String::from(&self.temp_sim_name);
                            self.temp_sim_name=String::new();
                        }
                    });
                });
            });
        }    
    }

    fn build_create_window(&self, egui_ctx: &Context, signals: &mut Signals) {
        if self.state.create {
            egui::Window::new("Creator").default_pos((5.0, 450.0))
            .default_width(125.0)
            .show(egui_ctx, |ui| {
                ui.horizontal(|head| {
                    head.heading("Spawn new creature");
                });
                ui.horizontal(|mid| {
                    mid.style_mut().visuals.extreme_bg_color = Color32::BLUE;
                    if mid.button(RichText::new("SPAWN").strong().color(Color32::WHITE)).clicked() {
                        //self.state.create = false;
                        signals.spawn_molecule = true;
                    }
                });
            });
        }    
    }

    pub fn ui_draw(&self) {
        egui_macroquad::draw();
    }

}


//?         [[[UISTATE]]]
pub struct UIState {
    pub performance: bool,
    pub inspect: bool,
    pub mouse: bool,
    pub create: bool,
    pub quit: bool,
    pub molecules_num: i32,
    pub new_sim: bool,
    pub new_sim_name: String,
}

impl UIState {
    pub fn new() -> Self {
        Self {
            performance: false,
            inspect: false,
            mouse: false,
            create: false,
            quit: false,
            molecules_num: 0,
            new_sim: false,
            new_sim_name: String::new(),
        }
    }
}