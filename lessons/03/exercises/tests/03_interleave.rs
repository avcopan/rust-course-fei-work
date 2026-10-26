//! Run this file with `cargo test --test 03_interleave`.

// TODO: Implement a function called `interleave`, which will take two string slices and return
// a string that contains the characters from the two input strings interleaved.
// The first character of the output should start with the first character of the first argument.
// See tests for details.
//
// Can you write the function without any explicit indexing (`str[index]`)?
//
// Hint: you can use `string.chars()` to create an iterator over the Unicode characters of a string.

// // When a function creates a string internally, you just pass back the string
// // itself and it gets moved out with no copy
// // First solution:
// use std::cmp;
// fn interleave<'a>(s1: &str, s2: &str) -> String {
//     let mut iter1 = s1.chars();
//     let mut iter2 = s2.chars();

//     let mut s = String::new();

//     for _ in 0..cmp::max(s1.len(), s2.len()) {
//         if let Some(c1) = iter1.next() {
//             s.push(c1);
//         };
//         if let Some(c2) = iter2.next() {
//             s.push(c2);
//         };
//     }

//     s
// }

// Better solution:
fn interleave(s1: &str, s2: &str) -> String {
    let mut iter1 = s1.chars();
    let mut iter2 = s2.chars();

    let mut s = String::new();
    loop {
        match (iter1.next(), iter2.next()) {
            (None, None) => break,
            (c1, c2) => {
                s.extend(c1);
                s.extend(c2);
            }
        }
    }

    s
}

/// Below you can find a set of unit tests.
#[cfg(test)]
mod tests {
    use crate::interleave;

    #[test]
    fn interleave_empty() {
        assert_eq!(interleave("", ""), "");
    }

    #[test]
    fn interleave_only_left() {
        assert_eq!(interleave("a", ""), "a");
        assert_eq!(interleave("zxjas", ""), "zxjas");
    }

    #[test]
    fn interleave_only_right() {
        assert_eq!(interleave("", "z"), "z");
        assert_eq!(interleave("", "foobar"), "foobar");
    }

    #[test]
    fn interleave_same_length() {
        assert_eq!(interleave("abcdef", "012345"), "a0b1c2d3e4f5");
    }

    #[test]
    fn interleave_first_longer() {
        assert_eq!(
            interleave("Programming Rust", "O'Reilly"),
            "POr'oRgerialmlmying Rust"
        );
    }

    #[test]
    fn interleave_second_longer() {
        assert_eq!(
            interleave("ahoj, jak se máš?", "díky za optání, mám se dobře"),
            "adhíokjy,  zjaa ko psteá nmíá,š ?mám se dobře"
        );
    }
}
