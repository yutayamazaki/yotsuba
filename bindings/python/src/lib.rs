use pyo3::prelude::*;
use pyo3::{wrap_pyfunction, wrap_pymodule};
use yotsuba as yotsubars;

#[pyfunction]
fn normalize_neologd(text: &str) -> PyResult<String> {
    let ret = yotsubars::ja::normalize(text);
    Ok(ret)
}

#[pyfunction]
fn pad_sequence(sequence: Vec<i32>, maxlen: usize, value: i32) -> PyResult<Vec<i32>> {
    let ret = yotsubars::utils::pad_sequence(&sequence, maxlen, value);
    Ok(ret)
}

#[pyfunction]
fn pad_sequences(sequences: Vec<Vec<i32>>, maxlen: usize, value: i32) -> PyResult<Vec<Vec<i32>>> {
    let ret = yotsubars::utils::pad_sequences(&sequences, maxlen, value);
    Ok(ret)
}

#[pymodule]
fn ja(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(normalize_neologd))?;

    Ok(())
}

#[pymodule]
fn utils(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(pad_sequence))?;
    m.add_wrapped(wrap_pyfunction!(pad_sequences))?;

    Ok(())
}

#[pymodule]
fn yotsuba(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(ja))?;
    m.add_wrapped(wrap_pymodule!(utils))?;

    Ok(())
}
