use pyo3::prelude::*;

use crate::types::{AstRecordField, FieldSource};

#[pymethods]
impl FieldSource {
    /// The variant `FieldSource::Named(ast::RecordField)`, read: a copy of its `RecordField` if the value is that
    /// variant, otherwise `None`.
    #[pyo3(name = "as_Named")]
    fn as_named(&self) -> Option<AstRecordField> {
        match self.inner {
            ra_ap_hir::FieldSource::Named(inner) => Some(AstRecordField { inner: inner.clone() }),
            _ => None,
        }
    }
}
