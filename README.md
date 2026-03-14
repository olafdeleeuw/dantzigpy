# dantzigpy

A linear programming library for Python with a high-performance Rust backend, inspired by the [Simplex method](https://en.wikipedia.org/wiki/Simplex_algorithm) developed by George Dantzig.

## Architecture

dantzigpy follows the following pattern: the core algorithm lives in a Rust workspace crate and is exposed to Python via [PyO3](https://pyo3.rs) and [maturin](https://www.maturin.rs).

```
dantzigpy/
├── Cargo.toml                  # Cargo workspace root
├── crates/
│   └── dantzigrs/              # Pure Rust core crate (no Python dependency)
│       └── src/lib.rs          # solve_lp() and future LP solver logic
└── py-dantzigpy/               # Python package + PyO3 bindings (cdylib)
    ├── Cargo.toml
    ├── pyproject.toml          # maturin build config
    ├── src/lib.rs              # #[pymodule] wrapping dantzigrs
    └── python/
        └── dantzigpy/
            └── __init__.py     # Public Python API
```

## Requirements

- [Rust](https://rustup.rs) (stable toolchain)
- Python 3.13+
- [maturin](https://www.maturin.rs) (`pip install maturin`)

## Development Setup

```bash
# Clone the repository
git clone https://github.com/olafdeleeuw/dantzigpy.git
cd dantzigpy/py-dantzigpy

# Create and activate a virtual environment
python -m venv .venv
source .venv/bin/activate      # Linux / macOS
# .venv\Scripts\activate       # Windows

# Install maturin and build the Rust extension in development mode
pip install maturin
maturin develop
```

## Usage

```python
import dantzigpy

result = dantzigpy.solve_lp()
print(result)  # 42.0  (placeholder — real solver coming soon)
```

## Building for Distribution

```bash
# Build a wheel
maturin build --release

# The wheel will be in py-dantzigpy/target/wheels/
```

## Working with the Rust crate directly

The `dantzigrs` crate can be used as a standalone Rust library, independent of the Python bindings:

```toml
[dependencies]
dantzigrs = { path = "crates/dantzigrs" }
```

```rust
let result = dantzigrs::solve_lp();
println!("{}", result);  // 42.0
```

Run the Rust tests:

```bash
cargo test
```

## Type Checking

Python types are checked with [ty](https://github.com/astral-sh/ty), an extremely fast type checker written in Rust (by Astral, the creators of uv and Ruff).

```bash
# Run from py-dantzigpy/ (no install required — uvx fetches ty on demand)
cd py-dantzigpy
uvx ty check
```

Configuration lives in `py-dantzigpy/pyproject.toml` under `[tool.ty]`. The compiled Rust extension (`dantzigpy._dantzigpy`) is listed under `allowed-unresolved-imports` because it only exists after `maturin develop` is run.

## Roadmap

- [ ] Implement the Simplex algorithm in `dantzigrs`
- [ ] Accept objective function and constraint inputs
- [ ] Return structured solution objects
- [ ] Python type stubs (`.pyi`)
- [ ] Documentation site

## License

MIT
