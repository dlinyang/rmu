///
/// 

use std::ops::{Mul,Sub};
use std::cmp::PartialOrd;
use num::{One,Zero};

pub fn factorial<T: Copy + Clone + Mul<Output = T> + Sub<Output = T> + PartialOrd + Zero + One>(n: T) -> T {
    let mut result = n;
    let mut i = n - One::one();
    
    while n > Zero::zero() {
        result = result * i;
        i = i - One::one();
    }

    result
}