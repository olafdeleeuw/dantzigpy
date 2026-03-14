# dantzigpy Agent Setup

## Project Overview

dantzigpy is a linear programming library with a Rust backend (`dantzigrs`) and Python interface (`dantzigpy`). The architecture mirrors Polars: pure Rust logic in `crates/`, Python bindings via PyO3 + maturin in `py-dantzigpy/`.

## Project Structure

```
dantzigpy/
├── Cargo.toml                       # Cargo workspace (resolver = "2")
├── crates/
│   └── dantzigrs/                   # Pure Rust LP solver crate
│       ├── Cargo.toml
│       └── src/lib.rs               # solve_lp() -> f64
└── py-dantzigpy/                    # maturin cdylib crate + Python package
    ├── Cargo.toml                   # crate-type = ["cdylib"]
    ├── pyproject.toml               # maturin build backend
    ├── src/lib.rs                   # #[pymodule] exposing solve_lp
    └── python/
        └── dantzigpy/
            └── __init__.py          # Public Python API
```

## Key Conventions

- All new Rust crates go under `crates/` and must be added to the workspace `members` in the root `Cargo.toml`.
- Shared dependency versions are declared once in `[workspace.dependencies]` in the root `Cargo.toml`. Sub-crates reference them with `{ workspace = true }`.
- The Python package is `dantzigpy`. The native Rust extension is `dantzigpy._dantzigpy` (built by maturin).
- Build the Python extension in development mode with `maturin develop` from `py-dantzigpy/`.
- Run Rust tests with `cargo test` from the project root.

## Build Commands

```bash
# Rust tests
cargo test

# Python development build (requires active venv with maturin installed)
cd py-dantzigpy
maturin develop

# Verify
python -c "from dantzigpy import solve_lp; print(solve_lp())"
```

## Agent Guidelines

When working on this project, load the appropriate skills:

- Use the **rust** skill when writing or reviewing Rust code in `crates/` or `py-dantzigpy/src/`
- Use the **python** skill when writing or reviewing Python code in `py-dantzigpy/python/`
- Use the **documentation** skill when updating README.md or writing doc comments
