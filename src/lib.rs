use pyo3::prelude::*;

/// Prints a message.
#[pyfunction]
fn hello() -> PyResult<String> {
    Ok("Hello from xml-structure!".into())
}
#[pyfunction]
fn preprocess_sfs_json(source: String) -> PyResult<Vec<u8>> {
    Ok("<dokument></dokument>".into())
}

/// A Python module implemented in Rust.
#[pymodule]
fn _lowlevel(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(preprocess_sfs_json, m)?)?;
    m.add_function(wrap_pyfunction!(hello, m)?)?;
    Ok(())
}
