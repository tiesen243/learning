include!("03_control_flow.rs");

#[cfg(test)]
mod tests {
    #[test]
    fn fizzbuzz_cases() {
        assert_eq!(super::solution(3), "Fizz");
        assert_eq!(super::solution(5), "Buzz");
        assert_eq!(super::solution(15), "FizzBuzz");
        assert_eq!(super::solution(7), "7");
    }
}
