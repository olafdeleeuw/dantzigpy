---
name: python
description: Write idiomatic Python code for the dantzigpy public API, including the __init__.py interface and any future pure-Python modules
license: MIT
compatibility: opencode
---

## What I do

- Write and review Python code in `py-dantzigpy/python/dantzigpy/`
- Maintain `__init__.py` as the public API surface (re-exports from the native `_dantzigpy` module)
- Write Python type stubs (`.pyi` files) so IDEs can provide autocomplete for Rust-backed functions
- Manage `pyproject.toml` for maturin configuration and project metadata
- Write Python tests (pytest) that exercise the public `dantzigpy` API

## When to use me

Use this skill when:
- Adding new Python-level functions or classes that wrap Rust functionality
- Updating `__init__.py` to expose new symbols from the compiled extension
- Writing or updating `.pyi` type stubs
- Configuring `pyproject.toml` (dependencies, maturin settings, build options)
- Writing pytest tests for the Python API

## Key patterns for this project

- The compiled Rust extension is imported as `from dantzigpy._dantzigpy import <symbol>`
- `__init__.py` re-exports all public symbols and sets `__all__`
- The LSP "cannot resolve import" error for `dantzigpy._dantzigpy` is expected before `maturin develop` is run — it is not a real error
- Build the extension with `maturin develop` from `py-dantzigpy/` inside an active virtualenv
- Python 3.10+ is the minimum supported version (matches the `abi3-py310` PyO3 feature)
