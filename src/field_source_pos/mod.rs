use pyo3::prelude::*;

use crate::types::{AstTupleField, FieldSource};

#[pymethods]
impl FieldSource {
    /// The variant `FieldSource::Pos(ast::TupleField)`, read: a copy of its `TupleField` if the value is that
    /// variant, otherwise `None`.
    #[pyo3(name = "as_Pos")]
    fn as_pos(&self) -> Option<AstTupleField> {
        match self.inner {
            ra_ap_hir::FieldSource::Pos(inner) => Some(AstTupleField { inner: inner.clone() }),
            _ => None,
        }
    }
}
