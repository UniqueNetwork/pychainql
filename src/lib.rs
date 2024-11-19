mod address;

use pyo3::{exceptions::PyValueError, prelude::*};

#[pymodule]
mod chainql {
    use super::*;

    #[pymodule]
    mod hex {
        use super::*;
        use chainql_core::hex;

        /// Convert a hex string to a vector of bytes.
        #[pyfunction]
        fn from_hex(data: &str) -> PyResult<Vec<u8>> {
            hex::from_hex(data).map_err(to_value_error)
        }

        /// Convert an array of bytes to a hex string.
        #[pyfunction]
        fn to_hex(data: &[u8]) -> String {
            hex::to_hex(data)
        }
    }
}

#[inline(always)]
pub(crate) fn to_value_error(err: impl std::error::Error) -> PyErr {
    let human_err = err.to_string();

    if let Some(human_err) = human_err.strip_prefix("runtime error: ") {
        PyValueError::new_err(human_err.to_owned())
    } else {
        PyValueError::new_err(human_err)
    }
}
