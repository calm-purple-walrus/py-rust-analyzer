use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

use crate::types::{AstAssocItemList, SyntaxNode};

#[pymethods]
impl AstAssocItemList {
    /// `impl AstNode for AssocItemList`: `fn cast(syntax: SyntaxNode) -> Option<Self>`. It takes the node, so it is
    /// given a copy.
    #[staticmethod]
    fn cast(syntax: &SyntaxNode) -> Option<AstAssocItemList> {
        ra_ap_syntax::ast::AssocItemList::cast((*syntax.inner).clone()).map(|inner| AstAssocItemList { inner })
    }
}
