mod exercises;
use exercises::from_trait;

use crate::exercises::factorial::*;
use crate::exercises::structs::*;
use crate::exercises::order_struct::*;
use crate::exercises::trait1::IsEven;
use crate::exercises::partial_eq::*;
use crate::exercises::from_trait::private::*;
fn main() {
    let a:bool = true;
    let b:u32 = if a {
        1 } else {0};

    println!("using recursion, factorial(5)={}", factorial(5));
    println!("using for loop, factorial(5)={}", factorial(5));

    let a: i32 = 6;
    let b: i32 = 7;
    println!(" a={} even ? {} \n b={} even ? {}", a, a.is_even(), b, b.is_even());

    let t1: Ticket = Ticket{title: "One".into(), description: "To continue".into(), status: "In Progress".into()};
        let t2: Ticket = Ticket { title: "One".into(), description: "To continue".into(), status: "In Progress".into()};
        let t3: Ticket = Ticket { title: "One".into(), description: "To continue".into(), status: "To-Do".into()};

    println!("t1 = {:?} \n t2= {:?} \n t3 = {:?} \n t1==t2 {} \n t1==t3 {}", t1, t2, t3, t1==t2, t1==t3)
    //let wrapping: WrappingU32 = 42.into();
    let wrapping = WrappingU32::<from_trait::private::From>::from(42);
}
