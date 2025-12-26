/*
Assignment 07: Two-Sum

Instruction:
- Given a slice of integers and a target, implement `solution(nums, target)` returning an `Option<(usize, usize)>` with indices of two numbers that add up to target (like LeetCode Two Sum).

Requirements & guidance:
- Signature: `pub fn solution(nums: &[i32], target: i32) -> Option<(usize, usize)>`.
- Return `Some((i, j))` where `i < nums.len()` and `j < nums.len()` and `nums[i] + nums[j] == target`.
- You may assume exactly one solution exists for canonical tests, but returning any valid pair is acceptable.
- Hint: use a `HashMap` to record seen values -> index mapping for an O(n) solution.

Examples:
- `solution(&[2,7,11,15], 9) -> Some((0,1))`
*/

pub fn solution(nums: &[i32], target: i32) -> Option<(usize, usize)> {
  for (i, &num) in nums.iter().enumerate() {
    let complement = target - num;
    if let Some(j) = nums
      .iter()
      .position(|&x| x == complement && nums.iter().position(|&y| y == x).unwrap() != i)
    {
      return Some((i, j));
    }
  }
  None
}
