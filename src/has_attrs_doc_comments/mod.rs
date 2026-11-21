use pyo3::prelude::*;
use ra_ap_syntax::ast::HasAttrs;

use crate::iter::Iter;
use crate::types::{AstDocComment, AstRecordField, AstVariant};

#[pymethods]
impl AstRecordField {
    /// `HasAttrs::doc_comments`: `fn doc_comments(&self) -> AstChildren<ast::DocComment>`
    fn doc_comments(&self) -> Iter {
        Iter::new(self.inner.doc_comments(), |py, inner| Ok(Py::new(py, AstDocComment { inner })?.into_any()), None)
    }
}

#[pymethods]
impl AstVariant {
    /// `HasAttrs::doc_comments`: `fn doc_comments(&self) -> AstChildren<ast::DocComment>`
    fn doc_comments(&self) -> Iter {
        Iter::new(self.inner.doc_comments(), |py, inner| Ok(Py::new(py, AstDocComment { inner })?.into_any()), None)
    }
}
