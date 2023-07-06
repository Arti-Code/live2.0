#![allow(unused)]
use std::collections::hash_map::{Iter, IterMut};
use std::collections::HashMap;
use std::f32::consts::PI;

use macroquad::{color, prelude::*};
use nalgebra::{vector, OPoint};
use crate::consts::*;
use crate::kinetic::make_isometry;
use crate::kinetic::{contact_circles, Detection};
use crate::util::*;
use crate::world::*;
use ::rand::{thread_rng, Rng};
use rapier2d::geometry::*;
use rapier2d::prelude::RigidBodyHandle;

pub trait DynamicElement {
    fn create() -> Self;
    fn draw(&self);
    fn update(&mut self, dt: f32, physics: &World);
}


pub struct Asteroid {
    pub key: u64,
    pub pos: Vec2,
    pub rot: f32,
    pub vel: f32,
    pub ang_vel: f32,
    pub size: f32,
    pub color: color::Color,
    pub shape: ConvexPolygon,
    pub physics_handle: Option<RigidBodyHandle>,
}


impl DynamicElement for Asteroid {
    fn create() -> Self {
        let size = rand::gen_range(ASTER_SIZE_MIN, ASTER_SIZE_MAX);
        let n = size / 6;
        let (points, opoints) = map_polygon(n as usize, size as f32, 0.1);
        Self {
            key: thread_rng().gen::<u64>(),
            pos: random_position(WORLD_W, WORLD_H),
            rot: random_rotation(),
            vel: rand::gen_range(0.0, 1.0) * ASTER_SPEED,
            ang_vel: rand::gen_range(-1.0, 1.0),
            size: (size as f32),
            color: random_color(),
            shape: ConvexPolygon::from_convex_polyline(opoints).unwrap(),
            physics_handle: None,
        }
    }
    fn draw(&self) {

    }
    fn update(&mut self, dt: f32, physics: &World) {

    }
}