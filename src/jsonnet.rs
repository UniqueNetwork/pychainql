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
    jsonnet_py::{jsonnet_to_py, py_to_jsonnet, pylist_to_jsonnet},
    jsonnet_tokio::execute_jsonnet,
    utils::{jsonnet_error, type_error},
};
use pyo3::{
    exceptions::{PyKeyError, PyTypeError},
    prelude::*,
    types::PyTuple,
};
use std::cell::RefCell;

/// TODO
#[pyclass(unsendable, mapping)]
#[derive(Clone)]
pub struct JsonnetObject(pub jrsonnet_evaluator::ObjValue);

#[pymethods]
impl JsonnetObject {
    fn __len__(&self) -> usize {
        self.0.len()
    }

    fn __contains__(&self, key: &Bound<'_, PyAny>) -> bool {
        key.extract::<&str>()
            .map(|key| self.0.has_field(key.into()))
            .unwrap_or(false)
    }

    fn __getattr__<'py>(
        &self,
        py: Python<'py>,
        name: &Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        execute_jsonnet(|| {
            let key = name
                .extract::<&str>()
                .map_err(|err| type_error(py, "key should be a string", err))?;

            let value = self
                .0
                .get(key.into())
                .map_err(jsonnet_error)?
                .ok_or_else(|| PyKeyError::new_err(key.to_owned()))?;

            jsonnet_to_py(py, value)
        })
    }

    fn __getitem__<'py>(
        &self,
        py: Python<'py>,
        key: &Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        self.__getattr__(py, key)
    }

    #[pyo3(signature = (include_hidden=false))]
    fn keys(&self, include_hidden: bool) -> JsonnetObjectKeys {
        JsonnetObjectKeys {
            iter: RefCell::new(Box::new(self.0.fields_ex(include_hidden, true).into_iter())),
        }
    }

    #[pyo3(signature = (include_hidden=false))]
    fn values(&self, include_hidden: bool) -> JsonnetObjectValues {
        JsonnetObjectValues {
            obj: self.clone(),
            iter: RefCell::new(Box::new(self.0.fields_ex(include_hidden, true).into_iter())),
        }
    }

    #[pyo3(signature = (include_hidden=false))]
    fn items(&self, include_hidden: bool) -> JsonnetObjectItems {
        JsonnetObjectItems {
            obj: self.clone(),
            iter: RefCell::new(Box::new(self.0.fields_ex(include_hidden, true).into_iter())),
        }
    }

    #[pyo3(signature = (minified=true))]
    fn manifest_json(&self, minified: bool) -> PyResult<String> {
        execute_jsonnet(|| {
            let preserve_order = true;
            let fmt = if minified {
                jrsonnet_evaluator::manifest::JsonFormat::minify(preserve_order)
            } else {
                jrsonnet_evaluator::manifest::JsonFormat::cli(2, preserve_order)
            };

            jrsonnet_evaluator::Val::Obj(self.0.clone())
                .manifest(fmt)
                .map_err(jsonnet_error)
        })
    }
}

#[pyclass(unsendable)]
pub struct JsonnetObjectKeys {
    iter: RefCell<Box<dyn Iterator<Item = jrsonnet_evaluator::IStr>>>,
}

#[pymethods]
impl JsonnetObjectKeys {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(slf: PyRefMut<'_, Self>) -> Option<String> {
        slf.iter
            .borrow_mut()
            .next()
            .map(|key| key.as_str().to_owned())
    }
}

#[pyclass(unsendable)]
pub struct JsonnetObjectValues {
    obj: JsonnetObject,
    iter: RefCell<Box<dyn Iterator<Item = jrsonnet_evaluator::IStr>>>,
}

#[pymethods]
impl JsonnetObjectValues {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__<'py>(
        slf: PyRefMut<'py, Self>,
        py: Python<'py>,
    ) -> PyResult<Option<Bound<'py, PyAny>>> {
        execute_jsonnet(|| {
            let Some(key) = slf.iter.borrow_mut().next() else {
                return Ok(None);
            };

            let obj = &slf.obj.0;
            let value = obj
                .get(key)
                .map_err(jsonnet_error)?
                .expect("iterating over keys, field exists");

            jsonnet_to_py(py, value).map(Some)
        })
    }
}

#[pyclass(unsendable)]
pub struct JsonnetObjectItems {
    obj: JsonnetObject,
    iter: RefCell<Box<dyn Iterator<Item = jrsonnet_evaluator::IStr>>>,
}

#[pymethods]
impl JsonnetObjectItems {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__<'py>(
        slf: PyRefMut<'py, Self>,
        py: Python<'py>,
    ) -> PyResult<Option<(String, Bound<'py, PyAny>)>> {
        execute_jsonnet(|| {
            let Some(key) = slf.iter.borrow_mut().next() else {
                return Ok(None);
            };

            let obj = &slf.obj.0;
            let value = obj
                .get(key.clone())
                .map_err(jsonnet_error)?
                .expect("iterating over keys, field exists");

            let pykey = key.as_str().to_string();
            let pyvalue = jsonnet_to_py(py, value)?;

            Ok(Some((pykey, pyvalue)))
        })
    }
}

/// TODO
#[pyclass(unsendable, sequence)]
#[derive(Clone)]
pub struct JsonnetArray(pub jrsonnet_evaluator::val::ArrValue);

#[pymethods]
impl JsonnetArray {
    fn __len__(&self) -> usize {
        self.0.len()
    }

    fn __contains__(&self, py: Python<'_>, object: Bound<'_, PyAny>) -> PyResult<bool> {
        execute_jsonnet(|| {
            let value = py_to_jsonnet(py, object)?;

            for c in self.0.iter_lazy() {
                let c = c.evaluate().map_err(jsonnet_error)?;

                if jrsonnet_evaluator::val::equals(&value, &c).map_err(jsonnet_error)? {
                    return Ok(true);
                }
            }

            Ok(false)
        })
    }

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

    fn __iter__(&self) -> JsonnetArrayIter {
        JsonnetArrayIter {
            array: self.clone(),
            idx: 0,
        }
    }

    #[pyo3(signature = (minified=true))]
    fn manifest_json(&self, minified: bool) -> PyResult<String> {
        execute_jsonnet(|| {
            let preserve_order = true;
            let fmt = if minified {
                jrsonnet_evaluator::manifest::JsonFormat::minify(preserve_order)
            } else {
                jrsonnet_evaluator::manifest::JsonFormat::cli(2, preserve_order)
            };

            jrsonnet_evaluator::Val::Arr(self.0.clone())
                .manifest(fmt)
                .map_err(jsonnet_error)
        })
    }
}

#[pyclass(unsendable)]
pub struct JsonnetArrayIter {
    array: JsonnetArray,
    idx: usize,
}

#[pymethods]
impl JsonnetArrayIter {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__<'py>(
        mut slf: PyRefMut<'py, Self>,
        py: Python<'py>,
    ) -> PyResult<Option<Bound<'py, PyAny>>> {
        let slf = &mut slf;

        execute_jsonnet(|| {
            slf.array
                .0
                .get(slf.idx)
                .map_err(jsonnet_error)?
                .map(|value| {
                    let pyvalue = jsonnet_to_py(py, value)?;
                    slf.idx += 1;
                    Ok(pyvalue)
                })
                .transpose()
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
