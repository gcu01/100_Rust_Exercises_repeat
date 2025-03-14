// Define a trait named `IsEven` that has a method `is_even` that returns a `true` if `self` is
// even, otherwise `false`.
//
// Then implement the trait for `u32` and `i32`.
use std::ops::Rem;
use std::prelude::*;


pub trait IsEven {
    fn is_even(&self) -> bool;
}

impl IsEven for i32{
    fn is_even(&self) -> bool {
            if *self % 2 == 0 {
                return true;
            } 
            false
        }
}
impl IsEven for u32{
    fn is_even(&self) -> bool {
            if *self % 2 == 0 {
                return true;
            } 
            false
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iseven_i32() {
        assert_eq!(true, 0_i32.is_even());
        assert_eq!(false, 1_i32.is_even());
        assert_eq!(true, 2_i32.is_even());
        assert_eq!(false, (-1_i32).is_even());
        assert_eq!(true, (-2_i32).is_even());
    }
}