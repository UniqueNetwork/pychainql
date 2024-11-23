use crate::{jsonnet_tokio::execute_jsonnet, utils::jsonnet_error};
use jrsonnet_evaluator as jsonnet;
use pyo3::{
    exceptions::{PyKeyError, PyTypeError},
    prelude::*,
    types::{PyBool, PyNone},
};

/// TODO
fn jsonnet_to_py(py: Python<'_>, value: jsonnet::Val) -> PyResult<Bound<'_, PyAny>> {
    use jsonnet::Val::*;

    Ok(match value {
        Bool(b) => PyBool::new(py, b).to_owned().into_any(),
        Null => PyNone::get(py).to_owned().into_any(),
        Str(s) => s.into_flat().as_str().into_pyobject(py)?.into_any(),
        Num(num) => num.into_pyobject(py)?.into_any(),
        BigInt(bignum) => bignum.into_pyobject(py)?.into_any(),
        Arr(arr) => JsonnetArray(arr).into_pyobject(py)?.into_any(),
        Obj(obj) => JsonnetObject(obj).into_pyobject(py)?.into_any(),
        Func(func) => JsonnetFunc(func).into_pyobject(py)?.into_any(),
    })
}

/// TODO
#[pyclass(unsendable, mapping)]
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
                return Err(PyTypeError::new_err("Key should be a string"));
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
                return Err(PyTypeError::new_err("Index should be a decimal"));
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
pub struct JsonnetFunc(pub jrsonnet_evaluator::function::FuncVal);

#[pymethods]
impl JsonnetFunc {}
