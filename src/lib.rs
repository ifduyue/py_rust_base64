use pyo3::{
    exceptions::PyValueError,
    prelude::*,
    types::{PyBytes, PyString},
};

#[pyfunction]
fn encode(input: &PyBytes) -> PyResult<String> {
    Ok(base64::encode(input.as_bytes()).to_string())
}

#[pyfunction]
fn decode<'a>(py: Python<'a>, input: &PyString) -> PyResult<&'a PyBytes> {
    match base64::decode(input.to_string()) {
        Ok(bytes) => Ok(PyBytes::new(py, &bytes[..])),
        Err(_err) => Err(PyValueError::new_err("failed to decode")),
    }
}


#[pymodule]
fn rust_base64(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(encode, m)?)?;
    m.add_function(wrap_pyfunction!(decode, m)?)?;
    Ok(())
}
