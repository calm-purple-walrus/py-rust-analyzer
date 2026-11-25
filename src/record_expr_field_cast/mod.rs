use crate::types::{AstRecordExprField, SyntaxNode};
use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

#[pymethods]
impl AstRecordExprField {
    /// `AstNode::cast(syntax: SyntaxNode) -> Option<RecordExprField>`. It takes a copy of the node.
    #[staticmethod]
    fn cast(syntax: &SyntaxNode) -> Option<Self> {
        ra_ap_syntax::ast::RecordExprField::cast((*syntax.inner).clone())
            .map(|inner| AstRecordExprField { inner })
    }
}
