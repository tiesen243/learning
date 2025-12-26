include!("12.macros_and_metaprogramming.rs");

#[cfg(test)]
mod tests {
    #[test]
    fn macro_generates_function() {
        // This uses the `solution!` macro to generate a function `five`.
        solution!(five, 5);
        assert_eq!(five(), 5);
    }
}
