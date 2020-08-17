use pyo3::prelude::*;
use pyo3::{wrap_pyfunction};
use neologd;

#[pyfunction]
fn normalize_neologd(text: &str) -> PyResult<String> {
    let ret = neologd::normalize(text);
    Ok(ret)
}

#[pyfunction]
fn pad_sequence(sequence: Vec<i32>, maxlen: i32, value: i32) -> PyResult<Vec<i32>> {
    let ret = neologd::pad_sequence(&sequence, maxlen, value);
    Ok(ret)
}

#[pymodule]
fn yotsuba(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(normalize_neologd))?;
    m.add_wrapped(wrap_pyfunction!(pad_sequence))?;

    Ok(())
}
