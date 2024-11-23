use crate::{
    jsonnet_py::{jsonnet_to_py, py_to_jsonnet, pydict_to_jsonnet, pylist_to_jsonnet},
    jsonnet_tokio::execute_jsonnet,
    utils::jsonnet_error,
};
use jrsonnet_evaluator as jsonnet;
use pyo3::{
    exceptions::{PyKeyError, PyRuntimeError, PyTypeError},
    prelude::*,
    types::{PyBool, PyDict, PyFunction, PyList, PyNone, PySet, PyTuple},
};

/// TODO
#[pyclass(unsendable, mapping)]
#[derive(Clone)]
pub struct JsonnetObject(pub jrsonnet_evaluator::ObjValue);

#[pymethods]
impl JsonnetObject {
    fn __getitem__<'py>(
        &self,
        py: Python<'py>,
        key: &Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        execute_jsonnet(|| {
            let Ok(key) = key.extract::<&str>() else {
                return Err(PyTypeError::new_err("key should be a string"));
            };

            let Some(value) = self.0.get(key.into()).map_err(jsonnet_error)? else {
                return Err(PyKeyError::new_err(key.to_owned()));
            };

            jsonnet_to_py(py, value)
        })
    }
}

/// TODO
#[pyclass(unsendable, mapping)]
#[derive(Clone)]
pub struct JsonnetArray(pub jrsonnet_evaluator::val::ArrValue);

#[pymethods]
impl JsonnetArray {
    fn __getitem__<'py>(
        &self,
        py: Python<'py>,
        key: &Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        execute_jsonnet(|| {
            let Ok(idx) = key.extract::<usize>() else {
                return Err(PyTypeError::new_err("index should be a decimal"));
            };

            let Some(value) = self.0.get(idx).map_err(jsonnet_error)? else {
                return Err(PyKeyError::new_err(idx));
            };

            jsonnet_to_py(py, value)
        })
    }
}

/// TODO
#[pyclass(unsendable)]
#[derive(Clone)]
pub struct JsonnetFunc(pub jrsonnet_evaluator::function::FuncVal);

#[pymethods]
impl JsonnetFunc {
    #[pyo3(signature = (*args))]
    fn __call__<'py>(
        &self,
        py: Python<'py>,
        args: &Bound<'_, PyTuple>,
    ) -> PyResult<Bound<'py, PyAny>> {
        execute_jsonnet(|| {
            let args = pylist_to_jsonnet(py, args.iter())?;

            let out = self
                .0
                .evaluate_simple(&args, false)
                .map_err(jsonnet_error)?;

            jsonnet_to_py(py, out)
        })
    }
}
