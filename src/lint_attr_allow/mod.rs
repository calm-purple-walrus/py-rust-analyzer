use pyo3::prelude::*;

use crate::types::LintAttr;

#[pymethods]
impl LintAttr {
    /// The variant `LintAttr::Allow`, read: whether the value is that variant, which holds nothing.
    #[pyo3(name = "as_Allow")]
    fn as_allow(&self) -> bool {
        matches!(self.inner, ra_ap_hir::LintAttr::Allow)
    }
}
