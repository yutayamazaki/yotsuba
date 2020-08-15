use pyo3::prelude::*;
use pyo3::{wrap_pyfunction};


// This is the test function in Rust, getting prime number, which is called by python as get_prime_numbers
#[pyfunction]
fn get_prime_numbers() -> PyResult<String> {
    Ok("Hello".to_string())
}

#[pymodule]
fn neologd(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(get_prime_numbers))?;

    Ok(())
}
