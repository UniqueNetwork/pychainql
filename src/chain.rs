use crate::{jsonnet::JsonnetObject, jsonnet_tokio::execute_jsonnet};
use pyo3::{exceptions::PyBaseException, prelude::*};

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
#[pyo3(signature = (url, opts=None))]
pub(crate) fn chain(url: String, opts: Option<ChainOpts>) -> PyResult<JsonnetObject> {
    execute_jsonnet(|| {
        let opts = opts.map(|opts| chainql_core::ChainOpts {
            include_defaults: opts.include_defaults,
            omit_empty: opts.omit_empty,
        });

        let chain = chainql_core::builtin_chain(url, opts)
            .map_err(|err| PyBaseException::new_err(err.to_string()))?;

        Ok(JsonnetObject(chain))
    })
}
