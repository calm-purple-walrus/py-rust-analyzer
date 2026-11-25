use pyo3::prelude::*;

use crate::types::Symbol;

#[pymethods]
impl Symbol {
    /// `pub fn intern(s: &str) -> Self`
    #[staticmethod]
    fn intern(s: &str) -> Symbol {
        Symbol { inner: ra_ap_hir::Symbol::intern(s) }
    }
}
