use pyo3::prelude::*;

use crate::types::{AstKeyValueMeta, AstPath};

#[pymethods]
impl AstKeyValueMeta {
    /// `pub fn path(&self) -> Option<Path>`
    fn path(&self) -> Option<AstPath> {
        self.inner.path().map(|inner| AstPath { inner })
    }
}
