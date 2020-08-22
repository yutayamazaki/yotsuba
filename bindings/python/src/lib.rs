use pyo3::prelude::*;
use pyo3::{wrap_pyfunction, wrap_pymodule};
use pyo3::exceptions::{ValueError};
use yotsuba as yotsubars;

#[pyfunction]
fn normalize_neologd(text: &str) -> PyResult<String> {
    let ret = yotsubars::ja::normalize(text);
    Ok(ret)
}

#[pyfunction]
fn pad_sequence(sequence: Vec<i32>, maxlen: usize, value: Option<i32>, padding: Option<&str>) -> PyResult<Vec<i32>> {
    let ret = yotsubars::utils::pad_sequence(&sequence, maxlen, value, padding);
    // Ok(ret)
    match ret {
        Ok(v) => return Ok(v),
        Err(e) => return Err(PyErr::new::<ValueError, String>(e.to_string())),
    };
}

#[pyfunction]
fn pad_sequences(sequences: Vec<Vec<i32>>, maxlen: Option<usize>, value: Option<i32>, padding: Option<&str>) -> PyResult<Vec<Vec<i32>>> {
    let ret = yotsubars::utils::pad_sequences(&sequences, maxlen, value, padding);
    match ret {
        Ok(v) => Ok(v),
        Err(e) => Err(PyErr::new::<ValueError, String>(e.to_string())),
    }
}

#[pyfunction]
fn remove_stopwords(tokens: Vec<&str>, stopwords: Vec<&str>) -> Vec<String> {
    yotsubars::utils::remove_stopwords(&tokens, &stopwords)
}

#[pyfunction]
fn get_stopwords(lang: &str) -> PyResult<Vec<&str>> {
    let ret = yotsubars::utils::get_stopwords(lang);
    match ret {
        Ok(v) => Ok(v),
        Err(e) => Err(PyErr::new::<ValueError, String>(e.to_string())),
    }
}

#[pymodule]
fn ja(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(normalize_neologd))?;

    Ok(())
}

#[pymodule]
fn yotsuba(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(ja))?;

    m.add_wrapped(wrap_pyfunction!(pad_sequence))?;
    m.add_wrapped(wrap_pyfunction!(pad_sequences))?;
    m.add_wrapped(wrap_pyfunction!(remove_stopwords))?;
    m.add_wrapped(wrap_pyfunction!(get_stopwords))?;

    Ok(())
}
