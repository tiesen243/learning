include!("13_oop.rs");

#[cfg(test)]
mod tests {
  #[test]
  fn oop_structs_exist() {
    let mut animals = super::solution();
    let dog = animals.remove(0);
    let cat = animals.remove(0);

    assert_eq!(dog.name(), "Rex");
    assert_eq!(cat.name(), "Mittens");

    assert_eq!(dog.speak(), "woof");
    assert_eq!(cat.speak(), "meow");
  }
}
