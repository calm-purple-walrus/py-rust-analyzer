use crate::types::{AstBinExpr, AstBinaryOp};
use pyo3::prelude::*;

#[pymethods]
impl AstBinExpr {
    /// `fn op_kind(&self) -> Option<BinaryOp>`
    fn op_kind(&self) -> Option<AstBinaryOp> {
        self.inner.op_kind().map(|inner| AstBinaryOp { inner })
    }
}
