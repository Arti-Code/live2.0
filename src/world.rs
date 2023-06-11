use std::collections::{HashMap, hash_map::{Iter, IterMut}};
use ::rand::{Rng, thread_rng};
use macroquad::prelude::*;
use crate::{
    prelude::*, 
    kinetic::*,
    agent::{Agent, AgentsBox},
};
use crate::object::ObjectType;

pub struct World {
    pub size: Vec2,
    pub agents: AgentsBox,
    pub hit_map: CollisionsMap,

}

impl World {

    pub fn new(world_size: Vec2, agents_num: usize) -> Self {
        let mut agents = AgentsBox::new();
        for _ in 0..agents_num {
            let agent = Agent::new();
            agents.add_agent(agent);
        }
        Self {
            size: world_size,
            agents: agents,
            hit_map: CollisionsMap::new(),
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.hit_map = self.map_collisions();
        for (unique, agent) in self.agents.get_iter_mut() {
            agent.update(dt);
            match self.hit_map.get_collision(*unique) {
                Some(hit) => {
                    agent.update_collision(&hit.normal, hit.overlap, dt);
                },
                None => {
                }
            }
        }
    }

    pub fn draw(&mut self, dt: f32) {
        for (id, agent) in self.agents.get_iter() {
            agent.draw(false);
        }
    }

    fn map_collisions(&self) -> CollisionsMap {
        let mut hits: CollisionsMap = CollisionsMap::new();
        for (id1, a1) in self.agents.get_iter() {
            for (id2, a2) in self.agents.get_iter() {
                let idx1 = *id1; let idx2 = *id2;
                if idx1 != idx2 {
                    let contact = contact_circles(a1.pos, a1.rot, a1.size, a2.pos,a2.rot, a2.size);
                    match contact {
                        Some(contact) => {
                            if contact.dist <= 0.0 {
                                let p = Vec2::new(contact.point1.x, contact.point1.y);
                                let norm = contact.normal1.data.0[0];
                                let n = Vec2::new(norm[0], norm[1]);
                                let penetration = contact.dist;
                                let hit: Hit=Hit{ normal: n, overlap: contact.dist, target_type: ObjectType::Agent };
                                hits.add_collision(idx1, hit);
                            }
                        },
                        None => {}
                    }
                }
            }
        }
        return hits;
    }

}