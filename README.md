# Learning

This repository is a hands-on learning workspace for Rust, Python, and TypeScript. It contains small exercises, examples, and tests you can implement for practice.

- `src/rust/` — Rust exercises.
- `src/python/` — Python exercises.
- `src/typescript/` — TypeScript exercises.

## Quick workflow

- Open an exercise in `src/<lang>/` and implement the placeholder.
- Run the exercise's test harness or script to verify your solution.

### Rust: run & test

Run a single Rust exercise test file (compile and run with `rustc`):

```bash
rustc --test src/rust/01.variables.test.rs -o tmp_test && ./tmp_test
rm -f tmp_test
```

Quick-run many Rust test harnesses (each compiles separately):

```bash
for t in src/rust/*.test.rs; do
	echo "Running $t";
	rustc --test "$t" -o tmp_test && ./tmp_test || true;
	rm -f tmp_test;
done
```

Make tests `cargo test`-friendly

- Option A (recommended): move test files into a top-level `tests/` directory and update `include!` paths to reference `../src/rust/NN.name.rs`; then run `cargo test`.
- Option B: expose exercises as modules in `src/lib.rs` so unit tests run under `cargo test`.

Notes

- Many exercises are intentionally left with `unimplemented!()` to give you practice implementing them.
- The async exercise test is marked `#[ignore]` because it needs an async executor (e.g., `tokio`). Use `cargo test -- --ignored` after wiring for cargo.

### Python: setup & run

Create and activate a virtual environment, then run examples or tests:

```bash
uv sync
source .venv/bin/activate
python src/python/some_exercise.py
```

### TypeScript: setup & run

Install Node dependencies and run TypeScript files (or compile first):

```bash
bun install
bun src/typescript/some_exercise.ts
```

## License

This repository is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
