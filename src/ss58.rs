use pyo3::prelude::*;

use crate::{ss58_registry::Ss58AccountFormat, to_value_error};
use ss58_registry as ss58_crate;

#[pyclass(str)]
#[derive(Clone, Copy)]
pub(crate) struct Ss58AddressFormat(pub ss58_crate::Ss58AddressFormat);

#[pymethods]
impl Ss58AddressFormat {
    #[new]
    pub fn new(registry: Ss58AccountFormat) -> Self {
        let registry = ss58_crate::Ss58AddressFormatRegistry::from(registry);
        Self(ss58_crate::Ss58AddressFormat::from(registry))
    }

    #[staticmethod]
    pub fn from_name(name: &str) -> PyResult<Self> {
        let format = ss58_crate::Ss58AddressFormat::try_from(name).map_err(to_value_error)?;
        Ok(Self(format))
    }

    #[staticmethod]
    pub fn custom(prefix: u16) -> Self {
        Self(ss58_registry::Ss58AddressFormat::custom(prefix))
    }

    /// Address prefix used on the network
    pub fn prefix(&self) -> u16 {
        self.0.prefix()
    }

    /// Network/AddressType is reserved for future use
    pub fn is_reserved(&self) -> bool {
        self.prefix() > 16384 || matches!(self.prefix(), 46u16 | 47u16)
    }

    // A custom format is one that is not already known
    pub fn is_custom(&self) -> bool {
        !matches!(self.prefix (), 0u16 ..= 58u16 | 63u16 ..= 63u16 | 65u16 ..= 69u16 | 71u16 ..= 73u16 | 77u16 ..= 78u16 | 81u16 ..= 81u16 | 88u16 ..= 90u16 | 92u16 ..= 93u16 | 98u16 ..= 101u16 | 105u16 ..= 105u16 | 110u16 ..= 110u16 | 113u16 ..= 113u16 | 117u16 ..= 117u16 | 126u16 ..= 126u16 | 128u16 ..= 129u16 | 131u16 ..= 131u16 | 136u16 ..= 137u16 | 172u16 ..= 172u16 | 252u16 ..= 252u16 | 255u16 ..= 255u16 | 268u16 ..= 268u16 | 420u16 ..= 420u16 | 440u16 ..= 440u16 | 666u16 ..= 666u16 | 777u16 ..= 777u16 | 789u16 ..= 789u16 | 995u16 ..= 995u16 | 1110u16 ..= 1110u16 | 1221u16 ..= 1222u16 | 1284u16 ..= 1285u16 | 1328u16 ..= 1328u16 | 1337u16 ..= 1337u16 | 1516u16 ..= 1516u16 | 1985u16 ..= 1985u16 | 2007u16 ..= 2007u16 | 2009u16 ..= 2009u16 | 2021u16 ..= 2021u16 | 2024u16 ..= 2024u16 | 2032u16 ..= 2032u16 | 2092u16 ..= 2092u16 | 2106u16 ..= 2106u16 | 2112u16 ..= 2112u16 | 2199u16 ..= 2199u16 | 2206u16 ..= 2207u16 | 2254u16 ..= 2254u16 | 3333u16 ..= 3333u16 | 4450u16 ..= 4450u16 | 5234u16 ..= 5234u16 | 5845u16 ..= 5845u16 | 6094u16 ..= 6094u16 | 7007u16 ..= 7007u16 | 7013u16 ..= 7013u16 | 7306u16 ..= 7306u16 | 7391u16 ..= 7391u16 | 8866u16 ..= 8866u16 | 8883u16 ..= 8883u16 | 8886u16 ..= 8886u16 | 8888u16 ..= 8888u16 | 9072u16 ..= 9072u16 | 9807u16 ..= 9807u16 | 9935u16 ..= 9935u16 | 10041u16 ..= 10041u16 | 11330u16 ..= 11331u16 | 11486u16 ..= 11486u16 | 11820u16 ..= 11820u16 | 12155u16 ..= 12155u16 | 12191u16 ..= 12191u16 | 12850u16 ..= 12850u16 | 13116u16 ..= 13116u16 | 14697u16 ..= 14697u16 | 14998u16 ..= 14998u16 | 29972u16 ..= 29972u16)
    }
}

impl Default for Ss58AddressFormat {
    fn default() -> Self {
        Self(ss58_registry::Ss58AddressFormatRegistry::SubstrateAccount.into())
    }
}

impl From<Ss58AddressFormat> for ss58_registry::Ss58AddressFormat {
    fn from(val: Ss58AddressFormat) -> Self {
        val.0
    }
}

impl std::fmt::Display for Ss58AddressFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

/// Encode bytes to SS58 string.
#[pyfunction]
#[pyo3(signature = (raw, format=None))]
pub(crate) fn ss58_encode(raw: Vec<u8>, format: Option<Ss58AddressFormat>) -> PyResult<String> {
    let raw = chainql_core::hex::Hex(raw);
    let format = format.map(|f| chainql_core::address::Ss58Format(f.0));

    chainql_core::builtin_ss58_encode(raw, format)
        .map(|encoded| encoded.to_string())
        .map_err(to_value_error)
}

/// Parse SS58 address to bytes.
#[pyfunction]
pub(crate) fn ss58_decode(ss58: &str) -> PyResult<Vec<u8>> {
    chainql_core::builtin_ss58(ss58.into())
        .map(|hex| hex.0)
        .map_err(to_value_error)
}
