use std::f32;
use num::Float;
use std::ops::{Add,Sub,Mul,Div,Neg,Index,IndexMut};

/// a vector in R³ space with 32bit float number
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Vector3{
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

///  color  as a vector alias for  computer graphics
pub type Color = Vector3;
/// alias of vector to present point in R³
pub type Point3 = Vector3;

impl Vector3 {
    /// get a new vector3 from x y z
    pub fn new(x: f32, y: f32,z: f32) -> Self {
        Self {x: x, y: y, z: z}
    }

    /// get a new vector3 form a
    pub fn broadcast(a: f32) -> Self {
        Self {x: a, y: a, z: a}
    }

    /// compute length² of vector3
    pub fn length_square(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// compute length of vector3
    pub fn length(&self) -> f32 {
        Float::sqrt(self.length_square())
    }

    /// get a normalized vector3
    pub fn normalized(&self) -> Self {
        let l = self.length();
        Self {x: self.x / l, y: self.y / l, z: self.z / l }
    }

    /// dot production for vector3
    pub fn dot(a: Self, b: Self) -> f32 {
        a.x * b.x + a.y * b.y + a.z * b.z
    }

    /// cross production for vector3  
    pub fn cross(a: Vector3, b: Vector3) -> Vector3 {
        let x = a.y * b.z - a.z * b.y;
        let y = -(a.x * b.z - a.z * b.x);
        let z = a.x * b.y - a.y * b.x;
        Vector3{x: x, y: y, z: z}
    }
}

impl Default for Vector3 {
    fn default() -> Self {
        Self {x: 0.0, y: 0.0, z: 0.0}
    }
}


impl Add for Vector3 {
    type Output = Self;

    fn add(self,rhs: Self) -> Self {
        Self {x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z}
    }
}

impl Sub for Vector3 {
    type Output = Self;

    fn sub(self,rhs: Self) -> Self {
        Self {x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z}
    }
}

impl Mul for Vector3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self{x: self.x * rhs.x, y: self.y * rhs.y, z: self.z * rhs.z}
    }
}

impl Mul<f32> for Vector3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self {x: self.x *rhs, y: self.y *rhs, z: self.z * rhs}
    }
}

impl Mul<Vector3> for f32 {
    type Output = Vector3;

    fn mul(self,rhs: Vector3) -> Vector3 {
        Vector3 {x: self * rhs.x, y: self * rhs.y, z: self * rhs.z}
    }
}

impl Div for Vector3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Self {x: self.x / rhs.x, y: self.y / rhs.y , z: self.z / rhs.z}
    }
}

impl Div<f32> for Vector3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self{
        Self {x: self.x / rhs, y: self.y / rhs, z: self.z / rhs}
    }
}

impl Neg for Vector3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {x: -self.x, y: -self.y, z: -self.z}
    }
}

impl Index<usize> for Vector3 {
    type Output = f32;

    fn index(&self, i: usize) -> &f32 {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Invalid index into Vector3"),
        }
    }
}

impl IndexMut<usize> for Vector3 {
    fn index_mut(&mut self, i: usize) -> &mut f32 {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Invalid index into Vector3"),
        }
    }
}

use crate::raw::{Vec3f, ToRaw};

impl ToRaw<Vec3f> for Vector3 {
    fn to_raw(&self) -> Vec3f {
        [self.x, self.y, self.z]
    }
}

impl From<Vec3f> for Vector3 {
    fn from(a: Vec3f) -> Self {
        Self::new(a[0], a[1], a[2])
    }
}