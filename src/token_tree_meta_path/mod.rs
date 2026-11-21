use pyo3::prelude::*;

use crate::types::{AstTokenTreeMeta, AstPath};

#[pymethods]
impl AstTokenTreeMeta {
    /// `pub fn path(&self) -> Option<Path>`
    fn path(&self) -> Option<AstPath> {
        self.inner.path().map(|inner| AstPath { inner })
    }
}
