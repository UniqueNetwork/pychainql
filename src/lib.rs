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

mod address;
mod chain;
mod jsonnet;
mod jsonnet_py;
mod jsonnet_tokio;
mod ss58;
mod ss58_registry;
mod utils;

use pyo3::prelude::*;

use std::sync::{atomic::{AtomicBool, Ordering}, LazyLock, OnceLock};
use utils::value_error;

pub(crate) static ENABLE_LOGGER: AtomicBool = AtomicBool::new(false);

pub(crate) static RUNTIME: LazyLock<tokio::runtime::Runtime> = LazyLock::new(|| {
    tokio::runtime::Builder::new_multi_thread()
        .thread_name("chainql-tokio-runtime")
        .enable_all()
        .build()
        .unwrap()
});

#[pymodule]
mod chainql {
    use super::*;

    #[pyfunction]
    fn enable_logs() {
        ENABLE_LOGGER.store(true, Ordering::SeqCst);
    }

    #[pyfunction]
    fn disable_logs() {
        ENABLE_LOGGER.store(false, Ordering::SeqCst);
    }

    #[pymodule_export]
    use crate::chain::dump;
    #[pymodule_export]
    use crate::chain::{Chain, ChainOpts};
    #[pymodule_export]
    use crate::jsonnet::{JsonnetArray, JsonnetFunc, JsonnetObject};

    #[pymodule]
    mod address {
        #[pymodule_export]
        use crate::address::{address_seed, public_bytes_seed, SignatureSchema};
        #[pymodule_export]
        use crate::ss58::{ss58_decode, ss58_encode, Ss58AddressFormat};
        #[pymodule_export]
        use crate::ss58_registry::Ss58AccountFormat;
    }

    #[pymodule]
    mod ethereum {
        use super::*;
        use chainql_core::{ethereum::builtin_eth_encode, hex::Hex};

        /// Encode bytes to ethereum address string
        #[pyfunction]
        fn encode(address: Vec<u8>) -> PyResult<String> {
            builtin_eth_encode(Hex(address)).map_err(value_error)
        }
    }

    #[pymodule]
    mod hash {
        use super::*;
        use chainql_core::hex::Hex;

        /// Conduct a 128-bit XX hash
        #[pyfunction]
        fn twox128(data: Vec<u8>) -> Vec<u8> {
            chainql_core::builtin_twox128(Hex(data)).0
        }

        /// Conduct a 256-bit Keccak hash
        #[pyfunction]
        fn keccak256(data: Vec<u8>) -> Vec<u8> {
            chainql_core::builtin_keccak256(Hex(data)).0
        }
    }

    #[pymodule]
    mod hex {
        use super::*;
        use chainql_core::hex;

        /// Convert a hex string to bytes
        #[pyfunction]
        fn from_hex(data: &str) -> PyResult<Vec<u8>> {
            hex::from_hex(data).map_err(value_error)
        }

        /// Convert bytes to a hex string
        #[pyfunction]
        fn to_hex(data: &[u8]) -> String {
            hex::to_hex(data)
        }
    }
}
