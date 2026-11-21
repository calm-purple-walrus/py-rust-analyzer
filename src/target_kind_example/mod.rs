use pyo3::prelude::*;

use crate::types::TargetKind;

#[pymethods]
impl TargetKind {
    /// The variant `TargetKind::Example`, read: whether the value is that variant.
    #[pyo3(name = "as_Example")]
    fn as_example(&self) -> bool {
        matches!(self.inner, ra_ap_project_model::TargetKind::Example)
    }
}
