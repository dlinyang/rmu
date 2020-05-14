use std::ops::{Add,Sub,Mul,Div,Neg,Index,IndexMut};

/// 3x3 matrix with 32bit float number
#[derive(Debug,Copy,Clone,PartialEq,PartialOrd)]
pub struct Matrix3x3 {
    pub data: [[f32;3];3],
}

impl Matrix3x3 {
    /// get a new 3x3 matrix with a00 a11 a22
    pub fn new(a00: f32, a11: f32, a22: f32) -> Self {
        Self {
            data: [[a00, 0.0, 0.0]
                  ,[0.0, a11, 0.0]
                  ,[0.0, 0.0, a22]],
        }
    }

    pub fn trace(&self) -> f32 {
        self[0][0] + self[1][1] + self[2][2]
    }

    pub fn determinate(&self) -> f32 {
        self[0][0] * (self[1][1] * self[2][2] - self[1][2] * self[2][1]) +
        -self[0][1] * (self[1][0] * self[2][2] - self[1][2] * self[2][0]) + 
        self[0][2] * (self[1][0] * self[2][1] - self[1][1] * self[2][0])
    }
}

use crate::raw::ID3F;

impl Default for Matrix3x3 {
    fn default() -> Self {
        Self {
            data: ID3F,
        }
    }
}

impl Index<usize> for Matrix3x3 {
    type Output = [f32;3];

    fn index(&self, i: usize) -> &[f32;3] {
        &self.data[i]
    }
}

impl IndexMut<usize> for Matrix3x3 {
    fn index_mut(&mut self, i: usize) -> &mut [f32;3] {
        &mut self.data[i]
    }
}

impl Add for Matrix3x3 {
    type Output = Self;
    
    fn add(self, rhs: Self) -> Self {
        Self{
            data: [[self[0][0] + rhs[0][0], self[0][1] + rhs[0][1], self[0][2] + rhs[0][2]]
                  ,[self[1][0] + rhs[1][0], self[1][1] + rhs[1][1], self[1][2] + rhs[1][2]]
                  ,[self[2][0] + rhs[2][2], self[2][1] + rhs[2][1], self[2][2] + rhs[2][2]]],
        }
    }
}

impl Add<f32> for Matrix3x3 {
    type Output = Self;

    fn add(self, rhs: f32) -> Self {
        Self {
            data: [[self[0][0] + rhs, self[0][1] + rhs, self[0][2] + rhs]
                  ,[self[1][0] + rhs, self[1][1] + rhs, self[1][2] + rhs]
                  ,[self[2][0] + rhs, self[2][1] + rhs, self[2][2] + rhs]],
        }
    }
}

impl Add<Matrix3x3> for f32 {
    type Output = Matrix3x3;

    fn add(self, rhs: Matrix3x3) -> Matrix3x3 {
        Matrix3x3 {
            data: [[self + rhs[0][0], self + rhs[0][1], self + rhs[0][2]]
                  ,[self + rhs[1][0], self + rhs[1][1], self + rhs[1][2]]
                  ,[self + rhs[2][0], self + rhs[2][1], self + rhs[2][2]]],
        }
    }
}

impl Sub for Matrix3x3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            data: [[self[0][0] - rhs[0][0], self[0][1] - rhs[0][1], self[0][2] - rhs[0][2]]
                  ,[self[1][0] - rhs[1][0], self[1][1] - rhs[1][1], self[1][2] - rhs[1][2]]
                  ,[self[2][0] - rhs[2][0], self[2][1] - rhs[2][1], self[2][2] - rhs[2][2]]]
        }
    }
}

impl Sub<f32> for Matrix3x3 {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self {
        Self {
            data: [[self[0][0] - rhs, self[0][1] - rhs, self[0][2] - rhs]
                  ,[self[1][0] - rhs, self[1][1] - rhs, self[1][2] - rhs]
                  ,[self[2][0] - rhs, self[2][1] - rhs, self[2][2] - rhs]],
        }
    }
}

impl Mul for Matrix3x3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let a00 = self[0][0] * rhs[0][0] + self[0][1] * rhs[1][0] + self[0][2] * rhs[2][0];
        let a01 = self[0][0] * rhs[0][1] + self[0][1] * rhs[1][1] + self[0][2] * rhs[2][1];
        let a02 = self[0][0] * rhs[0][2] + self[0][1] * rhs[1][2] + self[0][2] * rhs[2][2];

        let a10 = self[1][0] * rhs[1][0] + self[1][1] * rhs[1][0] + self[1][2] * rhs[2][0];
        let a11 = self[1][0] * rhs[1][1] + self[1][1] * rhs[1][1] + self[1][2] * rhs[2][1];
        let a12 = self[1][0] * rhs[1][2] + self[1][1] * rhs[1][2] + self[1][2] * rhs[2][2];
        
        let a20 = self[2][0] * rhs[0][0] + self[2][1] * rhs[1][0] + self[2][2] * rhs[2][0];
        let a21 = self[2][0] * rhs[0][1] + self[2][1] * rhs[1][1] + self[2][2] * rhs[2][1];
        let a22 = self[2][0] * rhs[0][2] + self[2][1] * rhs[1][2] + self[2][2] * rhs[2][2];

        Self {
            data: [[a00, a01, a02]
                  ,[a10, a11, a12]
                  ,[a20, a21, a22]],
        }
    }
}

impl Mul<f32> for Matrix3x3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self {
            data: [[self[0][0] * rhs, self[0][1] * rhs, self[0][2] * rhs]
                  ,[self[1][0] * rhs, self[1][1] * rhs, self[1][2] * rhs]
                  ,[self[2][0] * rhs, self[2][1] * rhs, self[2][2] * rhs]]
        }
    }
}

impl Mul<Matrix3x3> for f32 {
    type Output = Matrix3x3;

    fn mul(self, rhs: Matrix3x3) -> Matrix3x3 {
        Matrix3x3 {
            data: [[self * rhs[0][0], self * rhs[0][1], self * rhs[0][2]]
                  ,[self * rhs[1][0], self * rhs[1][1], self * rhs[1][2]]
                  ,[self * rhs[2][0], self * rhs[2][1], self * rhs[2][2]]],
        }
    }
}

impl Div<f32> for Matrix3x3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        Self {
            data: [[self[0][0] / rhs, self[0][1] / rhs, self[0][2] / rhs]
                  ,[self[1][0] / rhs, self[1][1] / rhs, self[1][2] / rhs]
                  ,[self[2][0] / rhs, self[2][1] / rhs, self[2][2] / rhs]],
        }
    }
}

impl Neg for Matrix3x3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            data: [[-self[0][0], -self[0][1], -self[0][2]]
                  ,[-self[1][0], -self[1][1], -self[1][2]]
                  ,[-self[2][0], -self[2][1], -self[2][2]]],
        }
    }
}

use crate::raw::Mat3f;

impl From<Mat3f> for Matrix3x3 {
    fn from(mat3: Mat3f) -> Self {
        Self {
            data: mat3,
        }
    }
}

impl From<Matrix3x3> for Mat3f {
    fn from(matrix3x3: Matrix3x3) -> Mat3f {
        matrix3x3.data
    }
}