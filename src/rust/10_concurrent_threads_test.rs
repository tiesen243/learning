include!("10_concurrent_threads.rs");

#[cfg(test)]
mod tests {
    #[test]
    fn parallel_sum_cases() {
        let nums = [1i64, 2, 3, 4];
        assert_eq!(super::solution(&nums), 10);
    }
}
