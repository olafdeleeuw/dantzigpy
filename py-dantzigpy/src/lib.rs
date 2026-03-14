use pyo3::prelude::*;

/// Solve a linear program.
///
/// Placeholder implementation — returns a fixed float value.
/// A real LP solver will be implemented in a future version.
#[pyfunction]
fn solve_lp() -> f64 {
    dantzigrs::solve_lp()
}

#[pymodule]
#[pyo3(name = "_dantzigpy")]
fn dantzigpy_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(solve_lp, m)?)?;
    Ok(())
}
