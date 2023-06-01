
use std::path::Path;

use egui_macroquad;
use macroquad::prelude::*;
use egui::{self, Context};
use egui::{RichText, Color32};
use egui_extras::image::RetainedImage;
use image::open;

use crate::agent::Agent;
use crate::consts::{SCREEN_WIDTH, SCREEN_HEIGHT};
use crate::{progress_bar::*, Signals};
use crate::sim::*;


static V: Vec2 = Vec2::ZERO;

pub struct UISystem {
    pub state: UIState,
}

impl UISystem {
    pub fn new() -> Self {
        Self {
            state: UIState::new(),
        }
    }
    
    pub fn ui_process(&mut self, fps: i32, delta: f32, agent: Option<&Agent>, mouse_state: &MouseState, signals: &mut Signals) {
        egui_macroquad::ui(|egui_ctx| {
            self.build_top_menu(egui_ctx);
            self.build_quit_window(egui_ctx);
            self.build_monit_window(egui_ctx, fps, delta);
            self.build_mouse_window(egui_ctx, mouse_state);
            match agent {
                Some(agent) => {
                    self.build_inspect_window(egui_ctx, agent)
                },
                None => {}
            }
            self.build_create_window(egui_ctx, signals);
    });
}

    fn build_top_menu(&mut self, egui_ctx: &Context) {
        egui::TopBottomPanel::top("top_panel").show(egui_ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                //ui.image(texture_id, size)
                    //ui.image(texture_id, size)
                    ui.heading(RichText::new( "LIVE 2.0").color(Color32::GREEN).strong());
                    ui.add_space(5.0);
                    ui.separator();
                    ui.add_space(5.0);
                    egui::menu::menu_button(ui, RichText::new("Simulation").strong(), |ui| {
                        if ui.button(RichText::new("New Simulation").color(Color32::from_gray(150))).clicked() {
                        }
                        if ui.button(RichText::new("Load Simulation").color(Color32::from_gray(150))).clicked() {
                        }
                        if ui.button(RichText::new("Save Simulation").color(Color32::from_gray(150))).clicked() {
                        }
                        if ui.button(RichText::new("Quit").color(Color32::RED).strong()).clicked() {
                            self.state.quit = true;
                        }
                    });
                    ui.add_space(10.0);
                    ui.separator();
                    ui.add_space(10.0);
                    egui::menu::menu_button(ui, RichText::new("Tools").strong(), |ui| {
                        if ui.button(RichText::new("Performance").strong().color(Color32::YELLOW)).clicked() {
                            self.state.performance = !self.state.performance;
                        }
                        if ui.button(RichText::new("Inspector").strong().color(Color32::YELLOW)).clicked() {
                            self.state.inspect = !self.state.inspect;
                        }
                        if ui.button(RichText::new("Mouse Input").strong().color(Color32::YELLOW)).clicked() {
                            self.state.mouse = !self.state.mouse;
                        }
                        if ui.button(RichText::new("Creator").strong().color(Color32::YELLOW)).clicked() {
                            self.state.create = !self.state.create;
                        }
                    });
                    ui.add_space(10.0);
                    ui.separator();
                });
            });
        }


    fn build_monit_window(&self, egui_ctx: &Context, fps: i32, delta: f32) {
        if self.state.performance {
            egui::Window::new("Monitor").default_pos((5.0, 100.0))
            .default_width(125.0)
            .show(egui_ctx, |ui| {
                ui.label(format!("DELTA: {}ms", (delta*1000.0).round()));
                ui.label(format!("FPS: {}", fps));
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

    fn build_mouse_window(&self, egui_ctx: &Context, mouse_state: &MouseState) {
        if self.state.mouse {
            egui::Window::new("Mouse").default_pos((5.0, 325.0))
            .default_width(125.0)
            .show(egui_ctx, |ui| {
                ui.label(format!("X: {} | Y: {}", mouse_state.pos.x, mouse_state.pos.y));
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

    fn build_create_window(&self, egui_ctx: &Context, signals: &mut Signals) {
        if self.state.create {
            egui::Window::new("Creator").default_pos((5.0, 450.0))
            .default_width(125.0)
            .show(egui_ctx, |ui| {
                ui.horizontal(|head| {
                    head.heading("Spawn new creature");
                });
                ui.horizontal(|mid| {
                    if mid.button(RichText::new("SPAWN").strong().color(Color32::LIGHT_GREEN)).clicked() {
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
        }
    }
}