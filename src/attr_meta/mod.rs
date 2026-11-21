use pyo3::prelude::*;

use crate::types::{AstAttr, AstMeta};

#[pymethods]
impl AstAttr {
    /// `pub fn meta(&self) -> Option<Meta>`
    fn meta(&self) -> Option<AstMeta> {
        self.inner.meta().map(|inner| AstMeta { inner })
    }
}
