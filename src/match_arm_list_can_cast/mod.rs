use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

use crate::types::{AstMatchArmList, SyntaxKind};

#[pymethods]
impl AstMatchArmList {
    /// `impl AstNode for MatchArmList`: `fn can_cast(kind: SyntaxKind) -> bool`
    #[staticmethod]
    fn can_cast(kind: &SyntaxKind) -> bool {
        ra_ap_syntax::ast::MatchArmList::can_cast(kind.inner)
    }
}
