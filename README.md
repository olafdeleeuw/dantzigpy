# dantzigpy

A linear programming library for Python with a high-performance Rust backend, inspired by the [Simplex method](https://en.wikipedia.org/wiki/Simplex_algorithm) developed by George Dantzig.

## Architecture

dantzigpy follows a Pydantic-style layout: the root is a Python workspace managed by [uv](https://docs.astral.sh/uv/), containing two projects — `dantzig_core` (the Rust extension, built with [maturin](https://www.maturin.rs)) and `dantzigpy` (the pure-Python public API, built with [hatchling](https://hatch.pypa.io)).

```
dantzigpy/
├── Cargo.toml                      # Cargo workspace root
├── pyproject.toml                  # Root Python package (dantzigpy) — hatchling build, uv workspace
├── uv.lock                         # uv workspace lock file
├── dantzigpy/                      # Pure-Python public API
│   └── __init__.py                 # Re-exports from dantzig_core
└── dantzig_core/                   # Rust + PyO3 extension (maturin build)
    ├── Cargo.toml                  # cdylib crate — workspace member
    ├── pyproject.toml              # maturin build config
    ├── python/
    │   └── dantzig_core/
    │       └── __init__.py         # Package stub
    └── src/
        ├── lib.rs                  # #[pymodule] — PyO3 entry point
        └── simplex/
            └── mod.rs              # pub fn solve_lp() and future Simplex logic
```

## Requirements

- [Rust](https://rustup.rs) (stable toolchain)
- Python 3.13+
- [uv](https://docs.astral.sh/uv/) (`curl -LsSf https://astral.sh/uv/install.sh | sh`)

## Development Setup

```bash
# Clone the repository
git clone https://github.com/olafdeleeuw/dantzigpy.git
cd dantzigpy

# Install everything (builds the Rust extension automatically)
uv sync
```

The compiled Rust extension (`dantzig_core._dantzig_core`) is installed as a
regular package into `.venv` — not into the source tree. There are no generated
files to commit or clean up manually.

If you ever need to force a rebuild of the Rust extension (e.g. after switching
branches or modifying `Cargo.toml`):

```bash
uv sync --reinstall-package dantzig-core
```

## Usage

```python
import dantzigpy

result = dantzigpy.solve_lp()
print(result)  # 42.0  (placeholder — real solver coming soon)
```

## Running Tests

```bash
# Rust unit tests
cargo test

# Python type checking
uv run ty check
```

## Building for Distribution

```bash
# Build a wheel for dantzig_core (Rust extension)
uv build dantzig_core/

# Build a wheel for dantzigpy (pure-Python package)
uv build
```

## Type Checking

Python types are checked with [ty](https://github.com/astral-sh/ty), an extremely fast Python type checker written in Rust. Configuration lives in `pyproject.toml` under `[tool.ty]`.

```bash
uv run ty check
```

The compiled Rust extension (`dantzig_core._dantzig_core`) is listed under `allowed-unresolved-imports` because it only exists after `uv sync` is run.

## Roadmap

- [ ] Implement the Simplex algorithm in `dantzig_core/src/simplex/`
- [ ] Accept objective function and constraint inputs
- [ ] Return structured solution objects
- [ ] Python type stubs (`.pyi`)
- [ ] Documentation site

## License

MIT
