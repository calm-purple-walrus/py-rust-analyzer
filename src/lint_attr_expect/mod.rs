use pyo3::prelude::*;

use crate::types::LintAttr;

#[pymethods]
impl LintAttr {
    /// The variant `LintAttr::Expect`, read: whether the value is that variant, which holds nothing.
    #[pyo3(name = "as_Expect")]
    fn as_expect(&self) -> bool {
        matches!(self.inner, ra_ap_hir::LintAttr::Expect)
    }
}
