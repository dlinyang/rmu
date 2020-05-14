
use std::ops::{Add,Sub,Mul,Div,Neg,Index,IndexMut};

//generate a Rⁿ vector type
#[macro_export]
macro_rules! vector_define {
    ($name: ident, $dim: expr, $type: ty) => {
        #[derive(Debug,Copy,Clone,PartialEq)]
        pub struct $name {
            pub data: [$type; $dim],
        }

        impl $name {
            pub fn new(data: [$type;$dim]) -> Self {
                $name {
                    data,
                }
            }

            pub fn broadcast(a: $type) -> Self {
                $name {
                    data: [a;$dim],
                }
            }

            pub fn length_square(&self) -> $type {
                let mut result: $type = Default::default();

                for i in 0..$dim {
                    result = self[i] * self[i];
                }

                result
            }

            pub fn dot(a: $name, b: $name) -> $type {
                let mut result: $type = Default::default();

                for i in 0..$dim {
                    result = result + a[i] * b[i];
                }

                result
            }
        }

        impl  Add for $name {
            type Output = Self;
            fn add(self, rhs: Self) -> Self {
                let mut result: [$type;$dim] = Default::default();

                for i in 0..$dim {
                    result[i] = self[i] + rhs[i];
                }

                Self {
                    data: result,
                }
            }
        }

        impl Sub for $name {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self {
                let mut result: [$type;$dim] = Default::default();

                for i in 0..$dim {
                    result[i] = self[i] - rhs[i];
                }

                Self {
                    data: result,
                }
            }
        }

        impl Mul for $name {
            type Output = Self;

            fn mul(self, rhs: Self) -> Self {
                let mut result: [$type;$dim] = Default::default();

                for i in 0..$dim {
                    result[i] = self[i] * rhs[i];
                }

                Self {
                    data: result,
                }
            }
        }

        impl Mul<$type> for $name {
            type Output = Self;

            fn mul(self, rhs: $type) -> Self {
                let mut result: [$type;$dim] = Default::default();

                for i in 0..$dim {
                    result[i] = self[i] * rhs;
                }

                Self {
                    data: result,
                }
            }
        }

        impl Mul<$name> for $type {
            type Output = $name;

            fn mul(self, rhs: $name) -> $name {
                let mut result: [$type;$dim] = Default::default();

                for i in 0..$dim {
                    result[i] = self * rhs[i];
                }

                $name {
                    data: result,
                }
            }
        }

        impl Div for $name {
            type Output = Self;
            fn div(self, rhs: Self) -> Self {
                let mut result: [$type;$dim] = Default::default();

                for i in 0..$dim {
                    result[i] = self[i] / rhs[i];
                }

                Self {
                    data: result,
                }
            }
        }

        impl Div<$type> for $name {
            type Output = Self;
            fn div(self, rhs: $type) -> Self {
                let mut result: [$type;$dim] = Default::default();

                for i in 0..$dim {
                    result[i] = self[i] / rhs;
                }

                Self {
                    data: result,
                }
            }
        }
        
        impl Neg for $name {
            type Output = Self;
            fn neg(self) -> Self {
                let mut result: [$type;$dim] = Default::default();

                for i in 0..$dim {
                    result[i] = -self[i];
                }

                Self {
                    data: result,
                }
            }
        }

        impl Index<usize> for $name {
            type Output = $type;

            fn index(&self, i: usize) -> &$type {
                &self.data[i]
            }
        }

        impl IndexMut<usize> for $name {
            fn index_mut(&mut self, i: usize) -> &mut $type {
                &mut self.data[i]
            }
        }

        impl From<[$type;$dim]> for $name {
            fn from(vec: [$type;$dim]) -> Self {
                Self {
                    data: vec,
                }
            }
        }

        impl From<$name> for [$type;$dim] {
            fn from(vector: $name) -> [$type;$dim] {
                vector.data
            }
        }
    };
}


#[test]
fn vector_f32() {
    vector_define!(Vector3f,3,f32);
    let a = Vector3f::new([1.0,1.0,1.0]);
    let b = Vector3f::new([1.0,1.0,1.0]);
    assert_eq!(a + b,Vector3f::new([2.0,2.0,2.0]));
    assert_eq!(a - b,Vector3f::broadcast(0.0));
    assert_eq!(Vector3f::dot(a, b),3.0);
}
