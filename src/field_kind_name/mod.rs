use crate::types::{AstFieldKind, AstNameRef};
use pyo3::prelude::*;

#[pymethods]
impl AstFieldKind {
    /// The `FieldKind::Name` variant, read.
    #[pyo3(name = "as_Name")]
    fn as_name(&self) -> Option<AstNameRef> {
        match &self.inner {
            ra_ap_syntax::ast::FieldKind::Name(inner) => Some(AstNameRef {
                inner: inner.clone(),
            }),
            _ => None,
        }
    }
}
