use crate::{jsonnet::JsonnetObject, jsonnet_tokio::execute_jsonnet, utils::jsonnet_error};
use chainql_core::hex::Hex;
use either::Either;
use pyo3::{exceptions::PyBaseException, prelude::*};
use std::collections::BTreeMap;

/// Selection of optional flags for chain data processing
#[pyclass]
#[derive(Clone, Copy, Default)]
pub struct ChainOpts {
    /// Whether or not to ignore trie prefixes with no keys
    #[pyo3(get, set)]
    pub omit_empty: bool,

    /// Should default values be included in output
    #[pyo3(get, set)]
    pub include_defaults: bool,
}

#[pymethods]
impl ChainOpts {
    #[new]
    #[pyo3(signature = (omit_empty=false, include_defaults=false))]
    pub fn new(omit_empty: bool, include_defaults: bool) -> Self {
        ChainOpts {
            omit_empty,
            include_defaults,
        }
    }
}

impl From<ChainOpts> for chainql_core::ChainOpts {
    fn from(opts: ChainOpts) -> Self {
        chainql_core::ChainOpts {
            include_defaults: opts.include_defaults,
            omit_empty: opts.omit_empty,
        }
    }
}

/// TODO
#[pyclass(unsendable)]
pub struct Chain(JsonnetObject);

#[pymethods]
impl Chain {
    #[new]
    #[pyo3(signature = (url, opts=None))]
    pub fn new(url: String, opts: Option<ChainOpts>) -> PyResult<Self> {
        execute_jsonnet(|| {
            chainql_core::builtin_chain(url, opts.map(Into::into))
                .map(|chain| Self(JsonnetObject(chain)))
                .map_err(|err| PyBaseException::new_err(err.to_string()))
        })
    }

    pub fn latest(&self) -> PyResult<JsonnetObject> {
        execute_jsonnet(|| {
            let chain = &self.0 .0;

            let latest = chain
                .get("latest".into())
                .map_err(jsonnet_error)?
                .expect("`latest` field should be presented")
                .as_obj()
                .expect("`latest` field should be an object");

            Ok(JsonnetObject(latest))
        })
    }

    pub fn block(&self, block: u32) -> PyResult<JsonnetObject> {
        execute_jsonnet(|| {
            let chain = &self.0 .0;

            let block_func = chain
                .get("block".into())
                .map_err(jsonnet_error)?
                .expect("`block` field should be presented")
                .as_func()
                .expect("`block` field should be a function");

            let block = block_func
                .evaluate_simple(&(block,), false)
                .map_err(jsonnet_error)?
                .as_obj()
                .expect("`block` should return an object");

            Ok(JsonnetObject(block))
        })
    }
}

#[pyfunction]
#[pyo3(signature = (meta, data, opts=None))]
pub fn dump(
    meta: Either<JsonnetObject, Vec<u8>>,
    data: BTreeMap<Vec<u8>, Vec<u8>>,
    opts: Option<ChainOpts>,
) -> PyResult<JsonnetObject> {
    execute_jsonnet(|| {
        let meta = match meta {
            Either::Left(l) => jrsonnet_evaluator::typed::Either2::A(l.0),
            Either::Right(r) => jrsonnet_evaluator::typed::Either2::B(Hex(r)),
        };

        // SAFETY: Vec<u8> and Hex have the same layout and size.
        let data =
            unsafe { core::mem::transmute::<BTreeMap<Vec<u8>, Vec<u8>>, BTreeMap<Hex, Hex>>(data) };

        let opts = opts.map(Into::into);

        chainql_core::builtin_dump(meta, data, opts)
            .map(JsonnetObject)
            .map_err(|err| PyBaseException::new_err(err.to_string()))
    })
}
