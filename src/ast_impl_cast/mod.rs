use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

use crate::held::Held;
use crate::types::{AstImpl, SyntaxNode};

#[pymethods]
impl AstImpl {
    /// `impl AstNode for Impl`: `fn cast(syntax: SyntaxNode) -> Option<Self>`. It takes the node, so it is
    /// given a copy.
    #[staticmethod]
    fn cast(syntax: &SyntaxNode) -> Option<AstImpl> {
        ra_ap_syntax::ast::Impl::cast((*syntax.inner).clone()).map(|inner| AstImpl { inner: Held::Own(inner) })
    }
}
