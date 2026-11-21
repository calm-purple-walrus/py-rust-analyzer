use pyo3::prelude::*;

use crate::types::{TargetData, TargetKind};

#[pymethods]
impl TargetData {
    /// `pub kind: TargetKind`
    #[getter]
    fn kind(&self) -> TargetKind {
        TargetKind { inner: self.inner.kind }
    }
}
