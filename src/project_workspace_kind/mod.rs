use pyo3::prelude::*;

use crate::types::{ProjectWorkspace, ProjectWorkspaceKind};

#[pymethods]
impl ProjectWorkspace {
    /// `pub kind: ProjectWorkspaceKind`. Copied, because `set_build_scripts` changes the
    /// `ProjectWorkspace`, so it cannot lend its parts.
    #[getter]
    fn kind(&self) -> ProjectWorkspaceKind {
        ProjectWorkspaceKind { inner: self.inner.kind.clone() }
    }
}
