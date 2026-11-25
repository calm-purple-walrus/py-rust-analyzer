use pyo3::prelude::*;

use crate::types::{AstMethodCallExpr, AstNameRef};

#[pymethods]
impl AstMethodCallExpr {
    /// `pub fn name_ref(&self) -> Option<NameRef>`
    fn name_ref(&self) -> Option<AstNameRef> {
        self.inner.name_ref().map(|inner| AstNameRef { inner })
    }
}
