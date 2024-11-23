use pyo3::{
    exceptions::{PyRuntimeError, PyValueError},
    prelude::*,
};

/// TODO
#[inline(always)]
pub fn jsonnet_error(err: jrsonnet_evaluator::error::Error) -> PyErr {
    PyRuntimeError::new_err(format!("jsonnet: {err}"))
}

#[inline(always)]
pub fn value_error(err: impl std::error::Error) -> PyErr {
    let human_err = err.to_string();

    if let Some(human_err) = human_err.strip_prefix("runtime error: ") {
        PyValueError::new_err(human_err.to_owned())
    } else {
        PyValueError::new_err(human_err)
    }
}
