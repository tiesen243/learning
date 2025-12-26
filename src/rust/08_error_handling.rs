/*
Assignment 08: Parse Integer List

Instruction:
- Implement `solution(s)` that parses a comma-separated list of integers into `Result<Vec<i32>, String>`.

Requirements & guidance:
- Signature: `pub fn solution(s: &str) -> Result<Vec<i32>, String>`.
- Trim whitespace around numbers; treat empty input as an empty vector or return an error per your choice (tests expect normal CSV).
- For invalid tokens return `Err(String)` with a short error message.
- Hints: use `s.split(',').map(str::trim).map(str::parse::<i32>())` and collect with `collect::<Result<Vec<_>, _>>()`.

Examples:
- `solution("1,2,3") -> Ok(vec![1,2,3])`
- `solution("1, a") -> Err(...)`
*/

pub fn solution(s: &str) -> Result<Vec<i32>, String> {
  unimplemented!()
}
