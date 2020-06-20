use std::ops::Mul;

use crate::vector::*;
use crate::matrix::*;

impl Mul<Matrix3x3> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: Matrix3x3) -> Vector3 {
        let a = Vector3::dot(self, Vector3::from(rhs[0]));
        let b = Vector3::dot(self, Vector3::from(rhs[1]));
        let c = Vector3::dot(self, Vector3::from(rhs[2]));
        Vector3::new(a, b, c)
    }
}

impl Mul<Vector3> for Matrix3x3 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Vector3 {
        let a = Vector3::dot(Vector3::from(self[0]), rhs);
        let b = Vector3::dot(Vector3::from(self[1]), rhs);
        let c = Vector3::dot(Vector3::from(self[2]), rhs);
        Vector3::new(a, b, c)
    }
}

pub fn rotation3(x: f32, y: f32, z:f32) -> Matrix3x3 {
    Matrix3x3::from([[z.cos()*x.cos()-y.cos()*x.sin()*z.sin(), -z.sin()*y.sin()*x.sin() - x.cos()*z.sin(), x.sin()*y.sin()]
                    ,[z.cos()*x.sin()+x.cos()*y.cos()*z.sin(), x.cos()*y.cos()*z.cos() - x.sin()*z.cos() , -x.cos()*y.sin()]
                    ,[         y.sin()*z.cos()               ,                z.sin()*y.cos()            ,     y.cos()   ]])
}