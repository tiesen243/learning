include!("02_data_types.rs");

#[cfg(test)]
mod tests {
  #[test]
  fn sums_mixed_types() {
    assert_eq!(super::solution(2, 1.5), 3.5);
    assert_eq!(super::solution(-1, 2.0), 1.0);
  }
}
