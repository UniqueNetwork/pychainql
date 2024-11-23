use crate::{jsonnet::JsonnetObject, jsonnet_tokio::execute_jsonnet, utils::jsonnet_error};
use pyo3::{exceptions::PyBaseException, prelude::*};

/// Selection of optional flags for chain data processing
#[pyclass]
#[derive(Clone, Copy, Default)]
pub struct ChainOpts {
    /// Whether or not to ignore trie prefixes with no keys
    omit_empty: bool,
    /// Should default values be included in output
    include_defaults: bool,
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
            let opts = opts.map(|opts| chainql_core::ChainOpts {
                include_defaults: opts.include_defaults,
                omit_empty: opts.omit_empty,
            });

            let chain = chainql_core::builtin_chain(url, opts)
                .map_err(|err| PyBaseException::new_err(err.to_string()))?;

            Ok(Self(JsonnetObject(chain)))
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
