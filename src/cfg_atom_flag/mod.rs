use pyo3::prelude::*;

use crate::types::{CfgAtom, Symbol};

#[pymethods]
impl CfgAtom {
    /// The variant `CfgAtom::Flag(Symbol)`, made. It takes the symbol, so it is given a copy.
    #[staticmethod]
    #[pyo3(name = "Flag")]
    fn flag(symbol: &Symbol) -> CfgAtom {
        CfgAtom { inner: ra_ap_hir::CfgAtom::Flag(symbol.inner.clone()) }
    }
}
