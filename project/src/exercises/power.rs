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