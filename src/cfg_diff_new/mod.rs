use pyo3::prelude::*;

use crate::types::{CfgAtom, CfgDiff};

#[pymethods]
impl CfgDiff {
    /// `pub fn new(enable: Vec<CfgAtom>, disable: Vec<CfgAtom>) -> CfgDiff`. It takes the atoms, so it is given
    /// copies.
    #[staticmethod]
    fn new(enable: Vec<PyRef<'_, CfgAtom>>, disable: Vec<PyRef<'_, CfgAtom>>) -> CfgDiff {
        let copies = |atoms: Vec<PyRef<'_, CfgAtom>>| atoms.iter().map(|atom| atom.inner.clone()).collect();
        CfgDiff { inner: ra_ap_cfg::CfgDiff::new(copies(enable), copies(disable)) }
    }
}
