use pyo3::prelude::*;

use crate::types::ProcMacroServerChoice;

#[pymethods]
impl ProcMacroServerChoice {
    /// The variant `ProcMacroServerChoice::None`, made from Python. Python cannot name a method `None`,
    /// so it is `None_`.
    #[staticmethod]
    #[pyo3(name = "None_")]
    fn none() -> ProcMacroServerChoice {
        ProcMacroServerChoice { inner: ra_ap_load_cargo::ProcMacroServerChoice::None }
    }
}
