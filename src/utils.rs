use pyo3::{exceptions::PyValueError, prelude::*};

#[inline(always)]
pub(crate) fn to_value_error(err: impl std::error::Error) -> PyErr {
    let human_err = err.to_string();

    if let Some(human_err) = human_err.strip_prefix("runtime error: ") {
        PyValueError::new_err(human_err.to_owned())
    } else {
        PyValueError::new_err(human_err)
    }
}
