use pyo3::prelude::*;

use crate::types::TargetKind;

#[pymethods]
impl TargetKind {
    /// The variant `TargetKind::Bench`, read: whether the value is that variant.
    #[pyo3(name = "as_Bench")]
    fn as_bench(&self) -> bool {
        matches!(self.inner, ra_ap_project_model::TargetKind::Bench)
    }
}
