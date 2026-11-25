use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

use crate::held::Held;
use crate::types::{AstTrait, SyntaxNode};

#[pymethods]
impl AstTrait {
    /// `impl AstNode for Trait`: `fn cast(syntax: SyntaxNode) -> Option<Self>`. It takes the node, so it is
    /// given a copy.
    #[staticmethod]
    fn cast(syntax: &SyntaxNode) -> Option<AstTrait> {
        ra_ap_syntax::ast::Trait::cast((*syntax.inner).clone()).map(|inner| AstTrait { inner: Held::Own(inner) })
    }
}
