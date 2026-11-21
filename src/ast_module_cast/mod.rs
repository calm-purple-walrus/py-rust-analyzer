use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

use crate::held::Held;
use crate::types::{AstModule, SyntaxNode};

#[pymethods]
impl AstModule {
    /// `impl AstNode for Module`: `fn cast(syntax: SyntaxNode) -> Option<Self>`. It takes the node, so it is
    /// given a copy.
    #[staticmethod]
    fn cast(syntax: &SyntaxNode) -> Option<AstModule> {
        ra_ap_syntax::ast::Module::cast((*syntax.inner).clone()).map(|inner| AstModule { inner: Held::Own(inner) })
    }
}
