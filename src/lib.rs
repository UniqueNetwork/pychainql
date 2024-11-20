mod address;
mod chain;
mod ss58;
mod ss58_registry;

use pyo3::{exceptions::PyValueError, prelude::*};

#[pymodule]
mod chainql {
    use super::*;

    #[pymodule]
    mod chain {
        #[pymodule_export]
        use crate::chain::{chain, ChainOpts};
    }

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
            builtin_eth_encode(Hex(address)).map_err(to_value_error)
        }

        /// TODO
        #[pyfunction]
        fn cksum_address(address: [u8; 20]) -> String {
            eth_cksum_address(address)
        }

        /// TODO
        #[pyfunction]
        fn cksum_address_from_ecdsa(pubkey: [u8; 33]) -> PyResult<String> {
            eth_cksum_address_from_ecdsa(pubkey).map_err(to_value_error)
        }
    }

    #[pymodule]
    mod hex {
        use super::*;
        use chainql_core::hex;

        /// Convert a hex string to a vector of bytes
        #[pyfunction]
        fn from_hex(data: &str) -> PyResult<Vec<u8>> {
            hex::from_hex(data).map_err(to_value_error)
        }

        /// Convert an array of bytes to a hex string
        #[pyfunction]
        fn to_hex(data: &[u8]) -> String {
            hex::to_hex(data)
        }
    }
}

#[inline(always)]
pub(crate) fn to_value_error(err: impl std::error::Error) -> PyErr {
    let human_err = err.to_string();

    if let Some(human_err) = human_err.strip_prefix("runtime error: ") {
        PyValueError::new_err(human_err.to_owned())
    } else {
        PyValueError::new_err(human_err)
    }
}
