/*
Assignment 03: FizzBuzz Single Value

Instruction:
- Implement `solution(n)` returning:
  - "FizzBuzz" if divisible by 15
  - "Fizz" if divisible by 3
  - "Buzz" if divisible by 5
  - otherwise the number as a string

Requirements & guidance:
- Signature: `pub fn solution(n: i32) -> String`.
- Use modulus operations; return the textual result (not printing).

Examples:
- `solution(3) -> "Fizz"`
- `solution(5) -> "Buzz"`
- `solution(15) -> "FizzBuzz"`
*/

pub fn solution(n: i32) -> String {
  if n % 15 == 0 {
    String::from("FizzBuzz")
  } else if n % 3 == 0 {
    String::from("Fizz")
  } else if n % 5 == 0 {
    String::from("Buzz")
  } else {
    n.to_string()
  }
}
