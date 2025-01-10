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

use crate::{
    jsonnet::{JsonnetArray, JsonnetFunc, JsonnetObject},
    utils::type_error,
};
use jrsonnet_evaluator as jsonnet;
use pyo3::{
    exceptions::PyTypeError,
    prelude::*,
    types::{PyBool, PyCFunction, PyDict, PyList, PyNone, PySet, PyTuple},
};

/// Convert jsonnet value to python object.
pub fn jsonnet_to_py(py: Python<'_>, value: jsonnet::Val) -> PyResult<Bound<'_, PyAny>> {
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

/// Convert python object to jsonnet value.
pub fn py_to_jsonnet(py: Python<'_>, arg: Bound<'_, PyAny>) -> PyResult<jsonnet::Val> {
    if arg.is_none() {
        Ok(jsonnet::Val::Null)
    } else if let Ok(b) = arg.extract::<bool>() {
        Ok(jsonnet::Val::Bool(b))
    } else if let Ok(s) = arg.extract::<&str>() {
        Ok(jsonnet::Val::Str(s.into()))
    } else if let Ok(num) = arg.extract::<f64>() {
        Ok(jsonnet::Val::Num(num))
    } else if let Ok(num) = arg.extract::<num_bigint::BigInt>() {
        Ok(jsonnet::Val::BigInt(Box::new(num)))
    } else if let Ok(list) = arg.extract::<Bound<'_, PyList>>() {
        let arr = pylist_to_jsonnet(py, list.iter())?;
        Ok(jsonnet::Val::Arr(arr.into()))
    } else if let Ok(set) = arg.extract::<Bound<'_, PySet>>() {
        let arr = pylist_to_jsonnet(py, set.iter())?;
        Ok(jsonnet::Val::Arr(arr.into()))
    } else if let Ok(tuple) = arg.extract::<Bound<'_, PyTuple>>() {
        let arr = pylist_to_jsonnet(py, tuple.iter())?;
        Ok(jsonnet::Val::Arr(arr.into()))
    } else if let Ok(dict) = arg.extract::<Bound<'_, PyDict>>() {
        let obj = pydict_to_jsonnet(py, dict)?;
        Ok(jsonnet::Val::Obj(obj))
    } else if let Ok(_func) = arg.extract::<Bound<'_, PyCFunction>>() {
        Err(PyTypeError::new_err("functions are not supported"))
    } else {
        let ty_name = arg.get_type().name()?;
        Err(PyTypeError::new_err(format!("unsupported type {ty_name}")))
    }
}

/// Convert list of python objects to array of jsonnet values.
pub fn pylist_to_jsonnet<'py>(
    py: Python<'py>,
    list: impl Iterator<Item = Bound<'py, PyAny>>,
) -> PyResult<Vec<jsonnet::Val>> {
    list.enumerate()
        .map(|(idx, el)| {
            py_to_jsonnet(py, el)
                .map_err(|err| type_error(py, format!("unsupported type at index {idx}"), err))
        })
        .collect()
}

/// Convert python dictionary to jsonnet object.
pub fn pydict_to_jsonnet(py: Python<'_>, dict: Bound<'_, PyDict>) -> PyResult<jsonnet::ObjValue> {
    let mut obj = jsonnet::ObjValue::builder_with_capacity(dict.len());

    for (key, value) in dict {
        let key = key
            .extract::<&str>()
            .map_err(|err| type_error(py, "key should be a string", err))?;

        let value = py_to_jsonnet(py, value)
            .map_err(|err| type_error(py, format!("unsupported value type at '{key}'"), err))?;

        obj.field(key).value(value);
    }

    Ok(obj.build())
}
