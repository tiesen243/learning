/*
Assignment 12: Simple Macro

Instruction:
- Implement a declarative macro `solution!` that creates a function returning the given literal.

Requirements & guidance:
- Macro usage example: `solution!(five, 5)` should expand to `fn five() -> i32 { 5 }`.
- Keep the macro simple; support an identifier and an expression literal.
- Tests include macro usage; ensure the generated function is callable in the same module.

Examples:
- `solution!(ten, 10); assert_eq!(ten(), 10);`
*/

#[macro_export]
macro_rules! solution {
  ($name:ident, $val:expr) => {
    fn $name() -> i32 {
      unimplemented!()
    }
  };
}
