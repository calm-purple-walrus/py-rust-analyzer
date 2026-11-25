use crate::types::AstBinaryOp;
use pyo3::prelude::*;

#[pymethods]
impl AstBinaryOp {
    /// `impl Display for BinaryOp`
    fn __str__(&self) -> String {
        self.inner.to_string()
    }
}
