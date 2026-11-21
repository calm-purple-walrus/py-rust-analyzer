use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

use crate::types::{AstRecordFieldList, SyntaxNode};

#[pymethods]
impl AstRecordFieldList {
    /// `impl AstNode for RecordFieldList`: `fn cast(syntax: SyntaxNode) -> Option<Self>`. It takes the node, so it is
    /// given a copy.
    #[staticmethod]
    fn cast(syntax: &SyntaxNode) -> Option<AstRecordFieldList> {
        ra_ap_syntax::ast::RecordFieldList::cast((*syntax.inner).clone()).map(|inner| AstRecordFieldList { inner })
    }
}
