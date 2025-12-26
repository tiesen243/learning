include!("07_collections_iterators.rs");

#[cfg(test)]
mod tests {
  #[test]
  fn two_sum_cases() {
    assert_eq!(super::solution(&[2, 7, 11, 15], 9), Some((0, 1)));
    // other orderings are acceptable; check one canonical answer
  }
}
