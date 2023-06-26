#![allow(unused)]
use std::collections::HashMap;
use std::collections::hash_map::Iter;
use std::collections::hash_map::IterMut;
use std::f32::consts::PI;

use macroquad::{prelude::*, color}; 
use parry2d::shape::*;
use ::rand::{Rng, thread_rng};
use rapier2d::prelude::RigidBodyHandle;
use crate::kinetic::{Detection, contact_circles};
use crate::util::*;
use crate::consts::*;
use crate::timer::*;
use crate::neuro::*;
use crate::world::*;

pub struct Molecule {
    pub pos: Vec2,
    pub rot: f32,
    pub vel: f32,
    pub ang_vel: f32,
    pub size: f32,
    pub color: color::Color,
    pub shape: Ball,
    pub physics_handle: Option<RigidBodyHandle>,
}

impl Molecule {
    pub fn new() -> Self {
        let s = rand::gen_range(MOLECULE_SIZE_MIN, MOLECULE_SIZE_MAX) as f32;
        let motor = thread_rng().gen_bool(1.0);
        Self {
            pos: random_position(WORLD_W, WORLD_H),
            //pos: Vec2::new(0.0, 0.0),
            rot: random_rotation(),
            vel: rand::gen_range(0.0, 1.0)*MOLECULE_SPEED,
            ang_vel: 0.0,
            size: s,
            color: random_color(),
            shape: Ball { radius: s },
            physics_handle: None,
        }
    }
    pub fn draw(&self) {
        let x0 = self.pos.x;
        let y0 = self.pos.y;
        draw_circle(x0, y0, self.size, self.color);
    }

    pub fn update2(&mut self, physics: &mut World) {
        match self.physics_handle {
            Some(handle) => {
                let physics_data = physics.get_physics_data(handle);
                self.pos = physics_data.position;
                self.rot = physics_data.rotation;
            },
            None => {},
        }

    }
}



pub struct MoleculesBox {
    pub molecules: HashMap<u64, Molecule>
}

impl MoleculesBox {
    pub fn new() -> Self {
        Self {
            molecules: HashMap::new(),
        }
    }

    pub fn add_many_molecules(&mut self, molecules_num: usize, physics_world: &mut World) {
        for _ in 0..molecules_num {
            let molecule = Molecule::new();
            _ = self.add_molecule(molecule, physics_world);
        }
    }

    pub fn add_molecule(&mut self, mut molecule: Molecule, physics_world: &mut World) -> u64 {
        let key: u64 = thread_rng().gen::<u64>();
        let handle = physics_world.add_circle_body(&molecule.pos, molecule.size);
        molecule.physics_handle = Some(handle);
        self.molecules.insert(key, molecule);
        return key;
    }

    pub fn get(&self, id: u64) -> Option<&Molecule> {
        return self.molecules.get(&id);
    }

    pub fn remove(&mut self, id: u64) {
        self.molecules.remove(&id);
    }

    pub fn get_iter(&self) -> Iter<u64, Molecule> {
        return self.molecules.iter();
    }
    
    pub fn get_iter_mut(&mut self) -> IterMut<u64, Molecule> {
        return self.molecules.iter_mut();
    }

    pub fn count(&self) -> usize {
        return self.molecules.len();
    }
}
