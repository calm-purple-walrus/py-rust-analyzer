use pyo3::prelude::*;

use crate::types::TargetKind;

#[pymethods]
impl TargetKind {
    /// The variant `TargetKind::Lib { is_proc_macro: bool }`, read: its `is_proc_macro` if the value is that
    /// variant, otherwise `None`.
    #[pyo3(name = "as_Lib")]
    fn as_lib(&self) -> Option<bool> {
        match self.inner {
            ra_ap_project_model::TargetKind::Lib { is_proc_macro } => Some(is_proc_macro),
            _ => None,
        }
    }
}
