# Contributing to PohLang

Thanks for your interest in contributing! This guide explains how to add new language features—especially phrasal built-ins—and how to keep the repo healthy.

## Workflow overview

- Code lives in `runtime/` (Rust). Language syntax → AST → evaluation lives here.
- Documentation lives in `doc/` and `spec/`.
- Examples live under `examples/`.
- Tests live under `runtime/tests/`.

## Adding a new phrasal built-in

End-to-end pipeline:

1) AST
- Add a variant to `runtime/src/parser/ast.rs` under `Expr` (e.g. `Expr::FooOf(Box<Expr>)` or `Expr::FooWith(Box<Expr>, Box<Expr>)`).

2) Parser
- Centralize phrases in `runtime/src/parser/phrases.rs`. Add phrase constants and (if needed) binary separators.
- In `runtime/src/parser/parser.rs`, reference the constants and parse into your new AST variant.
- Keep parsing case-insensitive, use `phrases::strip_prefix_ci`.

3) VM
- Wire evaluation in `runtime/src/vm/vm.rs` in all four contexts:
  - `eval`
  - `eval_in_frame`
  - `eval_in_scope`
  - `eval_in_scope_with_capture`
- Delegate to a builtin if one already exists (e.g., `builtin_join`), or add a new builtin there.
- Update `dump_expr` so pretty-printing shows a friendly phrasal form.

4) Tests
- Add integration tests to `runtime/tests/`. Follow `functions.rs` style: use the `run([...])` helper to write short programs.
- Cover the happy path and one error case when reasonable.

5) Docs
- Update `spec/Vocabulary.md` and `doc/PohLang_Guide.md` with your new phrasal expression, examples, and any aliases.

6) Build and run tests
- Use `cargo test --manifest-path runtime/Cargo.toml` to validate.

## Coding style
- Keep parsing ASCII case-insensitive for phrases.
- Prefer small, focused changes. Avoid reformatting unrelated code.
- Use explicit, friendly error messages.

## Pull requests
- Include a brief summary of the feature and screenshots of example output when helpful.
- Ensure CI is green (build + tests).
