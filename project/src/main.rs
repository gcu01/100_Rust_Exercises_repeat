mod exercises;
use crate::exercises::factorial::*;
use crate::exercises::structs::*;
use crate::exercises::order_struct::*;
fn main() {
    let a:bool = true;
    let b:u32 = if a {
        1 } else {0};

    println!("using recursion, factorial(5)={}", factorial(5));
    println!("using for loop, factorial(5)={}", factorial(5));
}
