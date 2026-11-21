use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

use crate::types::{AstArgList, SyntaxKind};

#[pymethods]
impl AstArgList {
    /// `impl AstNode for ArgList`: `fn can_cast(kind: SyntaxKind) -> bool`
    #[staticmethod]
    fn can_cast(kind: &SyntaxKind) -> bool {
        ra_ap_syntax::ast::ArgList::can_cast(kind.inner)
    }
}
