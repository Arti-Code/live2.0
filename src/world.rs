use macroquad::prelude::*;
use crate::{
    prelude::*, 
    kinetic::*,
    agent::Agent,
};

pub struct World {
    pub size: Vec2,
    pub agents: Vec<Agent>,
    pub hit_map: CollisionsMap,
}

impl World {

    pub fn new(world_size: Vec2, agents_num: usize) -> Self {
        let mut agents: Vec<Agent> = vec![];
        for _ in 0..agents_num {
            let agent = Agent::new();
            agents.push(agent);
        }
        Self {
            size: world_size,
            agents: agents,
            hit_map: CollisionsMap::new(),
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.hit_map = self.map_collisions();
        for agent in self.agents.iter_mut() {
            let uid = agent.unique;
            agent.update(dt);
            match self.hit_map.get_collision(uid) {
                Some(hit) => {
                    agent.update_collision(&hit.normal, hit.overlap, dt);
                },
                None => {
                }
            }
        }
    }

    pub fn draw(&mut self, dt: f32) {
        for agent in self.agents.iter() {
            agent.draw();
        }
    }

    fn map_collisions(&self) -> CollisionsMap {
        let mut hits: CollisionsMap = CollisionsMap::new();
        for a1 in self.agents.iter() {
            for a2 in self.agents.iter() {
                if a1.unique != a2.unique {
                    let contact = contact_circles(a1.pos, a1.rot, a1.size, a2.pos,a2.rot, a2.size);
                    match contact {
                        Some(contact) => {
                            if contact.dist <= 0.0 {
                                let p = Vec2::new(contact.point1.x, contact.point1.y);
                                let norm = contact.normal1.data.0[0];
                                let n = Vec2::new(norm[0], norm[1]);
                                let penetration = contact.dist;
                                let hit: Hit=Hit{ normal: n, overlap: contact.dist };
                                hits.add_collision(a1.unique, hit);
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