include!("09.generics_traits.rs");

#[cfg(test)]
mod tests {
    #[test]
    fn generic_max_cases() {
        assert_eq!(super::solution(&[3, 1, 2]), Some(3));
        assert_eq!(super::solution::<i32>(&[]), None);
    }
}
