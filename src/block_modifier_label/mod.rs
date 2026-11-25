use crate::types::{AstBlockModifier, AstLabel};
use pyo3::prelude::*;

#[pymethods]
impl AstBlockModifier {
    /// The `BlockModifier::Label` variant, read.
    #[pyo3(name = "as_Label")]
    fn as_label(&self) -> Option<AstLabel> {
        match &self.inner {
            ra_ap_syntax::ast::BlockModifier::Label(inner) => Some(AstLabel {
                inner: inner.clone(),
            }),
            _ => None,
        }
    }
}
