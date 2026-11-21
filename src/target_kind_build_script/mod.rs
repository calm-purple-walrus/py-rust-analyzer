use pyo3::prelude::*;

use crate::types::TargetKind;

#[pymethods]
impl TargetKind {
    /// The variant `TargetKind::BuildScript`, read: whether the value is that variant.
    #[pyo3(name = "as_BuildScript")]
    fn as_build_script(&self) -> bool {
        matches!(self.inner, ra_ap_project_model::TargetKind::BuildScript)
    }
}
