use crate::types::{AstArrayExprKind, AstExpr};
use pyo3::prelude::*;

#[pymethods]
impl AstArrayExprKind {
    /// The `ArrayExprKind::Repeat` variant, read.
    #[pyo3(name = "as_Repeat")]
    fn as_repeat(&self) -> Option<(Option<AstExpr>, Option<AstExpr>)> {
        match &self.inner {
            ra_ap_syntax::ast::ArrayExprKind::Repeat {
                initializer,
                repeat,
            } => Some((
                initializer.clone().map(|inner| AstExpr { inner }),
                repeat.clone().map(|inner| AstExpr { inner }),
            )),
            _ => None,
        }
    }
}
