///
/// 

use std::ops::{Mul,Sub};
use std::cmp::PartialOrd;
pub fn factorial<T: Copy + Clone + Mul<Output = T> + Sub<Output = T> + PartialOrd + From<u32>>(n: T) -> T {
    let mut result = n;
    let mut i = n - From::from(1);
    
    while n > From::from(0) {
        result = result * i;
        i = i - From::from(1);
    }

    result
}