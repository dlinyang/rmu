use std::ops::{Add,Sub,Mul,Div,Neg,Index,IndexMut};

/// 4x4 matrix with 32bit float number
#[derive(Debug,Copy,Clone,PartialEq,PartialOrd)]
pub struct Matrix4x4 {
    pub data: [[f32;4];4]
}

impl Matrix4x4 {
    /// get new 4x4 matrix with a00 a11 a22 a33
    pub fn new(a00: f32, a11: f32, a22: f32, a33: f32) -> Self {
        Self {
            data: [[a00, 0.0, 0.0, 0.0]
                  ,[0.0, a11, 0.0, 0.0]
                  ,[0.0, 0.0, a22, 0.0]
                  ,[0.0, 0.0, 0.0, a33]]
        }
    }

    pub fn from(data: [[f32;4];4]) -> Self {
        Self { data }
    }

    pub fn trace(&self) -> f32 {
        self[0][0] + self[1][1] + self[2][2] + self[3][3]
    }

    // note: too long writing later
    pub fn determinate(&self) -> f32 {
        unimplemented!()
    }
}

use crate::raw::ID4F;

impl Default for Matrix4x4 {
    fn default() -> Self {
        Self {
            data: ID4F,
        }
    }
}

impl Index<usize> for  Matrix4x4 {
    type Output = [f32;4];

    fn index(&self, i: usize) -> &[f32;4] {
        &self.data[i]
    }
}

impl IndexMut<usize> for Matrix4x4 {
    fn index_mut(&mut self, i: usize) -> &mut [f32;4] {
        &mut self.data[i]
    }
}

impl Add for Matrix4x4 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            data: [[self[0][0] + rhs[0][0], self[0][1] + rhs[0][1], self[0][2] + rhs[0][2], self[0][3] + rhs[0][3]]
                  ,[self[1][0] + rhs[1][0], self[1][1] + rhs[1][1], self[1][2] + rhs[1][2], self[1][3] + rhs[1][3]]
                  ,[self[2][0] + rhs[2][0], self[2][1] + rhs[2][1], self[2][2] + rhs[2][2], self[2][3] + rhs[2][3]]
                  ,[self[3][0] + rhs[3][0], self[3][1] + rhs[3][1], self[3][2] + rhs[3][2], self[3][3] + rhs[3][3]]]
        }
    }
}

impl Add<f32> for Matrix4x4 {
    type Output = Self;

    fn add(self, rhs: f32) -> Self {
        Self {
            data: [[self[0][0] + rhs, self[0][1] + rhs, self[0][2] + rhs, self[0][3] + rhs]
                  ,[self[1][0] + rhs, self[1][1] + rhs, self[1][2] + rhs, self[1][3] + rhs]
                  ,[self[2][0] + rhs, self[2][1] + rhs, self[2][2] + rhs, self[2][3] + rhs]
                  ,[self[3][0] + rhs, self[3][1] + rhs, self[3][2] + rhs, self[3][3] + rhs]],
        }
    }
}

impl Add<Matrix4x4> for f32 {
    type Output = Matrix4x4;

    fn add(self, rhs: Matrix4x4) -> Matrix4x4 {
        Matrix4x4 {
            data: [[self + rhs[0][0], self + rhs[0][1], self + rhs[0][2], self + rhs[0][3]]
                  ,[self + rhs[1][0], self + rhs[1][1], self + rhs[1][2], self + rhs[1][3]]
                  ,[self + rhs[2][0], self + rhs[2][1], self + rhs[2][2], self + rhs[2][3]]
                  ,[self + rhs[3][0], self + rhs[3][1], self + rhs[3][2], self + rhs[3][3]]],
        }
    }
}

impl Sub for Matrix4x4 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            data: [[self[0][0] - rhs[0][0], self[0][1] - rhs[0][1], self[0][2] - rhs[0][2], self[0][3] - rhs[0][3]]
                  ,[self[1][0] - rhs[1][0], self[1][1] - rhs[1][1], self[1][2] - rhs[1][2], self[1][3] - rhs[1][3]]
                  ,[self[2][0] - rhs[2][0], self[2][1] - rhs[2][1], self[2][2] - rhs[2][2], self[2][3] - rhs[2][3]]
                  ,[self[3][0] - rhs[3][0], self[3][1] - rhs[3][1], self[3][2] - rhs[3][2], self[3][3] - rhs[3][3]]],
        }
    }
}

impl Sub<f32> for Matrix4x4 {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self {
        Self {
            data: [[self[0][0] - rhs, self[0][1] - rhs, self[0][2] - rhs, self[0][3] - rhs]
                  ,[self[1][0] - rhs, self[1][1] - rhs, self[1][2] - rhs, self[1][3] - rhs]
                  ,[self[2][0] - rhs, self[2][1] - rhs, self[2][2] - rhs, self[2][3] - rhs]
                  ,[self[3][0] - rhs, self[3][1] - rhs, self[3][2] - rhs, self[3][3] - rhs]],
        }
    }
}

