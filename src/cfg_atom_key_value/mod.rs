use pyo3::prelude::*;

use crate::types::{CfgAtom, Symbol};

#[pymethods]
impl CfgAtom {
    /// The variant `CfgAtom::KeyValue { key: Symbol, value: Symbol }`, made. It takes the symbols, so it is given
    /// copies.
    #[staticmethod]
    #[pyo3(name = "KeyValue")]
    fn key_value(key: &Symbol, value: &Symbol) -> CfgAtom {
        CfgAtom { inner: ra_ap_hir::CfgAtom::KeyValue { key: key.inner.clone(), value: value.inner.clone() } }
    }
}
