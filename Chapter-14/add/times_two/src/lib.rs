/// # Example
/// ```rust
/// use times_two::times_two;
/// let num = 21;
/// assert_eq!(42, times_two(num));
/// ```
pub fn times_two(x: i32) -> i32 {
    x * 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_times_two() {
        let num = 21;
        assert_eq!(42, times_two(num));
    }
}