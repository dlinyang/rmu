use std::f32;
use num::Float;
use std::ops::{Add,Sub,Mul, Div, Neg, Index, IndexMut};

/// a vector in R⁴ with 32 bit float number
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vector4 {
    /// get a new vector4 from x y z w
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self{x, y, z, w}
    }

    /// get a new vector4 from a
    pub fn broadcast(a: f32) -> Self {
        Self {x: a, y: a, z: a, w: a}
    }

    /// compute length² of vector4
    pub fn length_square(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }

    /// compute length of vector4
    pub fn length(&self) -> f32 {
        Float::sqrt(self.length_square())
    }

    /// get a normalized vector4
    pub fn normalized(&self) -> Self {
        let l = self.length();
        Self {x: self.x / l, y: self.y / l, z: self.z / l, w: self.w / l}
    }

    /// dot production for vector4
    pub fn dot(a: Self, b: Self) -> f32 {
        a.x * b.x + a.y * b.y + a.z * b.z + a.w * b.w
    }
}

impl Default for Vector4 {
    fn default() -> Self {
        Self {x: 0.0, y: 0.0, z: 0.0, w: 0.0}
    }
}

impl Add for Vector4 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z, w: self.w + rhs.w}
    }
}

impl Sub for Vector4 {
    type Output = Self;
    fn sub(self,rhs: Self) -> Self {
        Self {x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z, w: self.w - rhs.w}
    }
}

impl Mul for Vector4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {x: self.x * rhs.x, y: self.y * rhs.y, z: self.z * rhs.z, w: self.w * rhs.w}
    }
}

impl Mul<f32> for Vector4 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self {x: self.x * rhs, y: self.y * rhs, z: self.z * rhs, w: self.z * rhs}
    }
}

impl Mul<Vector4> for f32 {
    type Output = Vector4;
    
    fn mul(self, rhs: Vector4) -> Vector4 {
        Vector4 {x: self * rhs.z, y: self * rhs.y, z: self * rhs.z, w: self * rhs.w}
    }
}

impl Div for Vector4 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Self {x: self.x / rhs.x, y: self.y / rhs.y, z: self.z / rhs.z, w: self.w / rhs.w}
    }
}

impl Div<f32> for Vector4 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        Self {x: self.x / rhs, y: self.y / rhs, z: self.z / rhs, w: self.w / rhs}
    }
}

impl Neg for Vector4 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {x: -self.x, y: -self.y, z: -self.z, w: - self.w}
    }
}

impl Index<usize> for Vector4 {
    type Output = f32;

    fn index(&self, i: usize) -> &f32 {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("Invalid index into Vector4"),
        }
    }
}

impl IndexMut<usize> for Vector4 {
    fn index_mut(&mut self, i: usize) -> &mut f32 {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => panic!("Invalid index into Vector4")
        }
    }
}

use crate::raw::{Vec4f, ToRaw};

impl ToRaw<Vec4f> for Vector4 {
    fn to_raw(&self) -> Vec4f {
        [self.x, self.y, self.z, self.w]
    }
}