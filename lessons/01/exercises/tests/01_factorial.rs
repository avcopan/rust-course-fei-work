//! Run this file with `cargo test --test 01_factorial`.

// // Initial solution:
// fn factorial(num: u32) -> u32 {
//     if num == 0 {
//         return 1;
//     }

//     let mut val = num;
//     for i in 1..num {
//         val *= num - i;
//     }
//     val
// }

fn factorial(num: u32) -> u32 {
    (1..=num).product()
}

/// Below you can find a set of unit tests.
#[cfg(test)]
mod tests {
    use super::factorial;

    #[test]
    fn factorial_0() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn factorial_1() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn factorial_2() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn factorial_5() {
        assert_eq!(factorial(5), 120);
    }
}
