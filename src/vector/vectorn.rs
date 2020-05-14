use std::ops::{Add,Sub,Mul,Div,Neg,Index,IndexMut};

#[derive(Clone)]
pub struct VectorN<T> {
    pub data: Vec<T>,
}

impl<T> VectorN<T> 
where T: Default + Add<Output=T> + Mul<Output=T> + Copy + Clone + AsRef<T> {
    pub fn new(data: Vec<T>) -> Self {
        VectorN {
            data,
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn dot(a: VectorN<T>, b: VectorN<T>) -> T{
        let mut result: T = Default::default();
        
        let len = if a.len() > b.len()  { a.len() } else { b.len() };

        for i in 0..len {
            result = result + a[i] * b[i];
        }
        result
    }
}

impl<T:Default> Default for VectorN<T> {
    fn default() -> Self {
        VectorN {
            data: Vec::default(),
        }
    }
}

impl<T: AsRef<T>> Index<usize> for VectorN<T> {
    type Output = T;
    fn index(&self,i: usize) -> &T {
        &self.data[i]
    }
}

impl<T: AsRef<T>> IndexMut<usize> for VectorN<T> {
    fn index_mut(&mut self, i: usize) -> &mut T {
        &mut self.data[i]
    }
}

impl<T: Default + Add<Output=T> + Mul<Output=T>  + Copy + Clone + AsRef<T>> Add for VectorN<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let mut result:Vec<T> = Default::default();

        let len = if self.len() > rhs.len() {self.len()} else {rhs.len()};

        for i in 0..len {
            result.push(self[i] + rhs[i]);
        }

        Self {
            data: result,
        }
    }
}

impl<T: Default + Add<Output=T> + Mul<Output=T> + Sub<Output=T> + Copy + Clone + AsRef<T>> Sub for VectorN<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let mut result:Vec<T> = Default::default();

        let len = if self.len() > rhs.len() {self.len()} else {rhs.len()};

        for i in 0..len {
            result.push(self[i] - rhs[i]);
        }

        Self {
            data: result,
        }
    }
}

impl<T: Default + Add<Output=T> + Mul<Output=T> + Copy + Clone + AsRef<T>> Mul for VectorN<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let mut result:Vec<T> = Default::default();

        let len = if self.len() > rhs.len() {self.len()} else {rhs.len()};

        for i in 0..len {
            result.push(self[i] * rhs[i]);
        }

        Self {
            data: result,
        }
    }
}

impl<T: Default + Add<Output=T> + Mul<Output=T> + Copy + Clone + AsRef<T>> Mul<T> for VectorN<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self {
        let mut result:Vec<T> = Default::default();

        for i in 0..self.len() {
            result.push(self[i] * rhs);
        }

        Self {
            data: result,
        }
    }
}

impl<T: Default + Add<Output=T> + Mul<Output=T> + Div<Output=T> + Copy + Clone + AsRef<T>> Div for VectorN<T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        let mut result:Vec<T> = Default::default();

        let len = if self.len() > rhs.len() {self.len()} else {rhs.len()};

        for i in 0..len {
            result.push(self[i] / rhs[i]);
        }

        Self {
            data: result,
        }
    }
}


impl<T: Default + Add<Output=T> + Mul<Output=T> + Div<Output=T> + Copy + Clone + AsRef<T>> Div<T> for VectorN<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self {
        let mut result:Vec<T> = Default::default();

        for i in 0..self.len() {
            result.push(self[i] / rhs);
        }

        Self {
            data: result,
        }
    }
}


impl<T: Default + Add<Output=T> + Mul<Output=T> + Neg<Output=T> + Copy + Clone + AsRef<T>> Neg for VectorN<T> {
    type Output = Self;

    fn neg(self) -> Self {
        let mut result:Vec<T> = Default::default();

        for e in self.data.iter() {
            result.push(-*e);
        }

        Self {
            data: result,
        }
    }
}