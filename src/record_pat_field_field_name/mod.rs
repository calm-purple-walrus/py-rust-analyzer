use crate::types::{AstNameOrNameRef, AstRecordPatField};
use pyo3::prelude::*;

#[pymethods]
impl AstRecordPatField {
    /// `fn field_name(&self) -> Option<NameOrNameRef>`
    fn field_name(&self) -> Option<AstNameOrNameRef> {
        self.inner
            .field_name()
            .map(|inner| AstNameOrNameRef { inner })
    }
}
