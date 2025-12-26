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
for t in src/rust/*_test.rs; do
	echo "Running $t";
	rustc --test "$t" -o tmp_test && ./tmp_test || true;
	rm -f tmp_test;
done
```
