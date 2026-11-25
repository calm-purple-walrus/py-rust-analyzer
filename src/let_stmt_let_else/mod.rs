use crate::types::{AstLetElse, AstLetStmt};
use pyo3::prelude::*;

#[pymethods]
impl AstLetStmt {
    /// `fn let_else(&self) -> Option<LetElse>`
    fn let_else(&self) -> Option<AstLetElse> {
        self.inner.let_else().map(|inner| AstLetElse { inner })
    }
}
