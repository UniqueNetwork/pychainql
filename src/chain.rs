use jrsonnet_evaluator::{
    val::ArrValue as JsonnetArray, val::Val as JsonnetVal, ObjValue as JsonnetObj,
};
use pyo3::{
    exceptions::PyBaseException,
    prelude::*,
    types::{PyDict, PyList},
};

use crate::to_value_error;

/// Selection of optional flags for chain data processing
#[pyclass]
#[derive(Clone, Copy, Default)]
pub(crate) struct ChainOpts {
    /// Whether or not to ignore trie prefixes with no keys
    omit_empty: bool,
    /// Should default values be included in output
    include_defaults: bool,
}

/// Get chain data from a URL, including queryable storage, metadata, and blocks.
#[pyfunction]
#[pyo3(pass_module, signature = (url, opts=None))]
pub(crate) fn chain<'py>(
    module: &Bound<'py, PyModule>,
    url: String,
    opts: Option<ChainOpts>,
) -> PyResult<Bound<'py, PyDict>> {
    let opts = opts.map(|opts| chainql_core::ChainOpts {
        include_defaults: opts.include_defaults,
        omit_empty: opts.omit_empty,
    });

    // This is the amount of bytes that need to be left on the stack before increasing the size.
    // It must be at least as large as the stack required by any code that does not call
    // `ensure_sufficient_stack`.
    const RED_ZONE: usize = 1024 * 1024; // 100k

    // Only the first stack that is pushed, grows exponentially (2^n * STACK_PER_RECURSION) from then
    // on. This flag has performance relevant characteristics. Don't set it too high.
    const STACK_PER_RECURSION: usize = 10 * 1024 * 1024; // 1MB

    stacker::maybe_grow(RED_ZONE, STACK_PER_RECURSION, || {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                tokio::task::block_in_place(|| {
                    chainql_core::builtin_chain(url, opts)
                        .map_err(|err| PyBaseException::new_err(err.to_string()))
                        .and_then(|obj| obj_to_pydict(module.py(), obj))
                })
            })
    })
}

#[track_caller]
fn obj_to_pydict(py: Python<'_>, obj: JsonnetObj) -> PyResult<Bound<'_, PyDict>> {
    let dict = PyDict::new(py);

    for key in obj.fields_ex(true, false) {
        let value = obj.get(key.clone()).map_err(to_value_error)?.unwrap();
        let key = key.as_str();

        match value {
            JsonnetVal::Bool(boolean) => dict.set_item(key, boolean)?,
            JsonnetVal::Null => dict.set_item(key, py.None())?,
            JsonnetVal::Str(string) => dict.set_item(key, string.into_flat().as_str())?,
            JsonnetVal::Num(number) => dict.set_item(key, number)?,
            JsonnetVal::BigInt(bigint) => dict.set_item(key, &*bigint)?,
            JsonnetVal::Arr(array) => dict.set_item(key, array_to_pylist(py, array)?)?,
            JsonnetVal::Obj(obj) => dict.set_item(key, obj_to_pydict(py, obj)?)?,
            JsonnetVal::Func(_func) => unimplemented!(),
        }
    }

    Ok(dict)
}

fn array_to_pylist(py: Python<'_>, array: JsonnetArray) -> PyResult<Bound<'_, PyList>> {
    let arr = PyList::empty(py);

    for item in array.iter() {
        match item.unwrap() {
            JsonnetVal::Bool(boolean) => arr.append(boolean)?,
            JsonnetVal::Null => arr.append(py.None())?,
            JsonnetVal::Str(string) => arr.append(string.into_flat().as_str())?,
            JsonnetVal::Num(number) => arr.append(number)?,
            JsonnetVal::BigInt(bigint) => arr.append(&*bigint)?,
            JsonnetVal::Arr(array) => arr.append(array_to_pylist(py, array)?)?,
            JsonnetVal::Obj(obj) => arr.append(obj_to_pydict(py, obj)?)?,
            JsonnetVal::Func(_func) => unimplemented!(),
        }
    }

    Ok(arr)
}
