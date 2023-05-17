#![allow(unused)]
use std::f32::consts::PI;

use macroquad::{prelude::*, color}; 
use parry2d::shape::*;
use crate::util::*;
use crate::consts::*;


#[derive(Clone, Copy)]
pub struct Contact {
    pub normal: Vec2,
    pub penetration: f32,
}

impl Contact {
    pub fn new(normal: Vec2, penetration: f32) -> Self {
        Self { 
            normal: normal, 
            penetration: penetration.abs(),
        }
    }
}

//#[derive(Clone, Copy)]
pub struct Contacts {
    pub contacts_list: Vec<Contact>,
}

impl Contacts {
    pub fn new() -> Self {
        Self { contacts_list: vec![] }
    }
    pub fn clean(&mut self) {
        self.contacts_list.clear();
    }
    pub fn add_contact(&mut self, new_contact: Contact) {
        self.contacts_list.push(new_contact);
    }
    pub fn add_contact2(&mut self, normal: Vec2, penetration: f32) {
        let contact = Contact::new(normal, penetration);
        self.add_contact(contact);
    }
    pub fn get_summarized_contact(&self) -> Contact {
        let mut normal = Vec2::ZERO;
        let mut penetration = 0.0;
        for contact in self.contacts_list.iter() {
            normal += contact.normal * contact.penetration 
        }
        penetration = normal.length();
        normal.normalize_or_zero();
        return Contact::new(normal, penetration);
    }
}

//#[derive(Clone)]
pub struct Agent {
    pub unique: u32,
    pub pos: Vec2,
    pub rot: f32,
    pub vel: f32,
    pub size: f32,
    pub color: color::Color,
    pub pulse: f32,
    pub shape: Ball,
    //pub contacts: Contacts,
}

impl Agent {
    pub fn new() -> Self {
        let s = rand::gen_range(4, 10) as f32;
        Self {
            unique: rand::rand(),
            pos: random_position(SCREEN_WIDTH, SCREEN_HEIGHT),
            rot: random_rotation(),
            vel: rand::gen_range(0.0, 1.0)*AGENT_SPEED,
            size: s,
            color: random_color(),
            pulse: rand::gen_range(0.0, 1.0),
            shape: Ball { radius: s },
            //contacts: Contacts::new(),
        }
    }
    pub fn draw(&self) {
        let dir = Vec2::from_angle(self.rot);
        let x0 = self.pos.x;
        let y0 = self.pos.y;
        let x1 = x0 + dir.x * self.size*1.0;
        let y1 = y0 + dir.y * self.size*1.0;
        let x2 = x0 + dir.x * self.size*2.0;
        let y2 = y0 + dir.y * self.size*2.0;
        let pulse = (self.pulse * 2.0) - 1.0;
        draw_circle_lines(x0, y0, self.size, 0.75, self.color);
        draw_circle_lines(x0, y0, (self.size/2.0)*pulse.abs(), 0.5, self.color);
        draw_line(x1, y1, x2, y2, 0.75, self.color);
    }
    pub fn update(&mut self, dt: f32) {
        self.pulse = (self.pulse + dt*0.25)%1.0;
        self.rot += rand::gen_range(-1.0, 1.0)*AGENT_ROTATION*2.0*PI*dt;
        self.rot = self.rot%(2.0*PI);
        let dir = Vec2::from_angle(self.rot);
        self.pos += dir * self.vel * dt;
        //let reflection = self.contacts.get_summarized_contact();
        //self.pos += reflection.normal * reflection.penetration;
        self.pos = wrap_around(&self.pos);
        //self.contacts.clean();
    }
    pub fn update_collision(&mut self, collision_normal: &Vec2, penetration: f32) {
        self.pos += *collision_normal * penetration.abs();
    }

/*     pub fn add_contact(&mut self, contact: Contact) {
        self.contacts.add_contact(contact);
    } */
}