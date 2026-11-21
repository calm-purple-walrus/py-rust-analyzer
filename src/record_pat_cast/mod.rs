use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

use crate::types::{AstRecordPat, SyntaxNode};

#[pymethods]
impl AstRecordPat {
    /// `impl AstNode for RecordPat`: `fn cast(syntax: SyntaxNode) -> Option<Self>`. It takes the node, so it is
    /// given a copy.
    #[staticmethod]
    fn cast(syntax: &SyntaxNode) -> Option<AstRecordPat> {
        ra_ap_syntax::ast::RecordPat::cast((*syntax.inner).clone()).map(|inner| AstRecordPat { inner })
    }
}
