/* use egui::epaint::ahash::HashMap;
use glam::Vec2;
use std::collections::hash_map::{Iter, IterMut};
use rand::{Rng, thread_rng};

pub enum ElementType {
    Agent,
    Obstacle,
    Action,
    Dynamic,
}

struct Rock;

pub trait Element {
    fn draw(&self);
    fn update(&mut self, dt: f32);
    fn update_collision(&mut self, normal: &Vec2, penetration: f32, dt: f32);
}

pub trait ElementBox<T> {
    fn add_element(&mut self, element: T) -> u64;
    fn add_many_elements(&mut self, number: usize);
    fn get(&self, id: u64) -> Option<&T>;
    fn remove(&mut self, id: u64);
    fn get_iter(&self) -> Iter<u64, T>;
    fn get_iter_mut(&mut self) -> IterMut<u64, T>;
    fn count(&self) -> usize;
}


struct RockBox<Rock> {
    collection: HashMap<u64, Rock>,
}


impl ElementBox<Rock> for RockBox<Rock> {
    fn add_element(&mut self, element: Rock) -> u64 {
        let k = thread_rng().gen::<u64>();
        self.collection.insert(k, element);
        return k;
    }
} */