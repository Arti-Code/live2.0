use parry2d::query::contact;
use parry2d::{math::*, query::Contact}; 
use parry2d::shape::*;
use glam::Vec2;
use nalgebra::*;

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

pub fn make_isometry(posx: f32, posy: f32, rotation: f32) -> nalgebra::Isometry2<f32> {
    let iso = Isometry2::new(Vector2::new(posx, posy), rotation);
    return iso;
}


pub fn contact_circles(pos1: Isometry2<f32>, rad1: f32, pos2: Isometry2<f32>, rad2: f32) -> f32 {
    let ball1 = Ball::new(rad1);
    let ball2 = Ball::new(rad2);
    let contact = contact(&pos1, &ball1, &pos2, &ball2, 0.0).unwrap();
    let c = match contact {
        Some(c) => c.dist,
        None => 111.111,
    };
    c
}