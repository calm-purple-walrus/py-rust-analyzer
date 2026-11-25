use pyo3::prelude::*;

use crate::types::SyntaxKind;

#[pymethods]
impl SyntaxKind {
    /// The variant `SyntaxKind::COMMA`, made from Python.
    #[staticmethod]
    #[pyo3(name = "COMMA")]
    fn comma() -> SyntaxKind {
        SyntaxKind { inner: ra_ap_syntax::SyntaxKind::COMMA }
    }
}
