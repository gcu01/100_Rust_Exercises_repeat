use std::ops::Add;

//pub mod private {

pub trait Power {
    type Output;
    fn power(&self, n: u32) -> Self::Output;
}

impl Power for u32 {
    type Output = u32;
    fn power(&self, n: u32) -> Self::Output {
        let mut res:u32 = 1_u32;
        let mut m = n;
        while m>0 {
            res *= *self;
            m -= 1;
        }
        res
    }
}

impl Power for &u32 {
    type Output = u32;
    fn power(&self, n: u32) -> u32 {
        let mut res:u32 = 1_u32;
        let mut m = n;
        while m>0 {
            res *= **self;
            m -= 1;
        }
        res
    }
}

// TODO: implement the necessary traits to make the test compile and pass.
//  You *can't* modify the test.
#[derive(Copy, Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub struct WrappingU32 {
    value: u32,
}

impl WrappingU32 {
    pub fn new(value: u32) -> Self {
        Self { value }
    }
}
/* 
pub trait Add<Rhs = Self> {
    type Output;

    // Required method
    fn add(self, rhs: Rhs) -> Self::Output;
} */

impl Add for WrappingU32 {
    type Output=WrappingU32;
    fn add(self, other:Self) -> Self {
        WrappingU32 { value: self.value + other.value }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ops() {
        let x = WrappingU32::new(42);
        let y = WrappingU32::new(31);
        let z = WrappingU32::new(u32::MAX);
        assert_eq!(x + y + y, WrappingU32::new(104));
    }
}
 
#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_power_u32(){
        let x: u32 = 2_u32.power(3);
        assert_eq!(8, x);
    }

    #[test]
    fn test_power_address_u32(){
        let x:&u32 = &2_u32.power(3);
        assert_eq!(7, *&2_u32.power(3));
    }
}
    
//}