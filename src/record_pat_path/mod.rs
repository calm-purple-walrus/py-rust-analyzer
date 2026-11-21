use pyo3::prelude::*;

use crate::types::{AstRecordPat, AstPath};

#[pymethods]
impl AstRecordPat {
    /// `pub fn path(&self) -> Option<Path>`
    fn path(&self) -> Option<AstPath> {
        self.inner.path().map(|inner| AstPath { inner })
    }
}
