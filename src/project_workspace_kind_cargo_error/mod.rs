use pyo3::prelude::*;
use ra_ap_project_model::ProjectWorkspaceKind as Kind;

use crate::types::ProjectWorkspaceKindCargo;

#[pymethods]
impl ProjectWorkspaceKindCargo {
    /// The field `error: Option<Arc<anyhow::Error>>` of `ProjectWorkspaceKind::Cargo`, as the text of the error
    /// with its causes.
    #[getter]
    fn error(&self) -> Option<String> {
        let Kind::Cargo { error, .. } = self.kind else { unreachable!("made only for the Cargo variant") };
        error.as_ref().map(|error| format!("{error:#}"))
    }
}
