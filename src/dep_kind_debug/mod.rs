use pyo3::prelude::*;

use crate::types::DepKind;

#[pymethods]
impl DepKind {
    /// `impl Debug for DepKind`. The variants cannot be read, because the type cannot be named.
    fn __repr__(&self) -> String {
        format!("{:?}", self.dependency.kind)
    }
}
