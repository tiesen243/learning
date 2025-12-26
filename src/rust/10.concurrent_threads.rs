/*
Assignment 10: Parallel Sum

Instruction:
- Implement `solution(nums)` that computes the sum of a slice of integers using multiple threads (split work and combine) and returns `i64`.

Requirements & guidance:
- Signature: `pub fn solution(nums: &[i64]) -> i64`.
- Split `nums` into roughly equal chunks, spawn threads to sum each chunk, then join and combine results.
- Use `std::thread::spawn` and `join`; you may use `Arc`/`Mutex` or have each thread return its partial sum and combine in the main thread.
- For small slices a single-threaded sum is fine; tests expect correct result, not specific threading strategy.

Examples:
- `solution(&[1,2,3,4]) -> 10`
*/

pub fn solution(nums: &[i64]) -> i64 {
	unimplemented!()
}
