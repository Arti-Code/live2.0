#![allow(unused)]
use std::f32::consts::PI;

use macroquad::{prelude::*, color}; 
use parry2d::shape::*;
use crate::kinetic::contact_circles;
use crate::util::*;
use crate::consts::*;
use crate::timer::*;
use crate::neuro::*;

pub struct Agent {
    pub unique: u32,
    pub pos: Vec2,
    pub rot: f32,
    pub vel: f32,
    pub ang_vel: f32,
    pub size: f32,
    pub vision_range: f32,
    pub max_eng: f32,
    pub eng: f32,
    pub color: color::Color,
    pub pulse: f32,
    pub shape: Ball,
    analize_timer: Timer,
    analizer: DummyNetwork,
    pub alife: bool,
    enemy: (f32, f32),
}

impl Agent {
    pub fn new() -> Self {
        let s = rand::gen_range(4, 10) as f32;
        Self {
            unique: rand::rand(),
            pos: random_position(SCREEN_WIDTH, SCREEN_HEIGHT),
            rot: random_rotation(),
            vel: rand::gen_range(0.0, 1.0)*AGENT_SPEED,
            ang_vel: 0.0,
            size: s,
            vision_range: (rand::gen_range(0.5, 1.5)*AGENT_VISION_RANGE).round(),
            max_eng: s.powi(2)*10.0,
            eng: s.powi(2)*10.0,
            color: random_color(),
            pulse: rand::gen_range(0.0, 1.0),
            shape: Ball { radius: s },
            analize_timer: Timer::new(0.3, true, true, true),
            analizer: DummyNetwork::new(2),
            alife: true,
            enemy: (f32::NAN, f32::NAN),
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
        let pulse = (self.pulse * 2.0) - 1.0;
        if !self.enemy.0.is_nan() {
            let dist = self.enemy.0;
            let ang = self.enemy.1;
            let tg_vec = Vec2::from_angle(ang);
            draw_line(x0, y0, x0 + tg_vec.x*dist, y0 + tg_vec.y*dist, 0.5, RED);
        }
        draw_circle_lines(x0, y0, self.size, 0.75, self.color);
        draw_circle_lines(x0, y0, (self.size/2.0)*pulse.abs(), 0.5, self.color);
        draw_line(x1, y1, x2, y2, 0.75, self.color);
        if field_of_view {
            draw_circle_lines(x0, y0, self.vision_range, 0.5, GRAY);
        }
    }
    pub fn update(&mut self, dt: f32){
        if self.analize_timer.update(dt) {
            let outputs = self.analizer.analize();
            if outputs[0] >= 0.0 {
                self.vel = outputs[0] * AGENT_SPEED;
            }
            else {
                self.vel = 0.0;
            }
            self.ang_vel = outputs[1] * AGENT_ROTATION;
        }
        self.rot += self.ang_vel * dt;
        self.rot = self.rot % (2.0*PI);
        self.pulse = (self.pulse + dt*0.25)%1.0;
        let dir = Vec2::from_angle(self.rot);
        self.pos += dir * self.vel * dt;
        self.pos = wrap_around(&self.pos);
        if self.eng > 0.0 {
            self.eng -= self.size * 1.0 * dt;
        } else {
            self.eng = 0.0;
            self.alife = false;
        }
    }

    pub fn update_collision(&mut self, collision_normal: &Vec2, penetration: f32, dt: f32) {
        self.pos -= *collision_normal * penetration.abs() * self.vel * dt * 0.3;
    }

    pub fn detect(&mut self, targets: &Vec<Agent>) {
        self.enemy = (f32::NAN, f32::NAN);
        let r = self.vision_range;
        let pos = self.pos;
        for target in targets.iter() {
            if self.unique == target.unique {
                continue;
            }
            let detection = contact_circles(pos, self.rot, r, target.pos, target.rot, target.size);
            match detection {
                Some(detected) => {
                    let dist = pos.distance(target.pos);
                    if self.enemy.0.is_nan() || self.enemy.0 > dist {
                        let dir = Vec2::from_angle(self.rot);
                        let ang = dir.angle_between(target.pos-pos);
                        self.enemy = (dist, ang);
                    }
                },
                None => {},
            }
        }
    }
}