//! # Matt's Lib
//!
//! `test_comments` is a collection of utilities to make performing certain
//! calculations more convenient.

pub mod art;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// use test_comments::add_one;
///
/// let arg = 5;
/// let answer = add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
