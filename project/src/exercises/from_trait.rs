// TODO: Implement the `From` trait for the `WrappingU32` type to make `example` compile.
pub mod private {
    pub struct WrappingU32 {
        value: u32,
    }

    pub trait From<T: Sized> {
        fn from(v: T) -> Self;
    }
    impl From<i32> for WrappingU32 
    {
        fn from(v: i32) -> Self {
            WrappingU32{ value: v as u32}
        }
    }
}