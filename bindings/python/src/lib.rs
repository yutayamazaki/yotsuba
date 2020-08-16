use pyo3::prelude::*;
use pyo3::{wrap_pyfunction};
use neologd::normalize;

#[pyfunction]
fn neologd_normalize(text: String) -> PyResult<String> {
    let ret = neologd::normalize(&text);
    Ok(ret)
}

#[pymodule]
fn neologd(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(neologd_normalize))?;

    Ok(())
}
