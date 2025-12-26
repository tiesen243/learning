/*
Assignment 09: Generic Max

Instruction:
- Implement `solution<T: Ord + Copy>(slice: &[T]) -> Option<T>` that returns the maximum element or `None` for empty slices.

Requirements & guidance:
- Signature: `pub fn solution<T: Ord + Copy>(slice: &[T]) -> Option<T>`.
- Iterate over the slice and keep track of the current max.
- Complexity: O(n) time, O(1) extra space.

Examples:
- `solution(&[3,1,2]) -> Some(3)`
- `solution::<i32>(&[]) -> None`
*/

pub fn solution<T: Ord + Copy>(slice: &[T]) -> Option<T> {
  unimplemented!()
}
