use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

use crate::types::{AstItemList, SyntaxNode};

#[pymethods]
impl AstItemList {
    /// `impl AstNode for ItemList`: `fn cast(syntax: SyntaxNode) -> Option<Self>`. It takes the node, so it is
    /// given a copy.
    #[staticmethod]
    fn cast(syntax: &SyntaxNode) -> Option<AstItemList> {
        ra_ap_syntax::ast::ItemList::cast((*syntax.inner).clone()).map(|inner| AstItemList { inner })
    }
}
