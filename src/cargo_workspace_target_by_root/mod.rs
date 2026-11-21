use pyo3::prelude::*;

use crate::types::{AbsPath, CargoWorkspace, Target};

#[pymethods]
impl CargoWorkspace {
    /// `pub fn target_by_root(&self, root: &AbsPath) -> Option<Target>`
    fn target_by_root(&self, root: &AbsPath) -> Option<Target> {
        self.inner.target_by_root(&root.inner).map(|inner| Target { inner })
    }
}
