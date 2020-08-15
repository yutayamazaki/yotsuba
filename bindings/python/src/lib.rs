use pyo3::prelude::*;
use pyo3::{wrap_pyfunction};
use neologd::normalize;


// This is the test function in Rust, getting prime number, which is called by python as get_prime_numbers
#[pyfunction]
fn get_prime_numbers() -> PyResult<String> {
    Ok("Hello".to_string())
}

#[pyfunction]
fn neologd_normalize(text: String) -> PyResult<String> {
    let ret = neologd::normalize(&text);
    Ok(ret)
}

#[pymodule]
fn neologd(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(get_prime_numbers))?;
    m.add_wrapped(wrap_pyfunction!(neologd_normalize))?;

    Ok(())
}
