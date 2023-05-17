#![allow(unused)]
use std::f32::consts::PI;

use macroquad::{prelude::*, color}; 
use parry2d::shape::*;
use crate::util::*;
use crate::consts::*;


#[derive(Clone, Copy)]
pub struct Particle {
    pub pos: Vec2,
    pub rot: f32,
    pub vel: f32,
    pub size: f32,
    pub color: color::Color,
    pub shape: Ball,
    pub collided: bool,
}

impl Particle {
    pub fn new() -> Self {
        let s = rand::gen_range(10, 20) as f32;
        Self {
            pos: random_position(SCREEN_WIDTH, SCREEN_HEIGHT),
            rot: random_rotation(),
            vel: rand::gen_range(0.0, 1.0)*AGENT_SPEED,
            size: s,
            color: random_color(),
            shape: Ball { radius: s },
            collided: false,
        }
    }
    pub fn draw(&self) {
        //let dir = angle2vec2(self.rot);
        let dir = Vec2::from_angle(self.rot);
        let x0 = self.pos.x;
        let y0 = self.pos.y;
        let x1 = x0 + dir.x * self.size*1.0;
        let y1 = y0 + dir.y * self.size*1.0;
        let x2 = x0 + dir.x * self.size*2.0;
        let y2 = y0 + dir.y * self.size*2.0;
        draw_circle_lines(x0, y0, self.size, 0.5, self.color);
        if self.collided {
            draw_circle_lines(x0, y0, self.size*2.0, 0.5, self.color);
        }
        draw_line(x1, y1, x2, y2, 0.5, self.color);
    }
    pub fn update(&mut self, dt: f32) {
        let dir = Vec2::from_angle(self.rot);
        self.pos += dir * self.vel * dt;
        self.pos = wrap_around(&self.pos);
    }
}