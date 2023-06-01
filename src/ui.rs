
use std::path::Path;

use egui_macroquad;
use macroquad::prelude::*;
use egui::{self, Context, Style};
use egui::{RichText, Color32};
use egui_extras::image::RetainedImage;
use image::open;
use macroquad::ui::StyleBuilder;

use crate::agent::Agent;
use crate::consts::{SCREEN_WIDTH, SCREEN_HEIGHT};
use crate::{progress_bar::*, Signals};
use crate::sim::*;


static V: Vec2 = Vec2::ZERO;

pub struct UISystem {
    pub state: UIState,
    pub pointer_over: bool,
}

impl UISystem {
    pub fn new() -> Self {
        Self {
            state: UIState::new(),
            pointer_over: false,
        }
    }
    
    pub fn ui_process(&mut self, fps: i32, delta: f32, time: f64, agent: Option<&Agent>, signals: &mut Signals) {
        egui_macroquad::ui(|egui_ctx| {
            self.pointer_over = egui_ctx.is_pointer_over_area();
            self.build_top_menu(egui_ctx);
            self.build_quit_window(egui_ctx);
            self.build_monit_window(egui_ctx, fps, delta, time);
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

    fn build_top_menu(&mut self, egui_ctx: &Context) {
        egui::TopBottomPanel::top("top_panel").show(egui_ctx, |ui| {
            if !self.pointer_over {
                self.pointer_over = ui.ui_contains_pointer();
            }
            egui::menu::bar(ui, |ui| {
                ui.heading(RichText::new( "LIVE 2.0").color(Color32::GREEN).strong());
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

    fn build_monit_window(&self, egui_ctx: &Context, fps: i32, delta: f32, time: f64) {
        if self.state.performance {
            egui::Window::new("Monitor").default_pos((5.0, 100.0))
            .default_width(125.0)
            .show(egui_ctx, |ui| {
                ui.label(format!("DELTA: {}ms", (delta*1000.0).round()));
                ui.separator();
                ui.label(format!("FPS: {}", fps));
                ui.separator();
                ui.label(format!("TIME: {}", time.round()));
            });
        }    
    }

    fn build_inspect_window(&self, egui_ctx: &Context, agent: &Agent) {
        if self.state.inspect {
            let rot = agent.rot;
            let size = agent.size;
            egui::Window::new("Inspector").default_pos((5.0, 200.0))
            .default_width(125.0)
            .show(egui_ctx, |ui| {
                ui.label(format!("ROTATION: {}", ((rot*10.0).round())/10.0));
                ui.label(format!("SIZE: {}", size));
                ui.label(format!("ENERGY: {}/{}", agent.eng.round(), agent.max_eng.round()));
                let eng_prog = agent.eng / agent.max_eng;
                ui.add(ProgressBar::new(eng_prog).desired_width(100.0).fill(Color32::BLUE).show_percentage());
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
            egui::Window::new("New Simulation").default_pos((SCREEN_WIDTH/2.0-65.0, SCREEN_HEIGHT/4.0))
            .default_width(125.0)
            .show(egui_ctx, |ui| {
                ui.horizontal(|head| {
                    head.heading("Start new simulation?");
                });
                ui.horizontal(|mid| {
                    mid.columns(2, |columns| {
                        if columns[0].button(RichText::new("No").color(Color32::WHITE)).clicked() {
                            self.state.new_sim = false;
                        }
                        if columns[1].button(RichText::new("Yes").color(Color32::RED)).clicked() {
                            self.state.new_sim = false;
                            signals.new_sim = true;

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
                        signals.spawn_agent = true;
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
    pub agents_num: i32,
    pub new_sim: bool,
}

impl UIState {
    pub fn new() -> Self {
        Self {
            performance: false,
            inspect: false,
            mouse: false,
            create: false,
            quit: false,
            agents_num: 0,
            new_sim: false,
        }
    }
}