use egui_macroquad;
use egui;
use egui::{RichText, Color32};


pub struct UIState {
    pub performance: bool,
    pub quit: bool,
}

impl UIState {
    pub fn new() -> Self {
        Self {
            performance: false,
            quit: false,
        }
    }
}


pub fn ui_process(ui_state: &mut UIState, fps: i32, delta: f32, contacts_num: usize) {
    egui_macroquad::ui(|egui_ctx| {
        
        egui::TopBottomPanel::top("top_panel").show(egui_ctx, |ui| {
            egui::menu::bar(ui, |ui| {
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
                });
                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);
            });
        });
            
        if ui_state.performance {
            egui::Window::new("Monitor")
            .default_width(125.0)
            .show(egui_ctx, |ui| {
                ui.label(format!("DELTA: {}ms", (delta*1000.0).round()));
                ui.label(format!("FPS: {}", fps));
                ui.label(format!("CONTACTS: {}", contacts_num));
            });
        }
        
        if ui_state.quit {
            egui::Window::new("Quit")
            .default_width(125.0)
            .show(egui_ctx, |ui| {
                ui.horizontal(|head| {
                    head.heading("Are you sure?");
                    //head.separator();
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
            
    });
}

pub fn ui_draw() {
    egui_macroquad::draw();
}