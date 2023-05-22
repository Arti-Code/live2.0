use std::collections::HashMap;

use parry2d::query::contact;
use parry2d::{math::*, query::Contact}; 
use parry2d::shape::*;
use glam;
use nalgebra::*;
use macroquad::math::Vec2;



#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rot {
    cos: f32,
    sin: f32,
}

impl Default for Rot {
    fn default() -> Self {
        Self::ZERO
    }
}

impl Rot {
    pub const ZERO: Self = Self { cos: 1., sin: 0. };

    pub fn from_radians(radians: f32) -> Self {
        Self {
            cos: radians.cos(),
            sin: radians.sin(),
        }
    }

    pub fn from_degrees(degrees: f32) -> Self {
        let radians = degrees.to_radians();
        Self::from_radians(radians)
    }

    pub fn as_radians(&self) -> f32 {
        f32::atan2(self.sin, self.cos)
    }

    pub fn rotate(&self, vec: Vec2) -> Vec2 {
        Vec2::new(
            vec.x * self.cos - vec.y * self.sin,
            vec.x * self.sin + vec.y * self.cos,
        )
    }

    pub fn inv(self) -> Self {
        Self {
            cos: self.cos,
            sin: -self.sin,
        }
    }

    pub fn mul(self, rhs: Rot) -> Self {
        Self {
            cos: self.cos * rhs.cos - self.sin * rhs.sin,
            sin: self.sin * rhs.cos + self.cos * rhs.sin,
        }
    }
}

fn make_isometry(posx: f32, posy: f32, rotation: f32) -> nalgebra::Isometry2<f32> {
    let iso = Isometry2::new(Vector2::new(posx, posy), rotation);
    return iso;
}

pub fn contact_circles(pos1: Vec2, rot1: f32, rad1: f32, pos2: Vec2, rot2: f32, rad2: f32) -> Option<Contact> {
    let v1 = glam::Vec2::new(pos1.x, pos1.y);
    let v2 = glam::Vec2::new(pos2.x, pos2.y);
    let pos1 = make_isometry(v1.x, v1.y, rot1);
    let pos2 = make_isometry(v2.x, v2.y, rot2);
    let ball1 = Ball::new(rad1);
    let ball2 = Ball::new(rad2);
    let contact = contact(&pos1, &ball1, &pos2, &ball2, 0.0).unwrap();
    return contact;
}

pub fn contact_mouse(mouse_pos: Vec2, target_pos: Vec2, target_rad: f32) -> bool {
    let v1 = glam::Vec2::new(mouse_pos.x, mouse_pos.y);
    let v2 = glam::Vec2::new(target_pos.x, target_pos.y);
    let pos1 = make_isometry(v1.x, v1.y, 0.0 );
    let pos2 = make_isometry(v2.x, v2.y, 0.0);
    let ball1 = Ball::new(2.0);
    let ball2 = Ball::new(target_rad);
    match contact(&pos1, &ball1, &pos2, &ball2, 0.0).unwrap() {
        Some(_) => true,
        None => false,
    }
}


pub struct Hit {
    pub normal: macroquad::math::Vec2,
    pub overlap: f32,
}

pub struct CollisionsMap {
    contacts: HashMap<u32, Hit>,
}

impl CollisionsMap {
    pub fn new() -> Self {
        Self { contacts: HashMap::new() }
    }
    pub fn add_collision(&mut self, unique: u32, hit: Hit) {
        self.contacts.insert(unique, hit);
    }
    pub fn clear(&mut self) {
        self.contacts.clear();
    }
    pub fn remove_collision(&mut self, unique: u32) {
        _ = self.contacts.remove(&unique);
    }
    pub fn get_collision(&mut self, unique: u32) -> Option<&Hit> {
        return self.contacts.get(&unique);
    } 
}