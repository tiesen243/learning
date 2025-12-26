include!("01_variables.rs");

#[cfg(test)]
mod tests {
    #[test]
    fn swaps_correctly() {
        assert_eq!(super::solution(1, 2), (2, 1));
        assert_eq!(super::solution(-5, 0), (0, -5));
        assert_eq!(super::solution(42, 42), (42, 42));
    }
}
