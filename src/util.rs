#![allow(unused)]

use std::f32::consts::PI;

use macroquad::{prelude::*, color};
use crate::consts::*;


pub fn random_position(x_max: f32, y_max: f32) -> Vec2 {
    let x = rand::gen_range(0.0, x_max);
    let y = rand::gen_range(0.0, y_max);
    return  Vec2::new(x, y);
}

pub fn random_rotation() -> f32 {
    let rot = rand::gen_range(0.0, PI*2.0);
    return rot;
}

pub fn random_unit_vec2() -> Vec2 {
    let x = rand::gen_range(-1.0, 1.0);
    let y = rand::gen_range(-1.0, 1.0);
    return  Vec2::new(x, y).normalize_or_zero();    
}

pub fn random_color() -> color::Color {
    let colors = vec![RED, GREEN, BLUE, YELLOW, ORANGE, GRAY, SKYBLUE, LIME];
    let num = colors.len();
    let c = rand::gen_range(0, num);
    return  colors[c];
}

pub fn angle2vec2(angle: f32) -> Vec2 {
    let (x, y) = angle.sin_cos();
    let mut v = Vec2::new(x, y).normalize_or_zero();
    return v;
}

pub fn wrap_around(v: &Vec2) -> Vec2 {
    let tolerance = 5.0;
    let mut vr = Vec2::new(v.x, v.y);
    if vr.x > WORLD_W+tolerance {
        vr.x = 0.0-tolerance;
    } else if vr.x < 0.0-tolerance {
        vr.x = WORLD_W+tolerance;
    }
    if vr.y > WORLD_H+tolerance {
        vr.y = 0.0-tolerance;
    } else if vr.y < 0.0-tolerance {
        vr.y = WORLD_H+tolerance;
    }
    return vr;
}


//?         [[[SIGNALS]]]
pub struct Signals {
    pub spawn_molecule: bool,
    pub new_sim: bool,
    pub new_sim_name: String,
}

impl Signals {
    pub fn new() -> Self {
        Self {
            spawn_molecule: false,
            new_sim: false,
            new_sim_name: String::new(),
        }
    }
}