use pyo3::prelude::*;

mod simplex;

/// Solve a linear program.
///
/// Placeholder implementation — returns a fixed float value.
/// A real Simplex solver will be implemented in a future version.
#[pyfunction]
fn solve_lp() -> f64 {
    simplex::solve_lp()
}

#[pymodule]
#[pyo3(name = "_dantzig_core")]
fn dantzig_core_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(solve_lp, m)?)?;
    Ok(())
}
