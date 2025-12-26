include!("11.async_await.rs");

#[cfg(test)]
mod tests {
    // Async tests require an executor; mark ignored until you run with an executor.
    #[test]
    #[ignore]
    fn async_fetch_example() {
        // Example (requires executor):
        // let res = futures::executor::block_on(super::solution(1));
        // assert_eq!(res, "result for 1");
    }
}
