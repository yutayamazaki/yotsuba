use pyo3::prelude::*;
use pyo3::{wrap_pyfunction};
use neologd::normalize;

#[pyfunction]
fn neologd_normalize(text: &str) -> PyResult<String> {
    let ret = neologd::normalize(text);
    Ok(ret)
}

// #[pyfunction]
// fn pad_sequence(text: String) -> PyResult<String> {
//     let ret = neologd::normalize(&text);
//     Ok(ret)
// }

#[pymodule]
fn yotsuba(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(neologd_normalize))?;

    Ok(())
}
