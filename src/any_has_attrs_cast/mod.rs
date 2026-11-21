use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

use crate::types::{AnyHasAttrs, SyntaxNode};

#[pymethods]
impl AnyHasAttrs {
    /// `impl AstNode for AnyHasAttrs`: `fn cast(syntax: SyntaxNode) -> Option<Self>`. It takes the
    /// node, so it is given a copy.
    #[staticmethod]
    fn cast(syntax: &SyntaxNode) -> Option<AnyHasAttrs> {
        ra_ap_syntax::ast::AnyHasAttrs::cast((*syntax.inner).clone()).map(|inner| AnyHasAttrs { inner })
    }
}
