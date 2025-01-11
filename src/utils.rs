// Copyright 2024 Valery Klachkov
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use pyo3::exceptions::PyKeyboardInterrupt;
use pyo3::{
    exceptions::{PyRuntimeError, PyTypeError, PyValueError},
    prelude::*,
};

#[inline(always)]
pub fn type_error(py: Python<'_>, description: impl ToString, cause: PyErr) -> PyErr {
    let new_err = PyTypeError::new_err(description.to_string());
    new_err.set_cause(py, Some(cause));
    new_err
}

#[inline]
pub fn jsonnet_error(err: jrsonnet_evaluator::error::Error) -> PyErr {
    let human_err = err.error().to_string();

    if human_err.ends_with("cancelled") {
        return PyKeyboardInterrupt::new_err(());
    }

    if let Some(human_err) = human_err.strip_prefix("runtime error: ") {
        PyRuntimeError::new_err(human_err.to_owned())
    } else {
        PyRuntimeError::new_err(human_err)
    }
}

#[inline]
pub fn value_error(err: impl std::error::Error) -> PyErr {
    let human_err = err.to_string();

    if let Some(human_err) = human_err.strip_prefix("runtime error: ") {
        PyValueError::new_err(human_err.to_owned())
    } else {
        PyValueError::new_err(human_err)
    }
}
