#![allow(unused)]

use std::collections::HashMap;
use std::collections::hash_map::Iter;
use std::collections::hash_map::IterMut;
use std::f32::consts::PI;

use macroquad::{prelude::*, color}; 
use parry2d::shape::*;
use ::rand::{Rng, thread_rng};
use crate::kinetic::{Detection, contact_circles};
use crate::util::*;
use crate::consts::*;
use crate::timer::*;

pub struct Source {
    pub pos: Vec2,
    pub rot: f32,
    pub size: f32,
    pub max_eng: f32,
    pub eng: f32,
    pub color: color::Color,
    pub shape: Ball,
    pub alife: bool,
}

impl Source {
    pub fn new() -> Self {
        let s = rand::gen_range(2, 5) as f32;
        Self {
            pos: random_position(WORLD_W, WORLD_H),
            rot: random_rotation(),
            size: s,
            max_eng: s.powi(2)*10.0,
            eng: s.powi(2)*10.0,
            color: random_color(),
            shape: Ball { radius: s },
            alife: true,
        }
    }
    pub fn draw(&self, field_of_view: bool) {
        let dir = Vec2::from_angle(self.rot);
        let x0 = self.pos.x;
        let y0 = self.pos.y;
        let x1 = x0 + dir.x * self.size*1.0;
        let y1 = y0 + dir.y * self.size*1.0;
        let x2 = x0 + dir.x * self.size*2.0;
        let y2 = y0 + dir.y * self.size*2.0;
        draw_circle_lines(x0, y0, self.size, 2.0, self.color);
        draw_circle(x0, y0, (self.size/2.0)*pulse.abs(), self.color);
        draw_line(x1, y1, x2, y2, 1.0, self.color);
    }
    pub fn update(&mut self, dt: f32){
        let dir = Vec2::from_angle(self.rot);
        self.pos = wrap_around(&self.pos);
        if self.eng <= 0.0 {
            self.eng = 0.0;
            self.alife = false;
        }
    }

    pub fn update_collision(&mut self, collision_normal: &Vec2, penetration: f32, dt: f32) {
        self.pos -= *collision_normal * penetration.abs() * dt * 0.3;
    }
}



pub struct SourcesBox {
    pub sources: HashMap<u32, Source>
}

impl SourcesBox {
    pub fn new() -> Self {
        Self {
            sources: HashMap::new(),
        }
    }

    pub fn add_many(&mut self, source_num: usize) {
        for _ in 0..source_num {
            let source = Source::new();
            _ = self.add_source(source);
        }
    }

    pub fn add_source(&mut self, source: Source) -> u32 {
        let key: u32 = thread_rng().gen::<u32>();
        self.sources.insert(key, Source);
        return key;
    }

    pub fn get(&self, id: u32) -> Option<&Source> {
        return self.sources.get(&id);
    }

    pub fn remove(&mut self, id: u32) {
        self.sources.remove(&id);
    }

    pub fn get_iter(&self) -> Iter<u32, Source> {
        return self.sources.iter();
    }
    
    pub fn get_iter_mut(&mut self) -> IterMut<u32, Source> {
        return self.sources.iter_mut();
    }

    pub fn count(&self) -> usize {
        return self.sources.len();
    }
}
