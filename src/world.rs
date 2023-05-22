use macroquad::prelude::*;
use crate::{
    prelude::*, 
    kinetic::CollisionsMap,
    agent::Agent,
};

pub struct World {
    pub aqents: Vec<Agent>,
    pub hit_map: CollisionsMap,
}

/* impl World {
    pub fn new() -> Self {
        Self {

        }
    }
} */