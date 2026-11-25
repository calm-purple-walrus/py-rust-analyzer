use pyo3::prelude::*;

use crate::types::TargetKind;

#[pymethods]
impl TargetKind {
    /// The variant `TargetKind::Bin`, read: whether the value is that variant.
    #[pyo3(name = "as_Bin")]
    fn as_bin(&self) -> bool {
        matches!(self.inner, ra_ap_project_model::TargetKind::Bin)
    }
}
