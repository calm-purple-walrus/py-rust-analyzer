use pyo3::prelude::*;
use ra_ap_project_model::ProjectWorkspaceKind as Kind;

use crate::held::{Held, owner};
use crate::types::{CargoWorkspace, ProjectWorkspaceKindCargo};

#[pymethods]
impl ProjectWorkspaceKindCargo {
    /// The field `cargo: CargoWorkspace` of `ProjectWorkspaceKind::Cargo`, lent by this variant.
    #[getter]
    fn cargo(slf: &Bound<'_, Self>) -> CargoWorkspace {
        let this = slf.borrow();
        let Kind::Cargo { cargo, .. } = this.kind else { unreachable!("made only for the Cargo variant") };
        // SAFETY: the workspace lives in the object this variant keeps alive, and no binding changes it.
        CargoWorkspace { inner: unsafe { Held::lent(cargo, owner(slf)) } }
    }
}
