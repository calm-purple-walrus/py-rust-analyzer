use pyo3::prelude::*;

use crate::types::ProcMacroServerChoice;

#[pymethods]
impl ProcMacroServerChoice {
    /// The variant `ProcMacroServerChoice::Sysroot`, made from Python.
    #[staticmethod]
    #[pyo3(name = "Sysroot")]
    fn sysroot() -> ProcMacroServerChoice {
        ProcMacroServerChoice { inner: ra_ap_load_cargo::ProcMacroServerChoice::Sysroot }
    }
}
