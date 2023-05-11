#![allow(unused)]
use std::f32::consts::PI;

use macroquad::{prelude::*, color}; 
use crate::util::*;
use crate::consts::*;


#[derive(Clone, Copy)]
pub struct Agent {
    pub pos: Vec2,
    pub rot: f32,
    pub vel: f32,
    pub size: f32,
    pub color: color::Color,
    pub pulse: f32,
}

impl Agent {
    pub fn new() -> Self {
        Self {
            pos: random_position(SCREEN_WIDTH, SCREEN_HEIGHT),
            rot: random_rotation(),
            vel: rand::gen_range(0.0, 1.0)*AGENT_SPEED,
            size: rand::gen_range(4, 14) as f32,
            color: random_color(),
            pulse: rand::gen_range(0.0, 1.0)
        }
    }
    pub fn draw(&self) {
        let dir = angle2vec2(self.rot);
        let x0 = self.pos.x;
        let y0 = self.pos.y;
        let x1 = x0 + dir.x * self.size*1.0;
        let y1 = y0 + dir.y * self.size*1.0;
        let x2 = x0 + dir.x * self.size*2.0;
        let y2 = y0 + dir.y * self.size*2.0;
        let pulse = (self.pulse * 2.0) - 1.0;
        draw_circle_lines(x0, y0, self.size, 2.0, self.color);
        draw_circle(x0, y0, (self.size/3.0)*pulse.abs(), self.color);
        draw_line(x1, y1, x2, y2, 3.0, self.color);
    }
    pub fn update(&mut self, dt: f32) {
        self.pulse = (self.pulse + dt)%1.0;
        self.rot += rand::gen_range(-1.0, 1.0)*2.0*PI*dt;
        self.rot = self.rot%(2.0*PI);
        let dir = angle2vec2(self.rot);
        self.pos += dir * self.vel * dt;
        self.pos = wrap_around(&self.pos);
    }
}