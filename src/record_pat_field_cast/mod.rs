use crate::types::{AstRecordPatField, SyntaxNode};
use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

#[pymethods]
impl AstRecordPatField {
    /// `AstNode::cast(syntax: SyntaxNode) -> Option<RecordPatField>`. It takes a copy of the node.
    #[staticmethod]
    fn cast(syntax: &SyntaxNode) -> Option<Self> {
        ra_ap_syntax::ast::RecordPatField::cast((*syntax.inner).clone())
            .map(|inner| AstRecordPatField { inner })
    }
}
