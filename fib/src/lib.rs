use pyo3::prelude::*;

fn fib_fn(num: usize) -> usize {
    if num == 0 || num == 1 {
        return num;
    }
    fib_fn(num - 1) + fib_fn(num - 2)
}
    
#[pyfunction]
pub fn fibonacci(num: usize) -> PyResult<usize> {
    Ok(fib_fn(num))
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
pub fn fib(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(fibonacci, m)?)?;
    Ok(())
}
