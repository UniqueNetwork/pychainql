use pyo3::{exceptions::PyValueError, prelude::*};

/// A Python module implemented in Rust.
#[pymodule]
fn pychainql(m: &Bound<'_, PyModule>) -> PyResult<()> {
    register_child_module(m)?;
    Ok(())
}

fn register_child_module(parent: &Bound<'_, PyModule>) -> PyResult<()> {
    let hex_module = PyModule::new_bound(parent.py(), "hex")?;
    hex_module.add_function(wrap_pyfunction!(from_hex, &hex_module)?)?;
    hex_module.add_function(wrap_pyfunction!(to_hex, &hex_module)?)?;

    parent.add_submodule(&hex_module)?;

    Ok(())
}

/// Convert a hex string to a vector of bytes.
#[pyfunction]
fn from_hex(data: &str) -> PyResult<Vec<u8>> {
    chainql_core::hex::from_hex(data).map_err(|err| PyValueError::new_err(err.to_string()))
}

/// Convert an array of bytes to a hex string.
#[pyfunction]
fn to_hex(data: &[u8]) -> String {
    chainql_core::hex::to_hex(data)
}
