use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

use crate::held::Held;
use crate::types::{AstAdt, SyntaxNode};

#[pymethods]
impl AstAdt {
    /// `impl AstNode for Adt`: `fn cast(syntax: SyntaxNode) -> Option<Self>`. It takes the node, so it is given a copy.
    #[staticmethod]
    fn cast(syntax: &SyntaxNode) -> Option<AstAdt> {
        ra_ap_syntax::ast::Adt::cast((*syntax.inner).clone()).map(|inner| AstAdt { inner: Held::Own(inner) })
    }
}
