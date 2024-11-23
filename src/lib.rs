mod address;
mod chain;
mod jsonnet;
mod jsonnet_tokio;
mod ss58;
mod ss58_registry;
mod utils;

use pyo3::prelude::*;

use std::sync::{
    atomic::{AtomicBool, Ordering},
    OnceLock,
};
use utils::value_error;

pub(crate) static ENABLE_LOGGER: AtomicBool = AtomicBool::new(false);
pub(crate) static RUNTIME: OnceLock<tokio::runtime::Runtime> = OnceLock::new();

#[pymodule]
mod chainql {
    use super::*;

    #[pymodule_init]
    fn init(_m: &Bound<'_, PyModule>) -> PyResult<()> {
        jsonnet_tokio::init();
        Ok(())
    }

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
        use chainql_core::{
            ethereum::{builtin_eth_encode, eth_cksum_address, eth_cksum_address_from_ecdsa},
            hex::Hex,
        };

        /// TODO
        #[pyfunction]
        fn encode(address: Vec<u8>) -> PyResult<String> {
            builtin_eth_encode(Hex(address)).map_err(value_error)
        }

        /// TODO
        #[pyfunction]
        fn cksum_address(address: [u8; 20]) -> String {
            eth_cksum_address(address)
        }

        /// TODO
        #[pyfunction]
        fn cksum_address_from_ecdsa(pubkey: [u8; 33]) -> PyResult<String> {
            eth_cksum_address_from_ecdsa(pubkey).map_err(value_error)
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

        /// Convert a hex string to a vector of bytes
        #[pyfunction]
        fn from_hex(data: &str) -> PyResult<Vec<u8>> {
            hex::from_hex(data).map_err(value_error)
        }

        /// Convert an array of bytes to a hex string
        #[pyfunction]
        fn to_hex(data: &[u8]) -> String {
            hex::to_hex(data)
        }
    }
}
