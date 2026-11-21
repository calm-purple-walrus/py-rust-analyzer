use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

use crate::types::{AstVariantList, SyntaxNode};

#[pymethods]
impl AstVariantList {
    /// `impl AstNode for VariantList`: `fn cast(syntax: SyntaxNode) -> Option<Self>`. It takes the node, so it is
    /// given a copy.
    #[staticmethod]
    fn cast(syntax: &SyntaxNode) -> Option<AstVariantList> {
        ra_ap_syntax::ast::VariantList::cast((*syntax.inner).clone()).map(|inner| AstVariantList { inner })
    }
}
