#![allow(unused)]
use std::collections::HashMap;
use std::collections::hash_map::Iter;
use std::collections::hash_map::IterMut;
use std::f32::consts::PI;

use macroquad::{prelude::*, color}; 
//use parry2d::shape::*;
use ::rand::{Rng, thread_rng};
use rapier2d::prelude::RigidBodyHandle;
use rapier2d::geometry::*;
use crate::kinetic::{Detection, contact_circles};
use crate::util::*;
use crate::consts::*;
use crate::timer::*;
use crate::neuro::*;
use crate::world::*;

pub struct Agent {
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
    motor: bool,
    motor_phase: f32,
    motor_side: bool,
    analize_timer: Timer,
    analizer: DummyNetwork,
    pub alife: bool,
    enemy: Detection,
    pub physics_handle: Option<RigidBodyHandle>,
}

impl Agent {
    pub fn new() -> Self {
        let s = rand::gen_range(AGENT_SIZE_MIN, AGENT_SIZE_MAX) as f32;
        let motor = thread_rng().gen_bool(1.0);
        Self {
            pos: random_position(WORLD_W, WORLD_H),
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
            motor: motor,
            motor_phase: thread_rng().gen_range(0.0..1.0),
            motor_side: true,
            analize_timer: Timer::new(0.3, true, true, true),
            analizer: DummyNetwork::new(2),
            alife: true,
            enemy: Detection::new_empty(),
            physics_handle: None,
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
        if self.motor {
            let tail = Vec2::from_angle(self.rot+(self.motor_phase*0.2));
            let x3 = x0 - tail.x * self.size*1.6;
            let y3 = y0 - tail.y * self.size*1.6;
            draw_circle_lines(x3, y3, self.size/2.0, 2.0, self.color);
        }
        let pulse = (self.pulse * 2.0) - 1.0;
        if field_of_view && !self.enemy.is_empty() {
            let x0 = self.pos.x; let y0 = self.pos.y;
            let x1 = self.enemy.pos.x; let y1 = self.enemy.pos.y;
            draw_line(x0, y0, x1, y1, 0.5, RED);
        }
        draw_circle_lines(x0, y0, self.size, 2.0, self.color);
        draw_circle(x0, y0, (self.size/2.0)*pulse.abs(), self.color);
        draw_line(x1, y1, x2, y2, 1.0, self.color);
        if field_of_view {
            draw_circle_lines(x0, y0, self.vision_range, 0.75, GRAY);
        }
    }

    pub fn update2(&mut self, physics: &mut World) {
        match self.physics_handle {
            Some(handle) => {
                let physics_data = physics.get_physics_data(handle);
                self.pos = physics_data.position;
                self.rot = physics_data.rotation;
                match physics.rigid_bodies.get_mut(handle) {
                    Some(body) => {
                        let dir = Vec2::from_angle(self.rot);
                        let v = dir*self.vel;
                        body.set_linvel([v.x, v.y].into(), true);
                        body.set_angvel(self.ang_vel, true);
                    },
                    None => {},
                }
            },
            None => {},
        }

    }

    pub fn update(&mut self, dt: f32) -> bool{
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
        //self.rot += self.ang_vel * dt;
        //self.rot = self.rot % (2.0*PI);
        self.pulse = (self.pulse + dt*0.25)%1.0;
        if self.motor {
            if self.motor_side {
                self.motor_phase = self.motor_phase + dt*5.0;
                if self.motor_phase >= 1.0 {
                    self.motor_side = false;
                }
            } else {
                self.motor_phase = self.motor_phase - dt*5.0;
                if self.motor_phase <= -1.0 {
                    self.motor_side = true;
                }
            }
        }
        //let dir = Vec2::from_angle(self.rot);
        //self.pos += dir * self.vel * dt;
        //self.pos = wrap_around(&self.pos);
        if self.eng > 0.0 {
            self.eng -= self.size * 1.0 * dt;
        } else {
            self.eng = 0.0;
            self.alife = false;
        }
        return self.alife;
    }

    pub fn update_collision(&mut self, collision_normal: &Vec2, penetration: f32, dt: f32) {
        self.pos -= *collision_normal * penetration.abs() * self.vel * dt * 0.3;
    }

    pub fn reset_detections(&mut self) {
        self.enemy = Detection::new_empty();
    }

    pub fn update_detection(&mut self, target: &Detection) {
        self.enemy.add_closer(target.distance, target.angle, target.pos.clone());
    }
    
    pub fn add_energy(&mut self, e: f32) {
        self.eng += e;
        if self.eng > self.max_eng {
            self.eng = self.max_eng;
        }
    }
}



pub struct AgentsBox {
    pub agents: HashMap<u64, Agent>
}

impl AgentsBox {
    pub fn new() -> Self {
        Self {
            agents: HashMap::new(),
        }
    }

    pub fn add_many_agents(&mut self, agents_num: usize, physics_world: &mut World) {
        for _ in 0..agents_num {
            let agent = Agent::new();
            _ = self.add_agent(agent, physics_world);
        }
    }

    pub fn add_agent(&mut self, mut agent: Agent, physics_world: &mut World) -> u64 {
        let key: u64 = thread_rng().gen::<u64>();
        let handle = physics_world.add_circle_body(&agent.pos, agent.size);
        agent.physics_handle = Some(handle);
        self.agents.insert(key, agent);
        return key;
    }

    pub fn get(&self, id: u64) -> Option<&Agent> {
        return self.agents.get(&id);
    }

    pub fn remove(&mut self, id: u64) {
        self.agents.remove(&id);
    }

    pub fn get_iter(&self) -> Iter<u64, Agent> {
        return self.agents.iter();
    }
    
    pub fn get_iter_mut(&mut self) -> IterMut<u64, Agent> {
        return self.agents.iter_mut();
    }

    pub fn count(&self) -> usize {
        return self.agents.len();
    }
}
