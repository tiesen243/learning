/*
Assignment 04: Greatest Common Divisor

Instruction:
- Implement `solution(a, b)` returning the greatest common divisor (GCD) using the Euclidean algorithm.

Requirements & guidance:
- Signature: `pub fn solution(a: u64, b: u64) -> u64`.
- Handle zero values: `gcd(0, b) == b` and `gcd(a, 0) == a`.
- Use an iterative loop for clarity.

Examples:
- `solution(8, 12) -> 4`
- `solution(7, 3) -> 1`
*/

pub fn solution(mut a: u64, mut b: u64) -> u64 {
  while b != 0 {
    let temp = b;
    b = a % b;
    a = temp;
  }
  a
}
