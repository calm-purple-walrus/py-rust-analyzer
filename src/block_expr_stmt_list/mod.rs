use pyo3::prelude::*;

use crate::types::{AstBlockExpr, AstStmtList};

#[pymethods]
impl AstBlockExpr {
    /// `pub fn stmt_list(&self) -> Option<StmtList>`
    fn stmt_list(&self) -> Option<AstStmtList> {
        self.inner.stmt_list().map(|inner| AstStmtList { inner })
    }
}
