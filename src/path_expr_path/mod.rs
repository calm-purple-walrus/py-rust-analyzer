use pyo3::prelude::*;

use crate::types::{AstPathExpr, AstPath};

#[pymethods]
impl AstPathExpr {
    /// `pub fn path(&self) -> Option<Path>`
    fn path(&self) -> Option<AstPath> {
        self.inner.path().map(|inner| AstPath { inner })
    }
}
