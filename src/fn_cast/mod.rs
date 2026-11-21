use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

use crate::held::Held;
use crate::types::{AstFn, SyntaxNode};

#[pymethods]
impl AstFn {
    /// `impl AstNode for Fn`: `fn cast(syntax: SyntaxNode) -> Option<Self>`. It takes the node, so it
    /// is given a copy.
    #[staticmethod]
    fn cast(syntax: &SyntaxNode) -> Option<AstFn> {
        ra_ap_syntax::ast::Fn::cast((*syntax.inner).clone()).map(|inner| AstFn { inner: Held::Own(inner) })
    }
}
