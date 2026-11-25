use crate::types::{AstFieldExpr, AstFieldKind};
use pyo3::prelude::*;

#[pymethods]
impl AstFieldExpr {
    /// `fn field_access(&self) -> Option<FieldKind>`
    fn field_access(&self) -> Option<AstFieldKind> {
        self.inner
            .field_access()
            .map(|inner| AstFieldKind { inner })
    }
}
