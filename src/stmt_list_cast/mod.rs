use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

use crate::types::{AstStmtList, SyntaxNode};

#[pymethods]
impl AstStmtList {
    /// `impl AstNode for StmtList`: `fn cast(syntax: SyntaxNode) -> Option<Self>`. It takes the node, so it is
    /// given a copy.
    #[staticmethod]
    fn cast(syntax: &SyntaxNode) -> Option<AstStmtList> {
        ra_ap_syntax::ast::StmtList::cast((*syntax.inner).clone()).map(|inner| AstStmtList { inner })
    }
}
