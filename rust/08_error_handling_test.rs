include!("08_error_handling.rs");

#[cfg(test)]
mod tests {
  #[test]
  fn parse_ok_and_err() {
    assert_eq!(super::solution("1,2,3"), Ok(vec![1, 2, 3]));
    assert!(super::solution("1,a").is_err());
  }
}
