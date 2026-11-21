use pyo3::prelude::*;

use crate::types::SyntaxKind;

#[pymethods]
impl SyntaxKind {
    /// The variant `SyntaxKind::WHITESPACE`, made from Python.
    #[staticmethod]
    #[pyo3(name = "WHITESPACE")]
    fn whitespace() -> SyntaxKind {
        SyntaxKind { inner: ra_ap_syntax::SyntaxKind::WHITESPACE }
    }
}
