
# Rust exercises

This folder contains Rust learning exercises (LeetCode-style). Each exercise is provided as two files:

- `NN.name.rs` — the assignment file containing the function(s) or macro to implement (currently many are `unimplemented!()` stubs).
- `NN.name.test.rs` — a lightweight test file that `include!`s the corresponding assignment file and contains basic assertions.

Quick notes
- Tests include concrete assertions; many will fail until you implement the stubs in the `NN.name.rs` files.
- The async assignment test is marked `#[ignore]` because it requires an async executor (e.g., `tokio`).

Run a single test file (compile with `rustc` and run):

```bash
rustc --test src/rust/01.variables.test.rs -o tmp_test && ./tmp_test
rm -f tmp_test
```

Run all tests under `src/rust` manually (quick way):

```bash
for t in src/rust/*.test.rs; do
	echo "Running $t";
	rustc --test "$t" -o tmp_test && ./tmp_test || true;
	rm -f tmp_test;
done
```

Make tests `cargo test`-friendly
- Option A (recommended): Move the test files into the crate-level `tests/` directory (each file becomes an integration test). Example:

	1. Create a `tests/` folder at project root.
	2. Copy `src/rust/01.variables.test.rs` -> `tests/01_variables.rs` and update the `include!` path if necessary (e.g., `include!("../src/rust/01.variables.rs");`).
	3. Run `cargo test`.

- Option B: Convert these exercises into modules and expose them from `src/lib.rs` or `src/main.rs` so `cargo test` finds unit tests.

Testing tips
- To work on an exercise without failing the whole suite, either run its test file directly (using `rustc`), or move its test to `tests/` and run `cargo test --test 01_variables`.
- To run ignored tests (e.g., async): `cargo test -- --ignored` or `cargo test --test <name> -- --ignored`.

How files are structured
- Most assignment files include a detailed comment block with:
	- the problem statement
	- function signature to implement
	- hints and examples

If you'd like, I can:
- convert all test files into `tests/` so `cargo test` works out of the box, or
- make tests less strict (commented examples) until you implement solutions.

