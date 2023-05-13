/* #![allow(unused)]
use std::f32::consts::PI;

use macroquad::{prelude::*, color}; 
use parry2d::math::Point;
use parry2d::na::Point2;
use parry2d::shape::*;
use crate::util::*;
use crate::consts::*;


#[derive(Clone, Copy)]
pub struct Rock {
    pub pos: Vec2,
    pub sides: usize,
    pub size: f32,
    pub deviation: f32,
    pub color: color::Color,
    pub shape: ConvexPolygon,
}

impl Agent {
    pub fn new() -> Self {
        let s = rand::gen_range(8, 24) as f32;
        Self {
            pos: random_position(SCREEN_WIDTH, SCREEN_HEIGHT),
            sides: rand::gen_range(5, 10),
            size: s,
            color: GRAY,
            shape: ConvexPolygon{
                points: vec![, Point2::new(1.0, 1.0), Point2::new(1.0, 1.0), Point2::new(1.0, 1.0)],
                normals: 
            }
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
        draw_circle(x0, y0, (self.size/2.0)*pulse.abs(), self.color);
        draw_line(x1, y1, x2, y2, 3.0, self.color);
    }
    pub fn update(&mut self, dt: f32) {
        self.pulse = (self.pulse + dt*0.25)%1.0;
        self.rot += rand::gen_range(-1.0, 1.0)*AGENT_ROTATION*2.0*PI*dt;
        self.rot = self.rot%(2.0*PI);
        let dir = angle2vec2(self.rot);
        self.pos += dir * self.vel * dt;
        self.pos = wrap_around(&self.pos);
    }
} */