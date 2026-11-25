use pyo3::prelude::*;

use crate::types::{ProjectWorkspace, Sysroot};

#[pymethods]
impl ProjectWorkspace {
    /// `pub sysroot: Sysroot`. Copied, because `set_build_scripts` changes the `ProjectWorkspace`, so it cannot
    /// lend its parts.
    #[getter]
    fn sysroot(&self) -> Sysroot {
        Sysroot { inner: self.inner.sysroot.clone() }
    }
}
