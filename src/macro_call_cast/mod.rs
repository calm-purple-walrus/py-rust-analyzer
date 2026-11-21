use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

use crate::types::{AstMacroCall, SyntaxNode};

#[pymethods]
impl AstMacroCall {
    /// `impl AstNode for MacroCall`: `fn cast(syntax: SyntaxNode) -> Option<Self>`. It takes the node, so
    /// it is given a copy.
    #[staticmethod]
    fn cast(syntax: &SyntaxNode) -> Option<AstMacroCall> {
        ra_ap_syntax::ast::MacroCall::cast((*syntax.inner).clone()).map(|inner| AstMacroCall { inner })
    }
}
