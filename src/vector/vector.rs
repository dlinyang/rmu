//generate a Rⁿ vector type
use std::ops::{Add, Div, Index, IndexMut, Mul, Neg, Sub};
use num::Zero;

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub struct Vector<T, const SIZE: usize> {
    pub data: [T; SIZE],
}

pub trait Sqrt {
    fn sqrt(self) -> Self;
}

impl<T: Zero + Mul<Output = T> + Add<Output = T> + Div<Output = T> + Copy + Sqrt, const SIZE: usize> Vector<T, SIZE> {
    /// get a new vector from array
    pub fn new(data: [T; SIZE]) -> Self {
        Self { data }
    }

    /// get a new vector form a
    pub fn broadcast(a: T) -> Self {
        Self { data: [a; SIZE] }
    }

    /// compute length² of vector
    pub fn length_square(&self) -> T {
        self.data.iter().fold(T::zero(), |acc, x| acc + (*x * *x))
    }

    /// compute length of vector
    pub fn length(&self) -> T {
        T::sqrt(self.length_square())
        //num::Integer::i(self.length_square())
    }

    /// get  a normalized vector
    pub fn normalized(&self) -> Self {
        let l = self.length();
        let mut  ret = [T::zero();SIZE];
        for i in 0usize..SIZE {
            ret[i] = self.data[i] / l;
        }
        Self{ data: ret}
    }

    /// dot production for vector
    pub fn dot(a: Self, b: Self) -> T {        
        (0usize..SIZE).fold(T::zero(),|acc,x| acc + a.data[x] * b.data[x])
    }

    #[inline]
    pub fn size() -> usize {
        SIZE
    }
}

impl<T: Default + Copy, const SIZE: usize> Default for Vector<T,SIZE>{
    fn default() -> Self {
        Self { data: [T::default();SIZE]}
    }
}

impl<T: Copy + Zero + Add<Output = T>, const SIZE: usize> Add for Vector<T, SIZE> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let mut  ret = [T::zero();SIZE];
        for i in 0usize..SIZE {
            ret[i] = self.data[i] + rhs.data[i];
        }
        Self {data: ret}
    }
}

impl<T: Copy + Zero + Sub<Output = T>,const SIZE: usize> Sub for Vector<T, SIZE> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let mut  ret = [T::zero();SIZE];
        for i in 0usize..SIZE {
            ret[i] = self.data[i] - rhs.data[i];
        }
        Self {data: ret}
    }
} 

impl<T: Copy + Zero + Mul<Output = T>,const SIZE: usize> Mul for Vector<T, SIZE> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let mut  ret = [T::zero();SIZE];
        for i in 0usize..SIZE {
            ret[i] = self.data[i] * rhs.data[i];
        }
        Self {data: ret}
    }
}

impl<T: Copy + Zero + Mul<Output = T>,const SIZE: usize> Mul<T> for Vector<T, SIZE> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self{
        let mut  ret = [T::zero();SIZE];
        for i in 0usize..SIZE {
            ret[i] = self.data[i] * rhs;
        }
        Self {data: ret}
    }
}

impl<T: Copy + Zero + Div<Output = T>,const SIZE: usize> Div for Vector<T,SIZE> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        let mut  ret = [T::zero();SIZE];
        for i in 0usize..SIZE {
            ret[i] = self.data[i] / rhs.data[i];
        }
        Self {data: ret}
    }
}

impl<T: Copy + Zero + Div<Output = T>,const SIZE: usize> Div<T> for Vector<T,SIZE> {
    type Output = Self;

    fn div(self, rhs: T) -> Self {
        let mut  ret = [T::zero();SIZE];
        for i in 0usize..SIZE {
            ret[i] = self.data[i] / rhs;
        }
        Self {data: ret}
    }
}

impl<T: Copy + Zero + Neg<Output = T>,const SIZE: usize> Neg for Vector<T,SIZE> {
    type Output = Self;

    fn neg(self) -> Self {
        let mut ret = [T::zero();SIZE];
        for i in 0usize..SIZE {
            ret[i] = -self.data[i]
        }
        Self {data: ret}
    }
}

impl<T: Copy,const SIZE: usize> Index<usize> for Vector<T,SIZE> {
    type Output = T;

    fn index(&self, i: usize) -> &T {
        if i < SIZE {
            &self.data[i]
        } else {
            panic!("Invalid index into Vector");
        }
    }
}

impl<T: Copy,const SIZE: usize> IndexMut<usize> for Vector<T,SIZE> {
    fn index_mut(&mut self, i: usize) -> &mut T {
        if i < SIZE {
            &mut self.data[i]
        } else {
            panic!("Invalid index into Vector");
        }
    }
}

impl <T: Copy + Eq, const SIZE: usize> Eq for Vector<T,SIZE>{
}

impl Sqrt for f32 {
    #[inline]
    fn sqrt(self) -> Self {
        f32::sqrt(self)
    }
}

impl Sqrt for f64 {
    #[inline]
    fn sqrt(self) -> Self {
        f64::sqrt(self)
    }    
}

impl Sqrt for u8 {
    #[inline]
    fn sqrt(self) -> Self {
        num::integer::sqrt(self)
    }
}

impl Sqrt for u16 {
    #[inline]
    fn sqrt(self) -> Self {
        num::integer::sqrt(self)
    }
}

impl Sqrt for u32 {
    #[inline]
    fn  sqrt(self) -> Self {
        num::integer::sqrt(self)
    } 
}

impl Sqrt for u64 {
    #[inline]
    fn sqrt(self) -> Self {
        num::integer::sqrt(self)
    }
}

impl Sqrt for u128 {
    #[inline]
    fn sqrt(self) -> Self {
        num::integer::sqrt(self)
    }
}

impl Sqrt for i8 {
    #[inline]
    fn sqrt(self) -> Self {
        num::integer::sqrt(self)
    }
}

impl Sqrt for i16 {
    #[inline]
    fn sqrt(self) -> Self {
        num::integer::sqrt(self)
    }
}

impl Sqrt for i32 {
    #[inline]
    fn  sqrt(self) -> Self {
        num::integer::sqrt(self)
    } 
}

impl Sqrt for i64 {
    #[inline]
    fn sqrt(self) -> Self {
        num::integer::sqrt(self)
    }
}

impl Sqrt for i128 {
    #[inline]
    fn sqrt(self) -> Self {
        num::integer::sqrt(self)
    }
}

#[test]
fn vector_f32() {
    type Vector3f = Vector<f32, 3>;
    let a = Vector3f::new([1.0, 1.0, 1.0]);
    let b = Vector3f::new([1.0, 1.0, 1.0]);
    assert_eq!(a + b, Vector3f::new([2.0, 2.0, 2.0]));
    assert_eq!(a - b, Vector3f::broadcast(0.0));
    assert_eq!(Vector3f::dot(a, b), 3.0);
    assert_eq!(a.length_square(), 3.0);
    assert_eq!(a.length(), f32::sqrt(3.0));
}

#[test]
fn vector_f64() {
    type Vector3d = Vector<f64,3>;
    let a = Vector3d::new([1.0,1.0,1.0]);
    let b = a.clone();
    assert_eq!(a + b, Vector3d::new([2.0,2.0,2.0]));
}
