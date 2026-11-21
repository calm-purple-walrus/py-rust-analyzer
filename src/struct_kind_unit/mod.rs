use pyo3::prelude::*;

use crate::types::StructKind;

#[pymethods]
impl StructKind {
    /// The variant `StructKind::Unit`, read: whether the value is that variant, which holds nothing.
    #[pyo3(name = "as_Unit")]
    fn as_unit(&self) -> bool {
        matches!(self.inner, ra_ap_hir::StructKind::Unit)
    }
}
