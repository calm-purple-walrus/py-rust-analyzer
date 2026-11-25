use crate::types::{Local, PathResolution};
use pyo3::prelude::*;

#[pymethods]
impl PathResolution {
    /// The `PathResolution::Local` variant, read.
    #[pyo3(name = "as_Local")]
    fn as_local(&self, py: Python<'_>) -> Option<Local> {
        match self.inner {
            ra_ap_hir::PathResolution::Local(inner) => Some(Local {
                inner,
                db: self._db.clone_ref(py),
            }),
            _ => None,
        }
    }
}
