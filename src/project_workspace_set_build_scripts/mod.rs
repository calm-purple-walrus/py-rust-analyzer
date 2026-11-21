use pyo3::prelude::*;

use crate::types::{ProjectWorkspace, WorkspaceBuildScripts};

#[pymethods]
impl ProjectWorkspace {
    /// `pub fn set_build_scripts(&mut self, bs: WorkspaceBuildScripts)`. It takes the value, so it is
    /// given a copy.
    fn set_build_scripts(&mut self, bs: &WorkspaceBuildScripts) {
        self.inner.set_build_scripts(bs.inner.clone());
    }
}
