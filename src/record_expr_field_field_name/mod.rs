use crate::types::{AstNameRef, AstRecordExprField};
use pyo3::prelude::*;

#[pymethods]
impl AstRecordExprField {
    /// `fn field_name(&self) -> Option<NameRef>`
    fn field_name(&self) -> Option<AstNameRef> {
        self.inner.field_name().map(|inner| AstNameRef { inner })
    }
}
