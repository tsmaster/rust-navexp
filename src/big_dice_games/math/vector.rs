// big_dice_games/math/vector.rs

use std::ops::Add;
use std::ops::Sub;

pub trait Vector {
    fn mag(&self) -> f32;

    fn scale(&self, factor: f32) -> Self;
}

#[derive(Copy, Clone, Debug)]
pub struct Vec2f
{
    pub x: f32,
    pub y: f32,
}

impl Vec2f {
    pub fn new(x: f32, y: f32) -> Vec2f {
	Vec2f {
	    x: x,
	    y: y
	}
    }
}

impl Add for Vec2f {
    type Output = Self;

    fn add(self, other: Self) -> Self {
	Self {
	    x: self.x + other.x,
	    y: self.y + other.y
	}
    }
}

impl Sub for Vec2f {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
	Self {
	    x: self.x - other.x,
	    y: self.y - other.y
	}
    }
}

impl Vector for Vec2f {
    fn mag(&self) -> f32 {
	(self.x * self.x + self.y * self.y).sqrt()
    }

    fn scale(&self, factor: f32) -> Self {
	Self {
	    x: self.x * factor,
	    y: self.y * factor
	}
    }
}

    

#[derive(Copy, Clone)]
pub struct Vec3f
{
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
