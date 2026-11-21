use pyo3::prelude::*;

use crate::types::StructKind;

#[pymethods]
impl StructKind {
    /// The variant `StructKind::Tuple`, read: whether the value is that variant, which holds nothing.
    #[pyo3(name = "as_Tuple")]
    fn as_tuple(&self) -> bool {
        matches!(self.inner, ra_ap_hir::StructKind::Tuple)
    }
}
