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

use pyo3::prelude::*;

use crate::{ss58::Ss58AddressFormat, value_error};
use chainql_core::address as chainql_address;

/// TODO
#[pyclass(frozen, eq, eq_int, str, hash)]
#[derive(Clone, Copy, Hash, PartialEq)]
pub enum SignatureSchema {
    Ed25519,
    Sr25519,
    Ecdsa,
    Ethereum,
}

impl From<SignatureSchema> for chainql_address::SignatureSchema {
    fn from(val: SignatureSchema) -> Self {
        match val {
            SignatureSchema::Ed25519 => chainql_address::SignatureSchema::Ed25519,
            SignatureSchema::Sr25519 => chainql_address::SignatureSchema::Sr25519,
            SignatureSchema::Ecdsa => chainql_address::SignatureSchema::Ecdsa,
            SignatureSchema::Ethereum => chainql_address::SignatureSchema::Ethereum,
        }
    }
}

impl std::fmt::Display for SignatureSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Ed25519 => "Ed25519",
            Self::Sr25519 => "Sr25519",
            Self::Ecdsa => "Ecdsa",
            Self::Ethereum => "Ethereum",
        })
    }
}

/// TODO
#[pyfunction]
#[pyo3(signature = (scheme, suri, format=None))]
pub fn address_seed(
    scheme: SignatureSchema,
    suri: &str,
    format: Option<Ss58AddressFormat>,
) -> PyResult<String> {
    chainql_address::address_seed(scheme.into(), suri, format.unwrap_or_default().into())
        .map_err(value_error)
}

/// TODO
#[pyfunction]
pub fn public_bytes_seed(scheme: SignatureSchema, suri: &str) -> PyResult<Vec<u8>> {
    chainql_address::public_bytes_seed(scheme.into(), suri).map_err(value_error)
}
