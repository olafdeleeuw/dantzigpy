# dantzigpy Agent Setup

## Project Overview

dantzigpy is a linear programming library with a Rust backend and Python interface. The architecture mirrors Pydantic: a uv workspace at the root, with `dantzig_core` (Rust + PyO3, built with maturin) as a sub-project and `dantzigpy` (pure-Python public API, built with hatchling) as the root package.

## Project Structure

```
dantzigpy/
├── Cargo.toml                       # Cargo workspace (resolver = "2")
├── pyproject.toml                   # Root package (dantzigpy) — hatchling build, uv workspace
├── uv.lock                          # uv workspace lock file
├── dantzigpy/                       # Pure-Python public API
│   └── __init__.py                  # Re-exports from dantzig_core
└── dantzig_core/                    # Rust + PyO3 extension (maturin build)
    ├── Cargo.toml                   # crate-type = ["cdylib"] — workspace member
    ├── pyproject.toml               # maturin build backend
    ├── python/
    │   └── dantzig_core/
    │       └── __init__.py          # Package stub (compiled .so lands here)
    └── src/
        ├── lib.rs                   # #[pymodule] — PyO3 entry point
        └── simplex/
            └── mod.rs               # pub fn solve_lp() and future Simplex logic
```

## Key Conventions

- `dantzig_core` is the only Cargo workspace member. Its `Cargo.toml` is under `dantzig_core/`.
- Shared dependency versions are declared once in `[workspace.dependencies]` in the root `Cargo.toml`. Sub-crates reference them with `{ workspace = true }`.
- The public Python package is `dantzigpy`. The native Rust extension is `dantzig_core._dantzig_core` (built by maturin).
- LP solver logic lives in `dantzig_core/src/simplex/mod.rs`. The PyO3 bindings layer is `dantzig_core/src/lib.rs`.
- Install everything (including building the Rust extension) with `uv sync` from the project root.
- Run Rust tests with `cargo test` from the project root.

## Build Commands

```bash
# Install everything and build the Rust extension
uv sync

# Rust tests
cargo test

# Python type checking
uv run ty check

# Verify end-to-end
uv run python -c "from dantzigpy import solve_lp; print(solve_lp())"
```

## Agent Guidelines

When working on this project, load the appropriate skills:

- Use the **rust** skill when writing or reviewing Rust code in `dantzig_core/src/`
- Use the **python** skill when writing or reviewing Python code in `dantzigpy/` or `dantzig_core/python/`
- Use the **documentation** skill when updating README.md or writing doc comments
