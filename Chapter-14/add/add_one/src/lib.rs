/// # Example
/// ```rust
/// use add_one::add_one;
/// let num = 41;
/// assert_eq!(42, add_one(num));
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_one() {
        assert_eq!(42, add_one(41));
    }
}