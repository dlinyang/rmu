use std::ops::{Add,Sub,Mul,Div,Neg,Index,IndexMut};

/// 2x2 matrix with 32bit float number
#[derive(Debug,Copy,Clone,PartialEq,PartialOrd)]
pub struct Matrix2x2 {
    pub data: [[f32;2];2],
}


impl Matrix2x2 {
    /// get a new 2x2 matrix with a00 a11
    pub fn new(a00: f32, a11: f32) -> Self {
        Self {
            data: [[a00, 0.0]
                  ,[0.0, a11]],
        }
    }

    pub fn trace(&self) -> f32 {
        self[0][0] + self[1][1]
    }

    pub fn determinate(&self) -> f32 {
        self[0][0] * self[1][1] - self[0][1] * self[1][1]
    }

    pub fn translation1(x: f32) -> Self {
        use crate::matrix::transform::translation1;

        Self {
            data: translation1(x)
        }
    }

    pub fn scale2(x: f32, y: f32) -> Self {
        Self::new(x, y)
    }

    pub fn rotation2(theta: f32) -> Self {
        use crate::matrix::transform::rotation2;
        
        Self {
            data: rotation2(theta)
        }
    }
}

use crate::raw::ID2F;

impl Default for Matrix2x2 {
    fn default() -> Self {
        Self {
            data: ID2F,
        }
    }
}

impl Index<usize> for Matrix2x2 {
    type Output = [f32;2];

    fn index(&self, i: usize) -> &[f32;2] {
        &self.data[i]
    }
}

impl IndexMut<usize> for Matrix2x2 {
    fn index_mut(&mut self, i: usize) -> &mut [f32;2] {
        &mut self.data[i]
    }
}

impl Add for Matrix2x2 {
    type Output = Self;

    fn add(self,rhs: Self) -> Self {
        Self {
            data: [[self[0][0] + rhs[0][0], self[0][1] + rhs[0][1]]
                  ,[self[1][0] + rhs[1][0], self[1][1] + rhs[1][1]]],
        }
    }
}

impl Add<f32> for Matrix2x2 {
    type Output = Self;

    fn add(self, rhs: f32) -> Self {
        Self {
            data: [[self[0][0] + rhs, self[0][1] + rhs]
                  ,[self[1][0] + rhs, self[1][1] + rhs]],
        }
    }
}

impl Add<Matrix2x2> for f32 {
    type Output = Matrix2x2;

    fn add(self, rhs: Matrix2x2) -> Matrix2x2 {
        Matrix2x2 {
            data: [[self + rhs[0][0], self + rhs[0][1]]
                  ,[self + rhs[1][0], self + rhs[1][1]]],
        }
    }
}

impl Sub for Matrix2x2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            data: [[self[0][0] - rhs[0][0], self[0][1] - rhs[0][1]]
                  ,[self[1][0] - rhs[1][0], self[1][1] - rhs[1][1]]],
        }
    }
}

impl Sub<f32> for Matrix2x2 {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self {
        Self {
            data: [[self[0][0] - rhs, self[0][1] - rhs]
                  ,[self[1][0] - rhs, self[1][1] - rhs]],
        }
    }
}


impl Mul for Matrix2x2 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let a00 = self[0][0] * self[0][0] + self[0][1] * rhs[1][0];
        let a01 = self[0][0] * self[0][1] + self[0][1] * rhs[1][1];
        let a10 = self[1][0] * self[0][0] + self[1][1] * rhs[1][0];
        let a11 = self[1][0] * self[0][1] + self[1][1] * rhs[1][1];

        Self {
            data: [[a00, a01]
                  ,[a10, a11]],
        }
    }
}

impl Mul<f32> for Matrix2x2 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self {
            data: [[self[0][0] * rhs, self[0][1] * rhs]
                  ,[self[1][0] * rhs, self[1][1] * rhs]],
        }
    }
}

impl Mul<Matrix2x2> for f32 {
    type Output = Matrix2x2;

    fn mul(self, rhs: Matrix2x2) -> Matrix2x2 {
        Matrix2x2 {
            data: [[self * rhs[0][0], self * rhs[0][1]]
                  ,[self * rhs[1][0], self * rhs[1][1]]],
        }
    }
}

impl Div<f32> for Matrix2x2 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        Self {
            data: [[self[0][0] / rhs, self[0][1] / rhs]
                  ,[self[1][0] / rhs, self[1][1] / rhs]],
        }
    }
}

impl Neg for Matrix2x2 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            data: [[-self[0][0], -self[0][1]]
                  ,[-self[1][0], -self[1][1]]],
        }
    }
}

use crate::raw::Mat2f;

impl From<Mat2f> for Matrix2x2 {
    fn from(mat2 : Mat2f) -> Self {
        Self {
            data: mat2,
        }
    }
}

impl From<Matrix2x2> for Mat2f {
    fn from(matrix2x2: Matrix2x2) -> Mat2f {
        matrix2x2.data
    }
}

use crate::vector::Vector2;

impl Mul<Matrix2x2> for Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: Matrix2x2) -> Vector2 {
        let a = Vector2::dot(self, Vector2::from(rhs[0]));
        let b = Vector2::dot(self, Vector2::from(rhs[1]));
        Vector2::new(a, b)
    }
}

impl Mul<Vector2> for Matrix2x2 {
    type Output = Vector2;

    fn mul(self, rhs: Vector2) -> Vector2 {
        let a = Vector2::dot(Vector2::from(self[0]), rhs);
        let b = Vector2::dot(Vector2::from(self[1]), rhs);
        Vector2::new(a, b)
    }
}