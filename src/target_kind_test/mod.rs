use pyo3::prelude::*;

use crate::types::TargetKind;

#[pymethods]
impl TargetKind {
    /// The variant `TargetKind::Test`, read: whether the value is that variant.
    #[pyo3(name = "as_Test")]
    fn as_test(&self) -> bool {
        matches!(self.inner, ra_ap_project_model::TargetKind::Test)
    }
}
