use pyo3::prelude::*;
use ra_ap_project_model::ProjectWorkspaceKind as Kind;

use crate::held::{extend, owner};
use crate::types::{ProjectWorkspaceKind, ProjectWorkspaceKindCargo};

#[pymethods]
impl ProjectWorkspaceKind {
    /// The variant `ProjectWorkspaceKind::Cargo { .. }`, read: its fields, lent by this
    /// `ProjectWorkspaceKind`, if the value is that variant, otherwise `None`.
    #[pyo3(name = "as_Cargo")]
    fn as_cargo(slf: &Bound<'_, Self>) -> Option<ProjectWorkspaceKindCargo> {
        let this = slf.borrow();
        // SAFETY: the value lives in this object, and no binding changes a `ProjectWorkspaceKind`.
        let kind = unsafe { extend(&this.inner) };
        matches!(kind, Kind::Cargo { .. }).then(|| ProjectWorkspaceKindCargo { kind, _owner: owner(slf) })
    }
}
