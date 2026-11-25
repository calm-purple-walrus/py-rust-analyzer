use crate::types::{AstElseBranch, AstIfExpr};
use pyo3::prelude::*;

#[pymethods]
impl AstIfExpr {
    /// `fn else_branch(&self) -> Option<ElseBranch>`
    fn else_branch(&self) -> Option<AstElseBranch> {
        self.inner
            .else_branch()
            .map(|inner| AstElseBranch { inner })
    }
}
