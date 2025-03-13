use crate::ui::ui::SELECTION_OFFSET;

use raylib::{core::collision::check_collision_circles, math::Vector2};
use std::hash::Hash;

use super::ui::PLUG_RADIUS;

#[derive(Copy, Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Vec2u {
    pub x: u32,
    pub y: u32,
}

impl Vec2u {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x: x.to_bits(),
            y: y.to_bits(),
        }
    }
}

impl Into<Vector2> for Vec2u {
    fn into(self) -> Vector2 {
        Vector2::new(f32::from_bits(self.x), f32::from_bits(self.y))
    }
}

impl From<Vector2> for Vec2u {
    fn from(v: Vector2) -> Vec2u {
        Vec2u::new(v.x, v.y)
    }
}

impl std::ops::Add for Vec2u {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: (f32::from_bits(self.x) + f32::from_bits(other.x)).to_bits(),
            y: (f32::from_bits(self.y) + f32::from_bits(other.y)).to_bits(),
        }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Circle {
    pub pos: Vector2,
    pub rad: f32,
}

impl Circle {
    pub fn new(pos: Vector2, rad: f32) -> Self {
        Self { pos, rad }
    }

    pub fn check_circles_collision(&self, x: &Circle) -> bool {
        check_collision_circles(self.pos, self.rad, x.pos, x.rad)
    }

    pub fn check_mouse_collision(&self, x: Vector2) -> bool {
        check_collision_circles(self.pos, self.rad, x, SELECTION_OFFSET)
    }

    pub fn check_plug_collision(&self, x: Vector2) -> bool {
        check_collision_circles(self.pos, self.rad, x, PLUG_RADIUS)
    }
}
