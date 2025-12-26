include!("06_structs_enums.rs");

#[cfg(test)]
mod tests {
  #[test]
  fn distance_cases() {
    let d = super::solution(0.0, 0.0, 3.0, 4.0);
    assert!((d - 5.0).abs() < 1e-6);
    let d2 = super::solution(-1.0, -1.0, 2.0, 3.0);
    assert!((d2 - 5.0).abs() < 1e-6);
  }
}
