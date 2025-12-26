/*
Assignment 05: First Word Slice

Instruction:
- Implement `solution(s)` that returns a slice (`&str`) of the first word in the string.

Requirements & guidance:
- Signature: `pub fn solution<'a>(s: &'a str) -> &'a str`.
- Do not allocate a new String; return a slice into the input string.
- Words are separated by ASCII space `' '`. If the string starts with a space, return an empty slice.
- Hints: use `bytes()`/`chars()` with `enumerate()` and return `&s[..i]` where appropriate.

Examples:
- `solution("hello world") -> "hello"`
- `solution("rust") -> "rust"`
*/

pub fn solution<'a>(s: &'a str) -> &'a str {
  str::from_utf8(&s.as_bytes()[..s.bytes().position(|b| b == b' ').unwrap_or(s.len())]).unwrap()
}