impl Mul for Matrix4x4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let a00 = self[0][0] * rhs[0][0] + self[0][1] * rhs[1][0] + self[0][2] * rhs[2][0] + self[0][3] * rhs[3][0];
        let a01 = self[0][0] * rhs[0][1] + self[0][1] * rhs[1][1] + self[0][2] * rhs[2][1] + self[0][3] * rhs[3][1];
        let a02 = self[0][0] * rhs[0][2] + self[0][1] * rhs[1][2] + self[0][2] * rhs[2][2] + self[0][3] * rhs[3][2];
        let a03 = self[0][0] * rhs[0][3] + self[0][1] * rhs[1][3] + self[0][2] * rhs[2][3] + self[0][3] * rhs[3][3];

        let a10 = self[1][0] * rhs[0][0] + self[1][1] * rhs[1][0] + self[1][2] * rhs[2][0] + self[1][3] * rhs[3][0];
        let a11 = self[1][0] * rhs[0][1] + self[1][1] * rhs[1][1] + self[1][2] * rhs[2][1] + self[1][3] * rhs[3][1];
        let a12 = self[1][0] * rhs[0][2] + self[1][1] * rhs[1][2] + self[1][2] * rhs[2][2] + self[1][3] * rhs[3][2];
        let a13 = self[1][0] * rhs[0][3] + self[1][1] * rhs[1][3] + self[1][2] * rhs[2][3] + self[1][3] * rhs[3][3];

        let a20 = self[2][0] * rhs[0][0] + self[2][1] * rhs[1][0] + self[2][2] * rhs[2][0] + self[2][3] * rhs[3][0];
        let a21 = self[2][0] * rhs[0][1] + self[2][1] * rhs[1][1] + self[2][2] * rhs[2][1] + self[2][3] * rhs[3][1];
        let a22 = self[2][0] * rhs[0][2] + self[2][1] * rhs[1][2] + self[2][2] * rhs[2][2] + self[2][3] * rhs[3][2];
        let a23 = self[2][0] * rhs[0][3] + self[2][1] * rhs[1][3] + self[2][2] * rhs[2][3] + self[2][3] * rhs[3][3];

        let a30 = self[3][0] * rhs[0][0] + self[3][1] * rhs[1][0] + self[3][2] * rhs[2][0] + self[3][3] * rhs[3][0];
        let a31 = self[3][0] * rhs[0][1] + self[3][1] * rhs[1][1] + self[3][2] * rhs[2][1] + self[3][3] * rhs[3][1];
        let a32 = self[3][0] * rhs[0][2] + self[3][1] * rhs[1][2] + self[3][2] * rhs[2][2] + self[3][3] * rhs[3][2];
        let a33 = self[3][0] * rhs[0][3] + self[3][1] * rhs[1][3] + self[3][2] * rhs[2][3] + self[3][3] * rhs[3][3];

        Self {
            data: [[a00, a01, a02, a03]
                  ,[a10, a11, a12, a13]
                  ,[a20, a21, a22, a23]
                  ,[a30, a31, a32, a33]],
        }
    }
}

impl Mul<f32> for Matrix4x4 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self {
            data: [[self[0][0] * rhs, self[0][1] * rhs, self[0][2] * rhs, self[0][3] * rhs]
                  ,[self[1][0] * rhs, self[1][1] * rhs, self[1][2] * rhs, self[0][3] * rhs]
                  ,[self[2][0] * rhs, self[2][1] * rhs, self[2][2] * rhs, self[2][3] * rhs]
                  ,[self[3][0] * rhs, self[3][1] * rhs, self[3][2] * rhs, self[3][3] * rhs]],
        }
    }
}

impl Mul<Matrix4x4> for f32 {
    type Output = Matrix4x4;

    fn mul(self, rhs: Matrix4x4) -> Matrix4x4 {
        Matrix4x4 {
            data: [[self * rhs[0][0], self * rhs[0][1], self * rhs[0][2], self * rhs[0][3]]
                  ,[self * rhs[1][0], self * rhs[1][1], self * rhs[1][2], self * rhs[1][3]]
                  ,[self * rhs[2][0], self * rhs[2][1], self * rhs[2][2], self * rhs[2][3]]
                  ,[self * rhs[3][0], self * rhs[3][1], self * rhs[3][2], self * rhs[3][3]]]
        }
    }
}

impl Div<f32> for Matrix4x4 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        Self { 
            data: [[self[0][0] / rhs, self[0][1] / rhs, self[0][2] / rhs, self[0][3] / rhs]
                  ,[self[1][0] / rhs, self[1][1] / rhs, self[1][2] / rhs, self[1][3] / rhs]
                  ,[self[2][0] / rhs, self[2][1] / rhs, self[2][2] / rhs, self[2][3] / rhs]
                  ,[self[3][0] / rhs, self[3][1] / rhs, self[3][2] / rhs, self[3][3] / rhs]],
        }
    }
}

impl Neg for Matrix4x4 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            data: [[-self[0][0], -self[0][1], -self[0][2], -self[0][3]]
                  ,[-self[1][0], -self[1][1], -self[1][2], -self[1][3]]
                  ,[-self[2][0], -self[2][1], -self[2][2], -self[2][3]]
                  ,[-self[3][0], -self[3][1], -self[3][2], -self[3][3]]],
        }
    }
}

use crate::raw::Mat4f;

impl From<Mat4f> for Matrix4x4 {
    fn from(mat4: Mat4f) -> Self {
        Self {
            data: mat4,
        }
    }
}

impl From<Matrix4x4> for Mat4f {
    fn from(matrix4x4: Matrix4x4) -> Mat4f {
        matrix4x4.data
    }
}