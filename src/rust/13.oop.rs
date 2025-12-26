/*
Assignment 13: Object-Oriented Design in Rust

Instruction:
- Define a trait `Animal` with methods `name(&self) -> &str` and `speak(&self) -> String`.
- Define structs `Dog` and `Cat` with a `name: String` field and `new(name: &str) -> Self` constructors.
- Implement `Animal` for both `Dog` and `Cat`.
- Implement `pub fn solution()` that returns a `Vec<Box<dyn Animal>>` containing some animals (constructor usage).

Requirements & guidance:
- Use trait objects (`Box<dyn Animal>`) to demonstrate polymorphism.
- Keep ownership rules clear: `new` should take `&str` and store `String`.
- Do not print from `solution`; return the collection so tests can inspect it.
- Leave implementations for you to complete (functions are `unimplemented!()` stubs).

Examples:
- After implementing, `let animals = solution(); assert_eq!(animals[0].speak(), "woof");`
*/

pub trait Animal {
    fn name(&self) -> &str;
    fn speak(&self) -> String;
}

pub struct Dog {
    pub name: String,
}

impl Dog {
    pub fn new(name: &str) -> Self {
        unimplemented!()
    }
}

impl Animal for Dog {
    fn name(&self) -> &str {
        unimplemented!()
    }
    fn speak(&self) -> String {
        unimplemented!()
    }
}

pub struct Cat {
    pub name: String,
}

impl Cat {
    pub fn new(name: &str) -> Self {
        unimplemented!()
    }
}

impl Animal for Cat {
    fn name(&self) -> &str {
        unimplemented!()
    }
    fn speak(&self) -> String {
        unimplemented!()
    }
}

pub fn solution() -> Vec<Box<dyn Animal>> {
    unimplemented!()
}
