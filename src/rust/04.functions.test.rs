include!("04.functions.rs");

#[cfg(test)]
mod tests {
    #[test]
    fn gcd_cases() {
        assert_eq!(super::solution(8, 12), 4);
        assert_eq!(super::solution(100, 10), 10);
        assert_eq!(super::solution(7, 3), 1);
    }
}
