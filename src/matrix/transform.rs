use crate::raw::{Mat2f, Mat3f, Mat4f};

pub fn scale2(x: f32, y:f32) -> Mat2f {
    [[  x,  0f32]
    ,[0f32,   y ]]
}

pub fn rotate2(theta: f32) -> Mat2f {
    [[theta.cos(), -theta.sin()]
    ,[theta.sin(),  theta.cos()]]
}

pub fn scale3(x:f32, y: f32, z:f32) -> Mat3f {
    [[ x , 0.0, 0.0]
    ,[0.0,  y , 0.0]
    ,[0.0, 0.0,  z]]
}

pub fn rotate3(x: f32, y: f32, z:f32) -> Mat3f {
    [[z.cos()*x.cos()-y.cos()*x.sin()*z.sin(), -z.sin()*y.sin()*x.sin() - x.cos()*z.sin(), x.sin()*y.sin() ]
    ,[z.cos()*x.sin()+x.cos()*y.cos()*z.sin(), x.cos()*y.cos()*z.cos() - x.sin()*z.cos() , -x.cos()*y.sin()]
    ,[         y.sin()*z.cos()               ,                z.sin()*y.cos()            ,     y.cos()     ]]
}

pub fn translate1(x: f32) -> Mat2f {
    [[1f32,   x ]
    ,[0f32, 1f32]]
}

pub fn translate2(x: f32, y: f32) -> Mat3f {
    [[1f32, 0f32,  x  ]
    ,[0f32, 1f32,  y  ]
    ,[0f32, 0f32, 1f32]]
}

pub fn translate3(x: f32, y: f32, z: f32) -> Mat4f {
    [[1f32, 0f32, 0f32,   x ]
    ,[0f32, 1f32, 0f32,   y ]
    ,[0f32, 0f32, 1f32,   z ]
    ,[0f32, 0f32, 0f32, 1f32]]
}