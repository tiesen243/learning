include!("05.ownership_borrowing.rs");

#[cfg(test)]
mod tests {
    #[test]
    fn first_word_cases() {
        assert_eq!(super::solution("hello world"), "hello");
        assert_eq!(super::solution("rust"), "rust");
        assert_eq!(super::solution(" leading"), "");
    }
}
