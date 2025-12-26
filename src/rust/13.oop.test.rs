include!("13.oop.rs");

#[cfg(test)]
mod tests {
    #[test]
    fn oop_structs_exist() {
        // This will compile but panic at runtime until you implement the constructors.
        let _ = super::Dog::new("Rex");
        let _ = super::Cat::new("Mittens");
    }
}
