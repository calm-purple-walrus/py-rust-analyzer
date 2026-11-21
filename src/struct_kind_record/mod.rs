use pyo3::prelude::*;

use crate::types::StructKind;

#[pymethods]
impl StructKind {
    /// The variant `StructKind::Record`, read: whether the value is that variant, which holds nothing.
    #[pyo3(name = "as_Record")]
    fn as_record(&self) -> bool {
        matches!(self.inner, ra_ap_hir::StructKind::Record)
    }
}
