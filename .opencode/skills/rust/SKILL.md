---
name: rust
description: Write idiomatic, safe Rust code for the dantzigrs crate and PyO3 bindings in py-dantzigpy
license: MIT
compatibility: opencode
---

## What I do

- Write and review Rust code in `crates/dantzigrs/` and `py-dantzigpy/src/`
- Follow Rust idioms: ownership, borrowing, iterators, error handling with `Result`
- Write PyO3 bindings (`#[pyfunction]`, `#[pymodule]`, `#[pyclass]`) correctly
- Manage the Cargo workspace: adding crates, declaring workspace dependencies, feature flags
- Write `#[cfg(test)]` unit tests using the standard `assert_eq!` / `assert!` macros
- Ensure the `cdylib` crate in `py-dantzigpy` correctly re-exports from `dantzigrs`

## When to use me

Use this skill when:
- Implementing LP solver logic in `crates/dantzigrs/src/lib.rs`
- Adding or modifying PyO3 bindings in `py-dantzigpy/src/lib.rs`
- Adding a new crate to `crates/` and wiring it into the workspace
- Reviewing Rust code for correctness, safety, or performance
- Debugging Cargo build or maturin compilation errors

## Key patterns for this project

- New Rust crates go in `crates/<name>/` and are added to workspace `members`
- All external dependency versions are declared in `[workspace.dependencies]` in the root `Cargo.toml`
- Sub-crates use `{ workspace = true }` to inherit versions
- PyO3 functions must return `PyResult<T>` or a plain type that PyO3 can convert automatically
- The `extension-module` feature must be enabled on `pyo3` only in the `cdylib` crate
- Run tests with `cargo test` from the workspace root
