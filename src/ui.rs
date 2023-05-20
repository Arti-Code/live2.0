use std::path::Path;

use egui_macroquad;

use egui::{self, Context};
use egui::{RichText, Color32};
use egui_extras::image::RetainedImage;
use image::open;

use crate::agent::Agent;
use crate::consts::{SCREEN_WIDTH, SCREEN_HEIGHT};


pub struct UIState {
    pub performance: bool,
    pub inspect: bool,
    pub quit: bool,
}

impl UIState {
    pub fn new() -> Self {
        Self {
            performance: false,
            inspect: false,
            quit: false,
        }
    }
}

/* pub struct UILogo {
    texture: Option<egui::TextureHandle>,
}

impl UILogo {
    fn new(&mut self, ui: &mut egui::Ui) {
        let texture: &egui::TextureHandle = self.texture.get_or_insert_with(|| {
            // Load the texture only once.
            ui.ctx().load_texture(
                "my-image",
                egui::ColorImage::example(),
                Default::default()
            )
        });

        // Show the image:
        ui.image(texture, texture.size_vec2());
    }
} */

pub fn ui_process(ui_state: &mut UIState, fps: i32, delta: f32, time: f32, agent: Option<&Agent>) {
    egui_macroquad::ui(|egui_ctx| {
        build_top_menu(egui_ctx, ui_state);
        build_quit_window(egui_ctx, ui_state);
        build_monit_window(egui_ctx, ui_state, fps, delta, time);
        match agent {
            Some(agent) => {
                build_inspect_window(egui_ctx, ui_state, agent)
            },
            None => {}
        }
    });
}

fn build_top_menu(egui_ctx: &Context, ui_state: &mut UIState) {
        egui::TopBottomPanel::top("top_panel").show(egui_ctx, |ui| {
            egui::menu::bar(ui, |ui| {
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
                        ui_state.quit = true;
                    }
                });
                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);
                egui::menu::menu_button(ui, RichText::new("Tools").strong(), |ui| {
                    if ui.button(RichText::new("Performance Monitor").strong().color(Color32::YELLOW)).clicked() {
                        ui_state.performance = !ui_state.performance;
                    }
                    if ui.button(RichText::new("Inspector").strong().color(Color32::YELLOW)).clicked() {
                        ui_state.inspect = !ui_state.inspect;
                    }
                });
                ui.add_space(10.0);
                ui.separator();
            });
        });
}


fn build_monit_window(egui_ctx: &Context, ui_state: &mut UIState, fps: i32, delta: f32, time: f32) {
        if ui_state.performance {
            egui::Window::new("Monitor").default_pos((50.0, 50.0))
            .default_width(125.0)
            .show(egui_ctx, |ui| {
                ui.label(format!("DELTA: {}ms", (delta*1000.0).round()));
                ui.label(format!("FPS: {}", fps));
                ui.label(format!("TIMER: {}", time));
            });
        }    
}

fn build_inspect_window(egui_ctx: &Context, ui_state: &mut UIState, agent: &Agent) {
    if ui_state.inspect {
        let rot = agent.rot;
        let size = agent.size;
        egui::Window::new("Inspector").default_pos((200.0, 50.0))
        .default_width(125.0)
        .show(egui_ctx, |ui| {
            ui.label(format!("ENERGY: 100/100"));
            ui.label(format!("ROTATION: {}", ((rot*10.0).round())/10.0));
            ui.label(format!("SIZE: {}", size));
        });
    }    
}


fn build_quit_window(egui_ctx: &Context, ui_state: &mut UIState) {
        if ui_state.quit {
            egui::Window::new("Quit").default_pos((SCREEN_WIDTH/2.0-65.0, SCREEN_HEIGHT/4.0))
            .default_width(125.0)
            .show(egui_ctx, |ui| {
                ui.horizontal(|head| {
                    head.heading("Are you sure?");
                });
                ui.horizontal(|mid| {
                    mid.columns(2, |columns| {
                        if columns[0].button(RichText::new("No").color(Color32::WHITE)).clicked() {
                            ui_state.quit = false;
                        }
                        if columns[1].button(RichText::new("Yes").color(Color32::RED)).clicked() {
                            std::process::exit(0);
                        }
                    });
                });
            });
        }    
}

pub fn ui_draw() {
    egui_macroquad::draw();
}