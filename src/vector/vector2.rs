use std::f32;
use num::Float;
use std::ops::{Add,Sub,Mul,Div,Neg,Index,IndexMut};

/// a vector in R² space with 32bit float number
#[derive(Debug,Copy,Clone,PartialEq,PartialOrd)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    /// get a new vector2 from x y
    pub fn new(x: f32, y: f32) -> Self {
        Self {x, y}
    }

    /// get a new vector2 form a
    pub fn broadcast(a: f32) -> Self {
        Self {x: a, y: a}
    }

    /// compute length² of vector2
    pub fn length_square(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    /// compute length of vector2
    pub fn length(&self) -> f32 {
        Float::sqrt(self.length_square())
    }

    /// get  a normalized vector2
    pub fn normalized(&self) -> Self {
        let l = self.length();
        Self {x: self.x / l, y: self.y/l}
    }

    /// dot production for vector2
    pub fn dot(a: Self, b: Self) -> f32 {
        a.x * b.x  + a.y * b.y
    }
}

impl Default for Vector2 {
    fn default() -> Self {
        Self{x: 0.0, y: 0.0}
    }
}

impl Add for Vector2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {x : self.x + rhs.x, y: self.y + rhs.y}
    }
}

impl Sub for Vector2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {x: self.x - rhs.x, y: self.y - rhs.y}
    }
} 

impl Mul for Vector2 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self { x: self.x * rhs.x, y: self.y * rhs.y}
    }
}

impl Mul<f32> for Vector2 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self{
        Self {x: self.x * rhs, y: self.y * rhs}
    }
}

impl Mul<Vector2> for f32 {
    type Output = Vector2;

    fn mul(self, rhs: Vector2) -> Vector2 {
        Vector2 {x: self * rhs.x, y: self * rhs.y}
    }
}

impl Div for Vector2 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Self {x: self.x / rhs.x, y: self.y / self.y}
    }
}

impl Div<f32> for Vector2 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        Self {x: self.x / rhs, y: self.y / rhs}
    }
}

impl Neg for Vector2 {
    type Output = Self;

    fn neg(self) -> Self {
        Self{x: -self.x, y: -self.y}
    }
}

impl Index<usize> for Vector2 {
    type Output = f32;

    fn index(&self, i: usize) -> &f32 {
        match i {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Invalid index into Vector2"),
        }
    }
}

impl IndexMut<usize> for Vector2 {
    fn index_mut(&mut self, i: usize) -> &mut f32 {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("Invalid index into Vector3"),
        }
    }
}

use crate::raw::{Vec2f, ToRaw};

impl ToRaw<Vec2f> for Vector2 {
    fn to_raw(&self) -> Vec2f {
        [self.x, self.y]
    }
}