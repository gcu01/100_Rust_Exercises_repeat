// Define a function named `factorial` that, given a non-negative integer `n`,
// returns `n!`, the factorial of `n`.
//
// The factorial of `n` is defined as the product of all positive integers up to `n`.
// For example, `5!` (read "five factorial") is `5 * 4 * 3 * 2 * 1`, which is `120`.
// `0!` is defined to be `1`.
//
// We expect `factorial(0)` to return `1`, `factorial(1)` to return `1`,
// `factorial(2)` to return `2`, and so on.
//
// Use only what you learned! No loops yet, so you'll have to use recursion!

pub fn factorial(a: i32) -> i32 {
    if a<0 {
        panic!("ups! negative number!");
    } else if a == 0 {
        return 0;
    } else if a == 1 {
        return 1;
    }

    a * factorial(a-1)
}

pub fn factorial_for(a: u32) -> u32 {
    let mut res:u32 = 1_u32;
    if a==0 {
        return 0;
    } else if a==1 {
        return 1;
    }

    for i in 2 ..=a {
        res *= i;
    }
    res
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test1_factorial() {
        assert_eq!(0, factorial(0));
        assert_eq!(1, factorial(1));
        assert_eq!(2, factorial(2));
    }

    #[test]
    #[should_panic]
    fn test2_factorial() {
        factorial(-1);
    }

    #[test]
    fn test_factorial_for () {
        assert_eq!(0, factorial(0));
        assert_eq!(1, factorial(1));
        assert_eq!(2, factorial(2));        
    }

}