use pyo3::prelude::*;

use crate::types::{AstAwaitExpr, AstExpr};

#[pymethods]
impl AstAwaitExpr {
    /// `pub fn expr(&self) -> Option<Expr>`
    fn expr(&self) -> Option<AstExpr> {
        self.inner.expr().map(|inner| AstExpr { inner })
    }
}
