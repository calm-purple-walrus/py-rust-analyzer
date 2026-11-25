use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

use crate::types::{AstMacroRules, SyntaxKind};

#[pymethods]
impl AstMacroRules {
    /// `impl AstNode for MacroRules`: `fn can_cast(kind: SyntaxKind) -> bool`
    #[staticmethod]
    fn can_cast(kind: &SyntaxKind) -> bool {
        ra_ap_syntax::ast::MacroRules::can_cast(kind.inner)
    }
}
